#![allow(non_upper_case_globals)]

use actix_web::{web, HttpResponse, Responder};
use mime_guess::from_path;
use rust_embed::RustEmbed;

use crate::TemplateProps;

pub const ASSETS_PREFIX: &'static str = "/assets";
pub const ASSETS_PATH: &'static str = "/assets/{asset_path:.*}";

#[derive(RustEmbed)]
#[folder = "./dist"]
pub struct Assets;

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

pub fn register_assets(props: &mut TemplateProps) {
    props.scripts.push("/lib.js".to_string());
    props.stylesheets.push("/lib.css".to_string());
}
