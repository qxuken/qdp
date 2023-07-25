#![allow(non_upper_case_globals)]

use actix_web::HttpResponse;
use mime_guess::from_path;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "./dist"]
pub struct Assets;

impl Assets {
    pub fn handle(path: &str) -> HttpResponse {
        match Assets::get(path) {
            Some(content) => HttpResponse::Ok()
                .content_type(from_path(path).first_or_octet_stream().as_ref())
                .body(content.data.into_owned()),
            None => HttpResponse::NotFound().body("404 Not Found"),
        }
    }
}
