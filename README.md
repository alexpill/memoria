# Memoria

A local-first knowledge base manager built in Rust. Write notes in Markdown, organize them with tags, backlinks, and metadata, and access them from a powerful CLI interface. Future extensions include:

- A GUI built with Tauri
- RSS feed aggregation
- AI enrichment (summarize blogs, docs, etc.)
- MCP server for AI agents to query and edit the knowledge base

## Features

- Markdown-based note storage
- Tags, backlinks, frontmatter metadata
- CLI interface to search, link, edit
- Future GUI via Tauri/Web
- Plugin system for scraping & AI enrichment
- RSS & blog watcher (planned)
- MCP-compatible backend (planned)

## Configuration

Memoria uses a TOML configuration file to customize its behavior. On first run, a default configuration file is created at:

- **Linux/Unix**: `~/.config/memoria/config.toml`
- **macOS**: `~/Library/Application Support/memoria/config.toml`
- **Windows**: `%APPDATA%\memoria\config.toml`

### Configuration Management

```bash
# Show current configuration
memoria config show

# Edit configuration with your default editor
memoria config edit

# Get a specific configuration value
memoria config get editor.default_editor

# Set a configuration value
memoria config set editor.default_editor nvim
memoria config set notes.notes_directory "~/Documents/my-notes"

# Reset configuration to defaults
memoria config reset
```

### Available Options

**General Settings:**
- `general.timezone` - Default timezone (e.g., "UTC", "Europe/Paris")
- `general.language` - Interface language (for future use)

**Editor Settings:**
- `editor.default_editor` - Command for editing files (e.g., "vim", "nvim", "code")
- `editor.editor_args` - Additional arguments for the editor

**Notes Settings:**
- `notes.notes_directory` - Directory where notes are stored
- `notes.default_extension` - File extension for new notes
- `notes.default_template` - Template content for new notes

**Filesystem Settings:**
- `filesystem.max_file_size` - Maximum file size in bytes (safety limit)
- `filesystem.create_backups` - Whether to create backups when editing
- `filesystem.backup_directory` - Directory for backup files

See `config.example.toml` for a complete example with all options documented.

## Philosophy

- Hacker-friendly
- Fully offline/local-first
- Extensible, composable, and cross-platform
