use actix_web::HttpResponse;
use handlebars::Handlebars;

use super::{assets::register_assets, helpers, templates_storage::TemplatesStorage, TemplateProps};

#[derive(Clone)]
pub struct Templates<'a> {
    registry: Handlebars<'a>,
    pub global_props: TemplateProps,
}

impl<'a> Templates<'a> {
    pub fn new() -> Self {
        let mut registry = Handlebars::new();
        helpers::register_helpers(&mut registry);
        registry
            .register_embed_templates::<TemplatesStorage>()
            .unwrap();

        let mut global_props = TemplateProps::default();
        global_props.title = Some("QDP".to_string());
        register_assets(&mut global_props);

        Templates {
            registry,
            global_props,
        }
    }

    pub fn get(&self) -> &Handlebars {
        &self.registry
    }

    pub fn handle(&self, template: &str, input: Option<TemplateProps>) -> HttpResponse {
        let data = if let Some(value) = input {
            value.merge(self.global_props.clone())
        } else {
            self.global_props.clone()
        };
        match self.get().render(template, &data) {
            Ok(body) => HttpResponse::Ok().body(body),
            Err(err) => HttpResponse::InternalServerError().body(err.desc),
        }
    }
}
