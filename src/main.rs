use chrono::NaiveDate;
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;
use tdy::date;
use tdy::document::{self, DEFAULT_NAMESPACE, Document};
use tdy::error::{Result, TdyError};
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
    /// Namespace that groups documents, for example `work` [env: TDY_NAMESPACE=, NAMESPACE=] [default: tdy]
    #[arg(short, long)]
    namespace: Option<String>,

    /// Day of the document: `2025-12-31`, `today`, `yesterday`, `tomorrow`,
    /// `last friday` or `next monday` [default: today]
    #[arg(short, long, value_parser = date::parse_raw_date)]
    date: Option<NaiveDate>,

    /// Use the most recent existing document, up to and including today
    #[arg(short, long, conflicts_with = "date")]
    last: bool,

    /// Directory where documents are stored
    #[arg(long, env, default_value = ".days")]
    tdy_files: PathBuf,
}

impl Locator {
    /// Resolves the namespace from CLI arguments, TDY_NAMESPACE, NAMESPACE,
    /// or falls back to the default "tdy".
    fn resolve_namespace(&self) -> String {
        if let Some(ref ns) = self.namespace
            && !ns.trim().is_empty()
        {
            return ns.clone();
        }
        if let Ok(ns) = std::env::var("TDY_NAMESPACE")
            && !ns.trim().is_empty()
        {
            return ns;
        }
        if let Ok(ns) = std::env::var("NAMESPACE")
            && !ns.trim().is_empty()
        {
            return ns;
        }
        DEFAULT_NAMESPACE.to_string()
    }

    /// The day this locator points at, with `--last` resolved to the newest
    /// existing document.
    fn resolve_date(&self, namespace: &str) -> Result<NaiveDate> {
        if self.last {
            document::latest_date(&self.tdy_files, namespace, date::today())?
                .ok_or_else(|| TdyError::NoDocuments(namespace.to_string()))
        } else {
            Ok(self.date.unwrap_or_else(date::today))
        }
    }
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

        /// Editor command used to open the document [env: EDITOR=]
        #[arg(long, env)]
        editor: Option<String>,
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
        let mut source = std::error::Error::source(&e);
        while let Some(cause) = source {
            eprintln!("  Caused by: {cause}");
            source = std::error::Error::source(cause);
        }
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
            let namespace = locator.resolve_namespace();
            let date = locator.resolve_date(&namespace)?;
            let document = Document::new(namespace, title, Some(date))?;
            let editor = open::resolve_editor(editor.as_deref())?;
            open::open(&editor, &locator.tdy_files, &document)
        }
        Command::Path { locator } => {
            let namespace = locator.resolve_namespace();
            let date = locator.resolve_date(&namespace)?;
            let document = Document::new(namespace, None, Some(date))?;
            let path = document.path_in(&locator.tdy_files);
            if path.exists() {
                println!("{}", path.display());
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_namespace_precedence() {
        let loc = Locator {
            namespace: Some("custom".to_string()),
            date: None,
            last: false,
            tdy_files: PathBuf::from(".days"),
        };
        assert_eq!(loc.resolve_namespace(), "custom");
    }
}
