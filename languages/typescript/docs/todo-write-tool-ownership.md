# TypeScript todo-write tool ownership

The archived `TodoWriteTool` files in bead `aicd-3ix.4.9.1` are not shipped as a one-for-one preserved tool package, but they do carry direct contract value because the current TypeScript proving slice explicitly preserves task-list state through the `tasks` command boundary in `languages/typescript/runtime/registry/coreCommands.ts`.

## Ownership and disposition

- `references/typescript/src/tools/TodoWriteTool/TodoWriteTool.ts` — owned conceptually by the shipped task/runtime boundary under `languages/typescript/runtime/registry/coreCommands.ts`, which preserves the proving-slice task-summary contract without shipping the archived interactive todo-writing tool implementation directly.
- `references/typescript/src/tools/TodoWriteTool/prompt.ts` — archive-only prompt module retained for source fidelity; the shipped proving slice preserves task behavior through command/runtime summaries rather than the archived tool prompt text.
- `references/typescript/src/tools/TodoWriteTool/constants.ts` — owned conceptually by the shipped task/runtime boundary under `languages/typescript/runtime/registry/coreCommands.ts`, which preserves the task-oriented command contract even though the archived tool constant is not shipped directly.

## Shipped-language-pack rule

This subset is complete when each archived `TodoWriteTool` row has a concrete shipped owner or an explicit archive-only rationale. The archived implementation and prompt surface must not be mistaken for the shipped runtime itself just because the proving slice still preserves task-list behavior.
