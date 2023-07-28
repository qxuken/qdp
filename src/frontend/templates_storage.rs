#![allow(non_upper_case_globals)]

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "./src"]
#[include = "*.hbs"]
pub struct TemplatesStorage;
