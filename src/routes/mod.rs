mod api;
mod dashboard;

use actix_web::{web, Scope};

use crate::frontend::{self, assets_route};

use self::dashboard::dashboard_route;

pub fn mount(path: &str) -> Scope {
    web::scope(path)
        .route("/", web::get().to(dashboard_route))
        .service(api::mount_api("/api"))
        .route(frontend::ASSETS_PATH, web::get().to(assets_route))
}
