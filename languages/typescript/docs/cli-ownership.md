# TypeScript CLI ownership

The archived TypeScript `src/cli/*` subtree in bead `aicd-3ix.4.5.1` is not preserved as a one-for-one shipped runtime package in the current AICD TypeScript proving slice.

The shipped TypeScript pack keeps long-term ownership under `languages/typescript/runtime`, while the archived CLI tree remains migration evidence under `references/`. The final disposition for this cluster is therefore a mix of future feature-pack ownership for remote or integration-heavy transport helpers and explicit archive-only treatment for snapshot-era CLI support files that the proving slice does not ship directly.

## Ownership and disposition

- `references/typescript/src/cli/ndjsonSafeStringify.ts` — archive-only CLI support helper retained for source fidelity; the current TypeScript proving slice does not ship this archived formatting utility as a standalone runtime module.
- `references/typescript/src/cli/transports/transportUtils.ts` — future feature-pack ownership under the declared structured remote transport family; transport helpers remain outside the shipped core runtime slice.
- `references/typescript/src/cli/transports/WebSocketTransport.ts` — future feature-pack ownership under the declared structured remote transport family; WebSocket transport is not part of the current shipped TypeScript runtime boundary.
- `references/typescript/src/cli/transports/ccrClient.ts` — future feature-pack ownership under the declared structured remote transport family; remote client transport remains optional advanced transport behavior.
- `references/typescript/src/cli/transports/SSETransport.ts` — future feature-pack ownership under the declared structured remote transport family; SSE transport is not part of the current shipped TypeScript proving slice.
- `references/typescript/src/cli/transports/HybridTransport.ts` — future feature-pack ownership under the declared structured remote transport family; hybrid remote transport is not part of the current shipped core runtime.
- `references/typescript/src/cli/transports/SerialBatchEventUploader.ts` — future feature-pack ownership under the declared structured remote transport family; archived event-upload transport helpers remain outside shipped core ownership.
- `references/typescript/src/cli/transports/WorkerStateUploader.ts` — future feature-pack ownership under the declared structured remote transport family; worker-state upload behavior remains optional advanced transport infrastructure.
- `references/typescript/src/cli/structuredIO.ts` — future feature-pack ownership under the declared structured remote transport family; structured remote IO is not a shipped TypeScript proving-slice runtime boundary.
- `references/typescript/src/cli/handlers/agents.ts` — future feature-pack ownership under the declared multi-agent workflow family; archived CLI agent handling is not shipped as a core runtime package.
- `references/typescript/src/cli/handlers/autoMode.ts` — future feature-pack ownership under optional advanced workflow behavior; auto-mode CLI orchestration is not part of the current shipped proving slice.
- `references/typescript/src/cli/handlers/mcp.tsx` — future or optional feature-pack ownership aligned to `mcp-integration`; the archived handler is not preserved one-for-one as a shipped runtime owner.
- `references/typescript/src/cli/handlers/plugins.ts` — future or optional feature-pack ownership aligned to `plugin-runtime` and `plugin-marketplace-ui`; the archived handler is not a shipped core runtime boundary.
- `references/typescript/src/cli/handlers/util.tsx` — archive-only CLI handler support file retained for source fidelity; the proving slice does not expose this archived React-oriented helper as a standalone runtime module.
- `references/typescript/src/cli/handlers/auth.ts` — future or optional feature-pack ownership aligned to `oauth-onboarding`; the archived auth handler is not preserved one-for-one as a shipped runtime owner.
- `references/typescript/src/cli/print.ts` — archive-only CLI presentation helper retained for source fidelity; the proving slice does not ship the archived print helper as a dedicated runtime module.
- `references/typescript/src/cli/exit.ts` — archive-only CLI exit helper retained for source fidelity; current command behavior is summarized through shipped registry surfaces rather than this archived helper file.
- `references/typescript/src/cli/update.ts` — archive-only snapshot-specific installer/update helper retained for source fidelity; snapshot-era update behavior is explicitly outside the current TypeScript language-pack contract.
- `references/typescript/src/cli/remoteIO.ts` — future feature-pack ownership under the declared structured remote transport family; remote IO remains optional advanced transport behavior rather than shipped core runtime ownership.

## Shipped-language-pack rule

This cluster is complete when each archived CLI row has either a clear future feature-pack owner or an explicit archive-only rationale. The archived TypeScript CLI transport and handler files must not be mistaken for already-shipped TypeScript runtime ownership just because adjacent feature-pack families or summarized command surfaces exist.
