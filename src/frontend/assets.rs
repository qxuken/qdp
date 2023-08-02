#![allow(non_upper_case_globals)]

use actix_web::{
    http::header::{ETag, EntityTag, LastModified},
    web, HttpResponse, Responder,
};
use rust_embed::RustEmbed;
use std::time::{Duration, SystemTime};

use crate::TemplateProps;

pub const ASSETS_PREFIX: &'static str = "/assets";
pub const ASSETS_PATH: &'static str = "/assets/{asset_path:.*}";

#[derive(RustEmbed)]
#[folder = "./dist"]
pub struct Assets;

pub async fn assets_route(asset_path: web::Path<String>) -> impl Responder {
    match Assets::get(&asset_path) {
        Some(content) => {
            let hash = hex::encode(content.metadata.sha256_hash());
            let mut res = HttpResponse::Ok();
            res.content_type(content.metadata.mimetype());
            res.insert_header(ETag(EntityTag::new_strong(hash)));
            if let Some(last_modified) = content.metadata.last_modified() {
                let last_modified = SystemTime::now() - Duration::from_secs(last_modified);
                res.insert_header(LastModified(last_modified.into()));
            }
            res.body(content.data.into_owned())
        }
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

pub fn register_assets(props: &mut TemplateProps) {
    props.scripts.push("/lib.js".to_string());
    props.stylesheets.push("/lib.css".to_string());
}
