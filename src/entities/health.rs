use actix_web::{HttpResponse, Responder};

pub async fn health_route() -> impl Responder {
    HttpResponse::Ok().body("Ok!")
}
