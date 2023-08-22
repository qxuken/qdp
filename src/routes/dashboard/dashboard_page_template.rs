use crate::{
    entities::links::LinksTemplate,
    frontend::{ScriptItemsTemplate, StylesheetItemsTemplate},
};
use askama::Template;

#[derive(Template)]
#[template(path = "routes/dashboard/page.html", escape = "none")]
pub struct DashboardPageTemplate {
    pub title: &'static str,
    pub data: &'static str,
    pub links: LinksTemplate,
    pub global_scripts: ScriptItemsTemplate,
    pub stylesheets: StylesheetItemsTemplate,
    pub local_scripts: ScriptItemsTemplate,
}
