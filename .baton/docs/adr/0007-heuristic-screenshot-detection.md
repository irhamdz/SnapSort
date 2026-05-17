# ADR-0007: Heuristic-based screenshot detection

- **Status:** Accepted
- **Date:** 2026-05-16
- **Derived from:** PRD §4.2 (F-01)

## Context

The file watcher (see [ADR-0006](0006-notify-os-native-file-watching.md))
reports every new file in a Watch Folder, but folders like `~/Desktop` also
contain downloads, exported images, icons, and avatars. SnapSort must admit
genuine screenshots into the pipeline while rejecting noise — without AI, since
detection runs before and independently of any AI step
(see [ADR-0009](0009-ai-optional-progressive-enhancement.md)).

## Decision

Apply an ordered set of cheap heuristics. A file is treated as a screenshot
only if it passes **rule 1 AND (rule 2 OR 3 OR 4 OR 5)**:

1. Extension is `.png`, `.jpg`, `.jpeg`, or `.webp`.
2. It is in a known default screenshot directory.
3. Filename matches known patterns (`Screenshot*`, `Capture*`, `Screen Shot*`,
   timestamp-only names, etc.).
4. It was created by a known screenshot process
   (`com.apple.screencaptureui`; `SnippingTool.exe`, `ScreenClippingHost.exe`).
5. Dimensions are at least 400×300 px (filters icons/avatars).

## Consequences

**Positive**

- Fast, fully local, no model required — keeps ingestion cheap and private.
- The AND/OR structure tolerates platform and capture-tool variation.

**Negative / Trade-offs**

- Heuristics are inherently imperfect: false negatives (unusual capture tools)
  and false positives (downloaded PNGs that match a pattern) are possible.
- Pattern and process lists are maintenance surface that drifts as OS tools
  change; they must be updated over time.
- A future explicit "scan existing files" / manual-add path is needed for
  screenshots the heuristics miss.
