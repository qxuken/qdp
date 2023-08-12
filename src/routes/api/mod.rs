mod health;
mod links;

use actix_web::{web, Scope};

use self::health::health_route;

pub fn mount_api(path: &str) -> Scope {
    web::scope(path)
        .route("/health", web::get().to(health_route))
        .service(links::mount_scope("/links"))
}
