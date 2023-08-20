use std::sync::Arc;

use crate::{
    app_state,
    frontend::{Assets, AssetsMetadataStore},
    SharedAppState,
};
use axum::{
    body::Bytes,
    extract::{Path, State},
    headers::{ContentType, ETag, LastModified},
    http::HeaderMap,
    http::{
        header::{CONTENT_TYPE, ETAG, LAST_MODIFIED},
        StatusCode,
    },
    response::{IntoResponse, Response},
    routing::{get, post, put},
    Json, Router, TypedHeader,
};

pub struct StaticFile(
    String,
    Option<TypedHeader<ETag>>,
    Option<TypedHeader<LastModified>>,
    SharedAppState,
);

impl IntoResponse for StaticFile {
    fn into_response(self) -> Response {
        let StaticFile(path, etag, last_modified, app_state) = self;

        match Assets::get(&path) {
            Some(content) => {
                let mut headers = HeaderMap::new();
                headers.insert(CONTENT_TYPE, content.metadata.mimetype().parse().unwrap());
                if let Some(etag) = app_state.assets_metadata.e_tag(&path) {
                    headers.insert(ETAG, etag.parse().unwrap());
                }
                if let Some(last_modified) = app_state.assets_metadata.last_modified(&path) {
                    headers.insert(LAST_MODIFIED, String::from(TypedHeader(last_modified)));
                }
                (headers, content.data).into_response()
            }
            None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
        }
    }
}

#[axum_macros::debug_handler]
pub async fn assets_route(
    headers: HeaderMap,
    last_modified: Option<TypedHeader<LastModified>>,
    etag: Option<TypedHeader<ETag>>,
    Path(params): Path<String>,
    State(app_state): State<SharedAppState>,
) -> StaticFile {
    let last_modified2 = headers.get("LastModified");
    let etag2 = headers.get("ETag");
    println!("{params}");
    println!("{last_modified:?}");
    println!("{last_modified2:?}");
    println!("{etag:?}");
    println!("{etag2:?}");
    println!("{app_state:?}");

    StaticFile(params, etag, last_modified, app_state)
}

// match Assets::get(&asset_path) {
//   Some(content) => {
//       let mut res = HttpResponse::Ok();

//       if let Some(e_tag) = meta.e_tag(&asset_path) {
//           if IfNoneMatch::parse(&req)
//               .map(|h| match h {
//                   IfNoneMatch::Any => vec![],
//                   IfNoneMatch::Items(tags) => tags,
//               })
//               .is_ok_and(|tags| tags.contains(&e_tag))
//           {
//               return HttpResponse::NotModified().finish();
//           }
//           res.insert_header(ETag(e_tag));
//       }

//       if let Some(http_date) = meta.last_modified(&asset_path) {
//           if IfModifiedSince::parse(&req).is_ok_and(|h| h.0 == http_date) {
//               return HttpResponse::NotModified().finish();
//           }
//           res.insert_header(LastModified(http_date));
//       }

//       res.content_type(content.metadata.mimetype());
//       res.append_header(CacheControl(vec![
//           CacheDirective::Public,
//           CacheDirective::NoCache,
//       ]));

//       res.body(content.data.into_owned())
//   }
//   None => HttpResponse::NotFound().body("404 Not Found"),
// }
