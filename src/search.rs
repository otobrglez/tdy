use crate::document::Document;
use chrono::{NaiveDate, Utc};
use log::info;
use rusqlite::{Connection, Result};
use std::fs;
use std::fs::create_dir_all;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

pub struct Engine {
    tdy_files: PathBuf,
    tdy_index: PathBuf,
}

impl Engine {
    pub fn new(tdy_files: PathBuf) -> Engine {
        let tdy_index = tdy_files.join("index");
        Engine {
            tdy_files,
            tdy_index,
        }
    }

    pub fn open_db(&mut self) -> Result<Connection, Box<dyn std::error::Error>> {
        let index_file_path = self.tdy_index.join("tdy.db");
        if let Some(parent) = index_file_path.parent() {
            create_dir_all(parent)?;
        }

        let db = Connection::open(index_file_path).unwrap();

        db.execute_batch(
            r#"
            PRAGMA journal_mode=WAL;
            PRAGMA synchronous=NORMAL;
            PRAGMA temp_store=MEMORY;
            PRAGMA mmap_size=536870912; -- 512 MiB
        "#,
        )?;

        Ok(db)
    }

    pub fn prepare_db(db: &mut Connection) -> Result<()> {
        info!("Preparing database.");
        db.execute_batch(
            r#"
            DROP TABLE IF EXISTS docs;
            CREATE TABLE IF NOT EXISTS docs (
                doc_id      INTEGER PRIMARY KEY,
                path        TEXT NOT NULL UNIQUE,
                namespace   TEXT,
                body        TEXT,
                mtime_unix  INTEGER NOT NULL,
                blake3_hex  TEXT NOT NULL
            );
            CREATE VIRTUAL TABLE IF NOT EXISTS fts USING fts5(
              body, namespace, content='docs', content_rowid='doc_id',
              tokenize = "unicode61 remove_diacritics 2 tokenchars '-_./#'"
            );
            CREATE TRIGGER IF NOT EXISTS docs_after_insert AFTER INSERT ON docs BEGIN
              INSERT INTO fts(rowid, body, namespace)
              VALUES (new.doc_id, new.body, new.namespace);
            END;
            CREATE TRIGGER IF NOT EXISTS docs_after_update AFTER UPDATE ON docs BEGIN
              INSERT INTO fts(fts, rowid) VALUES('delete', old.doc_id);
              INSERT INTO fts(rowid, body, namespace)
              VALUES (new.doc_id, new.body, new.namespace);
            END;
            CREATE TRIGGER IF NOT EXISTS docs_after_delete AFTER DELETE ON docs BEGIN
              INSERT INTO fts(fts, rowid) VALUES('delete', old.doc_id);
            END;

            CREATE INDEX IF NOT EXISTS docs_mtime_idx ON docs(mtime_unix);
            CREATE INDEX IF NOT EXISTS docs_path_idx  ON docs(path);
            "#,
        )
    }

    pub fn index(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut db = self.open_db()?;
        // Prepare the database
        Self::prepare_db(&mut db)?;

        // Loop over all the *.md files in the tdy_files directory
        for entry in fs::read_dir(self.tdy_files.as_path())? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file()
                && path
                    .extension()
                    .and_then(|s| s.to_str())
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("md"))
            {
                // do something with `path`

                // let bytes = fs::read(path)?;
                let document = self.parse_document(&path)?;
                println!("{:#?}", document);

                println!("{}", path.display());
            }
        }

        Ok(())
    }

    fn parse_document(&self, path: &PathBuf) -> Result<Document, Box<dyn std::error::Error>> {
        info!("Parsing {:#?}", path.display());
        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "invalid UTF-8 filename"))?;

        let parts: Vec<_> = filename.trim_end_matches(".md").split('-').collect();
        if parts.len() != 4 {
            return Err(Box::new(Error::new(
                ErrorKind::InvalidData,
                "Invalid filename format",
            )));
        }

        let namespace = parts[0].to_string();
        let date = NaiveDate::from_ymd_opt(parts[1].parse()?, parts[2].parse()?, parts[3].parse()?)
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "Invalid date"))?
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Utc)
            .unwrap();

        Ok(Document::new(namespace, None, Some(date)))
    }

    pub fn search(&self, query: String) {
        info!("Searching {:#?} for {:#?}", self.tdy_files.display(), query);
    }
}
