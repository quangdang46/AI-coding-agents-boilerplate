# Rust Language Pack

This directory is the owned Rust language-pack boundary for AICD.

Current contents in this migration slice:

- `language.manifest.json` — installer discovery contract
- `runtime/` — owned Rust runtime boundary required by `RULES.md`
- `template/base/` — generated-project template for manifest-driven `init`
- docs and test boundaries for future Rust extraction work

Archived Rust reference material remains available through the repo's reference archive during migration.

This pack now supports manifest-driven `init`, `feature add/remove`, and `doctor`, but only the first Rust feature-pack slice is currently shipped.

Generated Rust projects also now ship workspace instruction-memory files under `.agent/prompts/` alongside `CLAW.md`.

Those Rust prompt files now include layered prompt sections under `.agent/prompts/sections/`.

Generated Rust projects also now ship local file-backed agent roots under `.agent/agents/`.

Generated Rust projects also now ship local file-backed skill roots under `.agent/skills/`.
