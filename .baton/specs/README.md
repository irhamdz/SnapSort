# SnapSort Task Specs

Baton task specs, one per Architecture Decision Record in
`.baton/docs/adr/`. Each spec turns an ADR's decision into an actionable
implementation task and encodes that decision in its `decisions:` block
(`decided_by: human`). Schema: `.baton/docs/task-spec.examples.yaml`.

Dispatch (per `.baton/project-brief.md`):

```
baton run --spec .baton/specs/<spec>.yaml --task-id <id>
baton result <id> --output
```

## Suggested execution order

Foundational scaffolds first; feature specs depend on them.

| Order | Spec | Criticality | Depends on |
|---|---|---|---|
| 1 | 0001-tauri-2-application-framework | high | — |
| 1 | 0002-react-typescript-tailwind-zustand-frontend | high | — |
| 2 | 0003-sqlite-rusqlite-embedded-store | high | 0001 |
| 3 | 0017-typed-ipc-contract-single-boundary | medium | 0001, 0002 |
| 4 | 0004-sqlite-fts5-full-text-search | high | 0003 |
| 4 | 0012-fixed-category-taxonomy-provenance | medium | 0003 |
| 5 | 0006-notify-os-native-file-watching | high | 0001 |
| 6 | 0007-heuristic-screenshot-detection | high | 0006 |
| 6 | 0008-bundled-tesseract-offline-ocr | high | 0003 |
| 7 | 0005-thumbnails-as-sqlite-blobs | medium | 0003 |
| 7 | 0011-ingestion-pipeline-state-machine | high | 0006, 0008 |
| 8 | 0010-pluggable-aiprovider-trait-byok | medium | 0011 |
| 8 | 0013-api-keys-in-os-keychain | high | 0010 |
| 9 | 0009-ai-optional-progressive-enhancement | high | 0011 |
| 9 | 0014-batch-operations-first-class-transactional | high | 0003, 0017 |
| 10 | 0016-destructive-deletes-route-to-os-trash | high | 0014 |
| — | 0015-privacy-by-default | high | cross-cutting guard |
| — | 0018-tauri-bundler-github-actions-release | medium | 0001 |
