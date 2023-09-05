use askama::Template;
use axum::response::{IntoResponse, Response};

use crate::{entities::links::LinkSection, templates::html_template::HtmlTemplate};

#[derive(Template, Debug, Clone)]
#[template(path = "links/section_title.html")]
pub struct SectionTitle {
    section_id: String,
    section_title: String,
}

impl From<LinkSection> for SectionTitle {
    fn from(value: LinkSection) -> Self {
        SectionTitle {
            section_id: value.id.to_string(),
            section_title: value.title,
        }
    }
}

impl IntoResponse for SectionTitle {
    fn into_response(self) -> Response {
        HtmlTemplate(self).into_response()
    }
}
