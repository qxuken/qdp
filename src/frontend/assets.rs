#![allow(non_upper_case_globals)]

use actix_web::{get, web, HttpResponse, Responder};
use mime_guess::from_path;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "./dist"]
pub struct Assets;

#[get("/assets/{asset_path}")]
pub async fn assets_route(asset_path: web::Path<String>) -> impl Responder {
    match Assets::get(&asset_path) {
        Some(content) => HttpResponse::Ok()
            .content_type(
                from_path(asset_path.into_inner())
                    .first_or_octet_stream()
                    .as_ref(),
            )
            .body(content.data.into_owned()),
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}
