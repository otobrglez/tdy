use std::path::PathBuf;

use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use tdy::{cli_date, open_create, search::Engine};

#[derive(Parser)]
#[command(author, version, about, name = "tdy", bin_name = "tdy")]
#[command(propagate_version = true)]
#[command(args_conflicts_with_subcommands = true)]
struct TdyCli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Open {
        #[arg(short, long, env, default_value = "tdy")]
        namespace: String,
        #[arg(short, long, default_value = None)]
        title: Option<String>,
        #[arg(short, long, value_parser = cli_date::parse_raw_date)]
        date: Option<DateTime<Utc>>,
        #[arg(long, env, default_value = ".days")]
        tdy_files: PathBuf,
        #[arg(long, env)]
        editor: String,
    },
    Path {
        #[arg(short, long, env, default_value = "tdy")]
        namespace: String,
        #[arg(short, long, value_parser = cli_date::parse_raw_date)]
        date: Option<DateTime<Utc>>,
        #[arg(long, env, default_value = ".days")]
        tdy_files: PathBuf,
    },
    Index {
        #[arg(long, env, default_value = ".days")]
        tdy_files: PathBuf,
    },
    Search {
        #[arg(long, env, default_value = ".days")]
        tdy_files: PathBuf,
        #[arg(short, long)]
        query: String,
    },
}

fn main() {
    env_logger::init();

    match TdyCli::parse().command {
        Commands::Open {
            editor,
            tdy_files,
            namespace,
            date,
            title,
        } => open_create::execute(editor, tdy_files, namespace, date, title),
        Commands::Path {
            namespace,
            date,
            tdy_files,
        } => {
            if let Some(path) = open_create::resolve_path(tdy_files, namespace, date) {
                println!("{}", path.display());
            }
        }
        Commands::Index { tdy_files } => {
            let _engine = Engine::new(tdy_files).index();
            println!("Indexing.");
        }
        Commands::Search { tdy_files, query } => {
            let _engine = Engine::new(tdy_files).search(query.clone());
            println!("Search.");
        }
    }
}
