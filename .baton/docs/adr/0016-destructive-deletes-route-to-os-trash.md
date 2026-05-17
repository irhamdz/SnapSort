# ADR-0016: Destructive deletes route to OS Trash

- **Status:** Accepted
- **Date:** 2026-05-16
- **Derived from:** PRD §4.2 (F-37), US-026, US-091

## Context

Delete — especially **batch** delete of dozens or hundreds of screenshots — is
irreversible from inside the app and operates on real user files. An accidental
mass delete that bypassed the OS Trash would be unrecoverable data loss. The
product needs a safety net without building an in-app undo/version store.

## Decision

Deleting a screenshot removes its DB record and sends the underlying file to
the **OS Trash** (never a hard `unlink` that bypasses Trash). There is no
in-app undo for delete; recoverability is delegated to the OS Trash for its
retention period. Delete confirmations require an explicit click on a labeled
destructive button (e.g. "Delete 47") — **not** the Enter key — to prevent
accidental mass deletion.

## Consequences

**Positive**

- Mistaken deletes — including large batch deletes — remain recoverable via the
  OS Trash; no bespoke recycle bin needed.
- Confirmation friction is deliberately calibrated to batch risk.

**Negative / Trade-offs**

- Recoverability depends on OS Trash configuration (e.g. "delete immediately"
  settings, network volumes that skip Trash) — outside the app's control.
- DB record removal is immediate while file-to-Trash is async; a transient
  divergence between library state and disk is possible and must be tolerated.
- "Move to Trash" is not "securely erase"; users needing guaranteed destruction
  must act outside SnapSort.
