# TypeScript type-contract ownership

The archived TypeScript `src/types/*` package in bead `aicd-3ix.4.4.1.1` carries snapshot-era type contracts and generated event typings that are not preserved as a one-for-one shipped runtime package in the current AICD TypeScript proving slice.

The shipped TypeScript pack preserves the base config and registry contract needed by the proving slice, but it does not currently expose the archived `src/types/*` subtree as a dedicated language-pack runtime boundary. These files therefore remain archive-only unless and until a future feature-pack or runtime extraction explicitly adopts them.

## Ownership and disposition

- `references/typescript/src/types/permissions.ts` — archive-only permission type surface retained for source fidelity; shipped permission behavior is owned by the proving-slice runtime/config boundary rather than this standalone snapshot type file.
- `references/typescript/src/types/generated/events_mono/growthbook/v1/growthbook_experiment_event.ts` — archive-only generated Growthbook event contract retained for source fidelity; analytics and experiment event surfaces are not part of the shipped proving slice.
- `references/typescript/src/types/generated/events_mono/common/v1/auth.ts` — archive-only generated auth event contract retained for source fidelity; generated event-contract packages are not shipped as runtime boundaries in the proving slice.
- `references/typescript/src/types/generated/events_mono/claude_code/v1/claude_code_internal_event.ts` — archive-only generated internal event contract retained for source fidelity; internal telemetry/event plumbing is outside the shipped proving slice.
- `references/typescript/src/types/generated/google/protobuf/timestamp.ts` — archive-only generated protobuf timestamp helper retained for source fidelity; the shipped proving slice does not expose generated protobuf contract code as a dedicated runtime module.
- `references/typescript/src/types/textInputTypes.ts` — archive-only text-input type helper retained for source fidelity; snapshot-era prompt-input typing is not shipped as a standalone runtime boundary.
- `references/typescript/src/types/connectorText.ts` — archive-only connector text contract retained for source fidelity; product-specific connector messaging remains outside the shipped proving slice.
- `references/typescript/src/types/hooks.ts` — archive-only hook contract retained for source fidelity; advanced hook-runtime feature work remains outside the shipped proving slice.
- `references/typescript/src/types/logs.ts` — archive-only log contract retained for source fidelity; snapshot-era logging type surfaces are not shipped as dedicated runtime modules.
- `references/typescript/src/types/command.ts` — archive-only command type contract retained for source fidelity; shipped command behavior is owned by the runtime registry and proving-slice command surfaces rather than this standalone snapshot type file.
- `references/typescript/src/types/plugin.ts` — archive-only plugin type contract retained for source fidelity; richer plugin runtime/type surfaces remain outside the proving slice.
- `references/typescript/src/types/ids.ts` — archive-only id helper contract retained for source fidelity; the shipped proving slice does not expose this snapshot type helper as a dedicated runtime boundary.

## Shipped-language-pack rule

This subset is complete when each archived type-contract row has an explicit archive-only or shipped-owner rationale. These snapshot-era type files must not be mistaken for already-shipped TypeScript runtime modules.
