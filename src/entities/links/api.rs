use super::{actions, model};
use crate::Database;
use actix_web::{delete, error, post, put, web, HttpResponse, Responder, Result, Scope};

#[post("/section")]
async fn create_section(
    database: web::Data<Database>,
    data: web::Json<model::NewLinkSection>,
) -> Result<impl Responder> {
    let data = web::block(move || {
        let mut conn = database.get_connection()?;

        actions::insert_link_section(&mut conn, data.into_inner())
    })
    .await?
    .map_err(error::ErrorBadRequest)?;

    Ok(HttpResponse::Created().json(data))
}

#[put("/section/{id}")]
async fn update_section(
    database: web::Data<Database>,
    path: web::Path<i32>,
    data: web::Json<model::UpdateLinkSection>,
) -> Result<impl Responder> {
    let data = web::block(move || {
        let mut conn = database.get_connection()?;

        actions::find_link_section(&mut conn, &path)?;

        actions::update_link_section(&mut conn, &path, data.into_inner())
    })
    .await?
    .map_err(error::ErrorBadRequest)?;

    Ok(HttpResponse::Ok().json(data))
}

#[delete("/section/{id}")]
async fn delete_section(
    database: web::Data<Database>,
    path: web::Path<i32>,
) -> Result<impl Responder> {
    web::block(move || {
        let mut conn = database.get_connection()?;

        actions::find_link_section(&mut conn, &path)?;

        actions::delete_link_section(&mut conn, *path)
    })
    .await?
    .map_err(error::ErrorBadRequest)?;

    Ok(HttpResponse::NoContent())
}

#[post("/item")]
async fn create_item(
    database: web::Data<Database>,
    data: web::Json<model::NewLinkItem>,
) -> Result<impl Responder> {
    let data: model::LinkItem = web::block(move || {
        let mut conn = database.get_connection()?;

        actions::find_link_section(&mut conn, &data.link_section_id)?;

        actions::insert_link_item(&mut conn, data.into_inner())
    })
    .await?
    .map_err(error::ErrorBadRequest)?;

    Ok(HttpResponse::Created().json(data))
}

#[put("/item/{id}")]
async fn update_item(
    database: web::Data<Database>,
    path: web::Path<i32>,
    data: web::Json<model::UpdateLinkItem>,
) -> Result<impl Responder> {
    let data = web::block(move || {
        let mut conn = database.get_connection()?;

        if let Some(link_section_id) = data.link_section_id {
            actions::find_link_section(&mut conn, &link_section_id)?;
        }

        actions::update_link_item(&mut conn, &path, data.into_inner())
    })
    .await?
    .map_err(error::ErrorBadRequest)?;

    Ok(HttpResponse::Ok().json(data))
}

#[delete("/item/{id}")]
async fn delete_item(
    database: web::Data<Database>,
    path: web::Path<i32>,
) -> Result<impl Responder> {
    web::block(move || {
        let mut conn = database.get_connection()?;

        actions::find_link_item(&mut conn, &path)?;

        actions::delete_link_item(&mut conn, *path)
    })
    .await?
    .map_err(error::ErrorBadRequest)?;

    Ok(HttpResponse::NoContent())
}

pub fn mount_scope(route: &str) -> Scope {
    web::scope(route)
        .service(create_section)
        .service(update_section)
        .service(delete_section)
        .service(create_item)
        .service(update_item)
        .service(delete_item)
}
