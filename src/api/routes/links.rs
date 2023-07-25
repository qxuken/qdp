use crate::{entities::links, Database};
use actix_web::{delete, error, get, post, put, web, HttpResponse, Responder, Result, Scope};

#[get("")]
async fn get_links(database: web::Data<Database>) -> Result<impl Responder> {
    let data = web::block(move || {
        let mut conn = database.get_connection()?;

        links::find_all_links(&mut conn)
    })
    .await?
    .map_err(error::ErrorBadRequest)?;

    Ok(HttpResponse::Ok().json(data))
}

#[post("/section")]
async fn create_section(
    database: web::Data<Database>,
    data: web::Json<links::NewLinkSection>,
) -> Result<impl Responder> {
    let data = web::block(move || {
        let mut conn = database.get_connection()?;

        links::insert_link_section(&mut conn, data.into_inner())
    })
    .await?
    .map_err(error::ErrorBadRequest)?;

    Ok(HttpResponse::Created().json(data))
}

#[put("/section/{id}")]
async fn update_section(
    database: web::Data<Database>,
    path: web::Path<i32>,
    data: web::Json<links::UpdateLinkSection>,
) -> Result<impl Responder> {
    let data = web::block(move || {
        let mut conn = database.get_connection()?;

        links::find_link_section(&mut conn, &path)?;

        links::update_link_section(&mut conn, &path, data.into_inner())
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

        links::find_link_section(&mut conn, &path)?;

        links::delete_link_section(&mut conn, *path)
    })
    .await?
    .map_err(error::ErrorBadRequest)?;

    Ok(HttpResponse::NoContent())
}

#[post("/item")]
async fn create_item(
    database: web::Data<Database>,
    data: web::Json<links::NewLinkItem>,
) -> Result<impl Responder> {
    let data: links::LinkItem = web::block(move || {
        let mut conn = database.get_connection()?;

        links::find_link_section(&mut conn, &data.link_section_id)?;

        links::insert_link_item(&mut conn, data.into_inner())
    })
    .await?
    .map_err(error::ErrorBadRequest)?;

    Ok(HttpResponse::Created().json(data))
}

#[put("/item/{id}")]
async fn update_item(
    database: web::Data<Database>,
    path: web::Path<i32>,
    data: web::Json<links::UpdateLinkItem>,
) -> Result<impl Responder> {
    let data = web::block(move || {
        let mut conn = database.get_connection()?;

        if let Some(link_section_id) = data.link_section_id {
            links::find_link_section(&mut conn, &link_section_id)?;
        }

        links::update_link_item(&mut conn, &path, data.into_inner())
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

        links::find_link_item(&mut conn, &path)?;

        links::delete_link_item(&mut conn, *path)
    })
    .await?
    .map_err(error::ErrorBadRequest)?;

    Ok(HttpResponse::NoContent())
}

pub fn get_scope() -> Scope {
    web::scope("/links")
        .service(get_links)
        .service(create_section)
        .service(update_section)
        .service(delete_section)
        .service(create_item)
        .service(update_item)
        .service(delete_item)
}
