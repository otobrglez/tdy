use crate::document::Document;
use crate::error::{Result, TdyError};
use log::info;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use tempfile::NamedTempFile;

/// Opens the document in `editor`. An existing file is edited in place. A new
/// one is drafted from the template in a temporary file and only copied into
/// `tdy_files` after the editor exits successfully, so an aborted edit leaves
/// nothing behind.
pub fn open(editor: &str, tdy_files: &Path, document: &Document) -> Result<()> {
    let destination = document.path_in(tdy_files);

    if destination.exists() {
        info!("Opening existing document: {}", destination.display());
        return run_editor(editor, &destination);
    }

    info!("Creating new document: {}", destination.display());
    let draft = write_draft(document)?;
    run_editor(editor, draft.path())?;
    save(draft.path(), &destination)
}

fn write_draft(document: &Document) -> Result<NamedTempFile> {
    let mut draft = NamedTempFile::new()?;
    draft.write_all(document.render()?.as_bytes())?;
    draft.flush()?;
    Ok(draft)
}

fn run_editor(editor: &str, path: &Path) -> Result<()> {
    info!("Opening {} with editor: {}", path.display(), editor);

    let status = Command::new(editor)
        .arg(path)
        .status()
        .map_err(|e| TdyError::EditorFailed(format!("Failed to start editor '{editor}': {e}")))?;

    if !status.success() {
        return Err(TdyError::EditorFailed(format!(
            "Editor '{editor}' exited with {status}"
        )));
    }

    Ok(())
}

fn save(source: &Path, destination: &Path) -> Result<()> {
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)?;
    }

    info!("Saving document to: {}", destination.display());
    fs::copy(source, destination)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use tempfile::tempdir;

    fn dec31() -> NaiveDate {
        NaiveDate::from_ymd_opt(2025, 12, 31).unwrap()
    }

    #[test]
    fn creates_new_file_from_template() {
        let dir = tempdir().unwrap();
        let doc = Document::new("test", Some("Test Title".to_string()), Some(dec31()));

        open("true", dir.path(), &doc).unwrap();

        let content = fs::read_to_string(dir.path().join("test-2025-12-31.md")).unwrap();
        assert_eq!(content, "---\ndate: 2025-12-31\n---\n# Test Title\n");
    }

    #[test]
    fn creates_missing_directories() {
        let dir = tempdir().unwrap();
        let nested = dir.path().join("nested").join("dirs");
        let doc = Document::new("project", None, Some(dec31()));

        open("true", &nested, &doc).unwrap();

        assert!(nested.join("project-2025-12-31.md").exists());
    }

    #[test]
    fn keeps_existing_file_untouched() {
        let dir = tempdir().unwrap();
        let existing = dir.path().join("work-2025-12-31.md");
        fs::write(&existing, "existing content").unwrap();
        let doc = Document::new("work", None, Some(dec31()));

        open("true", dir.path(), &doc).unwrap();

        assert_eq!(fs::read_to_string(&existing).unwrap(), "existing content");
    }

    #[test]
    fn missing_editor_is_an_error_and_writes_nothing() {
        let dir = tempdir().unwrap();
        let doc = Document::new("test", None, Some(dec31()));

        let result = open("nonexistent_editor_12345", dir.path(), &doc);

        assert!(matches!(result, Err(TdyError::EditorFailed(_))));
        assert!(!dir.path().join("test-2025-12-31.md").exists());
    }

    #[test]
    fn failing_editor_does_not_save_draft() {
        let dir = tempdir().unwrap();
        let doc = Document::new("test", None, Some(dec31()));

        let result = open("false", dir.path(), &doc);

        assert!(matches!(result, Err(TdyError::EditorFailed(_))));
        assert!(!dir.path().join("test-2025-12-31.md").exists());
    }

    #[test]
    fn save_overwrites_existing_destination() {
        let dir = tempdir().unwrap();
        let source = dir.path().join("source.md");
        let destination = dir.path().join("dest.md");
        fs::write(&source, "new content").unwrap();
        fs::write(&destination, "old content").unwrap();

        save(&source, &destination).unwrap();

        assert_eq!(fs::read_to_string(&destination).unwrap(), "new content");
    }
}
