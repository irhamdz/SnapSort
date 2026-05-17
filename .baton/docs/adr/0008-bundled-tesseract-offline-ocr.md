# ADR-0008: Bundle Tesseract for offline OCR

- **Status:** Accepted
- **Date:** 2026-05-16
- **Derived from:** PRD §1.5, §2.1, §2.4, §4.1 (F-11)

## Context

OCR is the backbone of search and runs on **every** screenshot at ingestion,
regardless of whether AI is configured (see
[ADR-0011](0011-ingestion-pipeline-state-machine.md)). Because search must be
fully offline and private, OCR cannot depend on a cloud API. The engine must be
embeddable in the binary, MIT-compatible for an open-source MIT project, and
support a wide range of languages.

## Decision

**Bundle Tesseract** as the OCR engine, invoked from the Rust backend. Its
output populates the `ocr_text` field, which is indexed by FTS5
(see [ADR-0004](0004-sqlite-fts5-full-text-search.md)).

## Consequences

**Positive**

- Fully offline OCR, satisfying privacy and offline-first principles
  (see [ADR-0015](0015-privacy-by-default.md)).
- License-compatible with the project's MIT licensing.
- Decouples search quality from AI configuration: search works in the
  `ready (no AI)` state.

**Negative / Trade-offs**

- Bundling Tesseract and language data increases binary/installer size.
- OCR accuracy on stylized UI, low-contrast, or non-Latin text is variable and
  generally below cloud OCR services.
- OCR is CPU-bound; it must run async off the watcher thread and be queued to
  avoid starving the UI on bursts of new screenshots.
