use crate::assets::ASSETS_PREFIX;
use askama::Template;

#[derive(Debug, Clone)]
pub struct ScriptItem {
    url: &'static str,
    modificator: Option<&'static str>,
    module: bool,
}

impl ScriptItem {
    pub fn new(url: &'static str, modificator: Option<&'static str>, module: bool) -> Self {
        Self {
            url,
            modificator,
            module,
        }
    }

    pub fn module(url: &'static str, modificator: Option<&'static str>) -> Self {
        Self::new(url, modificator, true)
    }

    pub fn async_module(url: &'static str) -> Self {
        Self::new(url, Some("async"), true)
    }

    pub fn absolute_url(&self) -> String {
        format!("{}{}", ASSETS_PREFIX, self.url)
    }

    pub fn modificator(&self) -> &'static str {
        self.modificator.unwrap_or("")
    }

    pub fn is_module(&self) -> bool {
        self.module
    }
}

#[derive(Template, Debug, Clone, Default)]
#[template(path = "headers/script_item.html", escape = "none")]
pub struct ScriptItemsTemplate {
    items: Vec<ScriptItem>,
}

impl From<Vec<ScriptItem>> for ScriptItemsTemplate {
    fn from(value: Vec<ScriptItem>) -> Self {
        ScriptItemsTemplate { items: value }
    }
}
