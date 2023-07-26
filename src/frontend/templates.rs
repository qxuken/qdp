#![allow(non_upper_case_globals)]

use actix_web::HttpResponse;
use handlebars::Handlebars;
use rust_embed::RustEmbed;
use serde::Serialize;

#[derive(RustEmbed)]
#[folder = "./src"]
#[include = "*.hbs"]
pub struct TemplatesStorage;

#[derive(Clone)]
pub struct Templates<'a> {
    registry: Handlebars<'a>,
}

impl<'a> Templates<'a> {
    pub fn new(dev_mode: bool) -> Self {
        for file in TemplatesStorage::iter() {
            log::trace!("Registered Handlebars template {}", file.as_ref());
        }
        let mut registry = Handlebars::new();
        registry.set_dev_mode(dev_mode);
        log::trace!("Handlebars registry dev mode: {}", registry.dev_mode());
        registry
            .register_embed_templates::<TemplatesStorage>()
            .unwrap();

        Templates { registry }
    }

    pub fn get(&self) -> &Handlebars {
        &self.registry
    }

    pub fn handle<T>(&self, template: &str, data: Option<T>) -> HttpResponse
    where
        T: Serialize,
    {
        match self.get().render(template, &data) {
            Ok(body) => HttpResponse::Ok().body(body),
            Err(err) => HttpResponse::InternalServerError().body(err.desc),
        }
    }
}
