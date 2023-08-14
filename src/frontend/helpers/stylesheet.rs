use handlebars::{
    Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext, RenderError,
};
use serde::{Deserialize, Serialize};
use serde_json::from_value;

use crate::frontend::ASSETS_PREFIX;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StylesheetItem {
    url: String,
    rel: Option<String>,
}

impl StylesheetItem {
    pub fn new(url: &'static str, rel: Option<&'static str>) -> Self {
        Self {
            url: url.to_string(),
            rel: rel.map(String::from),
        }
    }

    pub fn style(url: &'static str) -> Self {
        Self::new(url, None)
    }

    pub fn preload(url: &'static str) -> Self {
        Self::new(url, Some("preload"))
    }

    pub fn to_template(self) -> String {
        let rel = self.rel.unwrap_or("stylesheet".to_owned());
        let url = format!("{}{}", ASSETS_PREFIX, self.url);
        let rel_tag = if rel != "stylesheet" {
            format!("<link rel=\"{}\" href=\"{}\" as=\"style\"/>\n", rel, url)
        } else {
            String::default()
        };
        format!("{}<link rel=\"stylesheet\" href=\"{}\" />", rel_tag, url)
    }
}

#[derive(Clone, Copy)]
pub struct Stylesheet;

impl HelperDef for Stylesheet {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _rc: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        match h
            .param(0)
            .map(|p| p.value())
            .and_then(|v| from_value::<StylesheetItem>(v.clone()).ok())
        {
            Some(item) => {
                let template: String = item.to_template();
                out.write(&template)?;
                Ok(())
            }
            None => Err(RenderError::new("Url has wrong type for helper Style")),
        }
    }
}
