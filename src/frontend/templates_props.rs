use serde::Serialize;
use serde_json::{Map, Value};

#[derive(Clone, Default, Serialize)]
pub struct TemplateProps {
    pub title: Option<String>,
    pub data: Option<Map<String, Value>>,
    pub scripts: Vec<(String, Option<&'static str>)>,
    pub stylesheets: Vec<String>,
}

impl TemplateProps {
    pub fn merge(mut self, mut other: Self) -> Self {
        self.scripts.append(&mut other.scripts);
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
            scripts: self.scripts,
            stylesheets: self.stylesheets,
        }
    }
}
