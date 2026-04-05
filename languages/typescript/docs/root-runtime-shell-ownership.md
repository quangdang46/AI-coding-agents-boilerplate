# TypeScript root runtime-shell ownership

The archived root runtime shell files in bead `aicd-3ix.4.2.1` are not preserved as a one-for-one top-level source tree in the current AICD TypeScript proving slice. Instead, the monolithic shell has already been decomposed into the shipped runtime entrypoint, engine, context, state, config, and registry boundaries under `languages/typescript/runtime/`.

## Ownership and disposition

- `references/typescript/src/main.tsx` — owned conceptually by `languages/typescript/runtime/entrypoints/main.ts`, which now provides the shipped TypeScript entrypoint summary.
- `references/typescript/src/QueryEngine.ts` — owned conceptually by `languages/typescript/runtime/engine/sessionLoop.ts` plus `languages/typescript/runtime/config/loadRuntimeSummary.ts`, which carry the shipped session-loop orchestration boundary.
- `references/typescript/src/Task.ts` — owned conceptually by the shipped runtime engine/session-loop boundary under `languages/typescript/runtime/engine/` rather than preserved as a direct class port.
- `references/typescript/src/Tool.ts` — owned conceptually by `languages/typescript/runtime/registry/coreTools.ts`, which now carries the shipped base tool registry boundary.
- `references/typescript/src/commands.ts` — owned conceptually by `languages/typescript/runtime/registry/coreCommands.ts`, which now carries the shipped base command registry boundary.
- `references/typescript/src/tools.ts` — owned conceptually by `languages/typescript/runtime/registry/coreTools.ts`, which now carries the shipped base tool registry boundary.
- `references/typescript/src/context.ts` — owned conceptually by `languages/typescript/runtime/context/loadContextState.ts`, which now carries the shipped context-loading boundary.
- `references/typescript/src/history.ts` — owned conceptually by `languages/typescript/runtime/state/sessionState.ts`, which now carries shipped session/export history persistence.
- `references/typescript/src/setup.ts` — owned conceptually by `languages/typescript/runtime/config/loadRuntimeSummary.ts` and `languages/typescript/runtime/entrypoints/main.ts`, which now carry the shipped setup/bootstrap summary boundary.
- `references/typescript/src/query.ts` — owned conceptually by `languages/typescript/runtime/context/loadContextState.ts` and `languages/typescript/runtime/config/loadRuntimeSummary.ts`, which now carry prompt/context ingestion for the shipped runtime.
- `references/typescript/src/tasks.ts` — owned conceptually by the shipped command/engine boundary under `languages/typescript/runtime/registry/coreCommands.ts` and `languages/typescript/runtime/engine/sessionLoop.ts`, rather than preserved as a direct top-level task shell file.
- `references/typescript/src/dialogLaunchers.tsx` — archive-only interactive launcher shell retained for source fidelity; the shipped proving slice does not expose the archived dialog-launcher UI shell as a runtime boundary.
- `references/typescript/src/interactiveHelpers.tsx` — archive-only interactive helper shell retained for source fidelity; the shipped proving slice does not expose the archived interactive UI helper shell as a runtime boundary.
- `references/typescript/src/replLauncher.tsx` — archive-only REPL launcher shell retained for source fidelity; REPL launcher UI/runtime behavior is not part of the shipped proving slice.
- `references/typescript/src/projectOnboardingState.ts` — owned conceptually by the optional `oauth-onboarding` feature-pack direction rather than preserved as a direct root runtime file in the shipped proving slice.
- `references/typescript/src/cost-tracker.ts` — owned conceptually by `languages/typescript/runtime/state/sessionState.ts` and `languages/typescript/runtime/config/loadRuntimeSummary.ts`, which now carry shipped usage and cost accounting.
- `references/typescript/src/costHook.ts` — owned conceptually by `languages/typescript/runtime/config/loadRuntimeSummary.ts`, which now carries shipped cost aggregation during session persistence.

## Shipped-language-pack rule

This subset is complete when each archived root shell row has an explicit shipped-owner or archive-only rationale. The original root runtime shell remains recognizable through the decomposed runtime boundaries even though the monolithic top-level files are no longer shipped one-for-one.
