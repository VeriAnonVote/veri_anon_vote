use voter_registrar::{
    migrations,
    config::{
        // RegistrarConfig,
        Config,
    },
    // models::verifier::VerifierMap,
    handlers::{
        admin::{
            // self,
            AdminDoc,
            // manual_hello,
            upsert_verifier,
            delete_verifier,
            get_all_verifiers_details,
            toggle_registration_status,
        },
        verifier::{
            insert_voter,
            // hello,
        },
        public::{
            // echo,
            get_registration_closed_status,
            get_one_voter,
            get_all_voters,
            get_all_verifiers,
            get_voter_requirements,
        },
    },
    prelude::*,
};



struct ServerContext {
    verifier_map: VerifierMap,
    config_arc: RegistrarConfig,
    db_pool: DbPool,
    registration_closed: RegistrationCloseStatus,
}


#[actix_web::main]
async fn main() -> AResult<()> {
    tracing_subscriber::fmt()
        // .with_file(true)
        // .with_line_number(true)
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::DEBUG.into())
                .from_env_lossy(),
        )
        // .without_time()
        .init();

    let args: Vec<String> = env::args().collect();

    let default_config_path = "./crates/voter_registrar/config.toml".to_string();
    let file_path = args.get(1).unwrap_or(&default_config_path);
    let config_arc = Config::from_path(file_path)?;

    let db_pool = get_db_connection_pool(&config_arc.sqlite3_file_path);

    let verifier_map = VerifierMap::new_arc();
    verifier_map.refresh(&db_pool)?;

    let registration_closed = Arc::new(AtomicBool::new(false));
    let context = Arc::new(ServerContext{
        verifier_map,
        config_arc,
        db_pool,
        registration_closed,
    });

    tokio::try_join!(
        background_task(),
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
                .app_data(Data::new(this.config_arc.clone()))
                .app_data(Data::new(this.verifier_map.clone()))
                .app_data(Data::new(this.registration_closed.clone()))
                .service(
                    web::scope("/admin")
                    .wrap(admin_auth)
                    .service(upsert_verifier)
                    .service(delete_verifier)
                    .service(toggle_registration_status)
                    .service(get_all_verifiers_details)
                    // .configure(|cfg| {
                    //     cfg.service(SwaggerUi::new("/swagger-ui/{_:.*}").urls(vec![
                    //             (
                    //                 SwagUrl::new("admin", "/api-docs/admin.json"),
                    //                 AdminDoc::openapi(),
                    //             ),
                    //     ]));
                    // })
                )
                .service(SwaggerUi::new("/swagger-ui/admin/{_:.*}").urls(vec![
                        (
                            SwagUrl::new("admin", "/api-docs/admin.json"),
                            AdminDoc::openapi(),
                        ),
                ]))
                // .service(
                // web::scope("/admin")
                // )
                // .route("/manual_hello", web::get().to(manual_hello))
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
                .app_data(Data::new(this.config_arc.clone()))
                .app_data(Data::new(this.registration_closed.clone()))
                .service(
                    web::scope("/verifier")
                    .service(insert_voter)
                )
        });

        verifier_server = verifier_server.workers(2);

        for addr in &self.config_arc.verifier.ports {
            verifier_server = verifier_server.bind(addr)?;
        }
        for socket_path in &self.config_arc.verifier.sockets {
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
            actix_web::App::new()
                .wrap(Logger::default())
                .app_data(Data::new(this.db_pool.clone()))
                .app_data(Data::new(this.verifier_map.clone()))
                .app_data(Data::new(this.config_arc.clone()))
                .app_data(Data::new(this.registration_closed.clone()))
                // .service(echo)
                .service(get_all_verifiers)
                .service(get_all_voters)
                .service(get_one_voter)
                .service(get_registration_closed_status)
                .service(get_voter_requirements)
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

async fn background_task() -> AResult<()> {
    println!("The background task has started running.");
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    println!("Background task completed");
    // return Err(Error::msg("haha"));

    Ok(())
}


async fn admin_validator(
    req: ServiceRequest,
    credentials: Option<BearerAuth>,
) -> Result<ServiceRequest, (error::Error, ServiceRequest)> {
    let Some(credentials) = credentials else {
        return Err((error::ErrorUnauthorized("no bearer header"), req));
    };
    let config = req.app_data::<Data<RegistrarConfig>>().unwrap();
    // eprintln!("{:#?}", config.admin_key == credentials.token());
    // eprintln!("{credentials:?}");

    if config.admin_key != credentials.token() {
        // return Err((error::ErrorUnauthorized("token is wrong"), req));
        return Err((error::ErrorUnauthorized(""), req));
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
    let mut connection =  SqliteConnection::establish(database_url)
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
