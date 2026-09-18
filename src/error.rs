use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum TdyError {
    Io(io::Error),
    Template(minijinja::Error),
    EditorFailed(String),
}

impl fmt::Display for TdyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TdyError::Io(err) => write!(f, "IO error: {err}"),
            TdyError::Template(err) => write!(f, "Template error: {err}"),
            TdyError::EditorFailed(msg) => write!(f, "Editor error: {msg}"),
        }
    }
}

impl Error for TdyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TdyError::Io(err) => Some(err),
            TdyError::Template(err) => Some(err),
            TdyError::EditorFailed(_) => None,
        }
    }
}

impl From<io::Error> for TdyError {
    fn from(err: io::Error) -> Self {
        TdyError::Io(err)
    }
}

impl From<minijinja::Error> for TdyError {
    fn from(err: minijinja::Error) -> Self {
        TdyError::Template(err)
    }
}

pub type Result<T> = std::result::Result<T, TdyError>;
