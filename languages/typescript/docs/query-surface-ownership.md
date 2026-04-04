# TypeScript query surface ownership

The archived TypeScript `src/query/*` files in the special-surfaces lane are source-snapshot internals that do not currently ship as a standalone `languages/typescript/runtime/query/` boundary in the AICD TypeScript language pack.

## Ownership and disposition

- `references/typescript/src/query/config.ts` — owned conceptually by shipped TypeScript runtime config loading under `languages/typescript/runtime/utils/config.ts`, without preserving the archived query-internal module directly.
- `references/typescript/src/query/tokenBudget.ts` — archive-only query-budget helper retained for source fidelity; the shipped TypeScript proving slice does not yet expose a dedicated token-budget query module.
- `references/typescript/src/query/stopHooks.ts` — archive-only query-stop-hook helper retained for source fidelity; advanced hook/runtime behavior remains outside the shipped TypeScript proving slice.
- `references/typescript/src/query/deps.ts` — archive-only query dependency-injection helper retained for source fidelity; the shipped TypeScript proving slice does not currently expose a dedicated query-deps boundary.

## Shipped-language-pack rule

This subset is complete when each archived query row has either a concrete shipped owner or an explicit archive-only rationale. These source-snapshot query internals must not be mistaken for already-shipped runtime modules.
