# TypeScript remote-trigger tool ownership

The archived `RemoteTriggerTool` files in bead `aicd-3ix.4.9.1` are not shipped as part of the current TypeScript proving slice.

The shipped runtime boundaries under `languages/typescript/runtime/registry/coreTools.ts` and `languages/typescript/runtime/registry/coreCommands.ts` preserve only the proving-slice core tool and command surfaces. They do not expose a remote-trigger management runtime boundary for scheduled claude.ai CCR agents.

## Ownership and disposition

- `references/typescript/src/tools/RemoteTriggerTool/RemoteTriggerTool.ts` — archive-only remote-trigger integration retained for source fidelity; the shipped TypeScript proving slice does not preserve remote trigger management as a runtime-owned boundary.
- `references/typescript/src/tools/RemoteTriggerTool/prompt.ts` — archive-only prompt module retained for source fidelity; the shipped proving slice does not ship the archived remote-trigger prompt surface.
- `references/typescript/src/tools/RemoteTriggerTool/UI.tsx` — archive-only UI/result-rendering layer retained for source fidelity; the shipped proving slice does not ship the archived remote-trigger tool UI.

## Shipped-language-pack rule

This subset is complete when each archived `RemoteTriggerTool` row has an explicit archive-only or shipped-owner rationale. These archived remote-trigger modules must not be implied as already shipped runtime ownership when no matching proving-slice boundary exists.
