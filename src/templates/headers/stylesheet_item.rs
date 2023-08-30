use askama::Template;

use crate::assets::ASSETS_PREFIX;

#[derive(Debug, Clone)]
pub struct StylesheetItem {
    url: &'static str,
}

impl StylesheetItem {
    pub fn new(url: &'static str) -> Self {
        Self { url }
    }

    pub fn absolute_url(&self) -> String {
        format!("{}{}", ASSETS_PREFIX, self.url)
    }
}

#[derive(Template, Clone, Debug, Default)]
#[template(path = "headers/stylesheet_item.html", escape = "none")]
pub struct StylesheetItemsTemplate {
    items: Vec<StylesheetItem>,
}

impl From<Vec<StylesheetItem>> for StylesheetItemsTemplate {
    fn from(value: Vec<StylesheetItem>) -> Self {
        StylesheetItemsTemplate { items: value }
    }
}
