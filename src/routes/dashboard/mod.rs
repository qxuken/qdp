use crate::{frontend::ScriptItem, Database, TemplateProps, Templates};
use actix_web::{web, Responder, Result};
use serde_json::{json, Map};

use crate::entities::links::LinksView;

pub async fn dashboard_route(
    templates: web::Data<Templates<'_>>,
    database: web::Data<Database>,
) -> Result<impl Responder> {
    let links = web::block(move || {
        let mut conn = database.get_connection()?;

        LinksView::query(&mut conn)
    })
    .await??;

    let mut data = Map::new();
    data.insert("links".to_owned(), json!(links));

    let props = TemplateProps {
        title: Some("Dashboard"),
        data: Some(data),
        local_scripts: vec![ScriptItem::async_module("/routes/dashboard/mod.js")],
        ..TemplateProps::default()
    };

    Ok(templates.handle("routes/dashboard/mod.hbs", Some(props)))
}
