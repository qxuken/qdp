use dotenv::dotenv;
use env_logger::Env;
use qdp::{routes::create_router, AppState, Database};
use std::env;
use tower_http::compression::CompressionLayer;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let is_dev = env::var("APPLICATION_MODE").is_ok_and(|e| e == "development");
    log::info!(
        "Application mode is {}",
        if is_dev { "development" } else { "production" }
    );

    // Setup application bind addr
    let host = env::var("APPLICATION_HOST")
        .ok()
        .filter(|h| h != "localhost")
        .unwrap_or("127.0.0.1".to_owned());
    let port: usize = env::var("APPLICATION_PORT")
        .unwrap_or_else(|_| "8080".to_owned())
        .parse()
        .expect("PORT must be a number");

    // Setup Database
    let database_url = env::var("DATABASE_URL").ok();
    let database = Database::new(database_url);
    database.run_migrations();

    let app_state = AppState::new(database, is_dev).shared();

    let addr = format!("{}:{}", host, port).parse().unwrap();

    log::info!("Starting on: http://{:?}", &addr);

    let compression = CompressionLayer::new();
    let app = create_router().with_state(app_state).layer(compression);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
