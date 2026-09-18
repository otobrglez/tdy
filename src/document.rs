use crate::date::{self, DATE_FORMAT};
use crate::error::Result;
use chrono::NaiveDate;
use minijinja::{Environment, context};
use std::path::{Path, PathBuf};

pub const DEFAULT_NAMESPACE: &str = "tdy";

const TEMPLATE: &str = "---\ndate: {{ date }}\n---\n# {{ title }}\n";

/// One day's note within a namespace.
#[derive(Debug, PartialEq, Eq)]
pub struct Document {
    pub namespace: String,
    pub title: String,
    pub date: NaiveDate,
}

impl Document {
    /// Builds a document, falling back to the default namespace, today's date
    /// and a date-based title when those are missing or blank.
    pub fn new(
        namespace: impl Into<String>,
        title: Option<String>,
        date: Option<NaiveDate>,
    ) -> Self {
        let namespace = namespace.into();
        let namespace = if namespace.is_empty() {
            DEFAULT_NAMESPACE.to_string()
        } else {
            namespace
        };

        let date = date.unwrap_or_else(date::today);
        let title = title
            .filter(|s| !s.trim().is_empty())
            .unwrap_or_else(|| format_date(date));

        Document {
            namespace,
            title,
            date,
        }
    }

    pub fn file_name(&self) -> String {
        format!("{}-{}.md", self.namespace, format_date(self.date))
    }

    /// Where this document lives inside the `tdy_files` directory.
    pub fn path_in(&self, tdy_files: &Path) -> PathBuf {
        tdy_files.join(self.file_name())
    }

    /// Renders the initial content of a freshly created document.
    pub fn render(&self) -> Result<String> {
        let mut env = Environment::new();
        env.set_keep_trailing_newline(true);
        let content = env.render_str(
            TEMPLATE,
            context! {
                date => format_date(self.date),
                title => &self.title,
            },
        )?;
        Ok(content)
    }
}

fn format_date(date: NaiveDate) -> String {
    date.format(DATE_FORMAT).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dec31() -> NaiveDate {
        NaiveDate::from_ymd_opt(2025, 12, 31).unwrap()
    }

    #[test]
    fn defaults_to_today_and_date_title() {
        let doc = Document::new("test", None, None);
        assert_eq!(doc.namespace, "test");
        assert_eq!(doc.date, date::today());
        assert_eq!(doc.title, format_date(date::today()));
    }

    #[test]
    fn empty_namespace_uses_default() {
        let doc = Document::new("", None, None);
        assert_eq!(doc.namespace, DEFAULT_NAMESPACE);
    }

    #[test]
    fn custom_title_is_kept() {
        let doc = Document::new("work", Some("Meeting Notes".to_string()), None);
        assert_eq!(doc.title, "Meeting Notes");
    }

    #[test]
    fn whitespace_title_falls_back_to_date() {
        let doc = Document::new("work", Some("   ".to_string()), Some(dec31()));
        assert_eq!(doc.title, "2025-12-31");
    }

    #[test]
    fn file_name_and_path() {
        let doc = Document::new("test", None, Some(dec31()));
        assert_eq!(doc.file_name(), "test-2025-12-31.md");
        assert_eq!(
            doc.path_in(Path::new(".days")),
            PathBuf::from(".days/test-2025-12-31.md")
        );
    }

    #[test]
    fn render_uses_date_and_title() {
        let doc = Document::new("test", Some("Test Title".to_string()), Some(dec31()));
        assert_eq!(
            doc.render().unwrap(),
            "---\ndate: 2025-12-31\n---\n# Test Title\n"
        );
    }

    #[test]
    fn render_default_title_is_the_date() {
        let doc = Document::new("work", None, Some(dec31()));
        assert_eq!(
            doc.render().unwrap(),
            "---\ndate: 2025-12-31\n---\n# 2025-12-31\n"
        );
    }
}
