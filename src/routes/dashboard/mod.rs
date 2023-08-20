use crate::{
    frontend::{HtmlTemplate, ScriptItem, TemplateProps},
    SharedAppState,
};
use askama::Template;
use axum::extract::State;
use serde_json::{json, Map};

use crate::{entities::links::LinksView, result::Result};
#[derive(Template, Default)]
#[template(path = "routes/dashboard/page.html")]
pub struct HelloTemplate {
    pub data: &'static str,
}

pub async fn dashboard_route(
    State(app_state): State<SharedAppState>,
) -> Result<HtmlTemplate<HelloTemplate>> {
    let mut conn = app_state.db.get_connection()?;

    let links = LinksView::query(&mut conn)?;

    let mut data = Map::new();
    data.insert("links".to_owned(), json!(links));

    let props = TemplateProps {
        title: Some("Dashboard"),
        data: Some(data),
        local_scripts: vec![ScriptItem::async_module("/routes/dashboard/mod.js")],
        ..TemplateProps::default()
    };

    println!("{:?}", props);

    Ok(HtmlTemplate(HelloTemplate { data: "data2" }))
}
