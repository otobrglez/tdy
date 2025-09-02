use chrono::{DateTime, Utc};

const DATE_FORMAT: &str = "%Y-%m-%d";
const DEFAULT_NAMESPACE: &str = "tdy";

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
            .unwrap_or_else(|| String::from(DEFAULT_NAMESPACE))
    }

    fn title_or_default(
        title: Option<String>,
        maybe_date: Option<DateTime<Utc>>,
    ) -> Option<String> {
        title
            .and_then(|s| {
                let is_empty = s.trim().is_empty();
                if is_empty { None } else { Some(s) }
            })
            .or_else(|| {
                let date = maybe_date.unwrap_or_else(Utc::now);
                Some(date.format(DATE_FORMAT).to_string())
            })
    }

    fn date_or_default(date: Option<DateTime<Utc>>) -> DateTime<Utc> {
        date.unwrap_or(Utc::now())
    }

    pub fn new(namespace: String, title: Option<String>, date: Option<DateTime<Utc>>) -> Document {
        Document {
            namespace: Self::namespace_or_default(namespace),
            title: Self::title_or_default(title, date),
            date: Self::date_or_default(date),
        }
    }

    pub fn file_name(&self) -> String {
        format!("{}-{}.md", self.namespace, self.date.format(DATE_FORMAT))
    }
}
