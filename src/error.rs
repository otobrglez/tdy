use std::fmt;
use std::io;

#[derive(Debug)]
pub enum TdyError {
    Io(io::Error),
    Template(String),
    EditorFailed(String),
    DateTimeParse(String),
    PathOperation(String),
}

impl fmt::Display for TdyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TdyError::Io(err) => write!(f, "IO error: {}", err),
            TdyError::Template(msg) => write!(f, "Template error: {}", msg),
            TdyError::EditorFailed(msg) => write!(f, "Editor error: {}", msg),
            TdyError::DateTimeParse(msg) => write!(f, "Date parse error: {}", msg),
            TdyError::PathOperation(msg) => write!(f, "Path operation error: {}", msg),
        }
    }
}

impl std::error::Error for TdyError {}

impl From<io::Error> for TdyError {
    fn from(err: io::Error) -> Self {
        TdyError::Io(err)
    }
}

pub type Result<T> = std::result::Result<T, TdyError>;
