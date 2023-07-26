use crate::{frontend::Templates, Database};
use actix_web::{error, web, Responder, Result};
use serde_json::value::to_value;
use serde_json::Map;

pub async fn links_page<'a>(
    templates: web::Data<Templates<'a>>,
    database: web::Data<Database>,
) -> Result<impl Responder> {
    let tasks = web::block(move || {
        let mut conn = database.get_connection()?;

        super::actions::find_all_links(&mut conn)
    })
    .await?
    .map_err(error::ErrorBadRequest)?;

    let mut data = Map::new();
    data.insert("tasks".to_string(), to_value(tasks).unwrap_or_default());

    Ok(templates.handle("entities/links/tasks.hbs", Some(data)))
}
