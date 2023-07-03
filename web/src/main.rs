use actix_web::{middleware::Logger, App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());

    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    log::info!("Starting on: {}:{:?}", host, port);

    HttpServer::new(move || {
        let frontend_path = env::var("FRONTEND_PATH").ok();
        App::new()
            .wrap(Logger::default().log_target("app"))
            .service(api::create_service("/api"))
            .service(frontend::create_service("/", frontend_path))
    })
    .bind((host, port))?
    .run()
    .await
}
