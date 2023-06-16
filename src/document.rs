use chrono::{DateTime, Utc};
use std::ops::Not;

#[derive(Debug)]
pub struct Document {
    pub namespace: String,
    pub title: Option<String>,
    pub date: DateTime<Utc>,
}

impl Document {
    pub fn from(namespace: String, title: String, date: Option<DateTime<Utc>>) -> Document {
        Document {
            namespace: if namespace.is_empty() {
                "tdy".to_string()
            } else {
                namespace
            },
            title: title.is_empty().not().then_some(title),
            date: date.unwrap_or(Utc::now()),
        }
    }

    pub fn file_name(&self) -> String {
        format!("{}-{}.md", self.namespace, self.date.format("%Y-%m-%d"))
    }

    pub fn safe_title(&self) -> String {
        self.title
            .clone()
            .unwrap_or(self.date.format("%Y-%m-%d").to_string())
    }
}
