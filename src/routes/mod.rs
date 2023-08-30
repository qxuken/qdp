mod api;
mod assets;
mod dashboard;
mod links;

use axum::{routing::get, Router};

use crate::{assets::ASSETS_PATH, SharedAppState};

use self::{
    api::create_api_router,
    assets::{assets_route, favicon_route},
    dashboard::dashboard_route,
};

pub fn create_router() -> Router<SharedAppState> {
    Router::new()
        .nest("/api", create_api_router())
        .route("/", get(dashboard_route))
        .nest("/links", links::create_router())
        .route("/favicon.svg", get(favicon_route))
        .route(ASSETS_PATH, get(assets_route))
}
