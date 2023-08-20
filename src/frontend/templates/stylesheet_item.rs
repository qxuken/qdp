use serde::{Deserialize, Serialize};

use crate::frontend::ASSETS_PREFIX;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
}
