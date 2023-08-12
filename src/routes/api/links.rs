use crate::entities::links::{
    LinkItem, LinkSection, LinksView, NewLinkItem, NewLinkSection, UpdateLinkItem,
    UpdateLinkSection,
};
use crate::Database;
use actix_web::{delete, error, get, post, put, web, HttpResponse, Responder, Result, Scope};

#[get("")]
pub async fn links_data(database: web::Data<Database>) -> Result<impl Responder> {
    let data = web::block(move || {
        let mut conn = database.get_connection()?;

        LinksView::query(&mut conn)
    })
    .await?
    .map_err(error::ErrorBadRequest)?;

    Ok(HttpResponse::Ok().json(data))
}

#[post("/section")]
async fn create_section(
    database: web::Data<Database>,
    data: web::Json<NewLinkSection>,
) -> Result<impl Responder> {
    let data = web::block(move || {
        let mut conn = database.get_connection()?;

        LinkSection::create(&mut conn, data.into_inner())
    })
    .await??;

    Ok(HttpResponse::Created().json(data))
}

#[put("/section/{id}")]
async fn update_section(
    database: web::Data<Database>,
    section_id: web::Path<i32>,
    data: web::Json<UpdateLinkSection>,
) -> Result<impl Responder> {
    let data = web::block(move || {
        let mut conn = database.get_connection()?;

        let section = LinkSection::find_by_id(&mut conn, &section_id.into_inner())?;

        section.update(&mut conn, &data.into_inner())
    })
    .await??;

    Ok(HttpResponse::Ok().json(data))
}

#[delete("/section/{id}")]
async fn delete_section(
    database: web::Data<Database>,
    section_id: web::Path<i32>,
) -> Result<impl Responder> {
    web::block(move || {
        let mut conn = database.get_connection()?;

        let section = LinkSection::find_by_id(&mut conn, &section_id.into_inner())?;

        section.delete(&mut conn)
    })
    .await??;

    Ok(HttpResponse::NoContent())
}

#[post("/item")]
async fn create_item(
    database: web::Data<Database>,
    data: web::Json<NewLinkItem>,
) -> Result<impl Responder> {
    let data = web::block(move || {
        let mut conn = database.get_connection()?;

        LinkSection::find_by_id(&mut conn, &data.link_section_id)?;
        LinkItem::create(&mut conn, data.into_inner())
    })
    .await??;

    Ok(HttpResponse::Created().json(data))
}

#[put("/item/{id}")]
async fn update_item(
    database: web::Data<Database>,
    item_id: web::Path<i32>,
    data: web::Json<UpdateLinkItem>,
) -> Result<impl Responder> {
    let data = web::block(move || {
        let mut conn = database.get_connection()?;

        if let Some(link_section_id) = data.link_section_id {
            LinkSection::find_by_id(&mut conn, &link_section_id)?;
        }
        let item = LinkItem::find_by_id(&mut conn, &item_id)?;
        item.update(&mut conn, data.into_inner())
    })
    .await?
    .map_err(error::ErrorBadRequest)?;

    Ok(HttpResponse::Ok().json(data))
}

#[delete("/item/{id}")]
async fn delete_item(
    database: web::Data<Database>,
    item_id: web::Path<i32>,
) -> Result<impl Responder> {
    web::block(move || {
        let mut conn = database.get_connection()?;

        let item = LinkItem::find_by_id(&mut conn, &item_id)?;
        item.delete(&mut conn)
    })
    .await?
    .map_err(error::ErrorBadRequest)?;

    Ok(HttpResponse::NoContent())
}

pub fn mount_scope(route: &str) -> Scope {
    web::scope(route)
        .service(links_data)
        .service(create_section)
        .service(update_section)
        .service(delete_section)
        .service(create_item)
        .service(update_item)
        .service(delete_item)
}
