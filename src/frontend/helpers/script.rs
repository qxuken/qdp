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
        match h
            .param(0)
            .ok_or(RenderError::new("Param not found for helper Script"))?
            .value()
        {
            Value::String(script_url) => {
                let template: String = format!("<script src={ASSETS_PREFIX}{script_url}></script>");
                out.write(&template)?;
                Ok(())
            }
            _ => Err(RenderError::new("Param has wrong type for helper Script")),
        }
    }
}
