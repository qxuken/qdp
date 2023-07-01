use actix_web::{get, web, Scope, HttpResponse, Responder};

#[get("/test")]
async fn test() -> impl Responder {
    HttpResponse::Ok().body("Ok!")
}

pub fn create_service(path: &'static str) -> Scope {
    web::scope(path)
        .service(test)
}
