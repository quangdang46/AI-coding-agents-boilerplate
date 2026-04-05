# Rust plugin, harness, telemetry, and mock-service ownership

The archived support crates and bundled plugin assets in bead `aicd-3ix.5.8.1` are not preserved as a one-for-one shipped crate subtree in the current Rust language-pack slice.

The current Rust pack does ship the optional `local-plugins` feature-pack boundary, but it does not ship the archived `plugins`, `compat-harness`, `telemetry`, or `mock-anthropic-service` crates one-for-one. The workspace ownership map also already classifies these as adjacent archived crates whose final shipped disposition belongs to later Rust beads rather than to the minimum required runtime ownership cluster.

## Ownership and disposition

- `references/rust/crates/mock-anthropic-service/Cargo.toml` — archive-only mock-service crate retained for source fidelity; the current shipped Rust language pack does not expose a mock service crate.
- `references/rust/crates/mock-anthropic-service/src/main.rs` — archive-only mock-service crate retained for source fidelity; the current shipped Rust language pack does not expose a mock service crate.
- `references/rust/crates/mock-anthropic-service/src/lib.rs` — archive-only mock-service crate retained for source fidelity; the current shipped Rust language pack does not expose a mock service crate.
- `references/rust/crates/compat-harness/Cargo.toml` — archive-only compatibility harness retained for source fidelity; the current shipped Rust language pack does not expose a compat-harness crate.
- `references/rust/crates/compat-harness/src/lib.rs` — archive-only compatibility harness retained for source fidelity; the current shipped Rust language pack does not expose a compat-harness crate.
- `references/rust/crates/plugins/bundled/example-bundled/.claude-plugin/plugin.json` — future feature-pack ownership aligned to `local-plugins`; archived bundled sample plugin assets are not shipped one-for-one in the current Rust pack.
- `references/rust/crates/plugins/bundled/example-bundled/hooks/post.sh` — future feature-pack ownership aligned to `local-plugins`; archived bundled sample plugin hooks are not shipped one-for-one in the current Rust pack.
- `references/rust/crates/plugins/bundled/example-bundled/hooks/pre.sh` — future feature-pack ownership aligned to `local-plugins`; archived bundled sample plugin hooks are not shipped one-for-one in the current Rust pack.
- `references/rust/crates/plugins/bundled/sample-hooks/.claude-plugin/plugin.json` — future feature-pack ownership aligned to `local-plugins`; archived bundled sample plugin assets are not shipped one-for-one in the current Rust pack.
- `references/rust/crates/plugins/bundled/sample-hooks/hooks/post.sh` — future feature-pack ownership aligned to `local-plugins`; archived bundled sample plugin hooks are not shipped one-for-one in the current Rust pack.
- `references/rust/crates/plugins/bundled/sample-hooks/hooks/pre.sh` — future feature-pack ownership aligned to `local-plugins`; archived bundled sample plugin hooks are not shipped one-for-one in the current Rust pack.
- `references/rust/crates/plugins/Cargo.toml` — archive-only support crate retained for source fidelity; the current shipped Rust pack does not expose the archived plugins crate one-for-one.
- `references/rust/crates/plugins/src/lib.rs` — archive-only support crate retained for source fidelity; the current shipped Rust pack does not expose the archived plugins crate one-for-one.
- `references/rust/crates/plugins/src/hooks.rs` — archive-only support crate retained for source fidelity; the current shipped Rust pack does not expose the archived plugins crate one-for-one.
- `references/rust/crates/telemetry/Cargo.toml` — archive-only support crate retained for source fidelity; the current shipped Rust pack does not expose a telemetry crate one-for-one.
- `references/rust/crates/telemetry/src/lib.rs` — archive-only support crate retained for source fidelity; the current shipped Rust pack does not expose a telemetry crate one-for-one.

## Shipped-language-pack rule

This subset is complete when each archived plugin/harness/telemetry row has an explicit shipped-owner or archive-only rationale. These archived support crates and bundled assets must not be mistaken for one-for-one shipped Rust language-pack modules just because the current pack preserves a narrower `local-plugins` feature boundary.
