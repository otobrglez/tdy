use crate::document::Document;
use crate::ext::DateFmtExt;
use chrono::{DateTime, Utc};
use log::info;
use minijinja::{Environment as MinininjaEnvironment, context};
use std::env::temp_dir;
use std::fs::{File, copy as fs_copy, create_dir_all};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

const DEFAULT_TEMPLATE: &str = "---\n\
    date: {{ date }}\n\
    ---\n\
    # {{ title }}\n";

pub fn execute(
    editor: String,
    tdy_files: PathBuf,
    namespace: String,
    date: Option<DateTime<Utc>>,
    title: Option<String>,
) {
    let document = Document::new(namespace, title, date);
    let seek_path: PathBuf = tdy_files.join(document.file_name());
    let (new_document, working_document_path) = match seek_path.exists() {
        false => create_temp_document(document).map(|p| (true, p)),
        _ => Ok((false, seek_path.clone())),
    }
    .expect("Failed creating new working document.");

    open_document_with_editor(editor, new_document, seek_path, working_document_path);
}

fn create_temp_document(document: Document) -> Result<PathBuf, String> {
    let mut template_environment = MinininjaEnvironment::new();
    template_environment
        .add_template("today", DEFAULT_TEMPLATE)
        .unwrap();

    info!(
        "Using date: {:?} and title:{:?}",
        document.date, document.title
    );

    let temp_content = template_environment
        .get_template("tdy.md")
        .unwrap()
        .render(context!(
            namespace => document.namespace,
            title => document.title,
            year => document.date.format("%Y").to_string(),
            month => document.date.format("%m").to_string(),
            day => document.date.format("%d").to_string(),
            date => document.date.ymd()
        ))
        .unwrap();

    let mut dir: PathBuf = temp_dir();
    dir.push(document.file_name());
    let temp_file_name = dir.clone().to_str().unwrap().to_string();
    let mut file = File::create(dir).unwrap();
    file.write_all(temp_content.as_bytes()).unwrap();
    Ok(PathBuf::from(temp_file_name))
}

fn open_document_with_editor(
    editor: String,
    new_document: bool,
    seek_path: PathBuf,
    working_document_path: PathBuf,
) {
    let status = Command::new(&editor)
        .arg(&working_document_path)
        .status()
        .expect("Error: Failed to start the editor process.");

    // Copy the file from temp to files when the editor exits successfully
    if status.success() && new_document {
        info!("Written to {}", working_document_path.display());
        if let Some(parent) = seek_path.parent() {
            create_dir_all(parent).unwrap();
        }
        info!("Copying to {}", seek_path.to_str().unwrap());
        fs_copy(&working_document_path, &seek_path).unwrap();
    } else if status.success() && !new_document {
        info!("Saved to {}", working_document_path.display());
    }
}
