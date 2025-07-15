# TODO Memoria - Améliorations Architecture

## ✅ FAIT

### ✅ Architecture modulaire de base
- [x] Séparation lib/CLI (src/lib.rs, src/main.rs, src/cli.rs)
- [x] Module config.rs avec TOML robuste
- [x] Module errors.rs avec thiserror
- [x] Module notes.rs avec NotesManager
- [x] Tests de base avec tempfile

### ✅ Configuration système
- [x] Configuration TOML complète (GeneralConfig, EditorConfig, NotesConfig, FilesystemConfig)
- [x] Platform-specific config directories
- [x] Configuration loading avec fallbacks

---

## 🚀 AMÉLIORATIONS À FAIRE

### 1. 🔧 Gestion d'erreurs sophistiquée avec anyhow

**Status**: Partiellement fait (base avec thiserror OK) 
**Effort**: Faible
**Priority**: Haute

**Actions**:
- [ ] Ajouter extension trait `MemoriaContext` pour contexte anyhow
- [ ] Utiliser `.with_context()` dans notes.rs et cli.rs
- [ ] Améliorer messages d'erreur utilisateur

```rust
// src/errors.rs - À ajouter
use anyhow::{Context, Result as AnyhowResult};

pub trait MemoriaContext<T> {
    fn with_path_context(self, path: &str) -> AnyhowResult<T>;
    fn with_operation_context(self, operation: &str) -> AnyhowResult<T>;
}

impl<T> MemoriaContext<T> for Result<T, MemoriaError> {
    fn with_path_context(self, path: &str) -> AnyhowResult<T> {
        self.with_context(|| format!("Operation failed for path: {}", path))
    }
}

// src/notes.rs - Usage
impl NotesManager {
    pub fn list_notes(&self) -> anyhow::Result<Vec<Note>> {
        let entries = fs::read_dir(&self.notes_directory)
            .with_context(|| format!("Failed to read directory: {}", self.notes_directory.display()))?;
        // ...
    }
}
```

### 2. 📝 Logging structuré avec tracing

**Status**: Todo (actuellement env_logger basique)
**Effort**: Faible  
**Priority**: Moyenne

**Actions**:
- [ ] Remplacer env_logger par tracing-subscriber
- [ ] Ajouter `#[instrument]` sur fonctions clés
- [ ] Configure JSON logging optionnel

```toml
# Cargo.toml - Ajouter
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
```

```rust
// src/main.rs - Remplacer env_logger::init()
use tracing::{info, instrument};

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .compact()
        .init();
    
    info!("Starting Memoria v{}", env!("CARGO_PKG_VERSION"));
    // ...
}

// src/notes.rs - Ajouter instrumentation  
#[instrument(skip(self), fields(directory = %self.notes_directory.display()))]
pub fn list_notes(&self) -> Result<Vec<Note>> {
    info!("Scanning notes directory");
    // ...
}
```

### 3. 🧪 Tests complets

**Status**: Tests de base OK, manque couverture
**Effort**: Moyen
**Priority**: Haute

**Actions**:
- [ ] Tests pour chaque fonction dans notes.rs
- [ ] Tests d'intégration CLI 
- [ ] Tests avec fichiers réels (markdown)
- [ ] Tests gestion erreurs
- [ ] Property-based testing avec proptest

```rust
// src/notes.rs - Tests à ajouter
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_list_notes_with_markdown_files() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        
        // Create test files
        let mut file1 = File::create(temp_dir.path().join("note1.md"))?;
        writeln!(file1, "# Note 1\nContent here")?;
        
        let mut file2 = File::create(temp_dir.path().join("note2.md"))?;
        writeln!(file2, "# Note 2\nMore content")?;
        
        // Non-markdown should be ignored
        File::create(temp_dir.path().join("readme.txt"))?;
        
        let manager = NotesManager::new(temp_dir.path());
        let notes = manager.list_notes()?;
        
        assert_eq!(notes.len(), 2);
        assert!(notes.iter().any(|n| n.title == "note1"));
        Ok(())
    }
}
```

### 4. 🎯 Types plus expressifs

**Status**: Todo (actuellement Note basique)
**Effort**: Moyen
**Priority**: Moyenne

**Actions**:
- [ ] Créer `NoteId` newtype
- [ ] Étendre `Note` avec metadata (dates, tags, word_count)
- [ ] Lazy loading du contenu 
- [ ] Validation des types

```rust
// src/types.rs - Nouveau fichier
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use time::OffsetDateTime;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NoteId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteMeta {
    pub id: NoteId,
    pub title: String,
    pub created_at: OffsetDateTime,
    pub modified_at: OffsetDateTime, 
    pub tags: Vec<String>,
    pub word_count: usize,
}

#[derive(Debug, Clone)]
pub struct Note {
    pub meta: NoteMeta,
    pub path: PathBuf,
    pub content: Option<String>, // Lazy loading
}

impl Note {
    pub fn load_content(&mut self) -> anyhow::Result<&str> {
        if self.content.is_none() {
            self.content = Some(std::fs::read_to_string(&self.path)?);
        }
        Ok(self.content.as_ref().unwrap())
    }
}
```

### 5. ⚡ Async et performance

**Status**: Todo (actuellement sync)
**Effort**: Élevé
**Priority**: Basse

**Actions**:
- [ ] Ajouter tokio et async-trait
- [ ] Créer trait `NotesRepository` async
- [ ] Implémentation `FileSystemRepository` 
- [ ] Migration CLI vers async handlers

```toml
# Cargo.toml - Ajouter
tokio = { version = "1.0", features = ["full"] }
async-trait = "0.1"
```

```rust
// src/repository.rs - Nouveau fichier
use async_trait::async_trait;
use tokio::fs;

#[async_trait]
pub trait NotesRepository {
    async fn list_notes(&self) -> anyhow::Result<Vec<Note>>;
    async fn get_note(&self, id: &NoteId) -> anyhow::Result<Option<Note>>;
    async fn save_note(&self, note: &Note) -> anyhow::Result<()>;
    async fn delete_note(&self, id: &NoteId) -> anyhow::Result<()>;
}

pub struct FileSystemRepository {
    notes_directory: PathBuf,
}

#[async_trait]
impl NotesRepository for FileSystemRepository {
    async fn list_notes(&self) -> anyhow::Result<Vec<Note>> {
        let mut dir = fs::read_dir(&self.notes_directory).await?;
        let mut notes = Vec::new();
        
        while let Some(entry) = dir.next_entry().await? {
            if is_markdown_file(&entry.path()) {
                let note = Note::from_path_async(entry.path()).await?;
                notes.push(note);
            }
        }
        Ok(notes)
    }
}
```

### 6. 🛡️ Validation et sécurité

**Status**: Todo (aucune validation)
**Effort**: Moyen
**Priority**: Moyenne

**Actions**:
- [ ] Créer module `validation.rs`
- [ ] Validation titres/contenus 
- [ ] Sanitisation noms fichiers
- [ ] Protection path traversal

```rust
// src/validation.rs - Nouveau fichier
use anyhow::Result;
use std::path::Path;

pub struct NoteValidator;

impl NoteValidator {
    const MAX_TITLE_LENGTH: usize = 200;
    const MAX_CONTENT_LENGTH: usize = 1_000_000; // 1MB
    
    pub fn validate_title(title: &str) -> Result<()> {
        if title.is_empty() {
            anyhow::bail!("Title cannot be empty");
        }
        
        if title.len() > Self::MAX_TITLE_LENGTH {
            anyhow::bail!("Title too long (max {} chars)", Self::MAX_TITLE_LENGTH);
        }
        
        let dangerous_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
        if title.chars().any(|c| dangerous_chars.contains(&c)) {
            anyhow::bail!("Title contains invalid characters");
        }
        Ok(())
    }
    
    pub fn sanitize_filename(title: &str) -> String {
        title
            .chars()
            .map(|c| if c.is_alphanumeric() || " -_".contains(c) { c } else { '_' })
            .collect::<String>()
            .trim()
            .to_lowercase()
            .replace(' ', "-")
    }
    
    pub fn validate_path_safety(path: &Path) -> Result<()> {
        if path.components().any(|comp| {
            matches!(comp, std::path::Component::ParentDir)
        }) {
            anyhow::bail!("Path contains parent directory references");
        }
        Ok(())
    }
}
```

### 7. 🔌 Traits pour l'extensibilité

**Status**: Todo
**Effort**: Faible
**Priority**: Basse

**Actions**:
- [ ] Créer traits `Exporter` et `Importer`
- [ ] Implémentations de base (Markdown, JSON, HTML)
- [ ] Plugin système pour futures extensions

```rust
// src/exporters.rs - Nouveau fichier
use anyhow::Result;
use std::path::Path;

pub trait Exporter {
    fn export(&self, notes: &[Note], output: &Path) -> Result<()>;
}

pub struct MarkdownExporter;
pub struct JsonExporter; 
pub struct HtmlExporter;

impl Exporter for MarkdownExporter {
    fn export(&self, notes: &[Note], output: &Path) -> Result<()> {
        // Implementation
        todo!()
    }
}

// src/importers.rs - Nouveau fichier  
pub trait Importer {
    fn import(&self, source: &Path) -> Result<Vec<Note>>;
}

pub struct ObsidianImporter;
pub struct NotionImporter;
```

### 8. 📊 Métriques et observabilité

**Status**: Todo
**Effort**: Moyen
**Priority**: Basse

**Actions**:
- [ ] Créer module `metrics.rs`
- [ ] Compteurs atomic pour stats de base
- [ ] CLI command `memoria stats`
- [ ] Export metrics JSON

```rust
// src/metrics.rs - Nouveau fichier
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Default)]
pub struct Metrics {
    notes_read: AtomicU64,
    notes_written: AtomicU64,
    errors_count: AtomicU64,
}

impl Metrics {
    pub fn increment_notes_read(&self) {
        self.notes_read.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn get_stats(&self) -> MetricsSnapshot {
        MetricsSnapshot {
            notes_read: self.notes_read.load(Ordering::Relaxed),
            notes_written: self.notes_written.load(Ordering::Relaxed),
            errors_count: self.errors_count.load(Ordering::Relaxed),
        }
    }
}

#[derive(Debug)]
pub struct MetricsSnapshot {
    pub notes_read: u64,
    pub notes_written: u64,
    pub errors_count: u64,
}
```

### 9. 🚀 Cache et optimisation

**Status**: Todo 
**Effort**: Moyen
**Priority**: Basse

**Actions**:
- [ ] Créer module `cache.rs`
- [ ] Cache en mémoire pour notes fréquentes
- [ ] TTL configurable
- [ ] Cache invalidation sur changements fichiers

```rust
// src/cache.rs - Nouveau fichier
use std::collections::HashMap;
use std::time::{SystemTime, Duration};

#[derive(Debug)]
pub struct CacheEntry<T> {
    value: T,
    created_at: SystemTime,
    ttl: Duration,
}

pub struct MemoryCache<K, V> {
    data: HashMap<K, CacheEntry<V>>,
    default_ttl: Duration,
}

impl<K, V> MemoryCache<K, V> 
where 
    K: std::hash::Hash + Eq,
    V: Clone,
{
    pub fn new(default_ttl: Duration) -> Self {
        Self {
            data: HashMap::new(),
            default_ttl,
        }
    }
    
    pub fn get(&mut self, key: &K) -> Option<V> {
        if let Some(entry) = self.data.get(key) {
            if entry.created_at.elapsed().unwrap_or_default() < entry.ttl {
                return Some(entry.value.clone());
            } else {
                self.data.remove(key);
            }
        }
        None
    }
    
    pub fn put(&mut self, key: K, value: V) {
        let entry = CacheEntry {
            value,
            created_at: SystemTime::now(),
            ttl: self.default_ttl,
        };
        self.data.insert(key, entry);
    }
}
```

---

## 📋 PRIORITÉS RECOMMANDÉES

### 🔥 Immédiat (High Priority)
1. **Gestion d'erreurs sophistiquée** - Ajouter contexte anyhow 
2. **Tests complets** - Couvrir toutes les fonctions critiques

### ⚡ Court terme (Medium Priority)  
3. **Logging structuré** - Migration vers tracing
4. **Types expressifs** - Note metadata et NoteId
5. **Validation** - Sécurité et sanitization

### 🌟 Moyen terme (Low Priority)
6. **Métriques** - Stats d'usage basiques
7. **Cache** - Performance pour gros volumes
8. **Traits extensibilité** - Préparation plugins

### 🚀 Long terme (Optional)
9. **Async** - Si performance devient critique
10. **Export/Import** - Intégration autres outils

---

## 📚 ARCHITECTURE CIBLE

```
src/
├── main.rs          # Bootstrap (✅ fait)
├── cli.rs           # Interface CLI (✅ fait)  
├── lib.rs           # API publique (✅ fait)
├── config.rs        # Configuration TOML (✅ fait)
├── errors.rs        # Types erreurs (✅ fait)
├── notes.rs         # Logique métier core (✅ fait)
├── test.rs          # Tests de base (✅ fait)
├── types.rs         # 🔧 Types expressifs (à faire)
├── validation.rs    # 🛡️ Validation (à faire)
├── metrics.rs       # 📊 Métriques (à faire)
├── cache.rs         # 🚀 Cache (à faire)
├── repository.rs    # ⚡ Async traits (à faire)
├── exporters.rs     # 🔌 Export (à faire)
└── importers.rs     # 🔌 Import (à faire)
```

---

## 🎯 AVANCEMENT

- ✅ **Base architecture** : 85% fait
- 🚧 **Error handling** : 50% fait (base OK, contexte à faire)  
- 🚧 **Testing** : 30% fait (base OK, couverture à étendre)
- ❌ **Advanced features** : 10% fait (config système excellent)

**Prochaine étape recommandée** : Commencer par point #1 (anyhow context) - impact élevé, effort faible ! 🎯
