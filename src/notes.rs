use std::fs;
use std::path::{Path, PathBuf};

use crate::Result;
use crate::errors::{MemoriaContext, MemoriaError};
use crate::utils::get_utc_time;

/// Represents a note in the system
#[derive(Debug, Clone)]
pub struct Note {
    pub path: PathBuf,
    pub title: String,
}

impl Note {
    /// Create a new Note from a file path
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();

        if !path.exists() {
            return Err(MemoriaError::FileNotFound {
                path: path.to_string_lossy().to_string(),
            });
        }

        if !path.is_file() {
            return Err(MemoriaError::InvalidFormat {
                message: format!("Path is not a file: {}", path.display()),
            });
        }

        // Extract title from filename (without .md extension)
        let title = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| MemoriaError::InvalidFormat {
                message: format!("Cannot extract title from path: {}", path.display()),
            })?
            .to_string();

        Ok(Note { path, title })
    }

    /// Get the relative path as a string
    pub fn path_str(&self) -> String {
        self.path.to_string_lossy().to_string()
    }

    pub fn read_content(&self) -> Result<String> {
        fs::read_to_string(&self.path).with_path_context(&self.path.to_string_lossy())
    }
}

/// Core functionality for managing notes
pub struct NotesManager {
    notes_directory: PathBuf,
}

impl NotesManager {
    /// Create a new NotesManager with the specified directory
    pub fn new(notes_directory: impl AsRef<Path>) -> Self {
        Self {
            notes_directory: notes_directory.as_ref().to_path_buf(),
        }
    }

    /// Validate the notes directory
    pub fn validate_directory(&self) -> Result<()> {
        if !self.notes_directory.exists() {
            return Err(MemoriaError::DirectoryNotFound {
                path: self.notes_directory.to_string_lossy().to_string(),
            });
        }

        if !self.notes_directory.is_dir() {
            return Err(MemoriaError::InvalidFormat {
                message: format!(
                    "Path is not a directory: {}",
                    self.notes_directory.display()
                ),
            });
        }

        Ok(())
    }

    /// List all markdown notes in the notes directory
    pub fn list_notes(&self) -> Result<Vec<Note>> {
        let mut notes = Vec::new();

        let entries = fs::read_dir(&self.notes_directory)
            .with_path_context(&self.notes_directory.to_string_lossy())?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && is_markdown_file(&path) {
                notes.push(Note::from_path(path)?);
            }
        }

        if notes.is_empty() {
            return Err(MemoriaError::EmptyNotesDirectory {
                path: self.notes_directory.to_string_lossy().to_string(),
            });
        }

        Ok(notes)
    }

    pub fn create_note(&self, title: &str) -> Result<Note> {
        self.validate_directory()?;

        let filename = format!("{}.md", sanitize_filename(title));
        let note_path = self.notes_directory.join(&filename);

        if note_path.exists() {
            return Err(MemoriaError::NoteExists {
                path: note_path.to_string_lossy().to_string(),
            });
        }

        // Create the file with minimal content
        let metadata = get_minimal_metadata_content();
        let content = format!("{}# {}\n\n", metadata, title);
        fs::write(&note_path, content).with_path_context(&note_path.to_string_lossy())?;

        Note::from_path(note_path)
    }

    /// Get the notes directory path
    pub fn notes_directory(&self) -> &Path {
        &self.notes_directory
    }
}

fn get_minimal_metadata_content() -> String {
    format!("---\ncreated_at: {}\n---\n", get_utc_time())
}

fn is_markdown_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.eq_ignore_ascii_case("md") || ext.eq_ignore_ascii_case("markdown"))
        .unwrap_or(false)
}

/// Utility function to sanitize filenames
fn sanitize_filename(title: &str) -> String {
    title
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' | ' ' => '_',
            c => c,
        })
        .collect::<String>()
        .to_lowercase()
        .trim()
        .to_string()
}
