use crate::document::Document;
use crate::editor;
use crate::error::Result;
use crate::file_ops;
use chrono::{DateTime, Utc};
use log::info;
use std::path::PathBuf;

pub fn resolve_path(
    tdy_files: PathBuf,
    namespace: String,
    date: Option<DateTime<Utc>>,
) -> Option<PathBuf> {
    file_ops::resolve_path(tdy_files, namespace, date)
}

pub fn execute(
    editor: String,
    tdy_files: PathBuf,
    namespace: String,
    date: Option<DateTime<Utc>>,
    title: Option<String>,
) -> Result<()> {
    let document = Document::new(namespace, title, date);
    let destination_path = tdy_files.join(document.file_name());

    if destination_path.exists() {
        info!("Opening existing document: {}", destination_path.display());
        editor::open_with_editor(&editor, &destination_path)?;
    } else {
        info!("Creating new document: {}", destination_path.display());
        let temp_file = file_ops::create_temp_document(&document)?;
        let temp_path = temp_file.path();

        editor::open_with_editor(&editor, temp_path)?;
        file_ops::save_document(temp_path, &destination_path)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_resolve_path_delegates_to_file_ops() {
        let temp_dir = tempdir().unwrap();
        let namespace = "test";
        let date = Utc.with_ymd_and_hms(2025, 12, 31, 0, 0, 0).unwrap();

        let doc = Document::new(namespace, None, Some(date));
        let file_path = temp_dir.path().join(doc.file_name());
        fs::write(&file_path, "test").unwrap();

        let result = resolve_path(temp_dir.path().to_path_buf(), namespace.to_string(), Some(date));

        assert!(result.is_some());
        assert_eq!(result.unwrap(), file_path);
    }

    #[test]
    fn test_resolve_path_returns_none_for_missing_file() {
        let temp_dir = tempdir().unwrap();
        let result = resolve_path(
            temp_dir.path().to_path_buf(),
            "nonexistent".to_string(),
            Some(Utc::now()),
        );
        assert!(result.is_none());
    }

    #[test]
    fn test_execute_with_mock_editor_creates_new_file() {
        let temp_dir = tempdir().unwrap();
        let date = Utc.with_ymd_and_hms(2025, 12, 31, 0, 0, 0).unwrap();
        let mock_editor = "true";

        let result = execute(
            mock_editor.to_string(),
            temp_dir.path().to_path_buf(),
            "test".to_string(),
            Some(date),
            Some("Test Title".to_string()),
        );

        assert!(result.is_ok());
        let expected_file = temp_dir.path().join("test-2025-12-31.md");
        assert!(expected_file.exists());

        let content = fs::read_to_string(&expected_file).unwrap();
        assert!(content.contains("date: 2025-12-31"));
        assert!(content.contains("# Test Title"));
    }

    #[test]
    fn test_execute_with_mock_editor_opens_existing_file() {
        let temp_dir = tempdir().unwrap();
        let date = Utc.with_ymd_and_hms(2025, 12, 31, 0, 0, 0).unwrap();
        let file_path = temp_dir.path().join("work-2025-12-31.md");
        fs::write(&file_path, "existing content").unwrap();

        let mock_editor = "true";

        let result = execute(
            mock_editor.to_string(),
            temp_dir.path().to_path_buf(),
            "work".to_string(),
            Some(date),
            None,
        );

        assert!(result.is_ok());
        assert!(file_path.exists());
    }

    #[test]
    fn test_execute_with_invalid_editor_returns_error() {
        let temp_dir = tempdir().unwrap();
        let date = Utc.with_ymd_and_hms(2025, 12, 31, 0, 0, 0).unwrap();

        let result = execute(
            "nonexistent_editor_12345".to_string(),
            temp_dir.path().to_path_buf(),
            "test".to_string(),
            Some(date),
            None,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_execute_creates_directory_structure() {
        let temp_dir = tempdir().unwrap();
        let nested_dir = temp_dir.path().join("nested").join("dirs");
        let date = Utc.with_ymd_and_hms(2025, 1, 15, 0, 0, 0).unwrap();

        let result = execute(
            "true".to_string(),
            nested_dir.clone(),
            "project".to_string(),
            Some(date),
            Some("Planning".to_string()),
        );

        assert!(result.is_ok());
        let expected_file = nested_dir.join("project-2025-01-15.md");
        assert!(expected_file.exists());
    }

    #[test]
    fn test_execute_with_default_namespace() {
        let temp_dir = tempdir().unwrap();
        let date = Utc.with_ymd_and_hms(2025, 3, 10, 0, 0, 0).unwrap();

        let result = execute(
            "true".to_string(),
            temp_dir.path().to_path_buf(),
            "".to_string(),
            Some(date),
            None,
        );

        assert!(result.is_ok());
        let expected_file = temp_dir.path().join("tdy-2025-03-10.md");
        assert!(expected_file.exists());
    }
}
