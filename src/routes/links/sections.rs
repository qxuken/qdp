use crate::{
    entities::links::{LinkSection, NewLinkSection, UpdateLinkSection},
    result::Result,
    SharedAppState,
};
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::Redirect,
    routing::{patch, post},
    Form, Router,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateSectionForm {
    pub title: Option<String>,
    pub order_number: Option<i32>,
}

pub async fn create_link_section_route(
    State(app_state): State<SharedAppState>,
    Form(create_section_query): Form<CreateSectionForm>,
) -> Result<Redirect> {
    let mut conn = app_state.db.get_connection()?;

    let new_section = NewLinkSection {
        title: create_section_query
            .title
            .unwrap_or("New Section".to_owned()),
        order_number: create_section_query.order_number,
    };
    LinkSection::create(&mut conn, new_section)?;

    Ok(Redirect::to("/"))
}

pub async fn update_link_section_title_route(
    State(app_state): State<SharedAppState>,
    Path(section_id): Path<i32>,
    Form(update_section_title_form): Form<UpdateLinkSection>,
) -> Result<StatusCode> {
    let mut conn = app_state.db.get_connection()?;

    LinkSection::find_by_id(&mut conn, &section_id)?
        .update(&mut conn, &update_section_title_form)?;

    Ok(StatusCode::OK)
}

pub async fn delete_link_section_route(
    State(app_state): State<SharedAppState>,
    Path(section_id): Path<i32>,
) -> Result<(HeaderMap, StatusCode)> {
    let mut conn = app_state.db.get_connection()?;

    let section = LinkSection::find_by_id(&mut conn, &section_id)?;

    section.delete(&mut conn)?;

    let mut headers = HeaderMap::with_capacity(1);

    headers.append("HX-Trigger", "links-updated".parse().unwrap());

    Ok((headers, StatusCode::OK))
}

pub fn create_router() -> Router<SharedAppState> {
    Router::new()
        .route("/", post(create_link_section_route))
        .route(
            "/:id",
            patch(update_link_section_title_route).delete(delete_link_section_route),
        )
}
