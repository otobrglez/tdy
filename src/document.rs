use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Document {
    pub namespace: String,
    pub title: Option<String>,
    pub date: DateTime<Utc>,
}

impl Document {
    fn namespace_or_default(namespace: String) -> String {
        Some(namespace)
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| String::from("tdy"))
    }

    fn title_or_default(title: Option<String>) -> Option<String> {
        title
            .filter(|s| !s.is_empty())
            .or_else(|| Some(Utc::now().format("%Y-%m-%d").to_string()))
    }

    fn date_or_default(date: Option<DateTime<Utc>>) -> DateTime<Utc> {
        date.unwrap_or(Utc::now())
    }

    pub fn new(namespace: String, title: Option<String>, date: Option<DateTime<Utc>>) -> Document {
        Document {
            namespace: Self::namespace_or_default(namespace),
            title: Self::title_or_default(title),
            date: Self::date_or_default(date),
        }
    }

    pub fn file_name(&self) -> String {
        format!("{}-{}.md", self.namespace, self.date.format("%Y-%m-%d"))
    }
}
