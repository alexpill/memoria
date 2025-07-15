use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

use memoria::{MemoriaConfig, MemoriaError, NotesManager};

#[derive(Parser)]
#[command(name = "memoria")]
#[command(about = "A local-first knowledge management tool")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List all notes in the notes directory
    List,
    /// Initialize a new note
    Create { title: String },
    /// Initialize the notes directory
    Init { title: String },
    /// Configuration management
    Config {
        #[command(subcommand)]
        config_command: ConfigCommands,
    },
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Show current configuration
    Show,
    /// Edit configuration file with default editor
    Edit,
    /// Set a configuration value
    Set {
        /// Configuration key (e.g., editor.default_editor, notes.notes_directory)
        key: String,
        /// Configuration value
        value: String,
    },
    /// Get a configuration value
    Get {
        /// Configuration key (e.g., editor.default_editor, notes.notes_directory)
        key: String,
    },
    /// Reset configuration to defaults
    Reset,
}

/// Handle the list command
pub fn handle_list(config: &MemoriaConfig) -> Result<()> {
    let notes_dir = config.notes.notes_directory.to_string_lossy().to_string();
    let notes_manager = NotesManager::new(&notes_dir);

    let notes = notes_manager
        .list_notes()
        .map_err(handle_memoria_error)?;

    if notes.is_empty() {
        println!("No notes found in the '{}' directory.", notes_dir);
    } else {
        println!("Found {} note(s):", notes.len());
        for note in notes {
            println!("  {} ({})", note.title, note.path_str());
        }
    }

    Ok(())
}

pub fn handle_create(title: &str, config: &MemoriaConfig) -> Result<()> {
    let notes_dir = config.notes.notes_directory.to_string_lossy().to_string();
    let notes_manager = NotesManager::new(&notes_dir);
    let note = notes_manager
        .create_note(title)
        .map_err(handle_memoria_error)
        .with_context(|| format!("Failed to create note: {}", title))?;
    println!("Note created: {}", note.path_str());
    Ok(())
}

pub fn handle_init(note_dir: &str, config: &MemoriaConfig) -> Result<()> {
    use std::fs;
    let target_dir = if note_dir == "default" {
        config.notes.notes_directory.to_string_lossy().to_string()
    } else {
        note_dir.to_string()
    };

    if !std::path::Path::new(&target_dir).exists() {
        fs::create_dir_all(&target_dir)
            .with_context(|| format!("Failed to create notes directory: {}", target_dir))?;
        println!("Created notes directory: {}", target_dir);
    } else {
        println!("Notes directory already exists: {}", target_dir);
    }

    Ok(())
}

/// Handle config show command
pub fn handle_config_show(config: &MemoriaConfig) -> Result<()> {
    let config_path = MemoriaConfig::default_config_path()?;
    println!("Configuration loaded from: {}", config_path.display());
    println!(
        "\n{}",
        toml::to_string_pretty(config).context("Failed to serialize configuration")?
    );
    Ok(())
}

/// Handle config edit command
pub fn handle_config_edit(config: &MemoriaConfig) -> Result<()> {
    let config_path = MemoriaConfig::default_config_path()?;
    let editor = &config.editor.default_editor;

    let mut cmd = std::process::Command::new(editor);
    cmd.arg(&config_path);

    // Add any additional editor arguments
    for arg in &config.editor.editor_args {
        cmd.arg(arg);
    }

    let status = cmd
        .status()
        .with_context(|| format!("Failed to launch editor: {}", editor))?;

    if !status.success() {
        anyhow::bail!("Editor exited with non-zero status: {}", status);
    }

    println!("Configuration file updated: {}", config_path.display());
    Ok(())
}

/// Handle config set command
pub fn handle_config_set(key: &str, value: &str) -> Result<()> {
    let mut config = MemoriaConfig::load()?;

    // Parse the key and set the value
    match key {
        "general.timezone" => config.general.timezone = value.to_string(),
        "general.language" => config.general.language = value.to_string(),
        "editor.default_editor" => config.editor.default_editor = value.to_string(),
        "notes.notes_directory" => config.notes.notes_directory = std::path::PathBuf::from(value),
        "notes.default_extension" => config.notes.default_extension = value.to_string(),
        "notes.default_template" => config.notes.default_template = Some(value.to_string()),
        "filesystem.max_file_size" => {
            let size: u64 = value
                .parse()
                .with_context(|| format!("Invalid file size: {}", value))?;
            config.filesystem.max_file_size = size;
        }
        "filesystem.create_backups" => {
            let create_backups: bool = value
                .parse()
                .with_context(|| format!("Invalid boolean value: {}", value))?;
            config.filesystem.create_backups = create_backups;
        }
        "filesystem.backup_directory" => config.filesystem.backup_directory = value.to_string(),
        _ => anyhow::bail!("Unknown configuration key: {}", key),
    }

    config.save()?;
    println!("Configuration updated: {} = {}", key, value);
    Ok(())
}

/// Handle config get command
pub fn handle_config_get(key: &str, config: &MemoriaConfig) -> Result<()> {
    let value = match key {
        "general.timezone" => config.general.timezone.clone(),
        "general.language" => config.general.language.clone(),
        "editor.default_editor" => config.editor.default_editor.clone(),
        "notes.notes_directory" => config.notes.notes_directory.to_string_lossy().to_string(),
        "notes.default_extension" => config.notes.default_extension.clone(),
        "notes.default_template" => config
            .notes
            .default_template
            .clone()
            .unwrap_or_else(|| "None".to_string()),
        "filesystem.max_file_size" => config.filesystem.max_file_size.to_string(),
        "filesystem.create_backups" => config.filesystem.create_backups.to_string(),
        "filesystem.backup_directory" => config.filesystem.backup_directory.clone(),
        _ => anyhow::bail!("Unknown configuration key: {}", key),
    };

    println!("{}", value);
    Ok(())
}

/// Handle config reset command
pub fn handle_config_reset() -> Result<()> {
    let config_path = MemoriaConfig::default_config_path()?;
    let default_config = MemoriaConfig::default();

    default_config.save_to_file(&config_path)?;
    println!("Configuration reset to defaults: {}", config_path.display());
    Ok(())
}

/// Convert MemoriaError to user-friendly error messages
fn handle_memoria_error(error: MemoriaError) -> anyhow::Error {
    match error {
        MemoriaError::DirectoryNotFound { path } => {
            anyhow::anyhow!(
                "Notes directory not found: {}\nTry running 'memoria setup' first.",
                path
            )
        }
        MemoriaError::PermissionDenied { path } => {
            anyhow::anyhow!("Permission denied accessing: {}", path)
        }
        MemoriaError::FileNotFound { path } => {
            anyhow::anyhow!("File not found: {}", path)
        }
        MemoriaError::InvalidFormat { message } => {
            anyhow::anyhow!("Invalid format: {}", message)
        }
        MemoriaError::Io(source) => {
            anyhow::anyhow!("IO error: {}", source)
        }
        MemoriaError::EmptyNotesDirectory { path } => {
            anyhow::anyhow!("No notes found in directory: {}", path)
        }
        MemoriaError::NoteExists { path } => {
            anyhow::anyhow!("Note already exists: {}", path)
        }
        MemoriaError::NoteNotFound { path } => {
            anyhow::anyhow!("Note not found: {}", path)
        }
    }
}
