use crate::date::{self, DATE_FORMAT};
use crate::error::{Result, TdyError};
use chrono::NaiveDate;
use minijinja::syntax::SyntaxConfig;
use minijinja::{Environment, context};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub const DEFAULT_NAMESPACE: &str = "tdy";

const TEMPLATE: &str = "---\ndate: {{ date }}\n---\n# {{ title }}\n";

/// Validates that a namespace contains only alphanumeric characters, underscores, and hyphens.
pub fn validate_namespace(ns: &str) -> Result<()> {
    if ns.is_empty() {
        return Err(TdyError::InvalidNamespace(
            "namespace cannot be empty".to_string(),
        ));
    }
    if !ns
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
    {
        return Err(TdyError::InvalidNamespace(format!(
            "'{ns}' contains invalid characters; only letters, digits, '_' and '-' are allowed"
        )));
    }
    Ok(())
}

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
    ) -> Result<Self> {
        let namespace = namespace_or_default(namespace.into());
        validate_namespace(&namespace)?;
        let date = date.unwrap_or_else(date::today);
        let title = title
            .filter(|s| !s.trim().is_empty())
            .unwrap_or_else(|| format_date(date));

        Ok(Document {
            namespace,
            title,
            date,
        })
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
        env.set_syntax(
            SyntaxConfig::builder()
                .keep_trailing_newline(true)
                .build()?,
        );
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

/// The newest date, no later than `until`, that has a document of `namespace`
/// in `tdy_files`. Dates are read back from the file names.
pub fn latest_date(
    tdy_files: &Path,
    namespace: &str,
    until: NaiveDate,
) -> Result<Option<NaiveDate>> {
    let namespace = namespace_or_default(namespace.to_string());
    validate_namespace(&namespace)?;

    let entries = match fs::read_dir(tdy_files) {
        Ok(entries) => entries,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(e.into()),
    };

    let prefix = format!("{namespace}-");
    let mut latest = None;
    for entry in entries.flatten() {
        if !entry.file_type().map(|t| t.is_file()).unwrap_or(false) {
            continue;
        }
        let file_name = entry.file_name();
        let date = file_name
            .to_str()
            .and_then(|name| name.strip_prefix(&prefix))
            .and_then(|rest| rest.strip_suffix(".md"))
            .and_then(|raw| NaiveDate::parse_from_str(raw, DATE_FORMAT).ok())
            .filter(|date| *date <= until);
        latest = latest.max(date);
    }

    Ok(latest)
}

fn namespace_or_default(namespace: String) -> String {
    if namespace.trim().is_empty() {
        DEFAULT_NAMESPACE.to_string()
    } else {
        namespace
    }
}

fn format_date(date: NaiveDate) -> String {
    date.format(DATE_FORMAT).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn dec31() -> NaiveDate {
        NaiveDate::from_ymd_opt(2025, 12, 31).unwrap()
    }

    #[test]
    fn defaults_to_today_and_date_title() {
        let doc = Document::new("test", None, None).unwrap();
        assert_eq!(doc.namespace, "test");
        assert_eq!(doc.date, date::today());
        assert_eq!(doc.title, format_date(date::today()));
    }

    #[test]
    fn empty_namespace_uses_default() {
        let doc = Document::new("", None, None).unwrap();
        assert_eq!(doc.namespace, DEFAULT_NAMESPACE);
    }

    #[test]
    fn invalid_namespaces_are_rejected() {
        for invalid in ["../foo", "foo/bar", "foo bar", "work!", "ns@1"] {
            assert!(
                Document::new(invalid, None, None).is_err(),
                "{invalid:?} should be rejected"
            );
        }
    }

    #[test]
    fn custom_title_is_kept() {
        let doc = Document::new("work", Some("Meeting Notes".to_string()), None).unwrap();
        assert_eq!(doc.title, "Meeting Notes");
    }

    #[test]
    fn whitespace_title_falls_back_to_date() {
        let doc = Document::new("work", Some("   ".to_string()), Some(dec31())).unwrap();
        assert_eq!(doc.title, "2025-12-31");
    }

    #[test]
    fn file_name_and_path() {
        let doc = Document::new("test", None, Some(dec31())).unwrap();
        assert_eq!(doc.file_name(), "test-2025-12-31.md");
        assert_eq!(
            doc.path_in(Path::new(".days")),
            PathBuf::from(".days/test-2025-12-31.md")
        );
    }

    #[test]
    fn render_uses_date_and_title() {
        let doc = Document::new("test", Some("Test Title".to_string()), Some(dec31())).unwrap();
        assert_eq!(
            doc.render().unwrap(),
            "---\ndate: 2025-12-31\n---\n# Test Title\n"
        );
    }

    #[test]
    fn render_default_title_is_the_date() {
        let doc = Document::new("work", None, Some(dec31())).unwrap();
        assert_eq!(
            doc.render().unwrap(),
            "---\ndate: 2025-12-31\n---\n# 2025-12-31\n"
        );
    }

    #[test]
    fn latest_date_picks_newest_document_of_namespace() {
        let dir = tempdir().unwrap();
        for name in [
            "work-2025-12-01.md",
            "work-2025-12-24.md",
            "work-2026-01-05.md",
            "work-old-2025-12-30.md",
            "tdy-2025-12-30.md",
            "work-notes.md",
            "work-2025-12-29.txt",
        ] {
            fs::write(dir.path().join(name), "").unwrap();
        }

        assert_eq!(
            latest_date(dir.path(), "work", dec31()).unwrap(),
            NaiveDate::from_ymd_opt(2025, 12, 24)
        );
    }

    #[test]
    fn latest_date_includes_the_until_day() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("work-2025-12-31.md"), "").unwrap();

        assert_eq!(
            latest_date(dir.path(), "work", dec31()).unwrap(),
            Some(dec31())
        );
    }

    #[test]
    fn latest_date_empty_namespace_uses_default() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("tdy-2025-12-30.md"), "").unwrap();

        assert_eq!(
            latest_date(dir.path(), "", dec31()).unwrap(),
            NaiveDate::from_ymd_opt(2025, 12, 30)
        );
    }

    #[test]
    fn latest_date_without_documents_is_none() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("tdy-2025-12-30.md"), "").unwrap();

        assert_eq!(latest_date(dir.path(), "work", dec31()).unwrap(), None);
        assert_eq!(
            latest_date(&dir.path().join("missing"), "work", dec31()).unwrap(),
            None
        );
    }
}
