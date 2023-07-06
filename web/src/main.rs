use actix_cors::Cors;
use actix_web::{
    middleware::{Compress, Logger},
    web, App, HttpServer,
};
use dotenv::dotenv;
use env_logger::Env;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Setup application bind addr
    let host = env::var("APPLICATION_HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("APPLICATION_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");
    log::info!("Starting on: {}:{:?}", host, port);

    // Setup Database
    let database_url = env::var("APPLICATION_DATABASE_URL").ok();
    let pool = database::initialize_db_pool(database_url);
    database::run_migrations(pool.clone());

    // Frontend related params
    let frontend_path = env::var("APPLICATION_FRONTEND_PATH").ok();
    let cors_permisive = env::var("APPLICATION_CORS_DISABLED").is_ok();
    let compression = Compress::default();

    HttpServer::new(move || {
        let cors = if cors_permisive {
            Cors::permissive()
        } else {
            Cors::default()
        };
        App::new()
            .wrap(Logger::default().log_target("app"))
            .wrap(compression.clone())
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .service(api::create_service("/api"))
            .service(frontend::create_service("/", frontend_path.to_owned()))
    })
    .bind((host, port))?
    .run()
    .await
}
