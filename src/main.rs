use chrono::NaiveDate;
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;
use tdy::date;
use tdy::document::{DEFAULT_NAMESPACE, Document};
use tdy::error::Result;
use tdy::open;

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    name = "tdy",
    bin_name = "tdy",
    propagate_version = true
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

/// Arguments that identify a single document.
#[derive(Args, Debug)]
struct Locator {
    /// Namespace that groups documents, for example `work`
    #[arg(short, long, env, default_value = DEFAULT_NAMESPACE)]
    namespace: String,

    /// Day of the document: `2025-12-31`, `today`, `yesterday`, `tomorrow`,
    /// `last friday` or `next monday` [default: today]
    #[arg(short, long, value_parser = date::parse_raw_date)]
    date: Option<NaiveDate>,

    /// Directory where documents are stored
    #[arg(long, env, default_value = ".days")]
    tdy_files: PathBuf,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Open the document for a day in your editor, creating it if needed
    Open {
        #[command(flatten)]
        locator: Locator,

        /// Heading of a newly created document [default: the date]
        #[arg(short, long)]
        title: Option<String>,

        /// Editor command used to open the document
        #[arg(long, env)]
        editor: String,
    },
    /// Print the path of the document for a day, if it exists
    Path {
        #[command(flatten)]
        locator: Locator,
    },
}

fn main() {
    env_logger::init();

    if let Err(e) = run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    match Cli::parse().command {
        Command::Open {
            locator,
            title,
            editor,
        } => {
            let document = Document::new(locator.namespace, title, locator.date);
            open::open(&editor, &locator.tdy_files, &document)
        }
        Command::Path { locator } => {
            let document = Document::new(locator.namespace, None, locator.date);
            let path = document.path_in(&locator.tdy_files);
            if path.exists() {
                println!("{}", path.display());
            }
            Ok(())
        }
    }
}
