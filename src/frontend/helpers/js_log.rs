use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext};

#[derive(Clone, Copy)]
pub struct JSLog;

impl HelperDef for JSLog {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _rc: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let param = h.param(0).unwrap();

        let value = serde_json::to_string(&param.value()).unwrap_or("[\"error\"]".to_string());
        let template = format!("<script>console.log({value})</script>");
        out.write(&template)?;
        Ok(())
    }
}
