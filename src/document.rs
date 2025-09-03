use crate::ext::DateFmtExt;
use chrono::{DateTime, Utc};

const DEFAULT_NAMESPACE: &str = "tdy";

#[derive(Debug)]
pub struct Document {
    pub namespace: String,
    pub title: Option<String>,
    pub date: DateTime<Utc>,
}

impl Document {
    pub fn new(
        namespace: String,
        title: Option<String>,
        maybe_date: Option<DateTime<Utc>>,
    ) -> Document {
        Document {
            namespace: Self::namespace_or_default(namespace),
            title: Self::title_or_default(title, maybe_date),
            date: Self::date_or_now(maybe_date),
        }
    }

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
            .and_then(|s| if s.trim().is_empty() { None } else { Some(s) })
            .or_else(|| Some(maybe_date.unwrap_or_else(Utc::now).ymd()))
    }

    fn date_or_now(date: Option<DateTime<Utc>>) -> DateTime<Utc> {
        date.unwrap_or(Utc::now())
    }

    pub fn file_name(&self) -> String {
        format!("{}-{}.md", self.namespace, self.date.ymd())
    }
}
