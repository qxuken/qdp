#![allow(non_upper_case_globals)]
mod assets_metadata;

pub use assets_metadata::{AssetsMetadata, AssetsMetadataStore};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "./dist"]
pub struct Assets;

pub const ASSETS_PREFIX: &str = "/assets";
pub const ASSETS_PATH: &str = "/assets/*assets_path";
