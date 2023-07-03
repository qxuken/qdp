mod routes;

use actix_web::{web, Scope};

pub fn create_service(path: &'static str) -> Scope {
    let pool = database::initialize_db_pool();

    web::scope(path)
        .app_data(web::Data::new(pool.clone()))
        .service(routes::health::get_scope())
        .service(routes::links::get_scope())
}
