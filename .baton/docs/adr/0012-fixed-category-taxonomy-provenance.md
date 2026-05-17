# ADR-0012: Fixed system Category taxonomy with provenance

- **Status:** Accepted
- **Date:** 2026-05-16
- **Derived from:** PRD §2.1, §2.3, §4.2 (F-38), US-092

## Context

Categories are used for filtering, smart folders, rename tokens, and AI
classification prompts. They can be assigned by AI **or** by the user (single
edit or Batch Categorize), and exactly one applies per screenshot. Free-form
categories would fragment the taxonomy, make AI prompting unreliable, and break
stable sidebar/filter UX. The system also needs to know whether a category came
from AI or a human (e.g. to avoid an AI re-analysis clobbering a user choice).

## Decision

Use a **closed, system-defined taxonomy of 10 categories** (`code`, `design`,
`document`, `web`, `communication`, `media`, `finance`, `reference`, `system`,
`other`), mutually exclusive per screenshot. Record provenance in a
**`category_source`** field that is always `"ai"` or `"user"` — never omitted.

## Consequences

**Positive**

- Stable, predictable filters/smart folders and a constrained AI prompt target
  (one of ten labels).
- `category_source` lets the system protect user intent — e.g. AND warn before
  batch-overwriting, and prevent AI from silently replacing a user category.

**Negative / Trade-offs**

- No user-defined categories; niche needs must be served by Tags or Collections
  instead. Extending the taxonomy is a coordinated change (enum, prompt, UI,
  migration).
- `category_source` is an invariant every write path must uphold; a missing or
  wrong value corrupts overwrite-protection logic.
- `other` will absorb ambiguous content and may grow large.
