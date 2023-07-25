use actix_web::{get, web, Responder};

use crate::frontend::{Assets, TemplatesRegistry};
use serde_json::json;

#[get("/")]
pub async fn index<'a>(templates: web::Data<TemplatesRegistry<'a>>) -> impl Responder {
    let data = json!({
        "test": "test"
    });
    templates.handle("pages/tasks.hbs", Some(&data))
}

#[get("/assets/{asset_path}")]
pub async fn static_assets(asset_path: web::Path<String>) -> impl Responder {
    Assets::handle(asset_path.as_str())
}
