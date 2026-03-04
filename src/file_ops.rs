use crate::document::Document;
use crate::error::Result;
use crate::template;
use log::info;
use std::fs::create_dir_all;
use std::io::Write;
use std::path::PathBuf;
use tempfile::NamedTempFile;

pub fn resolve_path(
    tdy_files: PathBuf,
    namespace: String,
    date: Option<chrono::DateTime<chrono::Utc>>,
) -> Option<PathBuf> {
    let document = Document::new(namespace, None, date);
    let seek_path: PathBuf = tdy_files.join(document.file_name());
    if seek_path.exists() {
        Some(seek_path)
    } else {
        None
    }
}

pub fn create_temp_document(document: &Document) -> Result<NamedTempFile> {
    info!(
        "Creating temp document for date: {:?}, title: {:?}",
        document.date, document.title
    );

    let content = template::render_document(document)?;

    let mut temp_file = NamedTempFile::new()?;
    temp_file.write_all(content.as_bytes())?;
    temp_file.flush()?;

    Ok(temp_file)
}

pub fn save_document(source: &std::path::Path, destination: &std::path::Path) -> Result<()> {
    if let Some(parent) = destination.parent() {
        create_dir_all(parent)?;
    }

    info!("Saving document to: {}", destination.display());
    std::fs::copy(source, destination)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_resolve_path_existing_file() {
        let temp_dir = tempdir().unwrap();
        let namespace = "test";
        let date = Utc.with_ymd_and_hms(2025, 12, 31, 0, 0, 0).unwrap();

        let doc = Document::new(namespace, None, Some(date));
        let file_path = temp_dir.path().join(doc.file_name());
        fs::write(&file_path, "test content").unwrap();

        let result = resolve_path(temp_dir.path().to_path_buf(), namespace.to_string(), Some(date));

        assert!(result.is_some());
        assert_eq!(result.unwrap(), file_path);
    }

    #[test]
    fn test_resolve_path_non_existing_file() {
        let temp_dir = tempdir().unwrap();
        let namespace = "test";
        let date = Utc.with_ymd_and_hms(2025, 12, 31, 0, 0, 0).unwrap();

        let result = resolve_path(temp_dir.path().to_path_buf(), namespace.to_string(), Some(date));

        assert!(result.is_none());
    }

    #[test]
    fn test_create_temp_document() {
        let date = Utc.with_ymd_and_hms(2025, 12, 31, 0, 0, 0).unwrap();
        let doc = Document::new("test", Some("Test Title".to_string()), Some(date));

        let result = create_temp_document(&doc);

        assert!(result.is_ok());
        let temp_file = result.unwrap();
        let content = fs::read_to_string(temp_file.path()).unwrap();
        assert!(content.contains("date: 2025-12-31"));
        assert!(content.contains("# Test Title"));
    }

    #[test]
    fn test_create_temp_document_with_default_title() {
        let date = Utc.with_ymd_and_hms(2025, 6, 15, 0, 0, 0).unwrap();
        let doc = Document::new("work", None, Some(date));

        let result = create_temp_document(&doc);

        assert!(result.is_ok());
        let temp_file = result.unwrap();
        let content = fs::read_to_string(temp_file.path()).unwrap();
        assert!(content.contains("date: 2025-06-15"));
        assert!(content.contains("# 2025-06-15"));
    }

    #[test]
    fn test_save_document() {
        let temp_dir = tempdir().unwrap();
        let source_file = temp_dir.path().join("source.md");
        let dest_file = temp_dir.path().join("subdir").join("dest.md");

        fs::write(&source_file, "test content").unwrap();

        let result = save_document(&source_file, &dest_file);

        assert!(result.is_ok());
        assert!(dest_file.exists());
        let content = fs::read_to_string(&dest_file).unwrap();
        assert_eq!(content, "test content");
    }

    #[test]
    fn test_save_document_creates_parent_dirs() {
        let temp_dir = tempdir().unwrap();
        let source_file = temp_dir.path().join("source.md");
        let dest_file = temp_dir.path().join("nested").join("deep").join("dest.md");

        fs::write(&source_file, "nested test").unwrap();

        let result = save_document(&source_file, &dest_file);

        assert!(result.is_ok());
        assert!(dest_file.exists());
        assert!(dest_file.parent().unwrap().exists());
    }

    #[test]
    fn test_save_document_overwrites_existing() {
        let temp_dir = tempdir().unwrap();
        let source_file = temp_dir.path().join("source.md");
        let dest_file = temp_dir.path().join("dest.md");

        fs::write(&source_file, "new content").unwrap();
        fs::write(&dest_file, "old content").unwrap();

        let result = save_document(&source_file, &dest_file);

        assert!(result.is_ok());
        let content = fs::read_to_string(&dest_file).unwrap();
        assert_eq!(content, "new content");
    }
}
