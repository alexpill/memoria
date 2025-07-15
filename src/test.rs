#[cfg(test)]
mod tests {
    use crate::{MemoriaError, NotesManager};

    use tempfile::TempDir;

    fn create_test_dir() -> TempDir {
        tempfile::tempdir().unwrap()
    }

    #[test]
    fn test_notes_manager_empty_directory() {
        let temp_dir = create_test_dir();
        let notes_manager = NotesManager::new(temp_dir.path());

        let result = notes_manager.list_notes();
        assert!(matches!(result, Err(MemoriaError::EmptyNotesDirectory { .. })));
    }

    #[test]
    fn test_notes_manager_nonexistent_directory() {
        let notes_manager = NotesManager::new("/nonexistent/path");

        let result = notes_manager.list_notes();
        assert!(matches!(
            result,
            Err(MemoriaError::DirectoryNotFound { .. })
        ));
    }

    #[test]
    fn test_create_and_list_notes() {
        let temp_dir = create_test_dir();
        let notes_manager = NotesManager::new(temp_dir.path());

        // Créer une note
        let note = notes_manager.create_note("Test Note").unwrap();
        assert_eq!(note.title, "Test Note");

        // Lister les notes
        let notes = notes_manager.list_notes().unwrap();
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].title, "Test Note");
    }

    #[test]
    fn test_duplicate_note_creation() {
        let temp_dir = create_test_dir();
        let notes_manager = NotesManager::new(temp_dir.path());

        // Créer la première note
        notes_manager.create_note("Test Note").unwrap();

        // Essayer de créer la même note
        let result = notes_manager.create_note("Test Note");
        assert!(matches!(result, Err(MemoriaError::NoteExists { .. })));
    }
}
