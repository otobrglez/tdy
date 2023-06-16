use std::path::PathBuf;

use chrono::{DateTime, NaiveDate, Utc};
use clap::{Parser, Subcommand};

mod document;
mod open_create;

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
        #[arg(short, default_value = "")]
        title: String,
        #[arg(short, long, value_parser = parse_or_set_date)]
        date: Option<DateTime<Utc>>,
        #[arg(long, env, default_value = ".days")]
        tdy_files: PathBuf,
        #[arg(long, env)]
        editor: String,
        #[arg(long, env)]
        shell: String,
    },
    Find {
        #[arg(short, long)]
        query: String,
    },
}

fn parse_or_set_date(date: &str) -> Result<DateTime<Utc>, String> {
    NaiveDate::parse_from_str(date, "%Y-%m-%d")
        .map_err(|e| e.to_string())
        .and_then(|naive_date| {
            naive_date
                .and_hms_opt(0, 0, 0)
                .ok_or_else(|| "Failed altering datetime.".to_string())
        })
        .map(|naive_date| DateTime::<Utc>::from_utc(naive_date, Utc))
}

fn main() {
    match TdyCli::parse().command {
        Commands::Open {
            shell,
            editor,
            tdy_files,
            namespace,
            date,
            title,
        } => open_create::execute(shell, editor, tdy_files, namespace, date, title),
        cmd => todo!("Command {:?} is not yet implemented.", cmd),
    }
}
