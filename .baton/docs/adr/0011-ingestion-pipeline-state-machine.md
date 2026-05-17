# ADR-0011: Ingestion pipeline modeled as an explicit state machine

- **Status:** Accepted
- **Date:** 2026-05-16
- **Derived from:** PRD §2.4, §2.5, US-043

## Context

A screenshot moves through several asynchronous stages — detection, thumbnail
generation, OCR, and (optionally) AI analysis — that can fail or stall
independently. The UI must reflect progress ("Analyzing N screenshots…",
per-card spinners, the "Unanalyzed" smart folder), failed AI must be retryable,
and the AI stage must be skippable entirely when no provider is configured
(see [ADR-0009](0009-ai-optional-progressive-enhancement.md)).

## Decision

Model ingestion as an explicit, persisted **state machine** on the
`screenshots.status` column:

`detected → queued → ocr_complete → ready` (no AI) **or**
`→ analyzing → enriched`, with `analyzing → partial` on AI failure
(retry-eligible). `archived` and `deleted` are terminal/auxiliary states.

## Consequences

**Positive**

- Status is durable, so progress and retry survive restarts and crashes.
- The branch after `ocr_complete` cleanly encodes the optional-AI principle;
  `ready` is a fully usable terminal state.
- `partial` makes failed AI a recoverable, queryable condition rather than a
  lost write.

**Negative / Trade-offs**

- Every stage transition is a DB write contending for the single SQLite writer
  (see [ADR-0003](0003-sqlite-rusqlite-embedded-store.md)); transitions must be
  short transactions.
- The status lifecycle is a contract shared across watcher, OCR, AI, and UI —
  changing it touches many layers.
- The state set must be kept exhaustive; unhandled transitions risk stuck
  screenshots.
