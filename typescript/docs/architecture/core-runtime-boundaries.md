# Core Runtime Boundaries

This document marks the first extraction boundary for turning the current `typescript/` source into a reusable boilerplate runtime.

## Planned boundaries

### `src/core/engine/`
Owns the reusable conversation runtime, session lifecycle, and message orchestration.

### `src/core/prompts/`
Owns prompt composition, prompt file loading, and prompt section layering.

### `src/core/config/`
Owns the typed boilerplate config contract and runtime config loading.

### `src/core/registry/`
Owns registry construction for commands, tools, skills, agents, and feature packs.

### `src/core/providers/`
Owns provider-neutral interfaces and concrete provider adapters.

## Migration rule
These directories are introduced as a no-behavior-change skeleton first. Runtime logic should be moved into them incrementally behind compatibility shims rather than through a single flag-day rewrite.
