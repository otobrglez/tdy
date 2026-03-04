use crate::constants::DEFAULT_NAMESPACE;
use crate::ext::DateFmtExt;
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Document {
    pub namespace: String,
    pub title: Option<String>,
    pub date: DateTime<Utc>,
}

impl Document {
    pub fn new(
        namespace: impl Into<String>,
        title: Option<String>,
        date: Option<DateTime<Utc>>,
    ) -> Self {
        let namespace = namespace.into();
        let namespace = if namespace.is_empty() {
            DEFAULT_NAMESPACE.to_string()
        } else {
            namespace
        };

        let date = date.unwrap_or_else(Utc::now);
        let title = title
            .filter(|s| !s.trim().is_empty())
            .or_else(|| Some(date.ymd()));

        Document {
            namespace,
            title,
            date,
        }
    }

    pub fn file_name(&self) -> String {
        format!("{}-{}.md", self.namespace, self.date.ymd())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_document_new_with_defaults() {
        let doc = Document::new("test", None, None);
        assert_eq!(doc.namespace, "test");
        assert!(doc.title.is_some());
    }

    #[test]
    fn test_document_empty_namespace_uses_default() {
        let doc = Document::new("", None, None);
        assert_eq!(doc.namespace, DEFAULT_NAMESPACE);
    }

    #[test]
    fn test_document_with_custom_title() {
        let doc = Document::new("work", Some("Meeting Notes".to_string()), None);
        assert_eq!(doc.namespace, "work");
        assert_eq!(doc.title, Some("Meeting Notes".to_string()));
    }

    #[test]
    fn test_document_whitespace_title_ignored() {
        let doc = Document::new("work", Some("   ".to_string()), None);
        assert!(doc.title.is_some());
        assert_ne!(doc.title, Some("   ".to_string()));
    }

    #[test]
    fn test_document_file_name() {
        let date = Utc.with_ymd_and_hms(2025, 12, 31, 0, 0, 0).unwrap();
        let doc = Document::new("test", None, Some(date));
        assert_eq!(doc.file_name(), "test-2025-12-31.md");
    }
}
