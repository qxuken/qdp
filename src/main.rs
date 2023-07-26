use actix_cors::Cors;
use actix_web::{
    middleware::{Compress, Logger},
    web, App, HttpServer,
};
use dotenv::dotenv;
use env_logger::Env;
use qdp::Database;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let is_dev = env::var("APPLICATION_MODE").unwrap_or("production".to_string()) == "development";

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Setup application bind addr
    let host = env::var("APPLICATION_HOST").unwrap_or("localhost".to_string());
    let port = env::var("APPLICATION_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");
    log::info!("Starting on: http://{}:{:?}", host, port);

    // Setup Database
    let database_url = env::var("DATABASE_URL").ok();
    let database = Database::new(database_url);
    database.run_migrations();

    // Frontend related params
    let cors_permissive = env::var("APPLICATION_CORS_DISABLED").is_ok();
    let compression = Compress::default();
    let registry = qdp::frontend::Templates::new(is_dev);

    HttpServer::new(move || {
        let cors = if cors_permissive {
            Cors::permissive()
        } else {
            Cors::default()
        };
        App::new()
            .wrap(Logger::default().log_target("app"))
            .wrap(compression.clone())
            .wrap(cors)
            .app_data(web::Data::new(database.clone()))
            .app_data(web::Data::new(registry.clone()))
            .service(qdp::api::create_api_service("/api"))
            .service(qdp::frontend::create_frontend_service(""))
    })
    .bind((host, port))?
    .run()
    .await
}