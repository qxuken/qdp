use askama::Template;

use crate::assets::ASSETS_PREFIX;

#[derive(Debug, Clone)]
pub struct PreloadItem {
    url: &'static str,
    attr_as: &'static str,
    attr_type: Option<&'static str>,
}

impl PreloadItem {
    pub fn new(url: &'static str, attr_as: &'static str, attr_type: Option<&'static str>) -> Self {
        Self {
            url,
            attr_as,
            attr_type,
        }
    }

    pub fn style(url: &'static str) -> Self {
        Self::new(url, "style", None)
    }

    pub fn font(url: &'static str, attr_type: Option<&'static str>) -> Self {
        Self::new(url, "font", attr_type)
    }

    pub fn absolute_url(&self) -> String {
        format!("{}{}", ASSETS_PREFIX, self.url)
    }

    pub fn attr_as(&self) -> &'static str {
        self.attr_as
    }

    pub fn attr_type(&self) -> &'static str {
        self.attr_type.unwrap_or_default()
    }
}

#[derive(Template, Clone, Debug, Default)]
#[template(path = "headers/preload_item.html", escape = "none")]
pub struct PreloadItemsTemplate {
    items: Vec<PreloadItem>,
}

impl From<Vec<PreloadItem>> for PreloadItemsTemplate {
    fn from(value: Vec<PreloadItem>) -> Self {
        PreloadItemsTemplate { items: value }
    }
}
