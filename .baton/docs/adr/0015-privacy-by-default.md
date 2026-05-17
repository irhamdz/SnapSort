# ADR-0015: Privacy by default — no telemetry, accounts, or cloud sync

- **Status:** Accepted
- **Date:** 2026-05-16
- **Derived from:** PRD §1.1, §1.3

## Context

Screenshots routinely contain highly sensitive content (credentials, financial
data, private messages). SnapSort's positioning and a key target persona
(privacy-first) depend on a credible guarantee that data never leaves the
machine unless the user explicitly opts into a cloud AI provider.

## Decision

**Privacy by default:** no telemetry, no analytics, no user accounts, no cloud
sync. All data — database, thumbnails, settings — stays in the local OS
app-data directory. The only outbound network traffic is to a user-configured
AI provider, and only when one is set (see
[ADR-0010](0010-pluggable-aiprovider-trait-byok.md)); the default provider
(Ollama) is local. Search and OCR are entirely offline
(see [ADR-0004](0004-sqlite-fts5-full-text-search.md),
[ADR-0008](0008-bundled-tesseract-offline-ocr.md)).

## Consequences

**Positive**

- Strong, easily explained privacy guarantee; supports the privacy-first
  persona and offline use.
- Reduces legal/compliance surface (no PII collection, no data processing
  agreements).

**Negative / Trade-offs**

- No usage telemetry means product decisions rely on qualitative feedback, not
  analytics; crash/error diagnostics must be local-only.
- No cloud sync: multi-device users have no built-in library portability beyond
  manually copying the DB file.
- Any outbound call is a privacy-sensitive event — cloud AI use must be
  explicit, opt-in, and visible, never silent.
