# TypeScript bootstrap, entrypoint, context, and state ownership

The archived startup-and-state files in bead `aicd-3ix.4.3.1` are not preserved as a one-for-one shipped package tree in the current AICD TypeScript proving slice. Instead, the shipped TypeScript pack keeps the base runtime entrypoint, context loading, and session persistence under `languages/typescript/runtime/`, while interactive React context/state scaffolding and optional integration-specific entrypoint contracts remain outside the base proving slice.

## Ownership and disposition

- `references/typescript/src/entrypoints/mcp.ts` — owned conceptually by the optional `mcp-integration` feature-pack direction rather than preserved as a direct shipped entrypoint in the proving slice.
- `references/typescript/src/entrypoints/init.ts` — owned conceptually by `languages/typescript/runtime/entrypoints/main.ts` and `languages/typescript/runtime/config/loadRuntimeSummary.ts`, which now carry the shipped initialization boundary.
- `references/typescript/src/entrypoints/agentSdkTypes.ts` — future or optional feature-pack ownership aligned to integration-facing SDK work rather than the shipped proving-slice runtime.
- `references/typescript/src/entrypoints/cli.tsx` — archive-only interactive CLI entrypoint retained for source fidelity; the shipped proving slice uses the decomposed runtime entrypoint under `languages/typescript/runtime/entrypoints/` instead of this archived UI-oriented shell.
- `references/typescript/src/entrypoints/sdk/coreTypes.ts` — future or optional feature-pack ownership aligned to integration-facing SDK work rather than the shipped proving-slice runtime.
- `references/typescript/src/entrypoints/sdk/coreTypes.generated.ts` — future or optional feature-pack ownership aligned to integration-facing SDK work rather than the shipped proving-slice runtime.
- `references/typescript/src/entrypoints/sdk/controlSchemas.ts` — future or optional feature-pack ownership aligned to `mcp-integration` and adjacent SDK-facing integration work rather than the shipped proving-slice runtime.
- `references/typescript/src/entrypoints/sdk/toolTypes.ts` — future or optional feature-pack ownership aligned to `mcp-integration` and `lsp-tooling` rather than the shipped proving-slice runtime.
- `references/typescript/src/entrypoints/sdk/runtimeTypes.ts` — future or optional feature-pack ownership aligned to integration-facing SDK work rather than the shipped proving-slice runtime.
- `references/typescript/src/entrypoints/sdk/coreSchemas.ts` — future or optional feature-pack ownership aligned to integration-facing SDK work rather than the shipped proving-slice runtime.
- `references/typescript/src/entrypoints/sandboxTypes.ts` — archive-only sandbox contract retained for source fidelity; the shipped proving slice exposes sandbox behavior through runtime config and command summaries rather than this archived type surface.
- `references/typescript/src/context/fpsMetrics.tsx` — archive-only React context/UI helper retained for source fidelity; performance overlay UI is not part of the shipped proving slice.
- `references/typescript/src/context/mailbox.tsx` — archive-only mailbox context retained for source fidelity; collaboration-oriented UI context remains outside the shipped proving slice.
- `references/typescript/src/context/overlayContext.tsx` — archive-only overlay context retained for source fidelity; interactive overlay UI state is not part of the shipped proving slice.
- `references/typescript/src/context/stats.tsx` — archive-only stats context retained for source fidelity; runtime stats are exposed through summary/command surfaces rather than this archived React context.
- `references/typescript/src/context/promptOverlayContext.tsx` — archive-only prompt overlay context retained for source fidelity; prompt-overlay UI state remains outside the shipped proving slice.
- `references/typescript/src/context/QueuedMessageContext.tsx` — archive-only queued-message context retained for source fidelity; interactive queued-message UI state is not part of the shipped proving slice.
- `references/typescript/src/context/notifications.tsx` — archive-only notifications context retained for source fidelity; runtime notifications UI remains outside the shipped proving slice.
- `references/typescript/src/context/modalContext.tsx` — archive-only modal context retained for source fidelity; modal UI state is not part of the shipped proving slice.
- `references/typescript/src/context/voice.tsx` — archive-only voice context retained for source fidelity; voice-mode UI/runtime state is not part of the shipped proving slice.
- `references/typescript/src/state/AppState.tsx` — archive-only app-state provider retained for source fidelity; the shipped proving slice uses runtime session persistence rather than this archived app-state tree.
- `references/typescript/src/state/store.ts` — archive-only app-state store retained for source fidelity; base shipped state ownership lives under `languages/typescript/runtime/state/sessionState.ts`.
- `references/typescript/src/state/onChangeAppState.ts` — archive-only app-state change helper retained for source fidelity; interactive app-state orchestration is not part of the shipped proving slice.
- `references/typescript/src/state/teammateViewHelpers.ts` — archive-only teammate-view helper retained for source fidelity; teammate UI state remains outside the shipped proving slice.
- `references/typescript/src/state/AppStateStore.ts` — archive-only app-state store model retained for source fidelity; base shipped state ownership lives under `languages/typescript/runtime/state/sessionState.ts`.
- `references/typescript/src/state/selectors.ts` — archive-only app-state selector helpers retained for source fidelity; selector-based UI state remains outside the shipped proving slice.
- `references/typescript/src/bootstrap/state.ts` — owned conceptually by `languages/typescript/runtime/state/sessionState.ts` and `languages/typescript/runtime/config/loadRuntimeSummary.ts`, which now carry the shipped startup/session persistence boundary.

## Shipped-language-pack rule

This subset is complete when each archived startup/state row has an explicit archive-only or shipped-owner rationale. The decomposed TypeScript runtime keeps the core startup, context, and session boundaries without shipping the archived React-heavy state tree one-for-one.
