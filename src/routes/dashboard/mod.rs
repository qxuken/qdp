mod dashboard_page_template;

use crate::{
    entities::links::LinksView,
    frontend::{HtmlTemplate, ScriptItem},
    result::Result,
    SharedAppState,
};
use axum::extract::State;
use dashboard_page_template::DashboardPageTemplate;

pub async fn dashboard_route(
    State(app_state): State<SharedAppState>,
) -> Result<HtmlTemplate<DashboardPageTemplate>> {
    let mut conn = app_state.db.get_connection()?;

    let links = LinksView::query(&mut conn)?;

    let template = DashboardPageTemplate {
        title: "QDP - Dashboard",
        data: "data",
        links: links.into(),
        global_scripts: app_state.global_scripts.clone(),
        stylesheets: app_state.stylesheets.clone(),
        local_scripts: vec![ScriptItem::async_module("/routes/dashboard/mod.js")].into(),
    };

    Ok(HtmlTemplate(template))
}
