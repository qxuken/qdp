mod sections;

use crate::{
    entities::links::LinksView, result::Result, templates::links::LinksTemplate, SharedAppState,
};
use axum::{extract::State, routing::get, Router};

pub async fn links_route(State(app_state): State<SharedAppState>) -> Result<LinksTemplate> {
    let mut conn = app_state.db.get_connection()?;

    let links = LinksView::query(&mut conn)?;

    Ok(LinksTemplate::from(links))
}

pub fn create_router() -> Router<SharedAppState> {
    Router::new()
        .route("/", get(links_route))
        .nest("/section", sections::create_router())
}
