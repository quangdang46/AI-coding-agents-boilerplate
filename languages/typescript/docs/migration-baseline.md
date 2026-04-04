# TypeScript Language Pack Migration Baseline

## Purpose
This document is the canonical migration baseline for the manifest-backed TypeScript language pack under `languages/typescript/`.

Any archived legacy TypeScript workspace belongs under `references/legacy-typescript-workspace/`. It is not the long-term authority for TypeScript language-pack ownership.

## Runtime seams to preserve
- **CLI/bootstrap semantics** — represented in the archived TypeScript source inventory under `references/source-typescript/SNAPSHOT.md`
- **Conversation/runtime engine** — owned extraction target under `languages/typescript/runtime`
- **Command and tool registry composition** — owned extraction target under `languages/typescript/runtime`
- **Prompt layering** — owned extraction target under `languages/typescript/runtime`
- **Template config contract** — owned extraction target under `languages/typescript/template/base`

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
- keep reusable runtime ownership under `languages/typescript/runtime`
- keep generated-project assets under `languages/typescript/template/base`
- make config explicit and typed through the language-pack contract
- replace plugins-first extensibility with local feature packs for v1

## Migration invariants
1. Skills remain file-backed and user-editable.
2. Agents remain manifest-backed and user-editable.
3. Prompt layering stays configurable.
4. Cleanup work must be protected by language-pack or installer-owned verification.
5. Generated projects should customize through config + `.agents/` assets, not kernel edits.
6. The Rust installer must target a stable template contract, not ad hoc source assumptions.
