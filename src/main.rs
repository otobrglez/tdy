use chrono::{Datelike, Utc};
use minijinja::{context, Environment as MinininjaEnvironment};
use std::env::{temp_dir, var as read_env};
use std::fs::{copy as fs_copy, File};
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug)]
struct DocumentWorld {
    namespace: String,
    title: String,
    date: String,
    year: i32,
    month: u32,
    day: u32,
}

fn main() {
    // Read the environment variables
    let editor = read_env("EDITOR").unwrap_or("echo".to_string());
    let shell = read_env("SHELL").expect("SHELL environment variable is needed");
    let files_folder: String = read_env("TDY_FILES").unwrap_or(".days".to_string());
    println!(
        "[Debug] editor: {:?}, shell: {:?}, files_folder: {:?}",
        editor, shell, files_folder
    );

    // Build filename
    let current_date = Utc::now();
    let world = DocumentWorld {
        namespace: "tdy".to_string(),
        title: format!("{}", current_date.format("%Y-%m-%d").to_string()),
        date: current_date.format("%Y-%m-%d").to_string(),
        year: current_date.year(),
        month: current_date.month(),
        day: current_date.day(),
    };

    // Find or create existing file.
    let file_name: String = format!("{}-{}.md", world.namespace, world.date);
    let seek_path = format!("{}/{}", files_folder.clone(), file_name.clone());
    let working_file = std::path::Path::new(seek_path.as_str());
    let mut is_new_file: bool = false;

    let working_file_path = if !working_file.exists() {
        is_new_file = true;
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

        let template = template_environment.get_template("today").unwrap();
        let content = template
            .render(context!(
                namespace => world.namespace,
                title => world.title,
                year => world.year,
                month => world.month,
                day => world.day,
                date => world.date
            ))
            .unwrap();

        // Save template to temp file
        let mut dir: PathBuf = temp_dir();
        dir.push(file_name);
        let temp_file_name = format!("{}", dir.clone().to_str().unwrap());
        let mut file = File::create(dir).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        temp_file_name
    } else {
        working_file.to_str().unwrap().to_string()
    };

    // Open the editor
    let exit_status = std::process::Command::new(shell)
        .arg("-c")
        .arg(format!("{} {}", editor, working_file_path).to_string())
        .spawn()
        .expect("Error: Failed spawning the editor.")
        .wait()
        .expect("Error: Editor returned a non-zero status code.");

    // Copy the file from temp to files
    if let Some(code) = exit_status.code() {
        if code == 0 && is_new_file {
            println!("Written to {}", seek_path);
            fs_copy(working_file_path, seek_path)
                .expect("Error: Copying from temp file has failed.");
        }
    }
}
