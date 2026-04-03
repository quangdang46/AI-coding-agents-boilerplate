# CLAW.md

This file provides guidance to Claw Code when working with `__PROJECT_NAME__`.

## Detected stack
- Languages: Rust.
- Frameworks: none detected from the starter template.

## Verification
- Run Rust verification from the repo root: `cargo fmt`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`

## Working agreement
- Prefer small, reviewable changes.
- Keep shared defaults in `.claw.json`; reserve `.claw/settings.local.json` for machine-local overrides.
- Do not overwrite existing `CLAW.md` content automatically; update it intentionally when repo workflows change.
