use serde::Serialize;
use serde_json::{Map, Value};

use super::{ScriptItem, StylesheetItem};

#[derive(Clone, Default, Serialize)]
pub struct TemplateProps {
    pub title: Option<&'static str>,
    pub data: Option<Map<String, Value>>,
    pub global_scripts: Vec<ScriptItem>,
    pub local_scripts: Vec<ScriptItem>,
    pub stylesheets: Vec<StylesheetItem>,
}

impl TemplateProps {
    pub fn merge(mut self, mut other: Self) -> Self {
        self.global_scripts.append(&mut other.global_scripts);
        self.local_scripts.append(&mut other.local_scripts);
        self.stylesheets.append(&mut other.stylesheets);
        let data = if let Some((m1, m2)) = self.data.as_mut().zip(other.data.as_mut()) {
            let mut result = m1.clone();
            result.append(m2);
            Some(result)
        } else {
            self.data.or(other.data)
        };
        Self {
            title: other.title.or(self.title),
            data,
            global_scripts: self.global_scripts,
            local_scripts: self.local_scripts,
            stylesheets: self.stylesheets,
        }
    }
}
