use actix_web::{web, Scope};

use crate::{
    entities::{health::health_route, links},
    frontend,
};

pub fn mount(path: &str) -> Scope {
    web::scope(path)
        .route("/", web::get().to(links::links_page))
        .route("/api/health", web::get().to(health_route))
        .service(links::mount_scope("/api/links"))
        .service(frontend::assets::assets_route)
}
