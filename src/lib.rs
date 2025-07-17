pub mod config;
pub mod errors;
pub mod notes;
pub mod utils;

// Re-export main types for easy access
pub use config::MemoriaConfig;
pub use errors::MemoriaError;
pub use notes::{Note, NotesManager};

/// Result type alias for the library
pub type Result<T> = std::result::Result<T, MemoriaError>;
