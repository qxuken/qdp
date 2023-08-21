use crate::{frontend::Assets, SharedAppState};
use axum::{
    extract::{Path, State},
    http::{
        header::{
            CACHE_CONTROL, CONTENT_TYPE, ETAG, IF_MODIFIED_SINCE, IF_NONE_MATCH, LAST_MODIFIED,
        },
        HeaderMap, StatusCode,
    },
    response::{IntoResponse, Response},
};

pub struct StaticFile(String, Option<String>, Option<String>, SharedAppState);

impl IntoResponse for StaticFile {
    fn into_response(self) -> Response {
        let StaticFile(path, if_none_match, if_modified_since, app_state) = self;

        match Assets::get(&path) {
            Some(content) => {
                let mut headers = HeaderMap::with_capacity(4);
                if let Some(etag) = app_state.assets_metadata.e_tag(&path) {
                    if if_none_match.is_some_and(|v| v == etag) {
                        return StatusCode::NOT_MODIFIED.into_response();
                    }
                    headers.insert(ETAG, etag.parse().unwrap());
                }
                if let Some(last_modified) = app_state.assets_metadata.last_modified(&path) {
                    if if_modified_since.is_some_and(|v| v == last_modified) {
                        return StatusCode::NOT_MODIFIED.into_response();
                    }
                    headers.insert(LAST_MODIFIED, last_modified.parse().unwrap());
                }
                headers.insert(CACHE_CONTROL, "no-cache".parse().unwrap());
                headers.insert(CONTENT_TYPE, content.metadata.mimetype().parse().unwrap());
                (headers, content.data).into_response()
            }
            None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
        }
    }
}

#[axum_macros::debug_handler]
pub async fn assets_route(
    headers: HeaderMap,
    Path(path): Path<String>,
    State(app_state): State<SharedAppState>,
) -> StaticFile {
    let if_none_match = headers
        .get(IF_NONE_MATCH)
        .and_then(|h| h.to_str().ok())
        .map(|v| v.to_owned());
    let if_modified_since = headers
        .get(IF_MODIFIED_SINCE)
        .and_then(|h| h.to_str().ok())
        .map(|v| v.to_owned());

    StaticFile(path, if_none_match, if_modified_since, app_state)
}
