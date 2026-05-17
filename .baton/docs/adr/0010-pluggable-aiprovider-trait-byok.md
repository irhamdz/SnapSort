# ADR-0010: Pluggable `AIProvider` trait with Bring-Your-Own-Key

- **Status:** Accepted
- **Date:** 2026-05-16
- **Derived from:** PRD §1.3, §3.2, §4.2 (F-07/13/14/15), US-050..054

## Context

SnapSort explicitly rejects AI vendor lock-in. Users must be able to choose
local Ollama, OpenAI, Anthropic, or any OpenAI-compatible endpoint (LM Studio,
Groq, Together AI, self-hosted), and switch the active provider at any time.
Open-source contributors should be able to add a new provider without touching
core ingestion logic.

## Decision

Define a single **`AIProvider` trait** in `src-tauri/src/ai/`. Each integration
(Ollama, OpenAI-compatible, Anthropic) is a separate implementation behind that
trait. Users supply their own credentials/endpoints (BYOK); the active provider
is selected in settings and only affects **future** analysis jobs. Ollama is
the default and is auto-detected at `http://localhost:11434`.

## Consequences

**Positive**

- No vendor lock-in; privacy-first users stay fully local while cloud users get
  higher accuracy — same code path.
- New providers are additive: implement the trait, no pipeline changes —
  a clean contributor extension point.
- Switching providers is a configuration change, not a migration.

**Negative / Trade-offs**

- The trait must abstract over heterogeneous APIs (vision payloads, model
  names, auth, rate limits, latency) — a lowest-common-denominator interface.
- Provider failures/timeouts must be handled uniformly, producing the `partial`
  state with retry (see [ADR-0011](0011-ingestion-pipeline-state-machine.md)).
- Switching providers does **not** retroactively re-analyze existing
  screenshots; mixed-provenance metadata is expected and acceptable.
