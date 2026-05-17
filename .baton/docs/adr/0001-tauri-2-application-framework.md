# ADR-0001: Tauri 2 (Rust + WebView) as the application framework

- **Status:** Accepted
- **Date:** 2026-05-16
- **Derived from:** PRD §1.4, §1.5

## Context

SnapSort is a cross-platform desktop application (macOS 12+, Windows 10 1809+)
that must run a background file watcher, an OCR engine, an embedded database,
and a rich gallery UI. It is open-source and distributed as a downloadable
binary, so install size and resource footprint directly affect adoption. The
realistic options were Electron (Chromium + Node), a fully native toolkit
(Swift/WinUI, two codebases), or Tauri 2 (system WebView + Rust core).

## Decision

Use **Tauri 2** as the application shell: a Rust backend exposing typed
commands over Tauri IPC, paired with a web-technology frontend rendered in the
OS-native WebView.

## Consequences

**Positive**

- Small binary (~10 MB target) and low memory vs. an Electron Chromium bundle.
- One UI codebase across macOS and Windows; Rust gives native-speed file
  watching, OCR orchestration, and SQLite access.
- Strong fit for privacy goals — no bundled browser runtime phoning home.

**Negative / Trade-offs**

- Rendering depends on the host OS WebView (WebKit on macOS, WebView2 on
  Windows); cross-WebView rendering and CSS quirks must be tested on both.
- Smaller ecosystem and contributor pool than Electron; some native
  integrations require writing Rust plugins.
- The Rust/JS IPC seam is a hard boundary that every feature must cross
  (see [ADR-0017](0017-typed-ipc-contract-single-boundary.md)).
