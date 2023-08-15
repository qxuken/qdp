#![allow(non_upper_case_globals)]

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "./src"]
#[include = "*.hbs"]
pub struct TemplatesStorage;

impl TemplatesStorage {
    pub(super) fn project_root_names() -> Vec<(String, String)> {
        TemplatesStorage::iter()
            .map(|path| {
                let mut root_path = path.to_string();
                root_path.insert_str(0, "src/");
                (root_path, path.to_string())
            })
            .collect()
    }
}
