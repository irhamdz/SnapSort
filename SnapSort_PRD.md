# SnapSort — Product Requirements Document

**Version:** 1.1.0  
**Status:** Draft  
**Last Updated:** 2026-05-15  
**Changelog:** v1.1 — AI made fully optional; batch delete, batch categorization, and batch rename elevated to core P0 features; batch operation model added to ontology; new epic and user stories added; gallery wireframe updated with selection mode.
**License:** MIT (Open Source)  
**Platforms:** macOS · Windows

---

## Table of Contents

1. [Overview](#1-overview)
2. [Ontology](#2-ontology)
3. [Roles & Role-Based Features](#3-roles--role-based-features)
4. [Features](#4-features)
5. [User Stories](#5-user-stories)
6. [Screens & Navigation](#6-screens--navigation)

---

## 1. Overview

### 1.1 Product Vision

SnapSort is an open-source, offline-first desktop application for macOS and Windows that automatically detects new screenshots and surfaces them through a powerful, searchable library. Out of the box — with zero configuration and no AI — users can browse, search (via OCR), batch delete, batch rename, and batch categorize their screenshots manually. AI enrichment is an entirely optional upgrade layer: users who want it can bring their own provider (local Ollama or any cloud API), and those who don't can ignore it entirely without losing any core functionality.

### 1.2 Mission Statement

> Make every screenshot instantly findable, without changing how you take them.

### 1.3 Design Principles

| Principle | Description |
|---|---|
| **Offline-first** | All core features — detection, OCR, search, batch operations — work without internet and without AI. |
| **AI is optional** | AI enrichment is a progressive enhancement, not a prerequisite. Every feature except AI-specific ones works without a configured provider. |
| **Zero friction** | Detection and ingestion are automatic. Users never manually import screenshots. |
| **Batch-first** | Bulk operations (delete, rename, categorize) are first-class features, not afterthoughts. Managing 500 screenshots must be as easy as managing 5. |
| **Bring Your Own AI** | No vendor lock-in. Users configure whichever LLM provider they trust: local or cloud. |
| **Privacy by default** | No telemetry, no accounts, no cloud sync. All data stays on the user's machine. |
| **Open & extensible** | MIT-licensed codebase. AI providers and rule engines are pluggable by contributors. |

### 1.4 Target Platforms

- **macOS:** 12 (Monterey) and above — arm64 and x86_64
- **Windows:** 10 (build 1809) and above — x86_64

### 1.5 Tech Stack Summary

| Layer | Choice | Rationale |
|---|---|---|
| App framework | Tauri 2 (Rust + WebView) | Native performance, small binary (~10MB), cross-platform |
| Frontend | React 19, TypeScript, Tailwind CSS v4 | Modern, well-supported, great OSS ecosystem |
| State management | Zustand | Lightweight, fits Tauri command model |
| Database | SQLite via `rusqlite` | Embedded, zero setup, FTS5 for full-text search |
| File watching | `notify` crate (Rust) | Cross-platform, low-latency OS-native events |
| OCR | Tesseract (bundled) | Offline, MIT-licensed, wide language support |
| AI integration | Provider trait pattern | Pluggable: Ollama, OpenAI-compat, Anthropic |
| Packaging | Tauri bundler + GitHub Actions | Auto `.dmg`, `.exe`, `.AppImage` on release |

---

## 2. Ontology

This section defines the core domain model — the entities, relationships, and vocabulary that the entire product is built on.

### 2.1 Entity Glossary

| Entity | Definition |
|---|---|
| **Screenshot** | The central entity. A captured image file detected by the file watcher. Has both filesystem properties and (optionally) AI-enriched metadata. |
| **Ingestion Event** | The system-level event triggered when a new screenshot file is created in a watched folder. The starting point of the pipeline. |
| **Analysis Job** | An async task that sends a Screenshot to the configured AI Provider and writes the result back to the database. Only created when an AI provider is configured. |
| **AI Provider** | A configured integration with an LLM/vision model endpoint. Can be local (Ollama) or remote (OpenAI-compatible, Anthropic). Entirely optional — the app functions fully without one. |
| **Category** | A top-level classification label assigned to a Screenshot. Can be assigned by AI (via Analysis Job) or manually by the user (via single-edit or Batch Categorization). Mutually exclusive per screenshot. Drawn from a system-defined taxonomy. |
| **Tag** | A non-exclusive label applied to a Screenshot. Tags can be AI-generated, user-created on a single screenshot, or applied via batch operation. Many-to-many with Screenshots. |
| **OCR Result** | The extracted text content from a Screenshot, produced by the embedded Tesseract engine. Always runs on ingestion regardless of AI configuration. Stored and indexed for search. |
| **Thumbnail** | A downsized preview image stored as a blob in SQLite, used to render the gallery without loading full-size files. |
| **Watch Folder** | A filesystem path actively monitored by the file watcher for new screenshot files. Users can add multiple Watch Folders. |
| **Batch Operation** | A user-initiated action applied simultaneously to two or more selected screenshots. Batch operations are always manual (user-driven) and do not require AI. Supported types: Delete, Rename (pattern-based), Categorize (assign category), Tag (add/remove tags), Archive, Move to folder, Add to collection. |
| **Selection Set** | The transient, in-memory set of screenshots the user has selected in the gallery for a pending Batch Operation. Cleared when the batch action completes or the user deselects. |
| **Rename Pattern** | A template string used during Batch Rename to derive a new filename for each selected screenshot. Supports tokens: `{date}`, `{time}`, `{index}`, `{category}`, `{original}`, `{ext}`. Example: `{date}-{category}-{index}` → `2026-04-12-code-001.png`. |
| **Rule** | A user-defined automation instruction. When a Screenshot matches defined conditions (category, tag, app, etc.), the Rule triggers an action (move file, apply tag, etc.). |
| **Smart Folder** | A virtual collection. Not a real directory. A saved search query that dynamically groups Screenshots matching its criteria. |
| **Collection** | A user-created, manually curated group of Screenshots. Analogous to an album. |
| **Search Query** | A user-issued text input that is matched against Screenshot summaries, OCR text, tags, categories, and app names via SQLite FTS5. |
| **App Name** | The name of the application inferred from the screenshot, either via AI vision analysis or (future) OS process metadata. |
| **Settings Profile** | The user's full configuration state: AI providers, watch folders, rules, UI preferences. Stored in the OS app data directory. |

### 2.2 Entity Relationship Diagram

```
┌──────────────┐       ┌──────────────────┐       ┌──────────────┐
│  WatchFolder │──1:N──│ IngestionEvent   │──1:1──│  Screenshot  │
└──────────────┘       └──────────────────┘       └──────┬───────┘
                                                          │
                    ┌─────────────────────────────────────┤
                    │                 │                   │
             ┌──────▼──────┐  ┌──────▼──────┐   ┌───────▼──────┐
             │  AnalysisJob│  │  OCRResult  │   │  Thumbnail   │
             └──────┬──────┘  └─────────────┘   └──────────────┘
                    │
             ┌──────▼──────┐
             │  AIProvider │
             └─────────────┘
                    
┌──────────────┐
│  Screenshot  │──M:N──│  Tag  │
│              │──1:1──│  Category  │
│              │──M:N──│  Collection│
│              │──M:N──│  SmartFolder (virtual)│
└──────────────┘

┌──────────────┐
│    Rule      │──triggers actions on──│  Screenshot  │
└──────────────┘
```

### 2.3 Category Taxonomy (System-Defined)

The AI is prompted to assign exactly one Category per Screenshot:

| Category ID | Display Name | Examples |
|---|---|---|
| `code` | Code & Terminal | IDE, terminal output, error logs, Stack Overflow |
| `design` | Design & UI | Figma, Sketch, mockups, color palettes |
| `document` | Documents & Text | PDFs, Word docs, presentations, spreadsheets |
| `web` | Web & Browser | Websites, articles, forms, web apps |
| `communication` | Communication | Slack, email, Discord, WhatsApp, Zoom |
| `media` | Media & Entertainment | Videos, music apps, streaming, games |
| `finance` | Finance & Commerce | Receipts, invoices, banking, checkout flows |
| `reference` | Reference & Notes | Diagrams, maps, instructions, how-tos |
| `system` | System & OS | System alerts, crash reports, file explorers |
| `other` | Other | Anything that doesn't fit above |

### 2.4 Ingestion Pipeline (State Machine)

```
[File Created in Watch Folder]
         │
         ▼
   DETECTED
   (heuristic check: is this a screenshot?)
         │
    ┌────┴────┐
   YES        NO → discard
    │
    ▼
   QUEUED
   (thumbnail generated, OCR starts in background — always runs)
         │
         ▼
   OCR COMPLETE
   (ocr_text written to DB, FTS5 index updated)
         │
    ┌────┴─────────────────────┐
    │  AI provider configured? │
   YES                         NO
    │                          │
    ▼                          ▼
 ANALYZING                  READY (no AI)
 (AI Provider called)       (searchable via OCR;
    │                        category = null,
    ├── SUCCESS              awaits manual action
    │      ▼                 or future AI setup)
    │   ENRICHED
    │   (full metadata)
    │
    └── FAILED
           ▼
        PARTIAL
        (OCR only, retry eligible)
```

> **Key principle:** A screenshot in `READY (no AI)` state is fully usable. It is visible in the gallery, searchable by OCR text, and eligible for all batch operations. AI enrichment only adds the category, tags, summary, and app name on top.

### 2.5 Screenshot Lifecycle States

| State | Description | AI Required? |
|---|---|---|
| `detected` | File found, heuristic passed, not yet processed | No |
| `queued` | Awaiting OCR and/or AI analysis | No |
| `ready` | OCR complete, no AI provider configured or AI skipped. Fully usable. | No |
| `analyzing` | AI job in progress | Yes |
| `enriched` | AI analysis complete, fully indexed | Yes |
| `partial` | AI failed or timed out; OCR data available, AI fields empty. Retry eligible. | Yes (failed) |
| `archived` | Hidden from main library; accessible via filter | No |
| `deleted` | Removed from DB (file may or may not still exist on disk) | No |

---

## 3. Roles & Role-Based Features

SnapSort is a single-user desktop application. However, it recognizes two meaningful "modes" a user operates in, plus a third role for contributors in the open-source context.

### 3.1 Role Definitions

---

#### 🧑‍💻 Role 1: End User (Primary)

The everyday person using the desktop app. May be a developer, designer, researcher, or knowledge worker.

**Persona variants:**

| Persona | Description |
|---|---|
| **Power User** | Wants full control — custom rules, multiple watch folders, multiple AI providers |
| **Casual User** | Wants it to "just work" — uses defaults, rarely opens settings |
| **Privacy-First User** | Insists on local AI (Ollama), no cloud calls whatsoever |
| **Cloud User** | Happy to use OpenAI or Anthropic for best accuracy, doesn't mind cost |

**Core capabilities (available with or without AI):**

| Capability | Description |
|---|---|
| Library browsing | View all screenshots in a scrollable, filterable gallery |
| Search | Full-text search via OCR text, tags, categories, summaries |
| Detail view | View full screenshot with all metadata, copy OCR text, edit tags |
| **Batch selection** | Select multiple screenshots via checkbox, Shift-click, or Cmd/Ctrl+A |
| **Batch delete** | Permanently delete all selected screenshots in one action |
| **Batch categorize** | Assign a category to all selected screenshots at once |
| **Batch rename** | Rename selected screenshots using a pattern template |
| **Batch tag** | Add or remove tags across all selected screenshots |
| **Batch archive** | Archive all selected screenshots in one action |
| **Batch move** | Move selected screenshot files to a chosen folder |
| **Batch add to collection** | Add selected screenshots to an existing or new collection |
| Manual categorization | Assign or change category on a single screenshot |
| Collections | Create, name, and curate personal screenshot albums |
| Smart Folders | Browse dynamically filtered groups (Today, By App, Favorites, etc.) |
| Watch folder management | Add/remove directories to monitor |
| Manual tagging | Add, remove, or rename tags on any screenshot |
| Archive/delete | Remove screenshots from library view or permanently |
| Export | Copy file path, open in Finder/Explorer, export with metadata |
| Preference management | UI theme, language, notification settings |

**AI-dependent capabilities (require a configured provider):**

| Capability | Description |
|---|---|
| AI auto-categorization | AI assigns category on ingestion |
| AI tag generation | AI suggests tags on ingestion |
| AI summary generation | AI writes a one-line description |
| App name detection | AI identifies the visible application |
| AI provider configuration | Add API keys, choose models, test connections |

---

#### 🔧 Role 2: Power User / Self-Hoster

A technically proficient user who wants to extend SnapSort beyond its defaults.

**Additional capabilities on top of End User:**

| Capability | Description |
|---|---|
| Custom AI provider endpoint | Point to any OpenAI-compatible URL (LM Studio, Groq, Together AI, etc.) |
| Custom prompt override | Edit the system prompt used for screenshot analysis |
| Rule chaining | Create multi-condition, multi-action rules |
| Database inspection | Access SQLite DB directly (path is exposed in settings) |
| CLI mode (future) | Trigger ingestion or search via command line |
| Plugin configuration | Enable/disable community-contributed AI provider plugins |

---

#### 🛠️ Role 3: Open Source Contributor

A developer contributing to the SnapSort codebase on GitHub.

**Concerns (not in-app, but in PRD scope):**

| Area | Description |
|---|---|
| Provider plugin interface | Must implement the `AIProvider` trait in Rust to add a new AI integration |
| Contribution guide | `CONTRIBUTING.md` defines PR standards, issue templates, and branch strategy |
| Issue labels | `bug`, `enhancement`, `provider-plugin`, `good first issue`, `needs-design` |
| Release process | GitHub Actions auto-builds binaries on version tag push |
| Localization | Strings extracted to i18n JSON files; contributors can add new locales |

---

## 4. Features

### 4.1 Feature List with Priority

> **Priority legend:** P0 = must ship for any release · P1 = ships in Phase 1 or 2 · P2 = Phase 2–3 · P3 = future  
> **AI Required** column marks features that only function with a configured AI provider.

| ID | Feature | Priority | AI Required | Role | Phase |
|---|---|---|---|---|---|
| F-01 | Automatic screenshot detection | P0 | No | End User | 1 |
| F-02 | Screenshot ingestion pipeline | P0 | No | End User | 1 |
| F-03 | Thumbnail generation | P0 | No | End User | 1 |
| F-04 | SQLite storage & indexing | P0 | No | End User | 1 |
| F-05 | Searchable gallery view | P0 | No | End User | 1 |
| F-06 | Detail panel with metadata | P0 | No | End User | 1 |
| F-11 | OCR text extraction | P0 | No | End User | 1 |
| F-17 | System tray icon & notifications | P0 | No | End User | 1 |
| F-18 | Watch folder management | P0 | No | End User | 1 |
| F-27 | Onboarding flow | P0 | No | End User | 1 |
| F-28 | Settings (general preferences) | P0 | No | End User | 1 |
| **F-36** | **Batch selection mode** | **P0** | **No** | **End User** | **1** |
| **F-37** | **Batch delete** | **P0** | **No** | **End User** | **1** |
| **F-38** | **Batch manual categorization** | **P0** | **No** | **End User** | **1** |
| **F-39** | **Batch rename (pattern-based)** | **P0** | **No** | **End User** | **1** |
| **F-40** | **Batch tag (add/remove)** | **P1** | **No** | **End User** | **1** |
| **F-41** | **Batch archive** | **P1** | **No** | **End User** | **1** |
| **F-42** | **Batch move to folder** | **P1** | **No** | **End User** | **2** |
| **F-43** | **Batch add to collection** | **P1** | **No** | **End User** | **2** |
| **F-44** | **Manual single-screenshot categorization** | **P0** | **No** | **End User** | **1** |
| F-12 | Full-text search (FTS5) | P1 | No | End User | 1 |
| F-19 | Smart folders (built-in) | P1 | No | End User | 2 |
| F-20 | Manual tagging (single screenshot) | P1 | No | End User | 2 |
| F-21 | Collections | P2 | No | End User | 2 |
| F-24 | Archive & delete (single) | P1 | No | End User | 1 |
| F-25 | Export / open in Finder/Explorer | P1 | No | End User | 2 |
| F-26 | Copy OCR text to clipboard | P1 | No | End User | 2 |
| F-29 | Light / dark theme | P2 | No | End User | 2 |
| F-32 | Screenshot deduplication | P3 | No | End User | 3 |
| F-33 | Auto-updater | P2 | No | End User | 3 |
| F-34 | i18n / localization | P3 | No | Contributor | 3 |
| F-35 | CLI interface | P3 | No | Power User | 3 |
| F-07 | Ollama (local) AI integration | P1 | Yes | End User | 2 |
| F-08 | AI category classification | P1 | Yes | End User | 2 |
| F-09 | AI tag generation | P1 | Yes | End User | 2 |
| F-10 | AI summary generation | P1 | Yes | End User | 2 |
| F-13 | OpenAI-compatible provider | P1 | Yes | End User | 2 |
| F-14 | Anthropic provider | P1 | Yes | End User | 2 |
| F-15 | Custom OpenAI-compat endpoint | P2 | Yes | Power User | 2 |
| F-16 | BYOK settings panel | P1 | Yes | End User | 2 |
| F-22 | Rule engine | P2 | No | Power User | 3 |
| F-23 | Custom AI prompt override | P2 | Yes | Power User | 3 |
| F-30 | App name detection (AI) | P1 | Yes | End User | 2 |
| F-31 | Retry failed AI analysis | P2 | Yes | End User | 2 |

### 4.2 Feature Details

---

#### F-01 · Automatic Screenshot Detection

The file watcher monitors all configured Watch Folders using OS-native filesystem events (`notify` crate). When a new file is created, the system applies heuristics to confirm it is a screenshot before entering the ingestion pipeline.

**Detection heuristics (all checked in order):**

1. File extension is `.png`, `.jpg`, `.jpeg`, or `.webp`
2. File appears in a known default screenshot directory
3. Filename matches known patterns: `Screenshot*`, `Capture*`, `Screen Shot*`, `*screen*`, `*capture*`, timestamp-only filenames
4. File was created by a known screenshot process (checked via file metadata on macOS: `com.apple.screencaptureui`; on Windows: `SnippingTool.exe`, `ScreenClippingHost.exe`)
5. File dimensions are at least 400×300 pixels (filters out icons and avatars)

A file must pass heuristic checks 1 AND (2 OR 3 OR 4 OR 5) to be considered a screenshot.

---

#### F-07 · Ollama (Local) AI Integration

Ollama is the **default** AI provider. On first launch, SnapSort checks if Ollama is running at `http://localhost:11434`. If detected, it automatically configures it. If not, the onboarding prompts the user to install it or choose an alternative.

**Default model priority (user can override):**

1. `moondream` — fastest, runs on CPU, ~1.7GB
2. `llava:7b` — more accurate, needs ~6GB RAM
3. `llava:13b` — best local quality, needs ~12GB RAM

---

#### F-12 · Full-Text Search (FTS5)

Search is powered by SQLite FTS5 and covers:
- `summary` — AI-generated one-line description
- `ocr_text` — raw OCR output
- `category` — category label
- `app_detected` — detected application name
- `tags` — all tag names
- `user_notes` — user-written notes

Search matches are ranked by BM25 relevance. Filters (date range, category, tag, app) can be composed on top of text search.

---

#### F-22 · Rule Engine

Rules follow an `IF [conditions] THEN [actions]` model:

**Condition types:**

| Condition | Example |
|---|---|
| Category equals | `category == "finance"` |
| Tag contains | `tags contains "receipt"` |
| App name equals | `app_detected == "Figma"` |
| OCR contains text | `ocr_text contains "ERROR"` |
| File size greater than | `file_size > 5MB` |
| Created before/after date | `created_at < 30 days ago` |

**Action types:**

| Action | Example |
|---|---|
| Move file to folder | Move to `~/Documents/Receipts/` |
| Add tag | Add tag `#work` |
| Add to collection | Add to "Design Reviews" |
| Rename file | Rename to `{date}-{app}-{summary}` |
| Archive | Mark as archived |
| Send notification | Notify user with summary |

---

#### F-36 · Batch Selection Mode

Multi-select is a first-class interaction mode in the gallery. It can be entered in three ways:

1. **Checkbox click** — hovering a thumbnail reveals a checkbox in its top-left corner; clicking it enters selection mode
2. **Shift+click** — click one thumbnail, Shift+click another to select the range between them
3. **Cmd+A / Ctrl+A** — selects all screenshots matching the current filter/search

**Selection mode behaviors:**
- A persistent **Batch Action Bar** slides up from the bottom of the gallery while any screenshots are selected
- The count of selected items is shown: "N selected"
- A "Select All" and "Deselect All" link appears in the toolbar header
- Individual cards show a filled checkbox when selected, empty on hover when not
- Selection persists through scrolling but is cleared when navigating to a different sidebar section
- Clicking a thumbnail normally while in selection mode toggles its selection (does not open detail view)
- Pressing Escape exits selection mode and clears the selection set

---

#### F-37 · Batch Delete

Permanently removes all selected screenshots from the library and sends the files to the OS trash.

**Behavior:**
- Triggered from the Batch Action Bar → "Delete" button
- Confirmation dialog lists the count: "Delete 47 screenshots? Files will be moved to Trash."
- Confirmation requires clicking a red "Delete 47" button (not Enter, to prevent accidents)
- On confirm: all DB records removed, files sent to OS trash asynchronously
- Gallery updates immediately (cards disappear with a fade-out animation)
- Undo is not possible from within SnapSort (files remain recoverable from OS Trash for the OS-defined retention period)
- Toast: "47 screenshots deleted. View in Trash →" (OS Trash link)

---

#### F-38 · Batch Manual Categorization

Assigns a single category from the system taxonomy to all selected screenshots simultaneously. Does not require AI.

**Behavior:**
- Triggered from Batch Action Bar → "Categorize" button
- Opens a compact popover listing all 10 system categories with icons
- User selects one category; the popover closes immediately
- All selected screenshots are updated in the DB in a single transaction
- Category badges on the affected thumbnail cards update in real time
- The source of the category is recorded as `"user"` (not `"ai"`) in the DB
- If some selected screenshots already have a different category, a warning is shown: "This will overwrite the existing category on N screenshots that already have one." with a "Continue" confirmation

---

#### F-39 · Batch Rename (Pattern-Based)

Renames the actual files on disk for all selected screenshots according to a user-defined naming pattern.

**Pattern tokens:**

| Token | Resolves to | Example |
|---|---|---|
| `{date}` | File creation date | `2026-04-12` |
| `{time}` | File creation time | `15-22-01` |
| `{datetime}` | Combined date + time | `2026-04-12_15-22-01` |
| `{index}` | Sequential number within the batch (zero-padded) | `001`, `002` ... |
| `{category}` | Assigned category (or "uncategorized" if none) | `code` |
| `{original}` | Original filename without extension | `Screenshot 2026-04-12` |
| `{ext}` | File extension without dot | `png` |

**Behavior:**
- Triggered from Batch Action Bar → "Rename" button
- Opens a Rename modal with:
  - A pattern input field (e.g. `{date}-{category}-{index}`)
  - A live preview table showing old name → new name for the first 5 selected files
  - Token reference guide (expandable)
  - Conflict detection: if any resulting filename already exists, that row is highlighted red and the "Apply" button is disabled with an error message
- On apply: files are renamed on disk; DB `filename` and `file_path` fields are updated
- Toast: "N files renamed successfully"
- If any rename fails (e.g. permissions error), a detailed error report is shown per file

---

#### F-40 · Batch Tag

Adds or removes one or more tags across all selected screenshots simultaneously.

**Behavior:**
- Triggered from Batch Action Bar → "Tag" button
- Opens a Tag popover with two modes:
  - **Add tags**: autocomplete multi-select; selected tags are added to all selected screenshots
  - **Remove tags**: shows tags that exist on at least one selected screenshot; selected tags are removed from all selected screenshots that have them
- Changes applied in a single DB transaction
- Toast: "Tags updated on N screenshots"

---

#### F-41 · Batch Archive

Marks all selected screenshots as archived, hiding them from the main library view.

**Behavior:**
- Triggered from Batch Action Bar → "Archive" button
- No confirmation dialog (reversible action)
- All selected screenshots set to `is_archived = true` in DB
- Cards disappear from the current view with a fade animation
- Toast: "N screenshots archived · Undo" — clicking "Undo" reverses the operation within 5 seconds

---

#### F-42 · Batch Move to Folder

Moves the actual files on disk for all selected screenshots to a chosen directory.

**Behavior:**
- Triggered from Batch Action Bar → "Move to Folder" button
- Opens OS native folder picker
- On folder selection: files moved on disk; `file_path` updated in DB; watch folder list checked — if destination is a watched folder, no re-ingestion occurs (deduplicated by file path)
- Toast: "N files moved to [folder path]"
- If any file fails to move, per-file error details shown in a results dialog

---

#### F-43 · Batch Add to Collection

Adds all selected screenshots to one or more existing collections, or to a newly created one.

**Behavior:**
- Triggered from Batch Action Bar → "Add to Collection" button
- Opens a collection picker popover listing all user collections with checkboxes
- "New collection..." option at the bottom creates and immediately selects a new collection
- Multiple collections can be selected; all are applied at once
- Adding to a collection is DB-only; no files are moved
- Toast: "N screenshots added to [collection name]"

### Role: End User

---

#### Epic: First Launch & Onboarding

---

**US-001 · First Launch — Welcome**

> As a new user, I want to be greeted with a clear onboarding flow so that I understand what SnapSort does and can get it working in under 2 minutes.

**Acceptance Criteria:**
- On first launch, the app shows a multi-step onboarding modal (not a full screen takeover)
- Step 1 explains the product in one sentence with a short animated illustration
- Step 2 detects and displays the default screenshot folder for the user's OS
- Step 3 checks for Ollama and guides the user if not found
- Step 4 confirms setup with a "Take your first screenshot to try it" prompt
- Onboarding can be dismissed at any step and resumed from settings
- Onboarding is not shown on subsequent launches

---

**US-002 · First Launch — Watch Folder Auto-Detection**

> As a new user, I want the app to automatically detect my default screenshot folder so I don't have to manually configure anything to get started.

**Acceptance Criteria:**
- macOS default: `~/Desktop` and `~/Pictures/Screenshots`
- Windows default: `%USERPROFILE%\Pictures\Screenshots` and Desktop
- Both are pre-populated in Watch Folders on first launch
- User sees a confirmation screen showing detected folders with file counts
- User can uncheck any folder before confirming

---

**US-003 · First Launch — AI Provider Setup (Optional)**

> As a new user, I want to optionally configure an AI provider during onboarding so that screenshots can be auto-analyzed — but I also want to be able to skip this entirely and still have a fully functional app.

**Acceptance Criteria:**
- Onboarding AI step is clearly labeled "Optional — set up AI" with a subtitle: "SnapSort works great without AI. You can always set this up later in Settings."
- A prominent "Skip for now →" link is equally visible to the "Continue" button — not hidden or de-emphasized
- If Ollama is detected at `localhost:11434`: show a green "Ollama detected!" callout with one-click opt-in; user picks a model and continues
- If Ollama is not detected: show three equal-weight cards: "Install Ollama", "Use an API Key", "Skip for now"
- "Use an API Key" shows provider cards: OpenAI, Anthropic, Custom
- A "Test Connection" button verifies the key before proceeding
- Skipping AI: screenshots will still be detected, OCR will run, and all batch operations will be available immediately
- The AI step is shown on first launch only. If skipped, a non-intrusive banner appears in the gallery on day 2: "Want smarter search? Set up AI in Settings. [Set up] [Dismiss]"

---

#### Epic: Library & Discovery

---

**US-010 · Browse Screenshot Library**

> As an end user, I want to see all my screenshots in a visual gallery so that I can quickly browse and find what I'm looking for.

**Acceptance Criteria:**
- Gallery is the default view on launch
- Screenshots are displayed as a responsive grid of thumbnails (default: medium size)
- Grid size can be toggled: small (more per row), medium, large
- Screenshots are sorted by date captured, newest first (default)
- Sort options: newest first, oldest first, by category, by app
- Each thumbnail card shows: image preview, date, category badge, top 2 tags
- Scrolling is virtual (only visible cards rendered) to handle 10,000+ screenshots without lag
- A header count shows "N screenshots" matching the current filter
- Empty state shows an illustration with "No screenshots yet" and a prompt to take one

---

**US-011 · Search Screenshots by Text**

> As an end user, I want to type natural language into a search bar and find relevant screenshots so that I can locate any screenshot without knowing its exact name or folder.

**Acceptance Criteria:**
- A persistent search bar is visible at the top of the library at all times
- Search triggers on keystroke with 300ms debounce (no submit required)
- Search covers: summary, OCR text, tags, category, app name, user notes
- Results are ranked by relevance (BM25), not just date
- Each result shows a highlighted snippet of matched OCR text beneath the thumbnail
- Clearing the search bar restores the full library
- If no results, show "No screenshots match '[query]'" with suggested alternatives
- Search is entirely local — no network call made

---

**US-012 · Filter by Category**

> As an end user, I want to filter my library to a specific category so that I can see only code screenshots or only receipts, for example.

**Acceptance Criteria:**
- Sidebar lists all categories with screenshot counts in parentheses
- Clicking a category filters the gallery to show only screenshots in that category
- Multiple categories can be selected simultaneously (OR logic)
- Active filters are shown as chips above the gallery
- Clicking a chip removes that filter
- "Clear all filters" link appears when any filter is active
- Category filter can be combined with text search

---

**US-013 · Filter by Tag**

> As an end user, I want to filter my library by one or more tags so that I can find screenshots related to a specific project or topic.

**Acceptance Criteria:**
- Sidebar has a "Tags" section listing all tags sorted by frequency
- Clicking a tag applies it as a filter
- Multiple tags can be selected (AND logic — screenshot must have all selected tags)
- The tag list is searchable with a small input field within the sidebar
- Tags show count badges

---

**US-014 · Filter by Date Range**

> As an end user, I want to filter screenshots by a date range so that I can find things I captured last week or in a specific month.

**Acceptance Criteria:**
- A date range picker is accessible from the filter panel
- Presets available: Today, Yesterday, Last 7 days, Last 30 days, This month, Custom range
- Custom range shows a calendar date picker
- Date filter composes with text search and category/tag filters

---

**US-015 · Smart Folders**

> As an end user, I want to see pre-built smart folders like "Today", "Favorites", and "Code" so that I can quickly jump to common views without setting up filters manually.

**Acceptance Criteria:**
- Sidebar contains a "Smart Folders" section above categories
- Built-in smart folders:
  - **All Screenshots** — no filter (default home view)
  - **Today** — created_at = today
  - **This Week** — created_at within last 7 days
  - **Favorites** — is_favorite = true
  - **Unanalyzed** — ai_analyzed_at IS NULL
  - **By App** — expandable list of distinct apps detected
- Smart folders show their live count
- Clicking a smart folder switches the gallery to that filtered view
- Smart folders cannot be deleted (they are system-defined)

---

#### Epic: Screenshot Detail

---

**US-020 · View Screenshot Detail**

> As an end user, I want to click a screenshot in the gallery to see its full-size view and all metadata so that I can read the content clearly and understand its context.

**Acceptance Criteria:**
- Clicking a thumbnail opens a detail panel (slide-in from the right, or full modal — TBD in design)
- Detail panel shows: full-resolution image with pan/zoom support
- Metadata section shows: date, time, file size, dimensions, file path, category badge, app name, all tags
- AI summary is shown as a short paragraph beneath the image
- OCR text is shown in a collapsible section with monospace font
- Navigation arrows allow cycling through gallery items without closing the panel
- Keyboard shortcut (Escape) closes the panel
- Keyboard arrows navigate between screenshots

---

**US-021 · Copy OCR Text**

> As an end user, I want to copy the OCR-extracted text from a screenshot with one click so that I can paste it elsewhere without retyping.

**Acceptance Criteria:**
- Detail panel shows a "Copy Text" button next to the OCR section
- Clicking it copies the full OCR content to clipboard
- A brief toast notification confirms: "Text copied to clipboard"
- If OCR text is empty, the button is disabled with tooltip: "No text detected in this screenshot"
- OCR text is also selectable/highlightable by mouse for partial copying

---

**US-022 · Edit Tags on a Screenshot**

> As an end user, I want to add, remove, or rename tags on a specific screenshot so that I can correct AI-generated tags or add my own context.

**Acceptance Criteria:**
- Tag chips in the detail panel have an "×" to remove them
- An "+ Add tag" input appears after the last tag chip
- Typing in the input shows an autocomplete dropdown with existing tags
- Pressing Enter or selecting from dropdown adds the tag
- New tags created here are immediately available in the tag filter list
- Tag changes are saved instantly (no save button required)
- Tags are visually distinguished: AI-generated tags show a small bot icon; user-added tags show a pencil icon

---

**US-023 · Mark as Favorite**

> As an end user, I want to mark a screenshot as a favorite so that I can quickly retrieve important captures later.

**Acceptance Criteria:**
- A star/heart icon is visible in the top-right corner of each thumbnail card on hover
- The same icon is always visible in the detail panel
- Clicking the icon toggles the favorite state
- Favorited screenshots appear in the "Favorites" smart folder
- Icon is filled/highlighted to indicate favorited state

---

**US-024 · Open in Finder / File Explorer**

> As an end user, I want to open a screenshot's containing folder in Finder (macOS) or File Explorer (Windows) so that I can access the original file directly.

**Acceptance Criteria:**
- Detail panel has a "Show in Finder" / "Show in Explorer" button
- Clicking it opens the OS file manager with the screenshot file highlighted
- This works even if the screenshot has been moved by a Rule

---

**US-025 · Archive a Screenshot**

> As an end user, I want to archive a screenshot so that it disappears from my main library view without permanently deleting the file.

**Acceptance Criteria:**
- "Archive" option is available in the detail panel and via right-click on thumbnail
- Archiving a screenshot sets `is_archived = true` in the database
- Archived screenshots do not appear in the main library or smart folder views by default
- A toggle in the sidebar enables "Show archived" to make them visible again
- Archived screenshots can be unarchived from the detail panel

---

**US-026 · Delete a Screenshot**

> As an end user, I want to permanently delete a screenshot both from the library and from disk so that I can clean up screenshots I no longer need.

**Acceptance Criteria:**
- "Delete" option available in detail panel and via right-click context menu
- A confirmation dialog appears: "Delete permanently? This will remove the file from your disk and cannot be undone."
- Confirmation requires clicking a red "Delete" button (not Enter)
- On confirmation, the DB record is removed and the file is sent to OS trash (not bypassed)
- Gallery updates immediately to remove the deleted card

---

#### Epic: Collections

---

**US-030 · Create a Collection**

> As an end user, I want to create named collections of screenshots so that I can group related captures for a project or topic.

**Acceptance Criteria:**
- "New Collection" button in the sidebar under "Collections"
- Clicking prompts a modal with a name input
- Collection name must be 1–80 characters, no restriction on characters
- On creation, the empty collection appears in the sidebar
- Collections are sorted alphabetically in the sidebar
- Maximum of 200 collections allowed

---

**US-031 · Add Screenshots to a Collection**

> As an end user, I want to add one or more screenshots to a collection so that I can build curated groups.

**Acceptance Criteria:**
- Right-clicking a thumbnail shows "Add to Collection" with a submenu of existing collections
- In the detail panel, a "Collections" section shows which collections the screenshot belongs to with an "Add" button
- Screenshots can belong to multiple collections simultaneously
- Adding to a collection does not move the file — it is a database-only relationship

---

**US-032 · Remove a Screenshot from a Collection**

> As an end user, I want to remove a screenshot from a collection so that I can curate it without deleting the screenshot.

**Acceptance Criteria:**
- In the detail panel, each collection badge has an "×" to remove membership
- Right-clicking a thumbnail while viewing a collection shows "Remove from this collection"
- Removal does not delete the screenshot — only the collection membership record

---

---

#### Epic: Batch Operations

---

**US-090 · Enter Batch Selection Mode**

> As an end user, I want to select multiple screenshots at once so that I can perform the same action on all of them without repeating it individually.

**Acceptance Criteria:**
- Hovering any thumbnail card reveals a checkbox in its top-left corner
- Clicking the checkbox selects that screenshot and enters selection mode
- While in selection mode, all other thumbnails also show their checkboxes permanently (not just on hover)
- Shift+clicking a second thumbnail selects all cards between the first and second click (inclusive), following the current sort order
- Cmd+A (macOS) / Ctrl+A (Windows) selects all screenshots in the current filtered view
- A Batch Action Bar slides up from the bottom of the gallery displaying: the count of selected items ("47 selected"), and action buttons
- "Select All" and "Deselect All" links appear in the gallery header
- Pressing Escape exits selection mode and clears the selection set
- Clicking a selected thumbnail in selection mode deselects it (does not open detail panel)
- Clicking an unselected thumbnail in selection mode selects it (does not open detail panel)

---

**US-091 · Batch Delete Screenshots**

> As an end user, I want to delete multiple screenshots at once so that I can quickly clean up large groups of unwanted captures.

**Acceptance Criteria:**
- "Delete" button is visible in the Batch Action Bar when one or more screenshots are selected
- Clicking "Delete" shows a confirmation dialog: "Delete [N] screenshots? These files will be moved to the Trash."
- The dialog shows the file count prominently
- Confirmation requires clicking a red "Delete [N]" button (not Enter) to prevent accidental mass deletion
- On confirmation: all DB records removed synchronously; files moved to OS Trash asynchronously
- Gallery cards disappear immediately with a subtle fade-out animation
- A toast notification appears: "[N] screenshots deleted. Open Trash →"
- Selection mode is exited after the operation completes
- If all screenshots in the current filtered view are deleted, the empty state is shown

---

**US-092 · Batch Manually Categorize Screenshots**

> As an end user without AI configured, I want to assign a category to multiple screenshots at once so that I can organize them without needing AI.

**Acceptance Criteria:**
- "Categorize" button is visible in the Batch Action Bar when one or more screenshots are selected
- Clicking "Categorize" opens a compact popover with the 10 system categories listed with their icons and names
- Selecting a category immediately closes the popover and applies it to all selected screenshots
- Category badges on all affected thumbnail cards update in real time without a page reload
- The category source is recorded as `"user"` in the DB (not `"ai"`)
- If some selected screenshots already have a category assigned (by user or AI), a warning prompt appears first: "This will overwrite the existing category on [N] screenshots. Continue?" with Cancel and Continue buttons
- If AI is configured, this action still works and overwrites any AI-assigned category
- Toast: "[N] screenshots categorized as [Category Name]"

---

**US-093 · Batch Rename Screenshots Using a Pattern**

> As an end user, I want to rename multiple screenshot files at once using a naming template so that I can give them meaningful, consistent names without doing it one by one.

**Acceptance Criteria:**
- "Rename" button is visible in the Batch Action Bar when two or more screenshots are selected
- Clicking "Rename" opens a Rename modal containing:
  - A pattern input field, pre-filled with the last-used pattern (default: `{date}_{index}`)
  - A collapsible token reference showing all available tokens and their descriptions
  - A live preview table with columns "Original Name" → "New Name" for the first 5 selected files, updating in real time as the pattern changes
  - A "N more files..." note if more than 5 are selected
- Pattern must include at least one token or static character (empty pattern is invalid)
- If any resulting filename would conflict with an existing file, that row is highlighted red and a warning shows: "Some filenames conflict with existing files." The "Apply" button is disabled until the pattern is changed
- Clicking "Apply": files are renamed on disk, `filename` and `file_path` are updated in DB
- Toast: "[N] files renamed successfully"
- If any rename fails (permissions, locked file), a result dialog lists failed files with error messages; successfully renamed files are not rolled back
- Rename modal can be dismissed with Escape or a Cancel button without any changes applied

---

**US-094 · Batch Tag Screenshots**

> As an end user, I want to add or remove tags across multiple screenshots at once so that I can bulk-label a group without editing each one.

**Acceptance Criteria:**
- "Tag" button is visible in the Batch Action Bar when one or more screenshots are selected
- Clicking "Tag" opens a popover with two tabs: "Add Tags" and "Remove Tags"
- **Add Tags tab**: a multi-select autocomplete input; existing tags are suggested; new tags can be typed and added; selected tags appear as chips
- **Remove Tags tab**: shows a list of tags that appear on at least one selected screenshot; user selects which to remove
- Applying adds/removes selected tags from all selected screenshots in one DB transaction
- Tags added via batch are marked with source `"user"` in the DB
- Tags already present on a screenshot are not duplicated
- Toast: "Tags updated on [N] screenshots"

---

**US-095 · Batch Archive Screenshots**

> As an end user, I want to archive multiple screenshots at once so that I can hide a group from my library without permanently deleting them.

**Acceptance Criteria:**
- "Archive" button is visible in the Batch Action Bar
- No confirmation dialog (archive is reversible)
- All selected screenshots set to `is_archived = true` in DB
- Cards disappear from the current view with a fade animation
- Toast: "[N] screenshots archived · Undo" — clicking "Undo" reverses the entire batch within 5 seconds
- After 5 seconds the Undo option disappears; archived screenshots remain retrievable by enabling "Show archived" in the sidebar

---

**US-096 · Batch Move Screenshots to a Folder**

> As an end user, I want to move the actual files of multiple screenshots to a specific folder on my disk so that I can organize my filesystem without renaming each file manually.

**Acceptance Criteria:**
- "Move to Folder" button is visible in the Batch Action Bar (Phase 2)
- Clicking opens the OS native folder picker
- Selected destination folder is shown in a confirmation dialog: "Move [N] files to [path]?"
- On confirmation: files are moved on disk; `file_path` is updated in DB for each file
- If the destination is already a watched folder, moved files are not re-ingested (identified by DB record match)
- If destination is NOT a watched folder, an informational note appears: "This folder is not in your Watch Folders. These screenshots will no longer be auto-tracked unless you add it."
- Toast: "[N] files moved to [folder name]"
- If any file fails to move, a result dialog shows per-file errors; successful moves are not rolled back

---

**US-097 · Batch Add Screenshots to a Collection**

> As an end user, I want to add multiple screenshots to a collection at once so that I can curate albums efficiently.

**Acceptance Criteria:**
- "Add to Collection" button is visible in the Batch Action Bar
- Clicking opens a collection picker popover listing all user collections with checkboxes
- Multiple collections can be checked simultaneously
- A "New collection..." option at the bottom opens an inline name input; on Enter, the new collection is created and immediately checked
- Clicking "Apply" adds all selected screenshots to all checked collections in a single DB transaction
- Toast: "[N] screenshots added to [collection name]" (or "[N] screenshots added to [X] collections" if multiple)
- Screenshots already in a selected collection are not duplicated

---

**US-040 · AI Auto-Categorization**

> As an end user, I want newly detected screenshots to be automatically categorized by AI so that I never have to manually sort them.

**Acceptance Criteria:**
- Within 10 seconds of detection (on local Ollama), the screenshot is assigned a category
- Category is displayed on the thumbnail card as a colored badge
- If AI is not configured, the category field shows "Uncategorized"
- Category can be manually corrected in the detail panel via a dropdown

---

**US-041 · AI Summary Generation**

> As an end user, I want the AI to generate a one-line human-readable summary of each screenshot so that I can understand its content without opening it.

**Acceptance Criteria:**
- Summary is shown in the detail panel beneath the image
- Summary is also used as the tooltip when hovering over a thumbnail in the gallery
- Summary is indexed in the FTS5 database for search
- Maximum 200 characters enforced on AI prompt output

---

**US-042 · Re-run AI Analysis**

> As an end user, I want to trigger AI re-analysis on a screenshot so that I can update its metadata if the initial analysis was wrong or if I've switched to a better AI provider.

**Acceptance Criteria:**
- Detail panel shows a "Re-analyze" button
- Clicking it triggers a new Analysis Job with the currently active AI provider
- During analysis, the button shows a spinner and is disabled
- On completion, all AI-generated fields (category, tags, summary, app name) are updated
- A toast notification confirms: "Analysis complete"

---

**US-043 · View Analysis Status**

> As an end user, I want to see which screenshots are pending analysis so that I know when the AI is still working.

**Acceptance Criteria:**
- Thumbnail cards in `queued` or `analyzing` state show a subtle loading indicator (animated border or spinner overlay)
- The "Unanalyzed" smart folder shows all screenshots not yet enriched
- A status indicator in the bottom status bar shows: "Analyzing N screenshots..." when jobs are active
- Screenshots in `partial` state show a warning badge; hovering shows "Analysis failed — click to retry"

---

#### Epic: AI Provider Configuration (BYOK)

---

**US-050 · Add Ollama as AI Provider**

> As a privacy-conscious user, I want to use a local Ollama instance as my AI provider so that no screenshot data ever leaves my computer.

**Acceptance Criteria:**
- In Settings → AI Providers, "Ollama (Local)" is listed as the first option
- User can set the base URL (default: `http://localhost:11434`)
- User can select the model from a dropdown populated by querying the Ollama API
- A "Test Connection" button verifies connectivity and shows model list
- If Ollama is unreachable, a clear error message is shown: "Cannot reach Ollama at [URL]. Is it running?"
- Ollama provider can be set as the active (default) provider

---

**US-051 · Add OpenAI as AI Provider**

> As an end user, I want to add my OpenAI API key so that I can use GPT-4o vision for higher accuracy screenshot analysis.

**Acceptance Criteria:**
- "OpenAI" option in Settings → AI Providers
- API key field (masked input)
- Model dropdown: `gpt-4o`, `gpt-4o-mini` (default), `gpt-4-turbo`
- "Test Connection" sends a minimal vision request and shows ✓ or an error
- API key is stored in the OS keychain (macOS Keychain / Windows Credential Manager) via `tauri-plugin-store`
- API key is never written to any log file or exposed in the UI after entry

---

**US-052 · Add Anthropic as AI Provider**

> As an end user, I want to add my Anthropic API key so that I can use Claude's vision capabilities.

**Acceptance Criteria:**
- "Anthropic" option in Settings → AI Providers
- API key field (masked input)
- Model dropdown: `claude-haiku-4-5` (default, cheapest), `claude-sonnet-4-6`
- "Test Connection" verifies the key
- Stored in OS keychain

---

**US-053 · Add Custom OpenAI-Compatible Endpoint**

> As a power user, I want to configure a custom OpenAI-compatible endpoint so that I can use LM Studio, Groq, Together AI, or any self-hosted model.

**Acceptance Criteria:**
- "Custom (OpenAI-Compatible)" option in Settings → AI Providers
- Fields: Base URL, API Key (optional), Model name (free text)
- Tooltip explains: "Any endpoint implementing the OpenAI /v1/chat/completions format works here"
- "Test Connection" sends a test request and shows response latency
- Multiple custom endpoints can be configured and named

---

**US-054 · Switch Active AI Provider**

> As an end user, I want to switch which AI provider is used for future analyses so that I can change my setup at any time.

**Acceptance Criteria:**
- Settings → AI Providers shows all configured providers
- A radio button or toggle designates the "active" provider
- Changing the active provider only affects future analysis jobs; already-analyzed screenshots are not retroactively changed
- If the active provider becomes unreachable, a system notification alerts the user

---

#### Epic: Watch Folders

---

**US-060 · Add a Watch Folder**

> As an end user, I want to add any folder on my machine to the watch list so that screenshots saved in non-default locations are also detected.

**Acceptance Criteria:**
- Settings → Watch Folders shows a list of currently watched directories
- "Add Folder" button opens the OS native folder picker
- Selected folder is added to the list and watching begins immediately
- Existing images in the folder are optionally scanned on first add (opt-in dialog: "Scan existing files?")
- Maximum of 20 watch folders

---

**US-061 · Remove a Watch Folder**

> As an end user, I want to remove a watch folder so that screenshots from that location are no longer tracked.

**Acceptance Criteria:**
- Each watch folder entry has a "Remove" button
- Removing stops file watching for that path immediately
- Existing screenshots already in the library from that folder are NOT deleted from the DB
- Confirmation dialog: "Stop watching [path]? Existing screenshots will remain in your library."

---

#### Epic: Rules

---

**US-070 · Create a Rule**

> As a power user, I want to create an automation rule so that screenshots matching specific criteria are automatically organized without manual intervention.

**Acceptance Criteria:**
- Settings → Rules → "New Rule" opens a rule builder UI
- Rule builder has condition row(s) with: field selector, operator selector, value input
- Rule builder has action row(s) with: action type selector, value input (e.g. folder path)
- "When to run" selector: "On detection", "On analysis complete", "Manually"
- Rule can be named
- A "Test Rule" button shows which existing screenshots would match (dry run)
- Rule is saved and listed in the Rules settings page
- Rules are applied in list order (drag to reorder)

---

**US-071 · Toggle a Rule On/Off**

> As a power user, I want to temporarily disable a rule without deleting it so that I can pause automation during edge cases.

**Acceptance Criteria:**
- Each rule in the list has an enabled/disabled toggle
- Disabled rules are grayed out but remain configured
- Disabling a rule takes effect immediately for future screenshots

---

### Role: Power User

---

**US-080 · Override AI System Prompt**

> As a power user, I want to customize the system prompt sent to the AI so that I can tune the categorization to my personal use cases.

**Acceptance Criteria:**
- Settings → AI Providers → [Provider] → "Advanced" section exposes the system prompt
- System prompt is an editable text area with the default prompt pre-filled
- A "Reset to default" button restores the original prompt
- A live character count is shown (max 2000 characters)
- Changes apply to all future analysis jobs

---

**US-081 · View Raw Database Path**

> As a power user, I want to know the path to the SQLite database so that I can inspect or back it up externally.

**Acceptance Criteria:**
- Settings → Advanced shows the full path to the database file
- A "Copy path" button copies it to clipboard
- A "Open in Finder/Explorer" button reveals the file

---

---

## 6. Screens & Navigation

### 6.1 Navigation Architecture

```
App Window
│
├── [System Tray Icon] ← always running even if window is closed
│       ├── Open SnapSort
│       ├── Screenshot count today
│       └── Quit
│
└── [Main Window]
        │
        ├── [Onboarding Modal] ← first launch only, overlays everything
        │
        ├── [Sidebar] ← always visible
        │       ├── Search Bar (top)
        │       ├── Smart Folders section
        │       ├── Categories section
        │       ├── Tags section
        │       ├── Collections section
        │       └── Settings button (bottom)
        │
        ├── [Main Content Area] ← changes based on selection
        │       ├── Library Gallery View (default)
        │       ├── Collection View
        │       └── Settings Panel (replaces gallery)
        │
        └── [Detail Panel] ← slides in from right on thumbnail click
                └── (overlay or split depending on window width)
```

---

### 6.2 Screen Inventory

| Screen ID | Screen Name | Trigger |
|---|---|---|
| S-01 | Onboarding (Welcome) | First launch |
| S-02 | Onboarding (Watch Folders) | Onboarding step 2 |
| S-03 | Onboarding (AI Setup — Optional) | Onboarding step 3 |
| S-04 | Onboarding (Complete) | Onboarding step 4 |
| S-05 | Library Gallery | Default / sidebar nav |
| S-05b | Library Gallery — Batch Selection Mode | Click checkbox on thumbnail |
| S-06 | Screenshot Detail Panel | Click thumbnail (no selection mode) |
| S-07 | Collection View | Click collection in sidebar |
| S-08 | Settings — General | Settings button |
| S-09 | Settings — AI Providers | Settings → AI Providers tab |
| S-10 | Settings — Watch Folders | Settings → Watch Folders tab |
| S-11 | Settings — Rules | Settings → Rules tab |
| S-12 | Settings — Advanced | Settings → Advanced tab |
| S-13 | New Rule Builder | Settings → Rules → New Rule |
| S-14 | New Collection Modal | Sidebar → + Collection |
| S-15 | Delete Confirmation Dialog | Delete action (single or batch) |
| S-16 | Batch Categorize Popover | Batch Action Bar → Categorize |
| S-17 | Batch Rename Modal | Batch Action Bar → Rename |
| S-18 | Batch Tag Popover | Batch Action Bar → Tag |
| S-19 | Batch Move Confirmation | Batch Action Bar → Move to Folder |
| S-20 | Batch Add to Collection Popover | Batch Action Bar → Add to Collection |
| S-21 | Update Available Dialog | Auto-updater trigger |

---

### 6.3 Screen Wireframes (Detailed)

---

#### S-01 · Onboarding — Welcome

```
┌───────────────────────────────────────────────┐
│                                               │
│          [SnapSort Logo + Wordmark]           │
│                                               │
│    📸                                         │
│    Your screenshots, finally organized.       │
│                                               │
│    SnapSort watches your screenshot folders,  │
│    uses AI to categorize them, and makes      │
│    every screenshot instantly searchable —    │
│    all on your own machine.                   │
│                                               │
│    ● ○ ○ ○    [Get Started →]                │
│                                               │
└───────────────────────────────────────────────┘
```

---

#### S-02 · Onboarding — Watch Folders

```
┌───────────────────────────────────────────────┐
│                                               │
│  📁  Where do your screenshots live?          │
│                                               │
│  We found these folders automatically:        │
│                                               │
│  ☑  ~/Desktop                     (24 files) │
│  ☑  ~/Pictures/Screenshots        (312 files)│
│                                               │
│  [+ Add another folder]                       │
│                                               │
│  ● ● ○ ○    [← Back]  [Continue →]           │
│                                               │
└───────────────────────────────────────────────┘
```

---

#### S-03 · Onboarding — AI Setup

```
┌───────────────────────────────────────────────┐
│                                               │
│  🤖  Set up AI analysis                       │
│                                               │
│  ✅ Ollama detected on your machine           │
│     (fastest, fully private, free)            │
│                                               │
│  Pick a model:                                │
│  ◉ moondream  — fastest, works on any Mac    │
│  ○ llava:7b   — more accurate, needs 8GB RAM │
│  ○ llava:13b  — best quality, needs 16GB RAM │
│                                               │
│  ── or use a cloud API ──                    │
│                                               │
│  [OpenAI]  [Anthropic]  [Custom]             │
│                                               │
│  ● ● ● ○    [← Back]  [Continue →]           │
│                                               │
└───────────────────────────────────────────────┘
```

---

#### S-04 · Onboarding — Complete

```
┌───────────────────────────────────────────────┐
│                                               │
│  🎉  You're all set!                          │
│                                               │
│  SnapSort is running in the background.       │
│  Take a screenshot now to see it in action.  │
│                                               │
│  [Show me the library]                        │
│                                               │
│  ● ● ● ●                                     │
│                                               │
└───────────────────────────────────────────────┘
```

---

#### S-05 · Library Gallery View

```
┌──────────────────────────────────────────────────────────────────────┐
│  [SnapSort]                                          [–][□][✕]        │
├─────────────────────────────────────────────────────────────────────  │
│ ┌─────────────┐ ┌─────────────────────────────────────────────────┐  │
│ │  SIDEBAR    │ │  MAIN CONTENT AREA                              │  │
│ │             │ │                                                  │  │
│ │ [🔍 Search  ]│ │  ┌─ Filter chips ──────────────────────────┐  │  │
│ │             │ │  │  Code ×   #error ×   Last 7 days ×  Clear│  │  │
│ │ SMART       │ │  └───────────────────────────────────────────┘  │  │
│ │ ──────────  │ │                                                  │  │
│ │ 📷 All(336) │ │  342 screenshots  [⊞ S][⊞ M][⊞ L]  [↕ Date]  │  │
│ │ 📅 Today(4) │ │                                                  │  │
│ │ ⭐ Favorites │ │  ┌───────┐ ┌───────┐ ┌───────┐ ┌───────┐      │  │
│ │ ⚠ Unanalyz │ │  │☐ img  │ │☐ img  │ │☐ img  │ │☐ img  │      │  │
│ │             │ │  │       │ │       │ │  ⌛   │ │       │      │  │
│ │ CATEGORIES  │ │  │[code] │ │[design│ │[analy-│ │[web]  │      │  │
│ │ ──────────  │ │  │ 2h ago│ │ 5h ago│ │ zing] │ │ 1d ago│      │  │
│ │ 💻 Code(89) │ │  └───────┘ └───────┘ └───────┘ └───────┘      │  │
│ │ 🎨 Design(44│ │                                                  │  │
│ │ 🌐 Web (102)│ │  ┌───────┐ ┌───────┐ ┌───────┐ ┌───────┐      │  │
│ │ 💬 Comm (31)│ │  │☐ img  │ │☐ img  │ │☐ img  │ │☐ img  │      │  │
│ │ 💰 Finance  │ │  └───────┘ └───────┘ └───────┘ └───────┘      │  │
│ │ ...         │ │                                                  │  │
│ │             │ │  ┌───────┐ ┌───────┐ ...                       │  │
│ │ TAGS        │ │  │☐ img  │ │☐ img  │                           │  │
│ │ ──────────  │ │  └───────┘ └───────┘                           │  │
│ │ #error (14) │ │                                                  │  │
│ │ #figma (22) │ │                                                  │  │
│ │ #receipt(8) │ │                                                  │  │
│ │ ...         │ │                                                  │  │
│ │             │ │                                                  │  │
│ │ COLLECTIONS │ │                                                  │  │
│ │ ──────────  │ │                                                  │  │
│ │ Project X   │ │                                                  │  │
│ │ Bug Reports │ │                                                  │  │
│ │ [+ New]     │ │                                                  │  │
│ │             │ │                                                  │  │
│ │ [⚙ Settings]│ │                                                  │  │
│ └─────────────┘ └─────────────────────────────────────────────────┘  │
│ ──────────────────────────────────────────────────────────────────── │
│  Analyzing 2 screenshots... ⣾                          v0.4.0        │
└──────────────────────────────────────────────────────────────────────┘

Note: ☐ icons on thumbnails represent the selection checkboxes
that appear on hover. Clicking any checkbox activates selection mode
and reveals the Batch Action Bar (see S-05b below).
```

---

#### S-05b · Gallery View — Batch Selection Mode Active

```
┌──────────────────────────────────────────────────────────────────────┐
│  [SnapSort]                                          [–][□][✕]        │
├─────────────────────────────────────────────────────────────────────  │
│ ┌─────────────┐ ┌─────────────────────────────────────────────────┐  │
│ │  SIDEBAR    │ │  MAIN CONTENT AREA                              │  │
│ │  (same)     │ │                                                  │  │
│ │             │ │  ┌─ Selection header ──────────────────────┐   │  │
│ │             │ │  │  ☑ 47 selected  [Select All] [Deselect] │   │  │
│ │             │ │  └─────────────────────────────────────────┘   │  │
│ │             │ │                                                  │  │
│ │             │ │  ┌───────┐ ┌───────┐ ┌───────┐ ┌───────┐      │  │
│ │             │ │  │☑ img  │ │☑ img  │ │☐ img  │ │☑ img  │      │  │
│ │             │ │  │ ████  │ │ ████  │ │       │ │ ████  │      │  │
│ │             │ │  │[code] │ │[design│ │[web]  │ │[code] │      │  │
│ │             │ │  └───────┘ └───────┘ └───────┘ └───────┘      │  │
│ │             │ │   ↑ selected (blue  ↑ selected   ↑ not sel    │  │
│ │             │ │     tint overlay)                              │  │
│ │             │ │                                                  │  │
│ │             │ │  ┌───────┐ ┌───────┐ ┌───────┐ ┌───────┐      │  │
│ │             │ │  │☑ img  │ │☑ img  │ │☑ img  │ │☐ img  │      │  │
│ │             │ │  └───────┘ └───────┘ └───────┘ └───────┘      │  │
│ │             │ │                                                  │  │
│ └─────────────┘ └─────────────────────────────────────────────────┘  │
│ ══════════════════════════════════════════════════════════════════════│
│  ✕  47 selected                                                       │
│  [🗑 Delete] [📁 Categorize] [✏ Rename] [🏷 Tag] [📦 Archive] [⋯]  │
│                                             ↑ overflow: Move, Collect │
└──────────────────────────────────────────────────────────────────────┘
```

---

#### S-06 · Screenshot Detail Panel

```
┌──────────────────────────────────────────────────────────────────────┐
│  [Gallery] ›  Code  ›  Screenshot detail              [–][□][✕]      │
├────────────────────────────────────────────────────────────────────── │
│ ┌─────────────┐ ┌──────────────────┐ ┌──────────────────────────┐   │
│ │  SIDEBAR    │ │  GALLERY (dimmed)│ │  DETAIL PANEL            │   │
│ │  (same as   │ │                  │ │                          │   │
│ │   S-05)     │ │   [◀] thumbnail  │ │  ┌──────────────────┐   │   │
│ │             │ │   grid, blurred  │ │  │                  │   │   │
│ │             │ │   in background  │ │  │  [Full image     │   │   │
│ │             │ │                  │ │  │   pan + zoom]    │   │   │
│ │             │ │   [▶]            │ │  │                  │   │   │
│ │             │ │                  │ │  └──────────────────┘   │   │
│ │             │ │                  │ │                          │   │
│ │             │ │                  │ │  2026-04-12 · 3:22 PM   │   │
│ │             │ │                  │ │  2560×1440 · 1.4 MB     │   │
│ │             │ │                  │ │                          │   │
│ │             │ │                  │ │  [💻 Code] [⭐] [⋯]     │   │
│ │             │ │                  │ │                          │   │
│ │             │ │                  │ │  VS Code                 │   │
│ │             │ │                  │ │  "NullPointerException   │   │
│ │             │ │                  │ │  in Java Spring Boot     │   │
│ │             │ │                  │ │  app, stack trace shown" │   │
│ │             │ │                  │ │                          │   │
│ │             │ │                  │ │  Tags:                   │   │
│ │             │ │                  │ │  [🤖#error] [🤖#java]   │   │
│ │             │ │                  │ │  [✏️#work] [+ Add]       │   │
│ │             │ │                  │ │                          │   │
│ │             │ │                  │ │  ▼ OCR Text (847 chars)  │   │
│ │             │ │                  │ │  ┌──────────────────┐   │   │
│ │             │ │                  │ │  │ NullPointerExcep │   │   │
│ │             │ │                  │ │  │ tion: Cannot inv │   │   │
│ │             │ │                  │ │  │ oke method...    │   │   │
│ │             │ │                  │ │  └──────────────────┘   │   │
│ │             │ │                  │ │  [Copy Text]             │   │
│ │             │ │                  │ │                          │   │
│ │             │ │                  │ │  Collections: [Bug Rpts] │   │
│ │             │ │                  │ │  [Add to collection +]   │   │
│ │             │ │                  │ │                          │   │
│ │             │ │                  │ │  [Show in Finder]        │   │
│ │             │ │                  │ │  [Re-analyze] [Archive]  │   │
│ │             │ │                  │ │  [Delete]                │   │
│ └─────────────┘ └──────────────────┘ └──────────────────────────┘   │
└──────────────────────────────────────────────────────────────────────┘
```

---

#### S-09 · Settings — AI Providers

```
┌──────────────────────────────────────────────────────────────────────┐
│  Settings                                             [–][□][✕]      │
├─────────────────────────────────────────────────────────────────────  │
│  [General] [AI Providers ●] [Watch Folders] [Rules] [Advanced]       │
├─────────────────────────────────────────────────────────────────────  │
│                                                                       │
│  AI Providers                                                         │
│  Configure one or more providers. The active provider is used         │
│  for all new analysis jobs.                                           │
│                                                                       │
│  ┌──────────────────────────────────────────────────────────────┐    │
│  │ ◉  Ollama (Local)                           ACTIVE  [Edit]  │    │
│  │    Model: moondream · http://localhost:11434                  │    │
│  │    ✅ Connected · ~1.2s avg analysis time                    │    │
│  └──────────────────────────────────────────────────────────────┘    │
│                                                                       │
│  ┌──────────────────────────────────────────────────────────────┐    │
│  │ ○  OpenAI                                          [Edit]   │    │
│  │    Model: gpt-4o-mini · Key: sk-••••••••••••••3f2a          │    │
│  │    ✅ Connected                                               │    │
│  └──────────────────────────────────────────────────────────────┘    │
│                                                                       │
│  ┌──────────────────────────────────────────────────────────────┐    │
│  │ ○  Anthropic                                       [Edit]   │    │
│  │    Model: claude-haiku-4-5 · Key: sk-ant-••••••••2b9c       │    │
│  │    ✅ Connected                                               │    │
│  └──────────────────────────────────────────────────────────────┘    │
│                                                                       │
│  [+ Add Provider ▼]                                                   │
│    ├── Ollama (Local)                                                 │
│    ├── OpenAI                                                         │
│    ├── Anthropic                                                      │
│    └── Custom OpenAI-Compatible                                       │
│                                                                       │
└──────────────────────────────────────────────────────────────────────┘
```

---

#### S-11 · Settings — Rules

```
┌──────────────────────────────────────────────────────────────────────┐
│  Settings                                             [–][□][✕]      │
├─────────────────────────────────────────────────────────────────────  │
│  [General] [AI Providers] [Watch Folders] [Rules ●] [Advanced]       │
├─────────────────────────────────────────────────────────────────────  │
│                                                                       │
│  Rules                            [+ New Rule]                        │
│  Rules run automatically when screenshots are analyzed.               │
│                                                                       │
│  ┌──────────────────────────────────────────────────────────────┐    │
│  │ ☰  💰 Receipt Organizer                       ● ON  [Edit] │    │
│  │    IF category = "finance"                                   │    │
│  │    THEN move to ~/Documents/Receipts/                        │    │
│  │         add tag #receipt                                     │    │
│  └──────────────────────────────────────────────────────────────┘    │
│                                                                       │
│  ┌──────────────────────────────────────────────────────────────┐    │
│  │ ☰  💻 Code Error Highlighter                  ● ON  [Edit] │    │
│  │    IF ocr_text contains "Error" OR "Exception"               │    │
│  │    THEN add tag #error                                       │    │
│  │         add to collection "Bug Reports"                      │    │
│  └──────────────────────────────────────────────────────────────┘    │
│                                                                       │
│  ┌──────────────────────────────────────────────────────────────┐    │
│  │ ☰  🎨 Design Review Folder                    ○ OFF [Edit] │    │
│  │    IF app_detected = "Figma"                                 │    │
│  │    THEN add to collection "Design Reviews"                   │    │
│  └──────────────────────────────────────────────────────────────┘    │
│                                                                       │
└──────────────────────────────────────────────────────────────────────┘
```

---

#### S-13 · New Rule Builder

```
┌──────────────────────────────────────────────────────────────────────┐
│  New Rule                                                    [✕]      │
├─────────────────────────────────────────────────────────────────────  │
│                                                                       │
│  Name: [Receipt Organizer                              ]              │
│                                                                       │
│  Run when:  ◉ On analysis complete  ○ On detection  ○ Manually       │
│                                                                       │
│  ── CONDITIONS ────────────────────────────────────────────────────  │
│                                                                       │
│  Match:  ◉ ALL of the following  ○ ANY of the following              │
│                                                                       │
│  [Category     ▼]  [equals         ▼]  [Finance       ▼]  [✕]       │
│  [+ Add condition]                                                    │
│                                                                       │
│  ── ACTIONS ───────────────────────────────────────────────────────  │
│                                                                       │
│  [Move file to folder  ▼]  [~/Documents/Receipts/      ] [📁] [✕]   │
│  [Add tag              ▼]  [#receipt                    ]      [✕]   │
│  [+ Add action]                                                       │
│                                                                       │
│  ── PREVIEW ───────────────────────────────────────────────────────  │
│  This rule would match 8 existing screenshots.   [Test Rule]         │
│                                                                       │
│                          [Cancel]  [Save Rule]                        │
└──────────────────────────────────────────────────────────────────────┘
```

---

---

#### S-16 · Batch Categorize Popover

```
┌────────────────────────────────────────┐
│  Categorize 47 screenshots             │
│                                        │
│  ○  💻  Code & Terminal                │
│  ○  🎨  Design & UI                    │
│  ○  📄  Documents & Text               │
│  ○  🌐  Web & Browser                  │
│  ○  💬  Communication                  │
│  ○  🎬  Media & Entertainment          │
│  ○  💰  Finance & Commerce             │
│  ○  📌  Reference & Notes              │
│  ○  🖥  System & OS                   │
│  ○  📦  Other                          │
│                                        │
│  ⚠ 12 of these already have a category│
│    This will overwrite them.           │
│                   [Cancel] [Apply →]   │
└────────────────────────────────────────┘
```

---

#### S-17 · Batch Rename Modal

```
┌──────────────────────────────────────────────────┐
│  Rename 47 screenshots                     [✕]   │
│                                                  │
│  Pattern:  [{date}_{category}_{index}         ]  │
│                                                  │
│  ▼ Available tokens                              │
│    {date} · {time} · {datetime} · {index}        │
│    {category} · {original} · {ext}               │
│                                                  │
│  Preview (first 5 files):                        │
│  ┌────────────────────────────────────────────┐  │
│  │ Original                → New Name         │  │
│  │ Screenshot 2026-04-12   → 2026-04-12_      │  │
│  │  at 3.22.01 PM.png        code_001.png     │  │
│  │ Screenshot 2026-04-11   → 2026-04-11_      │  │
│  │  at 9.14.42 AM.png        design_002.png   │  │
│  │ Screenshot 2026-04-10   → 2026-04-10_      │  │
│  │  at 11.02.17 PM.png       web_003.png      │  │
│  │  ... and 44 more files                     │  │
│  └────────────────────────────────────────────┘  │
│                                                  │
│                      [Cancel]  [Apply Rename →]  │
└──────────────────────────────────────────────────┘
```

---

#### S-18 · Batch Tag Popover

```
┌──────────────────────────────────────────┐
│  Tag 47 screenshots                      │
│                                          │
│  [Add Tags]  [Remove Tags]               │
│  ──────────────────────────              │
│  Add tags:                               │
│  [🔍 Search or create tag...   ]         │
│                                          │
│  Recently used:                          │
│  [#work] [#figma] [#error] [#receipt]    │
│                                          │
│  Selected to add:                        │
│  [#project-x ×] [#review ×]             │
│                                          │
│                   [Cancel]  [Add Tags →] │
└──────────────────────────────────────────┘
```

### 6.4 Navigation Flow Diagram

```
App Launch
    │
    ├─── First launch? ──YES──► Onboarding Flow (S-01→S-02→S-03→S-04)
    │                                     │
    │                       AI step skippable at any point
    │                                     │
    │                                     ▼
    └─── No ───────────────────► Library Gallery (S-05)  ◄────────────┐
                                         │                             │
                    ┌────────────────────┼───────────────────────┐    │
                    ▼                    ▼                         ▼    │
            Click thumbnail      Click collection          Click category
            (no selection mode)        │                         │    │
                    │                  ▼                         ▼    │
                    ▼           Collection View (S-07)  Filtered Gallery
            Detail Panel (S-06)        │                              │
                    │                  └──────────────────────────────┘
                    └──────────────────────────────[ESC / ✕ to close]

Library Gallery (S-05)
    │
    ├── Click checkbox on thumbnail ──► Batch Selection Mode (S-05b)
    │       │
    │       ├── Batch Action Bar → [Delete]       ──► S-15 Confirm Delete
    │       ├── Batch Action Bar → [Categorize]   ──► S-16 Category Popover
    │       ├── Batch Action Bar → [Rename]       ──► S-17 Rename Modal
    │       ├── Batch Action Bar → [Tag]          ──► S-18 Tag Popover
    │       ├── Batch Action Bar → [Archive]      ──► (instant, with Undo toast)
    │       ├── Batch Action Bar → [⋯ Move]       ──► OS folder picker → S-19 Confirm
    │       ├── Batch Action Bar → [⋯ Collect]    ──► S-20 Collection Popover
    │       └── [✕] or Escape                     ──► exit selection mode → S-05
    │
    └── [⚙ Settings] ──► Settings — General (S-08)
                               │
                    ┌──────────┼──────────────────────────┐
                    ▼          ▼            ▼              ▼
             AI Providers  Watch Folders  Rules (S-11)  Advanced
               (S-09)        (S-10)          │           (S-12)
                                             │
                                    [+ New Rule] ──► Rule Builder (S-13)
```

---

### 6.5 Key Interaction Patterns

| Pattern | Implementation |
|---|---|
| **Thumbnail hover (no selection mode)** | Show checkbox in top-left, show star icon in top-right, show summary tooltip |
| **Thumbnail hover (selection mode)** | Checkbox always visible; clicking toggles selection, does NOT open detail panel |
| **Thumbnail right-click** | Context menu: Open, Copy path, Add to collection, Add tag, Archive, Delete, Select |
| **Detail panel open** | Slide in from right (300ms ease-out). Gallery dims but stays visible. Only in non-selection mode. |
| **Detail panel navigation** | Left/right arrow keys cycle through current filtered set |
| **Batch Action Bar** | Slides up from bottom when ≥1 screenshot selected. Stays pinned above status bar. |
| **Batch Action Bar overflow** | "⋯" button reveals Move to Folder and Add to Collection when window is too narrow |
| **Search** | Instant results with 300ms debounce. No page reload. Works without AI (OCR-only). |
| **Drag thumbnail to sidebar collection** | Adds screenshot to that collection (single-item) |
| **New screenshot detected** | Thumbnail animates in at the top of gallery. Status bar updates. |
| **AI analysis complete** | Loading overlay on thumbnail fades out. Category badge fades in. (No-op if AI not configured.) |
| **Toast notifications** | Bottom-right corner. Auto-dismiss after 4s. Stackable. Undo toast stays for 5s. |
| **Keyboard shortcuts** | `⌘K`/`Ctrl+K`: search. `Esc`: close panel or exit selection. `←/→`: navigate in detail. `Space`: full preview. `⌘A`/`Ctrl+A`: select all. `⌘,`/`Ctrl+,`: Settings. `Del`/`Backspace`: delete selected (with confirm). |

---

*End of Document — SnapSort PRD v1.1.0*
