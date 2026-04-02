# TypeScript Boilerplate Migration Baseline

## Purpose
This document defines the baseline rules for migrating the current `typescript/` coding-agent source into a reusable boilerplate.

## Runtime seams to preserve
- **CLI bootstrap** — `src/entrypoints/cli.tsx`
- **Conversation engine** — `src/QueryEngine.ts`
- **Command registry** — `src/commands.ts`
- **Tool registry** — `src/tools.ts`
- **Skill loader** — `src/skills/loadSkillsDir.ts`
- **Agent loader** — `src/tools/AgentTool/loadAgentsDir.ts`
- **Prompt layering** — `src/utils/systemPrompt.ts`, `src/constants/prompts.ts`

## Keep / remove / generalize
### Keep
- registry-driven command and tool composition
- file-based skill loading
- manifest-based agent loading
- prompt layering hooks
- provider abstraction direction

### Remove
- duplicate snapshot tree at `free-code-main/`
- free-code branding/docs/install script surfaces
- default telemetry/product analytics behavior
- snapshot-specific installer behavior

### Generalize
- extract a runtime kernel under `src/core/`
- move prompts to file-backed template assets
- make config explicit and typed
- replace plugins-first extensibility with local feature packs for v1

## Migration invariants
1. Skills remain file-backed and user-editable.
2. Agents remain manifest-backed and user-editable.
3. Prompt layering stays configurable.
4. Cleanup work must be protected by smoke tests.
5. Generated projects should customize through config + `.agent/` assets, not kernel edits.
6. The Rust installer must target a stable template contract, not ad hoc source assumptions.
