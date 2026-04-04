# Rust Language Pack Config Reference

The Rust language pack now generates branded runtime/config surfaces instead of hardcoded `.claw` / `CLAW.md`.

Generated Rust projects scaffold:

- `.<brand>.json`
- `<BRAND>.md`
- `.<brand>/settings.json`
- `.<brand>/settings.local.json`
- `.<brand>/instructions.md`
- `.<brand>/<BRAND>.md`
- `.<brand>/agents/`
- `.<brand>/skills/`
- `.<brand>/commands/`
- `.<brand>/sessions/`

The branded compat JSON file carries Rust runtime configuration such as:

- `defaultProvider`
- `providers`
- `tools`
- `permissions`
- `features`
- `agents`
- `context`

The branded root owns Claude-compatible local surfaces such as:

- merged settings files
- branded instruction files
- markdown subagents
- skills and commands
- persisted session artifacts

`.agents/skills/` may still exist as a generic interoperability mirror for skills, but it is not the primary branded runtime root.
