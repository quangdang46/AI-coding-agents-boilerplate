# TypeScript bridge ownership

The archived TypeScript `src/bridge/*` subtree in bead `aicd-3ix.4.6.1` is not preserved as a one-for-one shipped runtime package in the current AICD TypeScript proving slice.

The shipped TypeScript pack keeps long-term ownership under `languages/typescript/runtime`, while bridge, remote-control, direct-connect, and relay behavior remain optional integration work. The final disposition for this cluster is therefore future feature-pack ownership under the declared `bridge-remote-control` family rather than shipped core runtime ownership.

## Ownership and disposition

- `references/typescript/src/bridge/debugUtils.ts` — future feature-pack ownership under the declared `bridge-remote-control` family; archived bridge debugging support is not part of the shipped TypeScript core runtime.
- `references/typescript/src/bridge/bridgeStatusUtil.ts` — future feature-pack ownership under the declared `bridge-remote-control` family; bridge status helpers remain optional remote-control behavior.
- `references/typescript/src/bridge/pollConfigDefaults.ts` — future feature-pack ownership under the declared `bridge-remote-control` family; bridge polling defaults remain outside the shipped proving slice.
- `references/typescript/src/bridge/remoteBridgeCore.ts` — future feature-pack ownership under the declared `bridge-remote-control` family; remote bridge orchestration is not shipped in the current TypeScript runtime.
- `references/typescript/src/bridge/bridgeMessaging.ts` — future feature-pack ownership under the declared `bridge-remote-control` family; bridge messaging remains optional remote-session infrastructure.
- `references/typescript/src/bridge/bridgeUI.ts` — future feature-pack ownership under the declared `bridge-remote-control` family; bridge-specific UI remains outside the shipped core runtime boundary.
- `references/typescript/src/bridge/workSecret.ts` — future feature-pack ownership under the declared `bridge-remote-control` family; bridge secret management remains optional integration behavior.
- `references/typescript/src/bridge/jwtUtils.ts` — future feature-pack ownership under the declared `bridge-remote-control` family; bridge token helpers are not shipped as part of the core TypeScript runtime.
- `references/typescript/src/bridge/replBridge.ts` — future feature-pack ownership under the declared `bridge-remote-control` family; REPL bridge control remains optional bridge/runtime behavior.
- `references/typescript/src/bridge/trustedDevice.ts` — future feature-pack ownership under the declared `bridge-remote-control` family; trusted-device bridge logic remains outside the proving slice.
- `references/typescript/src/bridge/sessionRunner.ts` — future feature-pack ownership under the declared `bridge-remote-control` family; bridge session execution remains optional remote-session behavior.
- `references/typescript/src/bridge/sessionIdCompat.ts` — future feature-pack ownership under the declared `bridge-remote-control` family; compatibility helpers for bridge sessions are not core runtime ownership.
- `references/typescript/src/bridge/codeSessionApi.ts` — future feature-pack ownership under the declared `bridge-remote-control` family; bridge-facing session APIs remain optional integration surfaces.
- `references/typescript/src/bridge/bridgeEnabled.ts` — future feature-pack ownership under the declared `bridge-remote-control` family; bridge enablement remains optional advanced-runtime behavior.
- `references/typescript/src/bridge/createSession.ts` — future feature-pack ownership under the declared `bridge-remote-control` family; bridge session creation remains part of optional remote-control work.
- `references/typescript/src/bridge/pollConfig.ts` — future feature-pack ownership under the declared `bridge-remote-control` family; bridge polling configuration is not part of the shipped core runtime.
- `references/typescript/src/bridge/types.ts` — future feature-pack ownership under the declared `bridge-remote-control` family; archived bridge types are not preserved one-for-one as shipped runtime types.
- `references/typescript/src/bridge/replBridgeHandle.ts` — future feature-pack ownership under the declared `bridge-remote-control` family; REPL bridge handle management remains optional remote control behavior.
- `references/typescript/src/bridge/initReplBridge.ts` — future feature-pack ownership under the declared `bridge-remote-control` family; bridge bootstrapping remains outside the shipped proving slice.
- `references/typescript/src/bridge/inboundMessages.ts` — future feature-pack ownership under the declared `bridge-remote-control` family; inbound bridge message handling remains optional remote-session infrastructure.
- `references/typescript/src/bridge/flushGate.ts` — future feature-pack ownership under the declared `bridge-remote-control` family; bridge flush coordination remains optional transport/runtime behavior.
- `references/typescript/src/bridge/bridgePointer.ts` — future feature-pack ownership under the declared `bridge-remote-control` family; bridge pointer bookkeeping remains outside shipped core ownership.
- `references/typescript/src/bridge/bridgeConfig.ts` — future feature-pack ownership under the declared `bridge-remote-control` family; bridge configuration stays with optional bridge integration work.
- `references/typescript/src/bridge/bridgeMain.ts` — future feature-pack ownership under the declared `bridge-remote-control` family; the main bridge runtime entry remains outside the shipped TypeScript proving slice.
- `references/typescript/src/bridge/envLessBridgeConfig.ts` — future feature-pack ownership under the declared `bridge-remote-control` family; bridge configuration variants remain optional remote-control behavior.
- `references/typescript/src/bridge/bridgeApi.ts` — future feature-pack ownership under the declared `bridge-remote-control` family; bridge API wiring is not part of the shipped core runtime.
- `references/typescript/src/bridge/bridgePermissionCallbacks.ts` — future feature-pack ownership under the declared `bridge-remote-control` family; bridge permission callbacks remain optional advanced-runtime behavior.
- `references/typescript/src/bridge/inboundAttachments.ts` — future feature-pack ownership under the declared `bridge-remote-control` family; bridge attachment handling remains optional remote-session infrastructure.
- `references/typescript/src/bridge/capacityWake.ts` — future feature-pack ownership under the declared `bridge-remote-control` family; bridge wake/capacity signaling remains outside the shipped proving slice.
- `references/typescript/src/bridge/replBridgeTransport.ts` — future feature-pack ownership under the declared `bridge-remote-control` family; bridge transport wiring remains optional integration/runtime behavior.
- `references/typescript/src/bridge/bridgeDebug.ts` — future feature-pack ownership under the declared `bridge-remote-control` family; archived bridge debug helpers are not shipped as core runtime ownership.

## Shipped-language-pack rule

This cluster is complete when each archived bridge row has a clear future feature-pack owner or an explicit archive-only rationale. The archived TypeScript bridge subtree must not be mistaken for already-shipped TypeScript runtime ownership just because the repository preserves bridge-related feature-pack taxonomy and command classification.
