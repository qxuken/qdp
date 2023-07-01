use actix_web::{get, middleware::Logger, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use env_logger::Env;
use std::env;

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("Ok!")
}

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
        App::new()
            .wrap(Logger::default().log_target("app"))
            .service(health)
            .service(api::create_service("/api"))
            .service(frontend::create_service("/"))
    })
    .bind((host, port))?
    .run()
    .await
}
