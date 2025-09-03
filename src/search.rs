use chrono::{DateTime, Utc};
use log::info;
use std::path::PathBuf;

pub struct Engine {
    pub tdy_files: PathBuf,
}

impl Engine {
    pub fn new(tdy_files: PathBuf) -> Engine {
        Engine { tdy_files }
    }

    pub fn index(&mut self) {
        info!("Indexing {:#?}", self.tdy_files.display());
    }

    pub fn search(&self, query: String) {
        info!("Searching {:#?} for {:#?}", self.tdy_files.display(), query);
    }
}
