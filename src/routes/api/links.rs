use crate::entities::links::{
    LinkItem, LinkSection, LinksView, NewLinkItem, NewLinkSection, UpdateLinkItem,
    UpdateLinkSection,
};
use crate::{result::Result, SharedAppState};
use axum::extract::Path;
use axum::Json;
use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post, put},
    Router,
};

async fn links_data(State(app_state): State<SharedAppState>) -> Result<Json<Vec<LinksView>>> {
    let mut conn = app_state.db.get_connection()?;

    let data = LinksView::query(&mut conn)?;

    Ok(Json(data))
}

async fn create_section(
    State(app_state): State<SharedAppState>,
    Json(data): Json<NewLinkSection>,
) -> Result<(StatusCode, Json<LinkSection>)> {
    let mut conn = app_state.db.get_connection()?;

    let data = LinkSection::create(&mut conn, data)?;

    Ok((StatusCode::CREATED, Json(data)))
}

async fn update_section(
    State(app_state): State<SharedAppState>,
    Path(section_id): Path<i32>,
    Json(data): Json<UpdateLinkSection>,
) -> Result<Json<LinkSection>> {
    let mut conn = app_state.db.get_connection()?;

    let section = LinkSection::find_by_id(&mut conn, &section_id)?;

    let data = section.update(&mut conn, &data)?;

    Ok(Json(data))
}

async fn delete_section(
    State(app_state): State<SharedAppState>,
    Path(section_id): Path<i32>,
) -> Result<StatusCode> {
    let mut conn = app_state.db.get_connection()?;

    let section = LinkSection::find_by_id(&mut conn, &section_id)?;

    section.delete(&mut conn)?;

    Ok(StatusCode::NO_CONTENT)
}

async fn create_item(
    State(app_state): State<SharedAppState>,
    Json(data): Json<NewLinkItem>,
) -> Result<(StatusCode, Json<LinkItem>)> {
    let mut conn = app_state.db.get_connection()?;

    LinkSection::find_by_id(&mut conn, &data.link_section_id)?;
    let data = LinkItem::create(&mut conn, data)?;

    Ok((StatusCode::CREATED, Json(data)))
}

async fn update_item(
    State(app_state): State<SharedAppState>,
    Path(item_id): Path<i32>,
    Json(data): Json<UpdateLinkItem>,
) -> Result<Json<LinkItem>> {
    let mut conn = app_state.db.get_connection()?;

    if let Some(link_section_id) = data.link_section_id {
        LinkSection::find_by_id(&mut conn, &link_section_id)?;
    }
    let item = LinkItem::find_by_id(&mut conn, &item_id)?;
    let data = item.update(&mut conn, data)?;
    Ok(Json(data))
}

async fn delete_item(
    State(app_state): State<SharedAppState>,
    Path(item_id): Path<i32>,
) -> Result<StatusCode> {
    let mut conn = app_state.db.get_connection()?;

    let item = LinkItem::find_by_id(&mut conn, &item_id)?;
    item.delete(&mut conn)?;

    Ok(StatusCode::NO_CONTENT)
}

pub fn links_router() -> Router<SharedAppState> {
    Router::new()
        .route("/", get(links_data))
        .route("/section", post(create_section))
        .route("/section/:id", put(update_section).delete(delete_section))
        .route("/item", post(create_item))
        .route("/item/:id", put(update_item).delete(delete_item))
}
