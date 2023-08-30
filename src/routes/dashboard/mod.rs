use crate::{
    entities::links::LinksView,
    result::Result,
    templates::{headers::ScriptItem, links::LinksTemplate, pages::DefaultPage},
    SharedAppState,
};
use axum::extract::State;

pub async fn dashboard_route(State(app_state): State<SharedAppState>) -> Result<DefaultPage> {
    let mut conn = app_state.db.get_connection()?;

    let links = LinksView::query(&mut conn)?;

    let template = DefaultPage::with_template(
        "QDP - Dashboard",
        app_state,
        LinksTemplate::from(links),
        Some(vec![ScriptItem::async_module("/routes/dashboard/mod.js")].into()),
    );

    Ok(template)
}
