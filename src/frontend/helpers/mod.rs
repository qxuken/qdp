mod js_log;
use handlebars::{handlebars_helper, Handlebars};
use serde_json::Value;

handlebars_helper!(stringify: |v: Value| serde_json::to_string(&v).unwrap_or("[error]".to_string()));

pub fn register_helpers(hb: &mut Handlebars) -> () {
    hb.register_helper("stringify", Box::new(stringify));
    hb.register_helper("js_log", Box::new(js_log::JSLog));
}