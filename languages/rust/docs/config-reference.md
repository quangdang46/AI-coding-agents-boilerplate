# Rust Language Pack Config Reference

Rust currently uses `.claw.json` as the primary shipped config file under `languages/rust/`.

Reference evidence shows the current Rust implementation uses `.claw.json`, `.claw/settings.local.json`, and `CLAW.md` as key workspace-local configuration and instruction surfaces.

In the current migration slice, the installer may scaffold those Rust workspace-local surfaces through `init`, `feature add/remove`, and `doctor`. Rust feature enablement is recorded in `.claw.json` under `features.enabled`.

Generated Rust projects now also include provider and model selection through `.claw.json` via `defaultProvider` and the `providers` map.

Generated Rust projects now also include tool defaults through `.claw.json`, including `web_fetch` as part of the shipped web/search baseline.

Those Rust tool defaults also include `bashTimeoutMs`, `webFetchTimeoutMs`, and `maxToolCallsPerTurn` so the workspace file/shell/web tool baseline is explicit rather than implied.

Generated Rust projects also include permission and sandbox controls through `.claw.json` under `permissions.defaultMode`.

Generated Rust projects also reserve `.agent/sessions/` as the local persisted-session root for future session and resume flows.

Generated Rust projects also reserve `.agent/context/` as the local workspace-context root, tracked in `.claw.json` under `context.paths`.

Generated Rust projects also reserve `.agent/usage/` as the local usage and cost tracking root for future accounting flows.

Generated Rust projects also expose local agent discovery through `.claw.json` under `agents`, including directories, enabled agents, and `defaultAgent`.

Generated Rust projects also ship workspace instruction-memory files under `.agent/prompts/`, with `CLAW.md` remaining the primary top-level instruction file rather than the runtime config anchor.

Prompt construction now includes layered prompt sections under `.agent/prompts/sections/`, starting with shared style and verification guidance.

Generated Rust projects also ship local agent roots under `.agent/agents/`, matching the editable agent seam already present in Python and TypeScript.

Rust also ships workspace-local skills under `.agent/skills/`, matching the local skill discovery seam already present in Python and TypeScript.
