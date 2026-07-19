# ADR 006: Visual Regression and Automated Screenshots

## Status

Accepted

## Context

UI libraries live and die by their visual documentation. Manually taking screenshots every time a padding or color token changes is tedious and error-prone. Additionally, while the project has strong compilation and linting rules (Zero-Defect Policy), we lack visual guarantees that refactors don't inadvertently break component visuals (like padding, borders, shadows).

## Decision

We have decided to integrate `egui_kittest` for headless component rendering.
1. We will use an `examples/render_docs.rs` binary to automatically generate up-to-date screenshots for all components in `docs/images/`.
2. We will use a `tests/visual.rs` integration test suite to take snapshots of components and pixel-diff them against approved baselines in `tests/snapshots/`.

## Consequences

- **Pros:** The `README.md` and Rustdoc documentation will always be 100% up-to-date. Visual regressions will be caught in CI. Refactors can be merged with confidence.
- **Cons:** Slightly longer test times, and developers must manually update and commit baseline snapshots (via `UPDATE_SNAPSHOTS=1 cargo test --test visual`) when making intentional visual changes.
