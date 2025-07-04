# Phases

## Phase 0 - Setup

- [x] Créer crate Rust `memoria`
- [x] Setup Clippy, Rustfmt, Git
- [x] Setup logging
  - [x] `env_logger` + `log`
  - [ ] advanced: `tracing`
- [ ] Gestion des erreurs (thiserror / anyhow)

---

## Phase 1 - Gestion des notes (core lib)

- [ ] Lire un dossier de notes Markdown
- [ ] Extraire frontmatter (tags, dates, refs)
- [ ] Créer structure `Note { title, content, tags, backlinks, created_at, updated_at }`
- [ ] Gérer un `KnowledgeBase` qui indexe les notes
- [ ] Rechargement dynamique (watcher)

---

## Phase 2 - CLI Interface

- [ ] `memoria list` : liste des notes
- [ ] `memoria view <note>` : afficher une note
- [ ] `memoria new <title>` : créer une note
- [ ] `memoria search <term>` : rechercher dans les titres / contenus
- [ ] Ergonomie : fuzzy search, highlight résultats

---

## Phase 3 - RSS Aggregator

- [ ] Stocker des flux RSS dans un fichier de conf
- [ ] Parser avec `rss` crate
- [ ] Afficher les nouveaux articles
- [ ] Créer une note automatiquement depuis un article
- [ ] (Optionnel) résumer les articles avec une API AI

---

## Phase 4 - GUI (optionnel mais fun)

- [ ] Architecture modulaire : séparer la lib de l’interface
- [ ] Prototype Tauri : liste des notes, édition dans textarea
- [ ] Ajout simple d'une note via GUI

---

## Phase 5 - IA / Scraping enrichi (stretch goal)

- [ ] Créer une note depuis une URL (scrap via `scraper` ou `select`)
- [ ] Intégration API AI pour résumé ou mise en contexte
- [ ] MCP server : exposer base via API JSON compatible avec IA (RAG)

---

## Phase 6 - WASM
- [ ] Créer une note depuis une URL (scrap via `scraper` ou `select`)
- [ ] Intégration API AI pour résumé ou mise en contexte
- [ ] MCP server : exposer base via API JSON compatible avec IA (RAG)

---

## Autres idées de plugins
- [ ] un moyen de lock les notes avec mdp / touch id sur macos
- [ ] conversion de notes en pdf
- [ ] ajout direct dans
  - [ ] notion
  - [ ] apple journal
  - [ ] obsidian ...

## Learnings

- [ ] File I/O et parsing en Rust
- [ ] CLI app ergonomique (`clap`, `dialoguer`, `tui`)
- [ ] WASM-ready architecture
- [ ] Tests unitaires & intégration
- [ ] Architecture modulaire Rust
- [ ] Parsing Markdown / frontmatter
- [ ] Scraping / RSS
- [ ] (optionnel) IA & RAG & WASM

## Ressources

### Architecture

```
memoria/
├── Cargo.toml                 # Workspace
├── README.md
├── TODO.md
├── core/                      # Bibliothèque principale : gestion notes, parsing, etc.
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── note.rs            # Structure Note, parsing markdown + frontmatter
│       ├── kb.rs              # KnowledgeBase : gestion de toutes les notes
│       ├── parser.rs          # Markdown + frontmatter
│       └── utils.rs
├── cli/                       # Interface CLI
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── commands/
│       │   ├── mod.rs
│       │   ├── new.rs
│       │   ├── list.rs
│       │   ├── view.rs
│       │   └── search.rs
│       └── args.rs            # Gestion des arguments (clap)
├── gui/                       # Interface graphique (Tauri, futur)
│   ├── Cargo.toml
│   └── src-tauri/
│       ├── main.rs
│       └── ...                # Logic Tauri (frontend dans `/gui/ui`)
│
│   └── ui/                    # Frontend (Tauri) — HTML/JS/TS/Yew, etc.
│       └── ...                
├── plugins/                   # Extensions : RSS, AI, Scraping...
│   ├── rss/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs         # Agrégateur de flux RSS
│   ├── ai/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs         # Connexion API OpenAI etc.
│   └── scraper/
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs         # Scraping de contenu HTML
├── common/                    # Types, erreurs, configuration partagés
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── config.rs
│       ├── error.rs
│       └── types.rs
├── tests/                     # Tests d’intégration multi-crate
│   └── integration.rs
└── notes/                     # Notes locales en Markdown (dossier indexé)
    ├── welcome.md
    └── daily/2025-07-04.md
```

```toml
[workspace]
members = [
  "core",
  "cli",
  "gui",
  "plugins/rss",
  "plugins/ai",
  "plugins/scraper",
  "common"
]
```