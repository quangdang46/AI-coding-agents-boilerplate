# Rust runtime core modules ownership

The archived Rust runtime-core files in bead `aicd-3ix.5.3.1` are not preserved as a one-for-one crate subtree in the shipped language-pack surface. Instead, the Rust pack owns this behavior through the generated-project template under `languages/rust/template/base/src/` together with the documented runtime boundary under `languages/rust/runtime/`.

## Ownership and disposition

- `references/rust/crates/runtime/src/config.rs` — owned conceptually by `languages/rust/template/base/src/config.rs`, which now provides the shipped runtime config contract and loader for generated Rust projects.
- `references/rust/crates/runtime/src/prompt.rs` — owned conceptually by `languages/rust/template/base/src/prompt.rs`, which now provides the shipped prompt/context loading boundary for generated Rust projects.
- `references/rust/crates/runtime/src/session.rs` — owned conceptually by `languages/rust/template/base/src/session.rs`, which now provides the shipped session persistence and usage summary boundary for generated Rust projects.
- `references/rust/crates/runtime/src/conversation.rs` — owned conceptually by `languages/rust/template/base/src/conversation.rs`, which now provides the shipped runtime-summary rendering boundary for generated Rust projects.
- `references/rust/crates/runtime/src/bootstrap.rs` — owned conceptually by `languages/rust/template/base/src/bootstrap.rs`, which now provides the shipped runtime bootstrap/session-loop boundary for generated Rust projects.
- `references/rust/crates/runtime/src/compact.rs` — owned conceptually by `languages/rust/template/base/src/runtime_summary.rs` and the template runtime-summary path rather than preserved one-for-one as a separate archived module.
- `references/rust/crates/runtime/src/usage.rs` — owned conceptually by `languages/rust/template/base/src/usage.rs`, which now provides the shipped usage/cost calculation boundary for generated Rust projects.
- `references/rust/crates/runtime/src/lib.rs` — owned conceptually by the shipped Rust runtime ownership map in `languages/rust/docs/workspace-ownership-map.md` plus the generated-project runtime modules under `languages/rust/template/base/src/`.
- `references/rust/crates/runtime/src/sse.rs` — archive-only transport helper retained for source fidelity; SSE-specific runtime transport is not part of the current shipped Rust language-pack slice.
- `references/rust/crates/runtime/src/json.rs` — owned conceptually by the shipped generated-project runtime modules under `languages/rust/template/base/src/`, where JSON/runtime summary behavior is represented without preserving this archived helper one-for-one.

## Shipped-language-pack rule

This subset is complete when each archived Rust runtime-core row has an explicit shipped-owner or archive-only rationale. These archived crate files must not be mistaken for one-for-one shipped modules just because the Rust language pack preserves their runtime responsibilities in decomposed template form.
