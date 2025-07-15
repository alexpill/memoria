use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use anyhow::{Context, Result};
use std::fs;

/// Configuration structure for Memoria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoriaConfig {
    /// General application settings
    pub general: GeneralConfig,
    /// Editor settings
    pub editor: EditorConfig,
    /// Notes management settings
    pub notes: NotesConfig,
    /// File system settings
    pub filesystem: FilesystemConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    /// Default timezone for timestamps (e.g., "UTC", "Europe/Paris")
    pub timezone: String,
    /// Default language for the interface
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorConfig {
    /// Default editor command (e.g., "nvim", "code", "vim")
    pub default_editor: String,
    /// Additional editor arguments
    pub editor_args: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotesConfig {
    /// Default directory for storing notes
    pub notes_directory: PathBuf,
    /// Default file extension for notes
    pub default_extension: String,
    /// Template to use for new notes
    pub default_template: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilesystemConfig {
    /// Maximum file size in bytes (for safety)
    pub max_file_size: u64,
    /// Whether to create backups when editing files
    pub create_backups: bool,
    /// Backup directory (relative to notes directory)
    pub backup_directory: String,
}

impl Default for MemoriaConfig {
    fn default() -> Self {
        Self {
            general: GeneralConfig {
                timezone: "UTC".to_string(),
                language: "en".to_string(),
            },
            editor: EditorConfig {
                default_editor: "vim".to_string(),
                editor_args: vec![],
            },
            notes: NotesConfig {
                notes_directory: PathBuf::from("./notes"),
                default_extension: "md".to_string(),
                default_template: None,
            },
            filesystem: FilesystemConfig {
                max_file_size: 10 * 1024 * 1024, // 10MB
                create_backups: true,
                backup_directory: ".backups".to_string(),
            },
        }
    }
}

impl MemoriaConfig {
    /// Get the default configuration file path
    pub fn default_config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .context("Could not determine config directory")?
            .join("memoria");
        
        Ok(config_dir.join("config.toml"))
    }

    /// Load configuration from a TOML file
    pub fn load_from_file(path: &PathBuf) -> Result<Self> {
        if !path.exists() {
            log::info!("Config file not found at {:?}, using defaults", path);
            return Ok(Self::default());
        }

        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {:?}", path))?;
        
        let config: MemoriaConfig = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {:?}", path))?;
        
        log::info!("Configuration loaded from {:?}", path);
        Ok(config)
    }

    /// Load configuration from default location
    pub fn load() -> Result<Self> {
        let path = Self::default_config_path()?;
        Self::load_from_file(&path)
    }

    /// Save configuration to a TOML file
    pub fn save_to_file(&self, path: &PathBuf) -> Result<()> {
        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create config directory: {:?}", parent))?;
        }

        let content = toml::to_string_pretty(self)
            .context("Failed to serialize configuration to TOML")?;
        
        fs::write(path, content)
            .with_context(|| format!("Failed to write config file: {:?}", path))?;
        
        log::info!("Configuration saved to {:?}", path);
        Ok(())
    }

    /// Save configuration to default location
    pub fn save(&self) -> Result<()> {
        let path = Self::default_config_path()?;
        self.save_to_file(&path)
    }

    /// Create a default configuration file if it doesn't exist
    pub fn ensure_config_exists() -> Result<()> {
        let path = Self::default_config_path()?;
        
        if !path.exists() {
            let default_config = Self::default();
            default_config.save_to_file(&path)?;
            log::info!("Created default configuration file at {:?}", path);
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_default_config() {
        let config = MemoriaConfig::default();
        assert_eq!(config.general.timezone, "UTC");
        assert_eq!(config.editor.default_editor, "vim");
        assert_eq!(config.notes.default_extension, "md");
        assert_eq!(config.filesystem.max_file_size, 10 * 1024 * 1024);
    }

    #[test]
    fn test_save_and_load_config() -> Result<()> {
        let temp_dir = tempdir()?;
        let config_path = temp_dir.path().join("test_config.toml");
        
        let original_config = MemoriaConfig {
            general: GeneralConfig {
                timezone: "Europe/Paris".to_string(),
                language: "fr".to_string(),
            },
            ..Default::default()
        };
        
        // Save configuration
        original_config.save_to_file(&config_path)?;
        
        // Load configuration
        let loaded_config = MemoriaConfig::load_from_file(&config_path)?;
        
        assert_eq!(loaded_config.general.timezone, "Europe/Paris");
        assert_eq!(loaded_config.general.language, "fr");
        
        Ok(())
    }

    #[test]
    fn test_load_nonexistent_config() -> Result<()> {
        let temp_dir = tempdir()?;
        let config_path = temp_dir.path().join("nonexistent.toml");
        
        let config = MemoriaConfig::load_from_file(&config_path)?;
        
        // Should return default configuration
        assert_eq!(config.general.timezone, "UTC");
        
        Ok(())
    }
} 