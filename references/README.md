# References Archive

This directory is the canonical home for archived source references used during the AICD migration.

The shipped runtime surfaces live under `install/` and `languages/`. Nothing under `references/` is part of the shipped runtime contract unless it is intentionally re-extracted into those owned surfaces.

## Current archive layout

The repository currently preserves archived source material in these roots:

- `references/python/` — archived Python/source-derived migration evidence
- `references/typescript/` — archived TypeScript/source-derived migration evidence
- `references/rust/` — archived Rust/source-derived migration evidence

## Normalized migration aliases

Some migration docs and tests refer to normalized archive identities:

- `references/source-python/`
- `references/source-typescript/`
- `references/source-rust/`
- `references/parity/`

These directories exist as documentation compatibility aliases so migration artifacts can use stable source-oriented names even while the underlying archived material remains in the current `references/python/`, `references/typescript/`, and `references/rust/` roots.

## Boundary rule

Reference-pack material is evidence-only. It may inform planning, parity audits, and port tables, but shipped runtime behavior must be implemented under `install/` and `languages/` rather than by reading directly from `references/` at runtime.
