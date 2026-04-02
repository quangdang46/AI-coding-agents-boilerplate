# Core Runtime Boundaries

This document introduces the first no-behavior-change extraction boundary for the TypeScript boilerplate migration.

## Added boundary namespaces
- `src/core/engine` — conversation/runtime kernel shims
- `src/core/prompts` — prompt composition shims
- `src/core/config` — config entrypoints and future schema home
- `src/core/registry` — command/tool/skill/agent registry entrypoints
- `src/core/providers` — provider/model selection entrypoints

## Current strategy
These files are compatibility shims over the existing runtime. They do **not** change behavior yet. Their purpose is to create stable import targets so later extraction PRs can move implementation behind them incrementally.

The current extraction slices already moved shared built-in command ownership, command/tool catalog assembly, source loading, filtering, loading, and command-runtime selection helpers under `src/core/registry/*`. Legacy top-level modules now increasingly delegate to these helpers instead of owning the logic directly.

## Migration rule
New extraction work should prefer adding behavior behind `src/core/*` rather than adding new direct dependencies on the legacy top-level modules.
