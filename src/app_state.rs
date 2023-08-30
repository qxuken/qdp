use crate::{
    assets::AssetsMetadataStore,
    templates::headers::{
        PreloadItem, PreloadItemsTemplate, ScriptItem, ScriptItemsTemplate, StylesheetItem,
        StylesheetItemsTemplate,
    },
    Database,
};
use askama::Template;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AppState {
    pub is_dev: bool,
    pub db: Database,
    pub assets_metadata: AssetsMetadataStore,
    pub global_head: String,
}

pub type SharedAppState = Arc<AppState>;

impl AppState {
    pub fn new(db: Database, is_dev: bool) -> Self {
        let mut global_scripts = vec![ScriptItem::async_module("/lib.js")];
        let mut global_head = String::new();

        if is_dev {
            global_scripts.push(ScriptItem::async_module("/utils/liveReload.js"));
        }

        let global_scripts_string = ScriptItemsTemplate::from(global_scripts).render().unwrap();
        global_head.push_str(&global_scripts_string);

        let stylesheets = vec![
            StylesheetItem::new("/fonts/inter.css"),
            StylesheetItem::new("/lib.css"),
        ];
        let stylesheet_string = StylesheetItemsTemplate::from(stylesheets).render().unwrap();
        global_head.push('\n');
        global_head.push_str(&stylesheet_string);

        let preloads = vec![
            PreloadItem::font(
                "/fonts/inter/inter-roman.var.woff2?v=3.15",
                Some("font/woff2"),
            ),
            PreloadItem::style("/fonts/inter.css"),
        ];
        let preloads_string = PreloadItemsTemplate::from(preloads).render().unwrap();

        global_head.push('\n');
        global_head.push_str(&preloads_string);

        let assets_metadata = AssetsMetadataStore::new(is_dev);

        Self {
            is_dev,
            db,
            assets_metadata,
            global_head,
        }
    }

    pub fn shared(self) -> SharedAppState {
        Arc::new(self)
    }
}
