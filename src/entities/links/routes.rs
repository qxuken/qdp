use crate::{frontend::Templates, Database};
use actix_web::{error, web, Responder, Result};
use serde_json::{json, Map};

pub async fn links_page<'a>(
    templates: web::Data<Templates<'a>>,
    database: web::Data<Database>,
) -> Result<impl Responder> {
    let links = web::block(move || {
        let mut conn = database.get_connection()?;

        super::actions::find_all_links(&mut conn)
    })
    .await?
    .map_err(error::ErrorBadRequest)?;

    let mut data = Map::new();
    data.insert("links".to_string(), json!(links));
    data.insert("title".to_string(), json!("Links".to_string()));

    Ok(templates.handle("entities/links/links.hbs", Some(data)))
}
