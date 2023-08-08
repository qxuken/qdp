#![allow(non_upper_case_globals)]
use std::{
    borrow::Cow,
    collections::HashMap,
    time::{Duration, UNIX_EPOCH},
};

use actix_web::{
    http::header::{
        CacheControl, CacheDirective, ETag, EntityTag, Header, HttpDate, IfModifiedSince,
        IfNoneMatch, LastModified,
    },
    web, HttpRequest, HttpResponse, Responder,
};
use rust_embed::RustEmbed;

use crate::TemplateProps;

pub const ASSETS_PREFIX: &'static str = "/assets";
pub const ASSETS_PATH: &'static str = "/assets/{asset_path:.*}";

#[derive(RustEmbed)]
#[folder = "./dist"]
pub struct Assets;

#[derive(Clone)]
pub struct AssetsMetadata {
    pub e_tag: Option<EntityTag>,
    pub last_modified: Option<HttpDate>,
}
#[derive(Clone)]
pub struct AssetsMetadataStore {
    map: HashMap<Cow<'static, str>, AssetsMetadata>,
}

impl AssetsMetadataStore {
    pub fn boot(is_dev: bool) -> Self {
        let mut store = AssetsMetadataStore {
            map: HashMap::new(),
        };
        if is_dev {
            return store;
        }
        for asset_path in Assets::iter() {
            let e_tag = store.e_tag(&asset_path);
            let last_modified = store.last_modified(&asset_path);
            store.map.insert(
                asset_path,
                AssetsMetadata {
                    e_tag,
                    last_modified,
                },
            );
        }
        return store;
    }

    pub fn e_tag(&self, asset_path: &str) -> Option<EntityTag> {
        match self.map.get(asset_path) {
            Some(meta) => meta.e_tag.clone(),
            None => Assets::get(&asset_path)
                .map(|content| hex::encode(content.metadata.sha256_hash()))
                .map(EntityTag::new_weak),
        }
    }

    pub fn last_modified(&self, asset_path: &str) -> Option<HttpDate> {
        match self.map.get(asset_path) {
            Some(meta) => meta.last_modified.clone(),
            None => Assets::get(&asset_path)
                .and_then(|content| content.metadata.last_modified())
                .map(|lm| UNIX_EPOCH + Duration::from_secs(lm))
                .map(HttpDate::from),
        }
    }
}

pub async fn assets_route(
    req: HttpRequest,
    meta: web::Data<AssetsMetadataStore>,
    asset_path: web::Path<String>,
) -> impl Responder {
    match Assets::get(&asset_path) {
        Some(content) => {
            let mut res = HttpResponse::Ok();

            if let Some(e_tag) = meta.e_tag(&asset_path) {
                if IfNoneMatch::parse(&req)
                    .map(|h| match h {
                        IfNoneMatch::Any => vec![],
                        IfNoneMatch::Items(tags) => tags,
                    })
                    .is_ok_and(|tags| tags.contains(&e_tag))
                {
                    return HttpResponse::NotModified().finish();
                }
                res.insert_header(ETag(e_tag));
            }

            if let Some(http_date) = meta.last_modified(&asset_path) {
                if IfModifiedSince::parse(&req).is_ok_and(|h| h.0 == http_date) {
                    return HttpResponse::NotModified().finish();
                }
                res.insert_header(LastModified(http_date));
            }

            res.content_type(content.metadata.mimetype());
            res.append_header(CacheControl(vec![
                CacheDirective::Public,
                CacheDirective::NoCache,
            ]));

            res.body(content.data.into_owned())
        }
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

pub fn register_assets(props: &mut TemplateProps) {
    props.scripts.push(("/lib.js", Some("async")));
    props.stylesheets.push("/lib.css");
}
