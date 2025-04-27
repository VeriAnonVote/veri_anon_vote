use election_organizer::{
    migrations,
    handlers::{
        admin::{
            // self,
            toggle_election_status,
        },
        public::{
            insert_vote_record,
            get_all_vote_records,
            get_pub_ring,
            get_one_vote_record,
            get_vote_requirements,
        },
    },
    prelude::*,
};



struct ServerContext {
    config_arc: OrganizerConfig,
    db_pool: DbPool,
    pub_ring: PubRing,
    election_closed: ElectionCloseStatus,
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

    let default_config_path = "./crates/election_organizer/config.toml".to_string();
    let file_path = args.get(1).unwrap_or(&default_config_path);
    let config_arc = Config::from_path(file_path)?;

    let db_pool = get_db_connection_pool(&config_arc.sqlite3_file_path);


    let client = OnionClient::try_new()?;
    let pub_ring = fetch_pub_ring(&client, &config_arc.voter_registrar_url).await
        .unwrap_or(Vec::new());

    let election_closed = Arc::new(AtomicBool::new(false));
    let context = Arc::new(ServerContext{
        config_arc,
        db_pool,
        pub_ring,
        election_closed,
    });

    tokio::try_join!(
        context.run_admin_server(),
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
                .wrap(admin_auth)
                .wrap(Logger::default())
                .app_data(Data::new(this.db_pool.clone()))
                .app_data(Data::new(this.config_arc.clone()))
                .app_data(Data::new(this.election_closed.clone()))
                .service(
                    web::scope("/admin")
                    .service(toggle_election_status)
                )
        });

        admin_server = admin_server.workers(2);

        for addr in &self.config_arc.admin.ports {
            admin_server = admin_server.bind(addr)?;
        }
        for socket_path in &self.config_arc.admin.sockets {
            admin_server = admin_server.bind_uds(Path::new(&socket_path))?;
        }

        admin_server.run()
            .await?;
        Ok(())
    }



    async fn run_public_server(
        self: &Arc<Self>
    ) -> AResult<()> {
        let this = self.clone();

        let mut public_server = HttpServer::new(move || {
            // let cors = Cors::default()
            //     // .allow_any_origin()
            // // .allowed_origin("http://127.0.0.1:8080")
            // // .allowed_origin_fn(|origin, _req_head| {
            // //     origin.as_bytes().ends_with(b".onion")
            // // })
            // // .allowed_methods(vec!["GET", "POST"])
            // // .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            // // .allowed_header(http::header::CONTENT_TYPE)
            // .max_age(3600);

            let cors = Cors::permissive();

            actix_web::App::new()
                .wrap(Logger::default())
                .wrap(cors)
                .app_data(Data::new(this.db_pool.clone()))
                .app_data(Data::new(this.config_arc.clone()))
                .app_data(Data::new(this.pub_ring.clone()))
                .app_data(Data::new(this.election_closed.clone()))
                .service(get_pub_ring)
                .service(insert_vote_record)
                .service(get_one_vote_record)
                .service(get_all_vote_records)
                .service(get_vote_requirements)
        });

        for addr in &self.config_arc.public.ports {
            public_server = public_server.bind(addr)?;
        }
        for socket_path in &self.config_arc.public.sockets {
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
    let config = req.app_data::<Data<OrganizerConfig>>().unwrap();

    if config.admin_key != credentials.token() {
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
    client: &ClientWithMiddleware,
    url: &str,
) -> AResult<PubRing> {
    let all_voters = client.get(url)
        .send()
        .await
        .map_err(msg)?
        .json::<Vec<Voter>>().await?;

    all_voters.into_iter()
        .map(|v| RistrettoPoint::from_bytes(v.voter_pubkey.as_slice()))
        .collect::<AResult<Vec<_>>>()
}
