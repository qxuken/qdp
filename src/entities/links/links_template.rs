use askama::Template;

use super::LinksView;

#[derive(Template, Debug, Clone)]
#[template(path = "entities/links/links.html", escape = "none")]
pub struct LinksTemplate {
    links: Vec<LinksView>,
}

impl From<Vec<LinksView>> for LinksTemplate {
    fn from(value: Vec<LinksView>) -> Self {
        LinksTemplate { links: value }
    }
}
