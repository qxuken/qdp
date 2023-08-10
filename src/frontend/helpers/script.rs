use handlebars::{
    Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext, RenderError,
};

use serde::{Deserialize, Serialize};
use serde_json::from_value;

use crate::frontend::ASSETS_PREFIX;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptItem {
    url: String,
    modificator: Option<String>,
    module: bool,
}

impl ScriptItem {
    pub fn new(url: &'static str, modificator: Option<&'static str>, module: bool) -> Self {
        Self {
            url: url.to_string(),
            modificator: modificator.map(String::from),
            module,
        }
    }

    pub fn module(url: &'static str, modificator: Option<&'static str>) -> Self {
        Self::new(url, modificator, true)
    }

    pub fn async_module(url: &'static str) -> Self {
        Self::new(url, Some("async"), true)
    }

    pub fn to_template(self) -> String {
        let modificator = self.modificator.unwrap_or_default();
        let script_type = if self.module { "type=\"module\"" } else { "" };
        format!(
            "<script {} {} src=\"{}{}\"></script>",
            modificator, script_type, ASSETS_PREFIX, self.url
        )
    }
}

#[derive(Clone, Copy)]
pub struct Script;

impl HelperDef for Script {
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
            .and_then(|v| from_value::<ScriptItem>(v.clone()).ok())
        {
            Some(item) => {
                let template: String = item.to_template();
                out.write(&template)?;
                Ok(())
            }
            None => Err(RenderError::new("Url has wrong type for helper Script")),
        }
    }
}
