# Python Language Pack Config Reference

The Python language pack generates projects with `agentkit.toml` as the language-specific app config file.

Generated Python projects also scaffold branded runtime/config surfaces:

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

`agentkit.toml` still owns Python app/runtime configuration such as providers, tools, and feature toggles.

The branded root owns Claude-compatible local surfaces such as:

- merged settings files
- branded instruction files
- markdown subagents
- skills and commands
- persisted session artifacts

`.agents/skills/` may still exist as a generic interoperability mirror for skills, but it is not the primary branded runtime root.
