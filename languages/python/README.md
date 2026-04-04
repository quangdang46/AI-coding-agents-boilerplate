# Python Language Pack

This directory is the manifest-backed Python language pack for AICD.

Current contents in this migration slice:

- `language.manifest.json` — installer discovery contract
- `runtime/` — owned Python runtime boundary required by `RULES.md`
- `template/base/` — generated-project template used by the installer

This pack is the proving slice for manifest-driven `init`, `feature add`, and `doctor` workflows.
