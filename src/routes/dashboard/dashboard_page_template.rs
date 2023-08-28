use crate::{entities::links::LinksTemplate, frontend::ScriptItemsTemplate};
use askama::Template;

#[derive(Template)]
#[template(path = "routes/dashboard/page.html", escape = "none")]
pub struct DashboardPageTemplate {
    pub title: &'static str,
    pub data: &'static str,
    pub links: LinksTemplate,
    pub global_scripts: String,
    pub stylesheets: String,
    pub local_scripts: ScriptItemsTemplate,
}
