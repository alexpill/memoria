use anyhow::Error;
use dotenv::dotenv;
use log::error;
use std::fs;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MemoriaError {
    #[error("Directory not found: {path}")]
    DirectoryNotFound { path: String },

    #[error("Permission denied: {path}")]
    PermissionDenied { path: String },

    #[error("Other IO error: {source}")]
    Io { source: std::io::Error },
}

fn main() -> Result<(), Error> {
    dotenv().ok();
    env_logger::init();

    let notes = read_notes_directory("notes")?;
    println!("Found {:?} notes", notes);

    Ok(())
}

fn map_io_error(e: std::io::Error, path: &str) -> MemoriaError {
    match e.kind() {
        std::io::ErrorKind::NotFound => MemoriaError::DirectoryNotFound {
            path: path.to_string(),
        },
        std::io::ErrorKind::PermissionDenied => MemoriaError::PermissionDenied {
            path: path.to_string(),
        },
        _ => MemoriaError::Io { source: e },
    }
}

fn read_notes_directory(path: &str) -> Result<Vec<String>, MemoriaError> {
    let mut notes = Vec::new();

    let entries = fs::read_dir(path).map_err(|e| map_io_error(e, path))?;

    for entry in entries {
        let entry = entry.map_err(|e| MemoriaError::Io { source: e })?;
        let path = entry.path();
        if path.is_file() && path.extension().is_some_and(|ext| ext == "md") {
            notes.push(path.to_string_lossy().to_string());
        }
    }

    Ok(notes)
}
