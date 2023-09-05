use askama::Template;
use axum::response::{IntoResponse, Response};

use crate::{templates::html_template::HtmlTemplate, SharedAppState};

#[derive(Template, Debug)]
#[template(path = "pages/default_page.html", escape = "none")]
pub struct DefaultPage {
    title: &'static str,
    global_head: String,
    content: String,
    local_scripts: String,
}

impl DefaultPage {
    pub fn new(
        title: &'static str,
        app_state: SharedAppState,
        content: String,
        local_scripts: Option<String>,
    ) -> Self {
        Self {
            title,
            content,
            global_head: app_state.global_head.clone(),
            local_scripts: local_scripts.unwrap_or_default(),
        }
    }
    pub fn with_template(
        title: &'static str,
        app_state: SharedAppState,
        template: impl Template,
        local_scripts: Option<String>,
    ) -> Self {
        match template.render() {
            Ok(content) => Self {
                title,
                content,
                global_head: app_state.global_head.clone(),
                local_scripts: local_scripts.unwrap_or_default(),
            },
            Err(err) => Self {
                title,
                content: err.to_string(),
                global_head: app_state.global_head.clone(),
                local_scripts: local_scripts.unwrap_or_default(),
            },
        }
    }
}

impl IntoResponse for DefaultPage {
    fn into_response(self) -> Response {
        HtmlTemplate(self).into_response()
    }
}
