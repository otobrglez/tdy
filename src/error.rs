use std::error::Error;
use std::fmt;
use std::io;

use std::path::PathBuf;

#[derive(Debug)]
pub enum TdyError {
    Io(io::Error),
    Template(minijinja::Error),
    EditorFailed(String),
    NoEditorFound,
    NoDocuments(String),
    InvalidNamespace(String),
    SaveFailed {
        path: PathBuf,
        draft_preserved_at: PathBuf,
        source: io::Error,
    },
}

impl fmt::Display for TdyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TdyError::Io(err) => write!(f, "IO error: {err}"),
            TdyError::Template(err) => write!(f, "Template error: {err}"),
            TdyError::EditorFailed(msg) => write!(f, "Editor error: {msg}"),
            TdyError::NoEditorFound => write!(
                f,
                "No editor found. Set $VISUAL or $EDITOR, pass --editor, or install nano/vim/vi"
            ),
            TdyError::NoDocuments(namespace) => {
                write!(f, "No documents found in namespace '{namespace}'")
            }
            TdyError::InvalidNamespace(msg) => write!(f, "Invalid namespace: {msg}"),
            TdyError::SaveFailed {
                path,
                draft_preserved_at,
                source,
            } => {
                write!(
                    f,
                    "Failed to save document to '{}': {source}. Your draft was preserved at '{}'",
                    path.display(),
                    draft_preserved_at.display()
                )
            }
        }
    }
}

impl Error for TdyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TdyError::Io(err) => Some(err),
            TdyError::Template(err) => Some(err),
            TdyError::SaveFailed { source, .. } => Some(source),
            TdyError::EditorFailed(_)
            | TdyError::NoEditorFound
            | TdyError::NoDocuments(_)
            | TdyError::InvalidNamespace(_) => None,
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
