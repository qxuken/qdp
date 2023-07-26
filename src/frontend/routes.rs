use actix_web::{get, web, Responder, Scope};

use crate::frontend::Templates;
use serde_json::json;

use super::assets::assets;

#[get("/")]
pub async fn index<'a>(templates: web::Data<Templates<'a>>) -> impl Responder {
    let data = json!({
        "test": "test"
    });
    templates.handle("pages/tasks.hbs", Some(&data))
}

pub fn mount(scope: Scope) -> Scope {
    scope.service(index).service(assets)
}
