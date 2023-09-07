use crate::{
    entities::links::LinksView,
    result::Result,
    templates::{
        // headers::{ScriptItem, ScriptItemsTemplate},
        links::LinksTemplate,
        pages::DefaultPage,
    },
    SharedAppState,
};

// use askama::Template;
use axum::extract::State;
// use once_cell::sync::Lazy;

// static LOCAL_SCRIPTS: Lazy<String> = Lazy::new(|| {
//     let scripts: ScriptItemsTemplate =
//         vec![ScriptItem::async_module("/routes/dashboard/mod.js")].into();
//     scripts.render().unwrap_or_default()
// });

pub async fn dashboard_route(State(app_state): State<SharedAppState>) -> Result<DefaultPage> {
    let mut conn = app_state.db.get_connection()?;

    let links = LinksView::query(&mut conn)?;

    let template = DefaultPage::with_template(
        "QDP - Dashboard",
        app_state,
        LinksTemplate::from(links),
        // Some(LOCAL_SCRIPTS.to_owned()),
        None,
    );

    Ok(template)
}
