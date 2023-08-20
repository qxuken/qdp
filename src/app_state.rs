use std::sync::Arc;

use crate::{
    frontend::{AssetsMetadataStore, ScriptItem, StylesheetItem},
    Database,
};

#[derive(Debug)]
pub struct AppState {
    pub db: Database,
    pub assets_metadata: AssetsMetadataStore,
    pub global_scripts: Vec<ScriptItem>,
    pub stylesheets: Vec<StylesheetItem>,
}

pub type SharedAppState = Arc<AppState>;

impl AppState {
    pub fn new(db: Database, is_dev: bool) -> Self {
        let mut global_scripts = vec![ScriptItem::async_module("/lib.js")];

        if is_dev {
            global_scripts.push(ScriptItem::async_module("/utils/liveReload.js"));
        }

        let stylesheets = vec![
            StylesheetItem::style("/lib.css"),
            StylesheetItem::preload("/fonts/inter.css"),
        ];

        let assets_metadata = AssetsMetadataStore::new(is_dev);

        Self {
            db,
            assets_metadata,
            global_scripts,
            stylesheets,
        }
    }

    pub fn shared(self) -> SharedAppState {
        Arc::new(self)
    }
}
