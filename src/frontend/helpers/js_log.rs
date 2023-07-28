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
        let params: Vec<String> = h
            .params()
            .iter()
            .map(|v| serde_json::to_string(v.value()).unwrap_or("[\"error\"]".to_string()))
            .collect();

        let template = format!("<script>console.log({})</script>", params.join(", "));
        out.write(&template)?;
        Ok(())
    }
}
