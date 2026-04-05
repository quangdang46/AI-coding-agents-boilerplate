# Rust runtime advanced modules ownership

The archived Rust advanced runtime files in bead `aicd-3ix.5.7.1` are not preserved as a one-for-one crate subtree in the shipped language-pack surface. Instead, the current Rust pack keeps only the generated-project template runtime boundary under `languages/rust/template/base/src/` and the owned runtime docs under `languages/rust/runtime/`, with many advanced subsystems still deferred beyond the current shipped slice.

Some rows in this bead’s neighborhood were already dispositioned by earlier Rust ownership work and are referenced here rather than redefined.

## Ownership and disposition

- `references/rust/crates/runtime/src/team_cron_registry.rs` — archive-only advanced workflow helper retained for source fidelity; the current shipped Rust slice does not expose a dedicated team/cron runtime registry module.
- `references/rust/crates/runtime/src/mcp_tool_bridge.rs` — archive-only advanced integration helper retained for source fidelity; the current shipped Rust slice does not expose a dedicated MCP tool-bridge runtime module.
- `references/rust/crates/runtime/src/summary_compression.rs` — owned conceptually by `languages/rust/template/base/src/runtime_summary.rs`, which now provides the shipped runtime-summary shaping boundary without preserving the archived helper one-for-one.
- `references/rust/crates/runtime/src/plugin_lifecycle.rs` — archive-only advanced plugin helper retained for source fidelity; the current shipped Rust slice does not expose a dedicated plugin lifecycle runtime module.
- `references/rust/crates/runtime/src/mcp.rs` — archive-only advanced MCP helper retained for source fidelity; the current shipped Rust slice does not expose a dedicated MCP runtime module.
- `references/rust/crates/runtime/src/mcp_client.rs` — archive-only advanced MCP client helper retained for source fidelity; the current shipped Rust slice does not expose a dedicated MCP client runtime module.
- `references/rust/crates/runtime/src/task_registry.rs` — archive-only advanced task helper retained for source fidelity; the current shipped Rust slice does not expose a dedicated task-registry runtime module.
- `references/rust/crates/runtime/src/mcp_stdio.rs` — archive-only advanced MCP stdio helper retained for source fidelity; the current shipped Rust slice does not expose a dedicated MCP stdio runtime module.
- `references/rust/crates/runtime/src/lsp_client.rs` — archive-only advanced LSP helper retained for source fidelity; the current shipped Rust slice does not expose a dedicated LSP client runtime module.
- `references/rust/crates/runtime/src/task_packet.rs` — archive-only advanced task helper retained for source fidelity; the current shipped Rust slice does not expose a dedicated task-packet runtime module.
- `references/rust/crates/runtime/src/remote.rs` — archive-only advanced remote helper retained for source fidelity; the current shipped Rust slice does not expose a dedicated remote runtime module.
- `references/rust/crates/runtime/src/mcp_lifecycle_hardened.rs` — archive-only advanced integration helper retained for source fidelity; the current shipped Rust slice does not expose a hardened MCP lifecycle runtime module.
- `references/rust/crates/runtime/src/session_control.rs` — archive-only advanced session-control helper retained for source fidelity; the current shipped Rust slice does not expose a dedicated session-control runtime module.

## Previously dispositioned rows still in this runtime neighborhood

- `references/rust/crates/runtime/src/permission_enforcer.rs` — already dispositioned in `languages/rust/docs/runtime-control-modules-ownership.md`.
- `references/rust/crates/runtime/src/trust_resolver.rs` — already dispositioned in `languages/rust/docs/runtime-control-modules-ownership.md`.
- `references/rust/crates/runtime/src/permissions.rs` — already dispositioned in `languages/rust/docs/runtime-control-modules-ownership.md`.
- `references/rust/crates/runtime/src/recovery_recipes.rs` — already dispositioned in `languages/rust/docs/runtime-control-modules-ownership.md`.
- `references/rust/crates/runtime/src/worker_boot.rs` — already dispositioned in `languages/rust/docs/runtime-control-modules-ownership.md`.
- `references/rust/crates/runtime/src/sandbox.rs` — already dispositioned in `languages/rust/docs/runtime-control-modules-ownership.md`.
- `references/rust/crates/runtime/src/stale_branch.rs` — already dispositioned in `languages/rust/docs/runtime-control-modules-ownership.md`.
- `references/rust/crates/runtime/src/policy_engine.rs` — already dispositioned in `languages/rust/docs/runtime-control-modules-ownership.md`.
- `references/rust/crates/runtime/src/hooks.rs` — already dispositioned in `languages/rust/docs/runtime-control-modules-ownership.md`.

## Shipped-language-pack rule

This subset is complete when each archived Rust advanced-runtime row has an explicit shipped-owner or archive-only rationale, whether documented here or by earlier canonical ownership docs referenced above. These archived crate files must not be mistaken for one-for-one shipped modules just because the Rust language pack preserves adjacent runtime behavior in template form.
