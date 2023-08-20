mod api;
mod assets;
mod dashboard;

use axum::{routing::get, Router};

use crate::{frontend, SharedAppState};

use self::{api::create_api_router, assets::assets_route, dashboard::dashboard_route};

pub fn create_router() -> Router<SharedAppState> {
    Router::new()
        .nest("/api", create_api_router())
        .route("/", get(dashboard_route))
        .route(frontend::ASSETS_PATH, get(assets_route))
}
