# Rust API crate ownership

The archived Rust `api` crate in bead `aicd-3ix.5.2.1` is not preserved as a one-for-one shipped crate in the current Rust language-pack slice. Instead, the shipped Rust pack keeps provider/runtime-facing ownership through the generated-project template and the runtime/provider summaries described in the Rust ownership map.

Some rows in this crate were already dispositioned by earlier Rust ownership work and are referenced here rather than redefined.

## Ownership and disposition

- `references/rust/crates/api/tests/client_integration.rs` — archive-only integration test retained for source fidelity; the current shipped Rust pack does not expose the archived API crate integration tests one-for-one.
- `references/rust/crates/api/tests/provider_client_integration.rs` — archive-only integration test retained for source fidelity; the current shipped Rust pack does not expose the archived API crate integration tests one-for-one.
- `references/rust/crates/api/tests/openai_compat_integration.rs` — archive-only integration test retained for source fidelity; the current shipped Rust pack does not expose the archived API crate integration tests one-for-one.
- `references/rust/crates/api/src/client.rs` — owned conceptually by `languages/rust/template/base/src/providers.rs`, which now provides the shipped provider/model summary boundary for generated Rust projects without preserving the archived API client one-for-one.
- `references/rust/crates/api/src/error.rs` — archive-only API helper retained for source fidelity; the current shipped Rust pack does not expose a dedicated API error crate module one-for-one.
- `references/rust/crates/api/src/types.rs` — archive-only API helper retained for source fidelity; the current shipped Rust pack does not expose a dedicated API types crate module one-for-one.
- `references/rust/crates/api/src/sse.rs` — archive-only transport helper retained for source fidelity; the current shipped Rust pack does not expose a dedicated API SSE crate module one-for-one.
- `references/rust/crates/api/src/prompt_cache.rs` — owned conceptually by `languages/rust/template/base/src/prompt.rs`, which now provides the shipped prompt/context digest boundary without preserving the archived API prompt-cache helper one-for-one.

## Previously dispositioned rows still in this crate's manifest

- `references/rust/crates/api/Cargo.toml` — already dispositioned in `languages/rust/docs/workspace-ownership-map.md` as part of the deferred `api` crate ownership map.
- `references/rust/crates/api/src/lib.rs` — already dispositioned via `languages/rust/docs/workspace-ownership-map.md` and the shipped Rust pack ownership boundary.
- `references/rust/crates/api/src/providers/mod.rs` — already dispositioned through the shipped provider boundary represented by `languages/rust/template/base/src/providers.rs`.
- `references/rust/crates/api/src/providers/anthropic.rs` — already dispositioned through the shipped provider boundary represented by `languages/rust/template/base/src/providers.rs`.
- `references/rust/crates/api/src/providers/openai_compat.rs` — already dispositioned through the shipped provider boundary represented by `languages/rust/template/base/src/providers.rs`.

## Shipped-language-pack rule

This subset is complete when each archived Rust API crate row has an explicit shipped-owner or archive-only rationale, whether documented here or by earlier canonical ownership docs referenced above. These archived crate files must not be mistaken for one-for-one shipped modules just because the Rust language pack preserves adjacent provider and prompt responsibilities in template form.
