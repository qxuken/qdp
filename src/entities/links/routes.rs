use crate::{Database, TemplateProps, Templates};
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

    let mut props = TemplateProps::default();
    props.data = Some(data);
    props.scripts.push("/entities/links/mod.js".to_string());

    Ok(templates.handle("entities/links/links.hbs", Some(props)))
}
