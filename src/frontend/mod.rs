mod assets;
mod routes;
mod templates;

use actix_web::{web, Scope};
use routes::{index, static_assets};

pub use assets::Assets;
pub use templates::TemplatesRegistry;

pub fn create_frontend_service(path: &'static str) -> Scope {
    web::scope(path).service(index).service(static_assets)
}
