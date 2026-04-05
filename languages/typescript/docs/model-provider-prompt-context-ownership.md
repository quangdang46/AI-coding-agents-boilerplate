# TypeScript model, provider, prompt, and context helper ownership

The archived helper files in bead `aicd-3ix.4.14.1.3` are not preserved as a one-for-one shipped utility subtree in the current AICD TypeScript proving slice.

The shipped TypeScript pack keeps current ownership for provider/model selection, prompt digestion, and context-loading behavior under `languages/typescript/runtime/config/`, `languages/typescript/runtime/context/`, and the declared runtime-provider boundary. Some rows in this bead’s original manifest were already dispositioned by earlier canonical ownership work and are referenced here rather than redefined.

## Ownership and disposition

- `references/typescript/src/utils/systemPrompt.ts` — owned conceptually by shipped TypeScript prompt/context assembly under `languages/typescript/runtime/context/loadContextState.ts` and `languages/typescript/runtime/config/loadRuntimeSummary.ts`, rather than preserved as a direct utility port.
- `references/typescript/src/utils/systemPromptType.ts` — owned conceptually by shipped TypeScript prompt/context assembly under `languages/typescript/runtime/context/` rather than preserved as a standalone utility file.
- `references/typescript/src/utils/queryContext.ts` — owned conceptually by shipped TypeScript prompt/context digest and load behavior under `languages/typescript/runtime/context/loadContextState.ts` and `languages/typescript/runtime/config/loadRuntimeSummary.ts`.
- `references/typescript/src/utils/model/modelCapabilities.ts` — owned conceptually by the shipped runtime-provider boundary described in `languages/typescript/docs/core-runtime-boundaries.md` and current config/runtime provider resolution surfaces.
- `references/typescript/src/utils/model/bedrock.ts` — owned conceptually by the shipped runtime-provider boundary and `boilerplate.config.ts` provider config surface, rather than preserved one-for-one as a utility file.
- `references/typescript/src/utils/model/modelSupportOverrides.ts` — owned conceptually by the shipped runtime-provider boundary and provider config surface, rather than preserved one-for-one as a utility file.
- `references/typescript/src/utils/model/modelOptions.ts` — owned conceptually by the shipped runtime-provider boundary and provider config surface, rather than preserved one-for-one as a utility file.
- `references/typescript/src/utils/model/check1mAccess.ts` — owned conceptually by the shipped runtime-provider boundary and provider config surface, rather than preserved one-for-one as a utility file.
- `references/typescript/src/utils/model/modelStrings.ts` — owned conceptually by the shipped runtime-provider boundary and provider config surface, rather than preserved one-for-one as a utility file.
- `references/typescript/src/utils/model/aliases.ts` — owned conceptually by the shipped runtime-provider boundary and provider config surface, rather than preserved one-for-one as a utility file.
- `references/typescript/src/utils/model/model.ts` — owned conceptually by the shipped runtime-provider boundary and provider config surface, rather than preserved one-for-one as a utility file.
- `references/typescript/src/utils/model/configs.ts` — owned conceptually by `languages/typescript/runtime/utils/config.ts` and the runtime-provider boundary, rather than preserved one-for-one as a utility file.
- `references/typescript/src/utils/model/validateModel.ts` — owned conceptually by the shipped runtime-provider boundary and provider config surface, rather than preserved one-for-one as a utility file.
- `references/typescript/src/utils/model/agent.ts` — owned conceptually by the shipped runtime-provider boundary and provider config surface, rather than preserved one-for-one as a utility file.
- `references/typescript/src/utils/model/antModels.ts` — owned conceptually by the shipped runtime-provider boundary and provider config surface, rather than preserved one-for-one as a utility file.
- `references/typescript/src/utils/model/contextWindowUpgradeCheck.ts` — owned conceptually by the shipped runtime-provider boundary and current model/config summary surfaces, rather than preserved one-for-one as a utility file.
- `references/typescript/src/utils/model/deprecation.ts` — owned conceptually by the shipped runtime-provider boundary and provider config surface, rather than preserved one-for-one as a utility file.
- `references/typescript/src/utils/model/modelAllowlist.ts` — owned conceptually by the shipped runtime-provider boundary and provider config surface, rather than preserved one-for-one as a utility file.
- `references/typescript/src/utils/model/providers.ts` — owned conceptually by `languages/typescript/runtime/utils/config.ts` and the shipped runtime-provider boundary, rather than preserved one-for-one as a utility file.

## Previously dispositioned row still in this bead's manifest

- `references/typescript/src/utils/context.ts` — already dispositioned in `languages/typescript/docs/root-runtime-shell-ownership.md` as owned conceptually by `languages/typescript/runtime/context/loadContextState.ts`.

## Shipped-language-pack rule

This subset is complete when each archived model/provider/prompt/context row has an explicit shipped-owner or archive-only rationale, whether documented here or by the earlier canonical ownership doc referenced above. These archived helper files must not be mistaken for already-shipped one-for-one runtime modules just because the proving slice preserves their underlying responsibilities.
