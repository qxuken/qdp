#![allow(non_upper_case_globals)]

use actix_web::HttpResponse;
use handlebars::Handlebars;
use rust_embed::RustEmbed;
use serde_json::{json, Map, Value};

use super::helpers;

#[derive(RustEmbed)]
#[folder = "./src"]
#[include = "*.hbs"]
pub struct TemplatesStorage;

#[derive(Clone)]
pub struct GlobalProps {
    application_title: String,
}

#[derive(Clone)]
pub struct Templates<'a> {
    registry: Handlebars<'a>,
    global_props: GlobalProps,
}

impl<'a> Templates<'a> {
    pub fn new() -> Self {
        let mut registry = Handlebars::new();
        helpers::register_helpers(&mut registry);
        registry
            .register_embed_templates::<TemplatesStorage>()
            .unwrap();

        let global_props = GlobalProps {
            application_title: "QDP".to_string(),
        };

        Templates {
            registry,
            global_props,
        }
    }

    pub fn get(&self) -> &Handlebars {
        &self.registry
    }

    pub fn template_props(&self, input: Option<Map<String, Value>>) -> Map<String, Value> {
        let mut data = Map::default();
        if let Some(mut value) = input {
            data.append(&mut value);
        }
        if let Some(Value::String(value)) = data.get("title") {
            data.insert(
                "page_title".to_string(),
                Value::String(format!(
                    "{} - {}",
                    self.global_props.application_title, value
                )),
            );
        } else {
            data.insert(
                "title".to_string(),
                json!(self.global_props.application_title.to_string()),
            );
            data.insert(
                "page_title".to_string(),
                json!(self.global_props.application_title.to_string()),
            );
        }
        data
    }

    pub fn handle(&self, template: &str, input: Option<Map<String, Value>>) -> HttpResponse {
        let data = self.template_props(input);
        match self.get().render(template, &data) {
            Ok(body) => HttpResponse::Ok().body(body),
            Err(err) => HttpResponse::InternalServerError().body(err.desc),
        }
    }
}
