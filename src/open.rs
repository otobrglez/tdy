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

    if let Err(err) = save(draft.path(), &destination) {
        let preserved_path = match draft.keep() {
            Ok((_file, path)) => path,
            Err(e) => return Err(TdyError::Io(e.error)),
        };
        return Err(TdyError::SaveFailed {
            path: destination,
            draft_preserved_at: preserved_path,
            source: match err {
                TdyError::Io(io_err) => io_err,
                _ => std::io::Error::other(err.to_string()),
            },
        });
    }

    Ok(())
}

/// Determines which editor command to use. Checks CLI argument, then $VISUAL,
/// then $EDITOR, and finally falls back to detecting standard editors in $PATH.
pub fn resolve_editor(cli_editor: Option<&str>) -> Result<String> {
    if let Some(cmd) = cli_editor.filter(|s| !s.trim().is_empty()) {
        return Ok(cmd.to_string());
    }

    if let Ok(visual) = std::env::var("VISUAL")
        && !visual.trim().is_empty()
    {
        return Ok(visual);
    }

    if let Ok(editor) = std::env::var("EDITOR")
        && !editor.trim().is_empty()
    {
        return Ok(editor);
    }

    for candidate in ["nano", "vim", "vi"] {
        if command_exists(candidate) {
            return Ok(candidate.to_string());
        }
    }

    Err(TdyError::NoEditorFound)
}

fn command_exists(cmd: &str) -> bool {
    if let Some(path_var) = std::env::var_os("PATH") {
        for dir in std::env::split_paths(&path_var) {
            let candidate = dir.join(cmd);
            if candidate.is_file() {
                return true;
            }
        }
    }
    false
}

/// Splits an editor command string into words, respecting single and double quotes.
pub fn split_command(cmd: &str) -> Result<Vec<String>> {
    let trimmed = cmd.trim();
    if trimmed.is_empty() {
        return Err(TdyError::EditorFailed("Empty editor command".to_string()));
    }

    let mut words = Vec::new();
    let mut current = String::new();
    let mut chars = trimmed.chars().peekable();
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut has_token = false;

    while let Some(c) = chars.next() {
        match c {
            '\\' if !in_single_quote => {
                if let Some(next) = chars.next() {
                    current.push(next);
                    has_token = true;
                } else {
                    current.push('\\');
                    has_token = true;
                }
            }
            '\'' if !in_double_quote => {
                in_single_quote = !in_single_quote;
                has_token = true;
            }
            '"' if !in_single_quote => {
                in_double_quote = !in_double_quote;
                has_token = true;
            }
            c if c.is_whitespace() && !in_single_quote && !in_double_quote => {
                if has_token {
                    words.push(current);
                    current = String::new();
                    has_token = false;
                }
            }
            c => {
                current.push(c);
                has_token = true;
            }
        }
    }

    if in_single_quote || in_double_quote {
        return Err(TdyError::EditorFailed(format!(
            "Unterminated quote in editor command: {cmd}"
        )));
    }

    if has_token {
        words.push(current);
    }

    if words.is_empty() {
        return Err(TdyError::EditorFailed("Empty editor command".to_string()));
    }

    Ok(words)
}

fn write_draft(document: &Document) -> Result<NamedTempFile> {
    let mut draft = tempfile::Builder::new()
        .prefix(&format!("{}-", document.namespace))
        .suffix(".md")
        .tempfile()?;
    draft.write_all(document.render()?.as_bytes())?;
    draft.flush()?;
    Ok(draft)
}

fn run_editor(editor: &str, path: &Path) -> Result<()> {
    let words = split_command(editor)?;
    let (program, args) = words.split_first().expect("non-empty from split_command");

    info!("Opening {} with editor: {}", path.display(), editor);

    let status = Command::new(program)
        .args(args)
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
        let doc = Document::new("test", Some("Test Title".to_string()), Some(dec31())).unwrap();

        open("true", dir.path(), &doc).unwrap();

        let content = fs::read_to_string(dir.path().join("test-2025-12-31.md")).unwrap();
        assert_eq!(content, "---\ndate: 2025-12-31\n---\n# Test Title\n");
    }

    #[test]
    fn creates_missing_directories() {
        let dir = tempdir().unwrap();
        let nested = dir.path().join("nested").join("dirs");
        let doc = Document::new("project", None, Some(dec31())).unwrap();

        open("true", &nested, &doc).unwrap();

        assert!(nested.join("project-2025-12-31.md").exists());
    }

    #[test]
    fn keeps_existing_file_untouched() {
        let dir = tempdir().unwrap();
        let existing = dir.path().join("work-2025-12-31.md");
        fs::write(&existing, "existing content").unwrap();
        let doc = Document::new("work", None, Some(dec31())).unwrap();

        open("true", dir.path(), &doc).unwrap();

        assert_eq!(fs::read_to_string(&existing).unwrap(), "existing content");
    }

    #[test]
    fn missing_editor_is_an_error_and_writes_nothing() {
        let dir = tempdir().unwrap();
        let doc = Document::new("test", None, Some(dec31())).unwrap();

        let result = open("nonexistent_editor_12345", dir.path(), &doc);

        assert!(matches!(result, Err(TdyError::EditorFailed(_))));
        assert!(!dir.path().join("test-2025-12-31.md").exists());
    }

    #[test]
    fn failing_editor_does_not_save_draft() {
        let dir = tempdir().unwrap();
        let doc = Document::new("test", None, Some(dec31())).unwrap();

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

    #[test]
    fn split_command_parses_arguments() {
        assert_eq!(
            split_command("code --wait").unwrap(),
            vec!["code", "--wait"]
        );
        assert_eq!(
            split_command("  subl   -n  -w  ").unwrap(),
            vec!["subl", "-n", "-w"]
        );
    }

    #[test]
    fn split_command_handles_quotes() {
        assert_eq!(
            split_command(r#"editor "file with spaces" 'another arg'"#).unwrap(),
            vec!["editor", "file with spaces", "another arg"]
        );
    }

    #[test]
    fn split_command_rejects_empty_and_unterminated() {
        assert!(split_command("").is_err());
        assert!(split_command("   ").is_err());
        assert!(split_command("editor 'unterminated").is_err());
        assert!(split_command(r#"editor "unterminated"#).is_err());
    }

    #[test]
    fn editor_with_flags_executes_successfully() {
        let dir = tempdir().unwrap();
        let doc = Document::new("test", None, Some(dec31())).unwrap();

        // `sh -c true --` tests multi-arg command execution
        open("sh -c true --", dir.path(), &doc).unwrap();
        assert!(dir.path().join("test-2025-12-31.md").exists());
    }

    #[test]
    fn draft_file_has_md_extension() {
        let doc = Document::new("test", None, Some(dec31())).unwrap();
        let draft = write_draft(&doc).unwrap();
        let path = draft.path();
        assert_eq!(path.extension().and_then(|e| e.to_str()), Some("md"));
    }
}
