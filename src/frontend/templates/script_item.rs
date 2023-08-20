use serde::{Deserialize, Serialize};

use crate::frontend::ASSETS_PREFIX;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
}
