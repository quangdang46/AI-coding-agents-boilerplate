# Rust workspace fallback ownership

The residual archived Rust rows in bead `aicd-3ix.5.11.1.1` are the remaining workspace-metadata and runtime-misc files that were left after the main Rust ownership clusters were dispositioned. They are not preserved as a one-for-one shipped subtree in the current Rust language-pack surface; instead, each row below is either owned conceptually by the shipped Rust workspace/template boundary or retained as archive-only evidence where the generated shell intentionally does not ship the archived behavior directly.

## Ownership and disposition

- `references/rust/crates/runtime/Cargo.toml` — owned conceptually by `languages/rust/docs/workspace-ownership-map.md`, which already maps the archived `runtime` crate manifest to the shipped Rust runtime boundary under `languages/rust/runtime/`.
- `references/rust/crates/runtime/src/oauth.rs` — archive-only OAuth helper retained for source fidelity; the current shipped Rust template intentionally leaves `login` and `logout` as host-runtime guidance in `languages/rust/template/base/src/main.rs` rather than shipping this archived OAuth implementation one-for-one.
- `references/rust/.sandbox-home/.rustup/settings.toml` — archive-only sandbox-home toolchain artifact retained for source fidelity; the current shipped Rust language-pack surface does not ship archived rustup state as part of the runtime contract.
- `references/rust/Cargo.toml` — owned conceptually by `languages/rust/docs/workspace-ownership-map.md`, which already marks the archived workspace root as the canonical archived workspace shape and maps its shipped ownership boundary.
- `references/rust/.gitignore` — archive-only workspace-local ignore policy retained for source fidelity; the current shipped Rust language-pack surface does not preserve this archived ignore file one-for-one.
- `references/rust/Cargo.lock` — archive-only workspace lockfile retained for source fidelity and reproducibility evidence; the current shipped Rust template does not ship the archived reference-workspace lockfile one-for-one.

## Shipped-language-pack rule

This subset is complete when each residual fallback row has an explicit shipped-owner or archive-only rationale. These leftover workspace metadata and runtime-misc rows must not remain unchecked just because they were swept into a stale broad fallback manifest after the canonical Rust ownership clusters were already closed.
