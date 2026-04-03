# Rust Workspace Ownership Map

This document freezes the crate-to-language-pack ownership mapping for the archived Rust workspace.

It is the canonical ownership map for Task `aicd-3ix.5.1`.

## Archived workspace root

- `references/rust/Cargo.toml` — canonical archived workspace root
- Archived workspace shape: `[workspace]` with `members = ["crates/*"]`

## Core crate ownership map

| Archived crate | Archived evidence | Shipped ownership / disposition |
| --- | --- | --- |
| `runtime` | `references/rust/crates/runtime/Cargo.toml` and `references/rust/crates/runtime/src/lib.rs` | owned by `languages/rust/runtime/` and its sub-boundaries (`config/`, `entrypoints/`, `prompts/`, `registry/`) |
| `commands` | `references/rust/crates/commands/Cargo.toml` and `references/rust/crates/commands/src/lib.rs` | deferred crate extraction; current command-facing behavior remains represented through the shipped core runtime contract and installer tests |
| `tools` | `references/rust/crates/tools/Cargo.toml` and `references/rust/crates/tools/src/lib.rs` | deferred crate extraction; current tool-facing behavior remains represented through the shipped core runtime contract and installer tests |
| `rusty-claude-cli` | `references/rust/crates/rusty-claude-cli/Cargo.toml` | deferred crate extraction; generated-project executable ownership currently lives under `languages/rust/template/base/Cargo.toml` and `languages/rust/template/base/src/main.rs` |

## Adjacent archived crates

These workspace members are real archived crates but are not part of the smallest required ownership cluster for Task `aicd-3ix.5.1`:

- `api`
- `plugins`
- `compat-harness`
- `telemetry`
- `mock-anthropic-service`

Their final shipped disposition belongs to later Rust lane beads.

## Current shipped Rust pack ownership

The following files already define the shipped Rust ownership boundary:

- `languages/rust/README.md`
- `languages/rust/language.manifest.json`
- `languages/rust/runtime/README.md`
- `languages/rust/runtime/config/README.md`
- `languages/rust/runtime/entrypoints/README.md`
- `languages/rust/runtime/prompts/README.md`
- `languages/rust/runtime/registry/README.md`
- `languages/rust/template/base/Cargo.toml`
- `languages/rust/template/base/src/main.rs`

## Decision rule

No archived Rust crate or workspace root may remain without an explicit shipped ownership mapping or an explicit deferred disposition.

For the current slice, the minimum required mapped set is:

- archived workspace root
- archived `runtime` crate
- archived `commands` crate
- archived `tools` crate
- archived `rusty-claude-cli` crate

Future Rust beads may refine or replace these deferred dispositions, but they must not erase the ownership map.
