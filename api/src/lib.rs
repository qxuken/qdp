mod routes;

use actix_web::{web, Scope};

pub fn create_service(path: &'static str) -> Scope {
    web::scope(path)
        .service(routes::health::get_scope())
        .service(routes::links::get_scope())
}
