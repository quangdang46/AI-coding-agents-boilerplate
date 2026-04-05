# Rust runtime control, safety, and worker-control modules ownership

The archived Rust runtime control files in bead `aicd-3ix.5.5.1` are not preserved as a one-for-one crate subtree in the shipped language-pack surface. Instead, the Rust pack owns the current control-path behavior through generated-project template modules under `languages/rust/template/base/src/`, while some archived modules remain archive-only because the current shipped slice does not expose direct replacements yet.

## Ownership and disposition

- `references/rust/crates/runtime/src/permissions.rs` — owned conceptually by `languages/rust/template/base/src/permissions.rs`, which now provides the shipped permission-mode, policy, request, and prompt contract for generated Rust projects.
- `references/rust/crates/runtime/src/permission_enforcer.rs` — owned conceptually by `languages/rust/template/base/src/permission_enforcer.rs`, which now provides the shipped runtime enforcement boundary for tool, bash, and file-write authorization.
- `references/rust/crates/runtime/src/sandbox.rs` — owned conceptually by `languages/rust/template/base/src/sandbox.rs`, which now provides the shipped sandbox configuration and status boundary for generated Rust projects.
- `references/rust/crates/runtime/src/policy_engine.rs` — archive-only policy helper retained for source fidelity; the current shipped Rust slice does not expose a dedicated `policy_engine.rs` template module.
- `references/rust/crates/runtime/src/recovery_recipes.rs` — owned conceptually by `languages/rust/template/base/src/recovery_recipes.rs`, which now provides the shipped recovery-recipe boundary for generated Rust projects.
- `references/rust/crates/runtime/src/trust_resolver.rs` — owned conceptually by `languages/rust/template/base/src/trust_resolver.rs`, which now provides the shipped trust-prompt detection and resolution boundary for generated Rust projects.
- `references/rust/crates/runtime/src/worker_boot.rs` — owned conceptually by `languages/rust/template/base/src/worker_boot.rs`, which now provides the shipped worker-registry and boot-state boundary for generated Rust projects.
- `references/rust/crates/runtime/src/stale_branch.rs` — owned conceptually by `languages/rust/template/base/src/stale_branch.rs`, which now provides the shipped stale-branch freshness and policy boundary for generated Rust projects.
- `references/rust/crates/runtime/src/green_contract.rs` — archive-only contract helper retained for source fidelity; the current shipped Rust slice does not expose a dedicated `green_contract.rs` template module.
- `references/rust/crates/runtime/src/hooks.rs` — archive-only hook-runtime helper retained for source fidelity; the current shipped Rust slice does not expose a dedicated `hooks.rs` template module.

## Shipped-language-pack rule

This subset is complete when each archived Rust control/safety row has an explicit shipped-owner or archive-only rationale. These archived crate files must not be mistaken for one-for-one shipped modules just because the Rust language pack preserves their control and safety responsibilities in decomposed template form.
