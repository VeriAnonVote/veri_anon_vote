use veri_anon_vote_server::{
    migrations,
    prelude::*,
};



struct ServerContext {
    verifier_map: VerifierMap,
    organizer_config: OrganizerConfig,
    registrar_config: RegistrarConfig,
    db_pool: DbPool,
    pub_ring: PubRing,
    election_closed: ElectionCloseStatus,
    registration_closed: RegistrationCloseStatus,
}


#[actix_web::main]
async fn main() -> AResult<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
            .with_default_directive(LevelFilter::INFO.into())
            .from_env_lossy(),
        )
        // .without_time()
        .init();

    let args: Vec<String> = env::args().collect();

    let default_config_path = "./crates/veri_anon_vote_server/config.toml".to_string();
    let file_path = args.get(1).unwrap_or(&default_config_path);
    let registrar_config = voter_registrar::config::Config::from_path(file_path)?;
    let organizer_config = election_shared::config::Config::from_path(file_path)?;

    let db_pool = get_db_connection_pool(&registrar_config.sqlite3_file_path);


    // let client = OnionClient::default()?;
    let pub_ring = fetch_pub_ring(&db_pool).await
        .unwrap_or(Vec::new());

    let verifier_map = VerifierMap::new_arc();
    verifier_map.refresh(&db_pool)?;

    let election_closed = Arc::new(AtomicBool::new(false));
    let registration_closed = Arc::new(AtomicBool::new(false));

    let context = Arc::new(ServerContext{
        registrar_config,
        organizer_config,
        db_pool,
        verifier_map,
        pub_ring,
        registration_closed,
        election_closed,
    });

    tokio::try_join!(
        context.run_admin_server(),
        context.run_verifier_server(),
        context.run_public_server(),
    )?;

    Ok(())
}



impl ServerContext {
    async fn run_admin_server(
        self: &Arc<Self>
    ) -> AResult<()> {
        let this = self.clone();

        let mut admin_server = HttpServer::new(move || {
            let admin_auth = HttpAuthentication::with_fn(admin_validator);
            actix_web::App::new()
                .wrap(Logger::default())
                .app_data(Data::new(this.db_pool.clone()))
                .app_data(Data::new(this.registrar_config.clone()))
                .app_data(Data::new(this.organizer_config.clone()))
                .app_data(Data::new(this.election_closed.clone()))
                .app_data(Data::new(this.verifier_map.clone()))
                .app_data(Data::new(this.registration_closed.clone()))
                .service(
                    web::scope("/admin")
                    .wrap(admin_auth)
                    .service(upsert_verifier)
                    .service(delete_verifier)
                    .service(toggle_registration_status)
                    .service(get_all_verifiers_details)
                    .service(toggle_election_status)
                )
                .service(SwaggerUi::new("/swagger-ui/admin/{_:.*}").urls(vec![
                        (
                            SwagUrl::new("admin", "/api-docs/admin.json"),
                            AdminDoc::openapi(),
                        ),
                ]))
        });

        admin_server = admin_server.workers(2);

        for addr in &self.registrar_config.admin.ports {
            admin_server = admin_server.bind(addr)?;
        }
        for socket_path in &self.registrar_config.admin.sockets {
            admin_server = admin_server.bind_uds(Path::new(&socket_path))?;
        }

        admin_server.run()
            .await?;
        Ok(())
    }



    async fn run_verifier_server(
        self: &Arc<Self>
    ) -> AResult<()> {
        let this = self.clone();

        let mut verifier_server = HttpServer::new(move || {
            let verifier_auth = HttpAuthentication::with_fn(verifier_validator);
            actix_web::App::new()
                .wrap(verifier_auth)
                .wrap(Logger::default())
                .app_data(Data::new(this.db_pool.clone()))
                .app_data(Data::new(this.verifier_map.clone()))
                .app_data(Data::new(this.registrar_config.clone()))
                .app_data(Data::new(this.organizer_config.clone()))
                .app_data(Data::new(this.registration_closed.clone()))
                .service(
                    web::scope("/verifier")
                    .service(insert_voter)
                )
        });

        verifier_server = verifier_server.workers(2);

        for addr in &self.registrar_config.verifier.ports {
            verifier_server = verifier_server.bind(addr)?;
        }
        for socket_path in &self.registrar_config.verifier.sockets {
            verifier_server = verifier_server.bind_uds(Path::new(&socket_path))?;
        }

        verifier_server.run()
            .await?;
        Ok(())
    }



    async fn run_public_server(
        self: &Arc<Self>
    ) -> AResult<()> {
        let this = self.clone();

        let mut public_server = HttpServer::new(move || {
            let cors = Cors::permissive();

            actix_web::App::new()
                .wrap(Logger::default())
                .wrap(cors)
                .app_data(Data::new(this.verifier_map.clone()))
                .app_data(Data::new(this.registration_closed.clone()))
                .app_data(Data::new(this.registrar_config.clone()))
                .app_data(Data::new(this.db_pool.clone()))

                .app_data(Data::new(this.organizer_config.clone()))
                .app_data(Data::new(this.pub_ring.clone()))
                .app_data(Data::new(this.election_closed.clone()))
                .service(get_all_verifiers)
                .service(get_all_voters)
                .service(get_one_voter)
                .service(get_registration_closed_status)
                .service(get_voter_requirements)

                .service(get_pub_ring)
                .service(insert_vote_record)
                .service(get_one_vote_record)
                .service(get_all_vote_records)
                .service(get_vote_requirements)
                .service(
                    actix_files::Files::new("/", "dx/voter_web/release/web/public")
                    // actix_files::Files::new("/testapp", "dx/voter_web/release/web/public")
                    // .show_files_listing()
                    // .redirect_to_slash_directory()
                )
        });

        for addr in &self.registrar_config.public.ports {
            public_server = public_server.bind(addr)?;
        }
        for socket_path in &self.registrar_config.public.sockets {
            public_server = public_server.bind_uds(Path::new(&socket_path))?;
        }

        public_server.run()
            .await?;
        Ok(())
    }
}



async fn admin_validator(
    req: ServiceRequest,
    credentials: Option<BearerAuth>,
) -> Result<ServiceRequest, (error::Error, ServiceRequest)> {
    let Some(credentials) = credentials else {
        return Err((error::ErrorUnauthorized("no bearer header"), req));
    };
    let config = req.app_data::<Data<RegistrarConfig>>().unwrap();

    if config.admin_key != credentials.token() {
        return Err((error::ErrorUnauthorized("token is wrong"), req));
    }

    Ok(req)
}


#[instrument]
async fn verifier_validator(
    req: ServiceRequest,
    credentials: Option<BearerAuth>,
) -> Result<ServiceRequest, (error::Error, ServiceRequest)> {
    let Some(credentials) = credentials else {
        return Err((error::ErrorUnauthorized("no bearer header"), req));
    };


    let verifier_map = req.app_data::<Data<VerifierMap>>().unwrap();

    if verifier_map.get(credentials.token()).is_none() {
        debug!("{:#?}", verifier_map);
        return Err((error::ErrorUnauthorized("token is wrong"), req));
    }

    Ok(req)
}



fn get_db_connection_pool(database_url: &str) -> DbPool {
    let mut connection = SqliteConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
    migrations::run(&mut connection).unwrap();
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    // Refer to the `r2d2` documentation for more methods to use
    // when building a connection pool
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}



pub async fn fetch_pub_ring(
    db_pool: &DbPool,
) -> AResult<PubRing> {
    // let all_voters = client.get(url)
    //     .send()
    //     .await
    //     .map_err(msg)?
    //     .json::<Vec<Voter>>().await?;

    Voter::get_all(db_pool)?
        .into_iter()
        .map(|v| RistrettoPoint::from_bytes(v.voter_pubkey.as_slice()))
        .collect::<AResult<Vec<_>>>()
}
