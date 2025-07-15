use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MemoriaError {
    #[error("Directory not found: {path}")]
    DirectoryNotFound { path: String },

    #[error("Permission denied: {path}")]
    PermissionDenied { path: String },

    #[error("File not found: {path}")]
    FileNotFound { path: String },

    #[error("Invalid file format: {message}")]
    InvalidFormat { message: String },

    #[error("IO error")]
    Io(#[from] io::Error),

    #[error("Empty notes directory: {path}")]
    EmptyNotesDirectory { path: String },

    #[error("Note exists: {path}")]
    NoteExists { path: String },

    #[error("Note not found: {path}")]
    NoteNotFound { path: String },
}

/// Utility function to map IO errors to domain-specific errors with context
pub fn map_io_error_with_context(e: io::Error, path: &str) -> MemoriaError {
    match e.kind() {
        io::ErrorKind::NotFound => {
            // ✅ Différencier fichier vs répertoire selon le contexte
            if path.ends_with(".md") {
                MemoriaError::FileNotFound {
                    path: path.to_string(),
                }
            } else {
                MemoriaError::DirectoryNotFound {
                    path: path.to_string(),
                }
            }
        }
        io::ErrorKind::PermissionDenied => MemoriaError::PermissionDenied {
            path: path.to_string(),
        },
        _ => MemoriaError::Io(e),
    }
}

pub trait MemoriaContext<T> {
    fn with_path_context(self, path: &str) -> Result<T, MemoriaError>;
}

impl<T> MemoriaContext<T> for Result<T, io::Error> {
    fn with_path_context(self, path: &str) -> Result<T, MemoriaError> {
        self.map_err(|e| map_io_error_with_context(e, path))
    }
}
