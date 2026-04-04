# Rust Language Pack

This directory is the owned Rust language-pack boundary for AICD.

Current contents in this migration slice:

- `language.manifest.json` — installer discovery contract
- `runtime/` — owned Rust runtime boundary required by `RULES.md`
- `template/base/` — generated-project template for manifest-driven `init`
- `docs/workspace-ownership-map.md` — canonical crate-to-language-pack ownership map for archived Rust workspace members

Generated Rust projects now use branded runtime/config surfaces:

- `.<brand>.json`
- `<BRAND>.md`
- `.<brand>/agents/`
- `.<brand>/skills/`
- `.<brand>/commands/`
- `.<brand>/sessions/`

`.agents/skills/` may still exist as a generic skill mirror, but it is not the primary runtime root.

The archived Rust workspace root and the minimum crate ownership mapping remain documented in `docs/workspace-ownership-map.md`.
