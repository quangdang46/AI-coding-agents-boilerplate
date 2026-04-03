# TypeScript Language Pack

This directory is the manifest-backed TypeScript language pack for AICD.

Current contents in this migration slice:

- `language.manifest.json` — installer discovery contract
- `runtime/` — owned TypeScript runtime boundary required by `RULES.md`
- `template/base/` — generated-project template for manifest-driven `init`

This pack now supports manifest-driven `init`, `feature add/remove`, and `doctor`, but it is not yet a full proving slice like Python because only the first TypeScript feature-pack slice is currently shipped.
