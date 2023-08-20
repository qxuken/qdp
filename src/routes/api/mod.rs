mod health;
mod links;

use axum::{routing::get, Router};

use crate::SharedAppState;

use self::{health::health_route, links::links_router};

pub fn create_api_router() -> Router<SharedAppState> {
    Router::new()
        .route("/health", get(health_route))
        .nest("/links", links_router())
}
