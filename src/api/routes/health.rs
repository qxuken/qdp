use actix_web::{get, web, HttpResponse, Responder, Scope};

#[get("")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("Ok!")
}

pub fn get_scope() -> Scope {
    web::scope("/health").service(health)
}
