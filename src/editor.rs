use crate::error::{Result, TdyError};
use log::info;
use std::path::Path;
use std::process::Command;

pub fn open_with_editor(editor: &str, file_path: &Path) -> Result<()> {
    info!("Opening {} with editor: {}", file_path.display(), editor);

    let status = Command::new(editor)
        .arg(file_path)
        .status()
        .map_err(|e| TdyError::EditorFailed(format!("Failed to start editor '{}': {}", editor, e)))?;

    if !status.success() {
        return Err(TdyError::EditorFailed(format!(
            "Editor '{}' exited with non-zero status",
            editor
        )));
    }

    Ok(())
}
