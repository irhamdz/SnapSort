# ADR-0018: Tauri bundler + GitHub Actions release pipeline

- **Status:** Accepted
- **Date:** 2026-05-16
- **Derived from:** PRD §1.5, §3.2 (Release process), §4.1 (F-33)

## Context

SnapSort ships native installers for macOS and Windows from an open-source
repo. Building platform binaries (and code-signing/notarization concerns) by
hand is error-prone and unscalable for a community project, and an auto-updater
(F-33) needs a consistent, versioned artifact source.

## Decision

Use the **Tauri bundler** to produce platform artifacts (`.dmg` on macOS,
`.exe` on Windows, `.AppImage` where applicable), driven by **GitHub Actions**
that auto-build and publish binaries on a version tag push. Releases are
tag-triggered and reproducible from CI.

## Consequences

**Positive**

- One push of a version tag yields all platform artifacts; no maintainer's
  local machine is in the release path.
- Provides the stable, versioned artifact feed the future auto-updater depends
  on, and a transparent build process suitable for OSS trust.

**Negative / Trade-offs**

- Cross-platform builds require platform-specific CI runners; macOS signing and
  notarization need securely managed secrets in CI.
- Release integrity depends on the CI configuration and its secrets — a
  supply-chain surface that must be guarded.
- Bundle size and platform packaging quirks must be regression-tested per
  release (relates to [ADR-0001](0001-tauri-2-application-framework.md)).
