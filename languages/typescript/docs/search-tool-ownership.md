# TypeScript search-tool ownership

The archived search-tool files in bead `aicd-3ix.4.9.1` are not shipped as part of the current default TypeScript core runtime.

The shipped TypeScript runtime registry under `languages/typescript/runtime/registry/coreTools.ts` preserves only the mandatory default tool surface: `bash`, `file_read`, `file_write`, `file_edit`, and `web_fetch`. That means the archived `GlobTool` and `GrepTool` families still carry product value, but they belong to a future search-oriented feature boundary rather than the shipped proving slice.

## Ownership and disposition

- `references/typescript/src/tools/GlobTool/prompt.ts` — future feature-pack ownership aligned to a search/discovery tool family; the shipped TypeScript core runtime does not currently expose a standalone glob tool.
- `references/typescript/src/tools/GlobTool/UI.tsx` — future feature-pack ownership aligned to a search/discovery tool family; archived glob-specific UI is not shipped in the current proving slice.
- `references/typescript/src/tools/GlobTool/GlobTool.ts` — future feature-pack ownership aligned to a search/discovery tool family; glob execution is not part of the current default TypeScript runtime registry.
- `references/typescript/src/tools/GrepTool/GrepTool.ts` — future feature-pack ownership aligned to a search/discovery tool family; grep execution is not part of the current default TypeScript runtime registry.
- `references/typescript/src/tools/GrepTool/prompt.ts` — future feature-pack ownership aligned to a search/discovery tool family; archived grep prompting is not shipped in the current proving slice.
- `references/typescript/src/tools/GrepTool/UI.tsx` — future feature-pack ownership aligned to a search/discovery tool family; archived grep-specific UI is not shipped in the current proving slice.

## Shipped-language-pack rule

This subset is complete when each archived search-tool row has a clear future feature-pack owner or an explicit archive-only rationale. These archived search tools must not be implied as already shipped default-runtime ownership just because the broader tool taxonomy is preserved.
