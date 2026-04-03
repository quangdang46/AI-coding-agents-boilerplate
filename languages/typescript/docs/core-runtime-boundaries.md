# TypeScript Language Pack Core Runtime Boundaries

This document is the canonical runtime-boundary note for the TypeScript language pack.

The archived root `typescript/` tree remains migration evidence only; the long-term owned runtime boundary is `languages/typescript/runtime`.

## Planned boundaries

### `languages/typescript/runtime/engine/`
Owns the reusable conversation runtime, session lifecycle, and message orchestration.

### `languages/typescript/runtime/prompts/`
Owns prompt composition, prompt file loading, and prompt section layering.

### `languages/typescript/runtime/config/`
Owns the typed boilerplate config contract and runtime config loading.

### `languages/typescript/runtime/registry/`
Owns registry construction for commands, tools, skills, agents, and feature packs.

### `languages/typescript/runtime/providers/`
Owns provider-neutral interfaces and concrete provider adapters.

## Migration rule
These boundaries are still being extracted incrementally. Archived TypeScript source material should be treated as historical migration input under `references/`, while new ownership stays inside `languages/typescript/`.
