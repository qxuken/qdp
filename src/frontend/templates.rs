#![allow(non_upper_case_globals)]

use actix_web::HttpResponse;
use handlebars::Handlebars;
use rust_embed::RustEmbed;
use serde_json::Value;

#[derive(RustEmbed)]
#[folder = "./src"]
#[include = "*.hbs"]
pub struct Templates;

#[derive(Clone)]
pub struct TemplatesRegistry<'a> {
    registry: Handlebars<'a>,
}

impl<'a> TemplatesRegistry<'a> {
    pub fn new() -> Self {
        for file in Templates::iter() {
            log::trace!("Registered Handlebars template {}", file.as_ref());
        }
        let mut registry = Handlebars::new();
        registry.register_embed_templates::<Templates>().unwrap();
        registry.set_dev_mode(true);

        TemplatesRegistry { registry }
    }

    pub fn get(&self) -> &Handlebars {
        &self.registry
    }

    pub fn handle(&self, template: &str, data: Option<&Value>) -> HttpResponse {
        match self
            .get()
            .render(template, data.unwrap_or(&Value::default()))
        {
            Ok(body) => HttpResponse::Ok().body(body),
            Err(err) => HttpResponse::InternalServerError().body(err.desc),
        }
    }
}
