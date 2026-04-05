# TypeScript singleton tool ownership

These archived singleton tool files in bead `aicd-3ix.4.9.1` are not shipped as part of the current TypeScript proving slice.

The shipped runtime boundaries under `languages/typescript/runtime/registry/coreTools.ts` and `languages/typescript/runtime/registry/coreCommands.ts` preserve only the proving-slice core tool and command surfaces. They do not expose a standalone sleep/wait tool or a synthetic structured-output tool.

## Ownership and disposition

- `references/typescript/src/tools/SleepTool/prompt.ts` — archive-only sleep-tool prompt retained for source fidelity; the shipped TypeScript proving slice does not preserve a standalone sleep/wait runtime boundary.
- `references/typescript/src/tools/SyntheticOutputTool/SyntheticOutputTool.ts` — archive-only non-interactive structured-output tool retained for source fidelity; the shipped TypeScript proving slice does not preserve a synthetic structured-output runtime boundary.

## Shipped-language-pack rule

This subset is complete when each archived singleton tool row has an explicit archive-only or shipped-owner rationale. These archived tool surfaces must not be implied as already shipped runtime ownership when no matching proving-slice boundary exists.
