# TypeScript tool-search tool ownership

The archived `ToolSearchTool` files in bead `aicd-3ix.4.10.1` are not shipped as part of the current default TypeScript core runtime.

The shipped TypeScript runtime registry under `languages/typescript/runtime/registry/coreTools.ts` preserves only the mandatory default tool surface, while the archived `ToolSearchTool` implements deferred-tool discovery and schema loading for optional tool families. That makes this archived tool family part of a future advanced tool-discovery feature boundary rather than the shipped proving slice.

## Ownership and disposition

- `references/typescript/src/tools/ToolSearchTool/ToolSearchTool.ts` — future feature-pack ownership aligned to an advanced deferred-tool discovery/loading capability; the shipped TypeScript core runtime does not currently expose a standalone `ToolSearch` runtime boundary.
- `references/typescript/src/tools/ToolSearchTool/prompt.ts` — future feature-pack ownership aligned to an advanced deferred-tool discovery/loading capability; archived tool-search prompting is not shipped in the current proving slice.
- `references/typescript/src/tools/ToolSearchTool/constants.ts` — future feature-pack ownership aligned to an advanced deferred-tool discovery/loading capability; the archived tool-name constant is not preserved as part of the shipped default runtime.

## Shipped-language-pack rule

This subset is complete when each archived `ToolSearchTool` row has a clear future feature-pack owner or an explicit archive-only rationale. These archived deferred-tool search modules must not be implied as already shipped default-runtime ownership just because the proving slice still preserves a smaller core-tool registry.
