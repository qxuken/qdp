use std::{
    borrow::Cow,
    collections::HashMap,
    time::{Duration, UNIX_EPOCH},
};

use axum::headers::LastModified;

use super::Assets;

#[derive(Debug, Clone)]
pub struct AssetsMetadata {
    e_tag: String,
    last_modified: LastModified,
}

#[derive(Debug, Clone)]
pub struct AssetsMetadataStore {
    map: HashMap<Cow<'static, str>, AssetsMetadata>,
}

impl AssetsMetadataStore {
    pub fn new(is_dev: bool) -> Self {
        let mut store = AssetsMetadataStore {
            map: HashMap::new(),
        };
        if is_dev {
            return store;
        }
        for asset_path in Assets::iter() {
            let e_tag = store
                .e_tag(&asset_path)
                .expect("Assets should be available");
            let last_modified = store
                .last_modified(&asset_path)
                .expect("Assets should be available");
            store.map.insert(
                asset_path,
                AssetsMetadata {
                    e_tag,
                    last_modified,
                },
            );
        }
        store
    }

    pub fn e_tag(&self, asset_path: &str) -> Option<String> {
        self.map
            .get(asset_path)
            .map(|d| d.e_tag.clone())
            .or_else(|| {
                Assets::get(asset_path).map(|content| hex::encode(content.metadata.sha256_hash()))
            })
    }

    pub fn last_modified(&self, asset_path: &str) -> Option<LastModified> {
        self.map
            .get(asset_path)
            .map(|d| d.last_modified)
            .or_else(|| {
                Assets::get(asset_path)
                    .and_then(|content| content.metadata.last_modified())
                    .map(|lm| UNIX_EPOCH + Duration::from_secs(lm))
                    .map(LastModified::from)
            })
    }
}
