use std::path::PathBuf;

use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use tdy::{cli_date, open_create};

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
    Find {
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
        cmd => todo!("Command {:?} is not yet implemented.", cmd),
    }
}
