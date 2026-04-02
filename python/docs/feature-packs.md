# Python Feature Packs

Feature packs are local bundles stored under `.agent/features/<feature-id>/`.

## Layout
- `feature.json` — manifest
- `files/` — files copied into the generated project when the feature is enabled

## Current sample feature
- `github-pr-review`
  - adds a review prompt section
  - adds a `review-pr` skill
  - marks `mcp` as a required capability in the manifest

## Supported flows
- Rust installer: `aicd feature add/remove <feature-id> --project <dir>`
- Python runtime helper: `python -m src.agent_boilerplate.entrypoints.cli feature add/remove <feature-id> --project <dir>`
