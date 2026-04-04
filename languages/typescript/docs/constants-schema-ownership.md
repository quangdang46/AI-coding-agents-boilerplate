# TypeScript constants and schema ownership

The archived TypeScript constants and schema rows in bead `aicd-3ix.4.4.1` carry contract value, but they are not preserved as a monolithic archive subtree in the shipped AICD runtime. Their final disposition is to map either to shipped runtime/config surfaces under `languages/typescript/` or to archive-only product-specific evidence when the constant was specific to the original product build.

## Ownership and disposition

- `references/typescript/src/constants/outputStyles.ts` — owned by shipped TypeScript prompt/config surfaces under `languages/typescript/runtime/context/loadContextState.ts` and template `boilerplate.config.ts`.
- `references/typescript/src/constants/errorIds.ts` — archive-only product/error catalog retained for source fidelity; not required by the shipped TypeScript proving slice.
- `references/typescript/src/constants/xml.ts` — archive-only product formatting constant retained for source fidelity; not required by the shipped TypeScript proving slice.
- `references/typescript/src/constants/keys.ts` — archive-only UI/input constant retained for source fidelity; not required by the shipped TypeScript proving slice.
- `references/typescript/src/constants/spinnerVerbs.ts` — archive-only UI/status constant retained for source fidelity; not required by the shipped TypeScript proving slice.
- `references/typescript/src/constants/apiLimits.ts` — archive-only product/API limit constant retained for source fidelity; not required by the shipped TypeScript proving slice.
- `references/typescript/src/constants/figures.ts` — archive-only terminal-display constant retained for source fidelity; not required by the shipped TypeScript proving slice.
- `references/typescript/src/constants/messages.ts` — archive-only product message catalog retained for source fidelity; not required by the shipped TypeScript proving slice.
- `references/typescript/src/constants/cyberRiskInstruction.ts` — archive-only product instruction constant retained for source fidelity; not part of the shipped AICD contract.
- `references/typescript/src/constants/toolLimits.ts` — owned conceptually by shipped TypeScript runtime tool execution and template tool-policy config, without preserving the archived constant module directly.
- `references/typescript/src/constants/product.ts` — archive-only product-branding constant retained for source fidelity; superseded by AICD brand/template surfaces.
- `references/typescript/src/constants/systemPromptSections.ts` — owned conceptually by shipped TypeScript runtime prompt/context assembly under `languages/typescript/runtime/context/`.
- `references/typescript/src/constants/oauth.ts` — archive-only provider/product OAuth constant retained for source fidelity; optional auth feature work owns future shipped behavior.
- `references/typescript/src/constants/codex-oauth.ts` — archive-only provider/product OAuth constant retained for source fidelity; optional auth feature work owns future shipped behavior.
- `references/typescript/src/constants/betas.ts` — archive-only feature-gate constant retained for source fidelity; not part of the shipped proving slice.
- `references/typescript/src/constants/turnCompletionVerbs.ts` — archive-only UI/interaction constant retained for source fidelity.
- `references/typescript/src/constants/system.ts` — archive-only product/system constant retained for source fidelity.
- `references/typescript/src/constants/common.ts` — archive-only common constant bundle retained for source fidelity rather than cloned into shipped runtime.
- `references/typescript/src/constants/files.ts` — owned conceptually by shipped TypeScript file/tool runtime helpers under `languages/typescript/runtime/utils/` and template `src/index.ts`.
- `references/typescript/src/constants/prompts.ts` — owned conceptually by shipped TypeScript prompt/context assembly under `languages/typescript/runtime/context/` and template config.
- `references/typescript/src/constants/tools.ts` — owned conceptually by shipped TypeScript runtime registry helpers under `languages/typescript/runtime/registry/`.
- `references/typescript/src/constants/github-app.ts` — archive-only product/integration constant retained for source fidelity; future Git/GitHub feature-pack work owns shipped behavior.
- `references/typescript/src/schemas/hooks.ts` — archive-only schema contract retained for source fidelity until advanced hook-runtime feature-pack work is shipped.

## Shipped-language-pack rule

This cluster is complete when each archived constant/schema row has either a clear shipped runtime/config owner or an explicit archive-only rationale. Product-specific constant catalogs and optional feature schemas must not be mistaken for required base-runtime extraction work.
