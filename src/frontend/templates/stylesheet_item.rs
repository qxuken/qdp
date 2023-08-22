use askama::Template;

use crate::frontend::ASSETS_PREFIX;

#[derive(Debug, Clone)]
pub struct StylesheetItem {
    url: &'static str,
    preload: bool,
}

impl StylesheetItem {
    pub fn new(url: &'static str, preload: bool) -> Self {
        Self { url, preload }
    }

    pub fn style(url: &'static str) -> Self {
        Self::new(url, false)
    }

    pub fn preload(url: &'static str) -> Self {
        Self::new(url, true)
    }

    pub fn absolute_url(&self) -> String {
        format!("{}{}", ASSETS_PREFIX, self.url)
    }

    pub fn is_preload(&self) -> bool {
        self.preload
    }
}

#[derive(Template, Clone, Debug, Default)]
#[template(path = "frontend/templates/stylesheet_item.html", escape = "none")]
pub struct StylesheetItemsTemplate {
    items: Vec<StylesheetItem>,
}

impl From<Vec<StylesheetItem>> for StylesheetItemsTemplate {
    fn from(value: Vec<StylesheetItem>) -> Self {
        StylesheetItemsTemplate { items: value }
    }
}
