use handlebars::{
    Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext, RenderError,
};
use serde_json::Value;

use crate::frontend::ASSETS_PREFIX;

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
        let modificator = if let Some(Value::String(modif)) = h.param(1).map(|v| v.value()) {
            modif
        } else {
            ""
        };
        match h
            .param(0)
            .ok_or(RenderError::new("Url not found for helper Script"))?
            .value()
        {
            Value::String(script_url) => {
                let template: String =
                    format!("<script {modificator} type=\"module\" src={ASSETS_PREFIX}{script_url}></script>");
                out.write(&template)?;
                Ok(())
            }
            _ => Err(RenderError::new("Url has wrong type for helper Script")),
        }
    }
}
