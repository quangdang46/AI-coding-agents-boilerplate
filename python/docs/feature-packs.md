# Python Feature Packs

Feature packs are local bundles stored under `.agent/features/<feature-id>/`.

## Layout
- `feature.json` — manifest
- `files/` — files copied into the generated project when the feature is enabled
- `patches` — reversible `agentkit.toml` updates that wire added prompts, agents, skills, and tools into the runtime

## Current sample feature
- `github-pr-review`
  - adds a pull-request review agent
  - adds a review prompt section
  - adds a `review-pr` skill
  - enables the new prompt, agent, skill, and tool config entries when applied
  - marks `mcp` as a required capability in the manifest

## Supported flows
- Rust installer: `aicd feature add/remove <feature-id> --project <dir>`
- Python runtime helper: `python -m src.agent_boilerplate.entrypoints.cli feature add/remove <feature-id> --project <dir>`
