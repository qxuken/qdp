mod assets;
mod routes;
mod templates;

use actix_web::{web, Scope};

pub use templates::Templates;

pub fn create_frontend_service(path: &'static str) -> Scope {
    routes::mount(web::scope(path))
}
