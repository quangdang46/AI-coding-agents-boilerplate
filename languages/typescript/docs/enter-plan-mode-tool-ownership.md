# TypeScript enter-plan-mode tool ownership

The archived `EnterPlanModeTool` files in bead `aicd-3ix.4.9.1` are not shipped as a one-for-one preserved tool package, but they do carry direct contract value because the current TypeScript proving slice explicitly preserves planning state through the `plan` command boundary in `languages/typescript/runtime/registry/coreCommands.ts`.

## Ownership and disposition

- `references/typescript/src/tools/EnterPlanModeTool/EnterPlanModeTool.ts` — owned conceptually by the shipped planning/runtime boundary under `languages/typescript/runtime/registry/coreCommands.ts`, which preserves the proving-slice plan-state summary contract without shipping the archived interactive tool implementation directly.
- `references/typescript/src/tools/EnterPlanModeTool/prompt.ts` — archive-only prompt module retained for source fidelity; the shipped proving slice preserves planning behavior through command/runtime summaries rather than the archived tool prompt text.
- `references/typescript/src/tools/EnterPlanModeTool/UI.tsx` — archive-only UI/result-rendering layer retained for source fidelity; the shipped proving slice does not ship the archived interactive plan-mode tool UI.
- `references/typescript/src/tools/EnterPlanModeTool/constants.ts` — owned conceptually by the shipped planning/runtime boundary under `languages/typescript/runtime/registry/coreCommands.ts`, which preserves the plan-oriented command contract even though the archived tool constant is not shipped directly.

## Shipped-language-pack rule

This subset is complete when each archived `EnterPlanModeTool` row has a concrete shipped owner or an explicit archive-only rationale. The archived implementation, prompt, and UI must not be mistaken for the shipped runtime itself just because the proving slice still preserves planning behavior.
