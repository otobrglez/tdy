use crate::document::Document;
use chrono::{DateTime, Utc};
use minijinja::{context, Environment as MinininjaEnvironment};
use std::env::temp_dir;
use std::fs::{copy as fs_copy, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

pub fn execute(
    shell: String,
    editor: String,
    tdy_files: PathBuf,
    namespace: String,
    date: Option<DateTime<Utc>>,
    title: String,
) {
    let document = Document::from(namespace, title, date);
    let seek_path: PathBuf = tdy_files.join(document.file_name());
    let (new_document, working_document_path) = match seek_path.exists() {
        false => create_temp_document(document).map(|p| (true, p)),
        _ => Ok((false, seek_path.clone())),
    }
    .expect("Failed creating new working document.");

    open_document_with_editor(
        shell,
        editor,
        new_document,
        seek_path,
        working_document_path,
    );
}

fn create_temp_document(document: Document) -> Result<PathBuf, String> {
    // Prepare the template
    let default_template = "---\n\
                                      date: {{date}}\n\
                                      ---\n\
                                      # {{ title }}\n";
    // Render the template
    let mut template_environment = MinininjaEnvironment::new();
    template_environment
        .add_template("today", default_template)
        .unwrap();

    let temp_content = template_environment
        .get_template("today")
        .unwrap()
        .render(context!(
            namespace => document.namespace,
            title => document.title,
            year => document.date.format("%Y").to_string(),
            month => document.date.format("%m").to_string(),
            day => document.date.format("%d").to_string(),
            date => document.date.format("%Y-%m-%d").to_string()
        ))
        .expect("Rendering has failed.");

    let mut dir: PathBuf = temp_dir();
    dir.push(document.file_name());
    let temp_file_name = dir.clone().to_str().unwrap().to_string();
    let mut file = File::create(dir).unwrap();
    file.write_all(temp_content.as_bytes()).unwrap();
    Ok(PathBuf::from(temp_file_name))
}

fn open_document_with_editor(
    shell: String,
    editor: String,
    new_document: bool,
    seek_path: PathBuf,
    working_document_path: PathBuf,
) {
    // Open the editor
    let exit_status = Command::new(shell)
        .arg("-c")
        .arg(format!(
            "{} {}",
            editor,
            working_document_path.to_str().unwrap()
        ))
        .spawn()
        .expect("Error: Failed spawning the editor.")
        .wait()
        .expect("Error: Editor returned a non-zero status code.");

    // Copy the file from temp to files
    if let Some(code) = exit_status.code() {
        if code == 0 && new_document {
            println!("Written to {}", working_document_path.to_str().unwrap());
            fs_copy(working_document_path, seek_path)
                .expect("Error: Copying from temp file has failed.");
        }
    }
}
