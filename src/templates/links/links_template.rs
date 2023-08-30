use askama::Template;
use axum::response::{IntoResponse, Response};

use crate::{entities::links::LinksView, templates::html_template::HtmlTemplate};

#[derive(Template, Debug, Clone)]
#[template(path = "links/links_template.html")]
pub struct LinksTemplate {
    links: Vec<LinksView>,
}

impl From<Vec<LinksView>> for LinksTemplate {
    fn from(value: Vec<LinksView>) -> Self {
        LinksTemplate { links: value }
    }
}

impl IntoResponse for LinksTemplate {
    fn into_response(self) -> Response {
        HtmlTemplate(self).into_response()
    }
}
