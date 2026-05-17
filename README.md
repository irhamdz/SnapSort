# SnapSort

> Screenshot organization made automatic — offline-first, open-source, AI-enriched desktop app for macOS and Windows.

## Features

- ✨ **Automatic Detection**: Auto-detect new screenshots from watched folders
- 📖 **OCR Search**: Full-text search powered by embedded Tesseract OCR
- 🤖 **AI Enrichment**: Optional AI-powered categorization, tagging, and summary generation
- 📁 **Batch Operations**: Delete, rename, categorize, tag, and move multiple screenshots at once
- 🔒 **Privacy First**: All data stays on your machine — no cloud sync, no telemetry
- 🌍 **Cross-Platform**: Runs on macOS 12+ and Windows 10+

## Prerequisites

- **Rust** 1.70+ (for Tauri backend)
- **Node.js** 20+ (for frontend)
- **Cargo** (comes with Rust)
- **npm** (comes with Node.js)

## Installation

### 1. Clone the repository

```bash
git clone https://github.com/yourusername/snapsort.git
cd snapsort
```

### 2. Install frontend dependencies

```bash
npm install
```

### 3. Install Tauri CLI (if not already installed)

```bash
# Using cargo (Rust package manager)
cargo install tauri-cli

# Or using npm (alternative)
npm install -D @tauri-apps/cli
```

### 4. Build and run in development mode

```bash
# Using npm script
npm run tauri dev

# Or using cargo directly
cargo tauri dev
```

## Configuration

### First Launch

On first launch, SnapSort will:

1. Detect your default screenshot folder(s) for your operating system
2. Prompt you to set up watch folders (can be skipped)
3. Optionally guide you through AI provider setup (Ollama, OpenAI, Anthropic, etc.)

### Settings

Access settings via:
- **macOS**: `SnapSort` → `Settings` in the menu bar
- **Windows**: Settings gear icon in the app

### AI Provider Setup (Optional)

SnapSort uses AI for optional enrichment (categorization, tagging, summaries). You can:

1. **Use Ollama (local)**: Install [Ollama](https://ollama.ai/), then configure it in Settings
2. **Use cloud providers**: Add API keys for OpenAI, Anthropic, or any OpenAI-compatible endpoint
3. **Skip AI entirely**: All core features (OCR, search, batch operations) work without AI

## Project Structure

```
.
├── src/                          # React 19 + TypeScript frontend
│   ├── api/                     # Tauri IPC wrapper stubs
│   ├── components/              # React components
│   ├── stores/                  # Zustand state management
│   │   ├── galleryStore.ts
│   │   ├── batchStore.ts
│   │   └── settingsStore.ts
│   ├── App.tsx
│   ├── main.tsx
│   └── styles.css
├── src-tauri/                   # Rust/Tauri 2 backend
│   ├── src/
│   │   ├── commands/            # Tauri command modules
│   │   ├── db/                  # SQLite repository layer
│   │   ├── ai/                  # AIProvider trait and implementations
│   │   ├── watcher.rs           # File watcher logic
│   │   ├── main.rs
│   │   └── lib.rs
│   ├── migrations/              # SQLite schema migrations
│   │   └── 001_initial.sql
│   ├── Cargo.toml
│   └── tauri.conf.json
├── .baton/                      # Project management (Baton CLI)
├── package.json
├── vite.config.ts
├── tailwind.config.ts
├── tsconfig.json
└── README.md
```

## Development

### Build for production

```bash
npm run tauri build
```

This creates:
- **macOS**: `.dmg` installer
- **Windows**: `.exe` installer
- **Linux**: `.AppImage`

### Run tests

```bash
npm test
```

### Run linter

```bash
npm run lint
```

## Tech Stack

| Layer | Technology |
|-------|------------|
| App Framework | Tauri 2 (Rust + WebView) |
| Frontend | React 19, TypeScript, Tailwind CSS v4 |
| State Management | Zustand |
| Database | SQLite (rusqlite) with FTS5 |
| File Watching | `notify` crate |
| OCR | Tesseract (bundled, offline) |
| AI Integration | Pluggable trait pattern |
| Packaging | Tauri bundler + GitHub Actions |

## Architecture Conventions

### Backend (Rust/Tauri)

- All Tauri commands live in `src-tauri/src/commands/`
- Database queries go through the repository layer (`src-tauri/src/db/`)
- Error handling uses `anyhow::Result` in command handlers
- File operations are always async via `tokio`
- AI providers implement the `AIProvider` trait

### Frontend (React 19/TypeScript)

- Component files use PascalCase (`*.tsx`)
- Hooks use `use` prefix (`useGalleryStore`)
- Zustand stores live in `src/stores/`
- Tailwind CSS only — no inline styles or CSS modules
- All Tauri IPC calls go through typed wrappers in `src/api/`

### Database

- Migrations are numbered sequentially in `src-tauri/migrations/`
- FTS5 virtual table (`screenshots_fts`) stays synced with `screenshots`
- Category source always recorded: `"ai"` or `"user"`

## Contributing

We welcome contributions! Please see:

- [CONTRIBUTING.md](CONTRIBUTING.md) — Contribution guidelines
- [SnapSort_PRD.md](SnapSort_PRD.md) — Product requirements document
- [AGENTS.md](AGENTS.md) — Agent-specific instructions (if applicable)

## License

MIT License — see [LICENSE](LICENSE) for details.

## Support

- **Issues**: [GitHub Issues](https://github.com/yourusername/snapsort/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourusername/snapsort/discussions)
- **Documentation**: [Wiki](https://github.com/yourusername/snapsort/wiki)

---

Built with ❤️ using Tauri, React, and Rust