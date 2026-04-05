# TypeScript web-search tool ownership

The archived `WebSearchTool` files in bead `aicd-3ix.4.9.1` are not shipped as part of the current default TypeScript core runtime.

The shipped TypeScript proving slice preserves only the default `web_fetch` web capability under `languages/typescript/runtime/registry/coreTools.ts` and does not expose a separate `web_search` core tool. The archived `WebSearchTool` family therefore belongs to a future optional current-information/search capability boundary rather than the shipped default runtime.

## Ownership and disposition

- `references/typescript/src/tools/WebSearchTool/WebSearchTool.ts` — future feature-pack ownership aligned to an optional web/current-information search capability; the shipped TypeScript core runtime does not currently expose a standalone `web_search` tool.
- `references/typescript/src/tools/WebSearchTool/prompt.ts` — future feature-pack ownership aligned to an optional web/current-information search capability; archived web-search prompting is not shipped in the current proving slice.
- `references/typescript/src/tools/WebSearchTool/UI.tsx` — future feature-pack ownership aligned to an optional web/current-information search capability; archived web-search result UI is not shipped in the current proving slice.

## Shipped-language-pack rule

This subset is complete when each archived `WebSearchTool` row has a clear future feature-pack owner or an explicit archive-only rationale. These archived web-search modules must not be implied as already shipped default-runtime ownership just because the proving slice still preserves `web_fetch`.
