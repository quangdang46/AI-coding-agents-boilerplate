# TypeScript skill-tool ownership

The archived `SkillTool` files in bead `aicd-3ix.4.9.1` are not shipped as a one-for-one preserved tool package, but they do carry direct contract value because the current TypeScript proving slice explicitly preserves skill discovery state through the `skills` command boundary in `languages/typescript/runtime/registry/coreCommands.ts`.

## Ownership and disposition

- `references/typescript/src/tools/SkillTool/SkillTool.ts` — owned conceptually by the shipped skills/runtime boundary under `languages/typescript/runtime/registry/coreCommands.ts`, which preserves the proving-slice skills-summary contract without shipping the archived interactive skill-loading tool implementation directly.
- `references/typescript/src/tools/SkillTool/prompt.ts` — archive-only prompt module retained for source fidelity; the shipped proving slice preserves skills behavior through command/runtime summaries rather than the archived tool prompt text.
- `references/typescript/src/tools/SkillTool/UI.tsx` — archive-only UI/result-rendering layer retained for source fidelity; the shipped proving slice does not ship the archived interactive skill-tool UI.
- `references/typescript/src/tools/SkillTool/constants.ts` — owned conceptually by the shipped skills/runtime boundary under `languages/typescript/runtime/registry/coreCommands.ts`, which preserves the skill-oriented command contract even though the archived tool constant is not shipped directly.

## Shipped-language-pack rule

This subset is complete when each archived `SkillTool` row has a concrete shipped owner or an explicit archive-only rationale. The archived implementation, prompt, and UI surface must not be mistaken for the shipped runtime itself just because the proving slice still preserves skills behavior.
