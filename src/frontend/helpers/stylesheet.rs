use handlebars::{
    Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext, RenderError,
};
use serde_json::Value;

use crate::frontend::ASSETS_PREFIX;

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
            .ok_or(RenderError::new("Param not found for helper Stylesheet"))?
            .value()
        {
            Value::String(stylesheet_url) => {
                let template =
                    format!("<link rel=\"stylesheet\" href=\"{ASSETS_PREFIX}{stylesheet_url}\" />");
                out.write(&template)?;
                Ok(())
            }
            _ => Err(RenderError::new(
                "Param has wrong type for helper Stylesheet",
            )),
        }
    }
}
