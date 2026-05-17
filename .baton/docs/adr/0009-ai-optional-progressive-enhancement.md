# ADR-0009: AI as an optional progressive enhancement (offline-first)

- **Status:** Accepted
- **Date:** 2026-05-16
- **Derived from:** PRD §1.1, §1.3, §2.4, §4 (AI Required column)

## Context

v1.1 of the PRD elevated "AI is optional" from a feature to a core design
principle. Many target users (privacy-first, casual, no-GPU) will never
configure a provider, yet must get a fully functional product: detection,
OCR, search, gallery, detail view, and **all** batch operations. AI must add
metadata on top, never gate baseline functionality.

## Decision

Treat AI as a **progressive enhancement**. Every feature except explicitly
AI-marked ones works with no provider configured. A screenshot with OCR but no
AI enters a first-class `ready` state — visible, searchable, and eligible for
every batch operation. AI enrichment only adds `category` (AI-sourced), `tags`,
`summary`, and `app_detected`.

## Consequences

**Positive**

- Removes adoption friction and satisfies the privacy-first persona; the app is
  useful within seconds of install with zero setup.
- Decouples the core roadmap from AI provider availability and reliability.

**Negative / Trade-offs**

- The data model must tolerate null AI fields everywhere; UI must degrade
  gracefully ("Uncategorized", no summary tooltip) rather than show empty/broken
  states.
- Two effective product modes (AI / no-AI) must both be designed, tested, and
  kept working — extra QA surface.
- Category may be set by AI *or* by the user, requiring explicit provenance
  tracking (see [ADR-0012](0012-fixed-category-taxonomy-provenance.md)).
