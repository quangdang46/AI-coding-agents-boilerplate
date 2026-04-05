# TypeScript assistant, remote, server, and upstream-proxy ownership

The remaining archived TypeScript special-surface files in bead `aicd-3ix.4.16.1` are not preserved as one-for-one shipped runtime packages in the current AICD TypeScript proving slice.

The shipped TypeScript pack keeps long-term ownership under `languages/typescript/runtime`, while assistant session-history UX, structured remote transport, bridge-style remote control, direct-connect server helpers, and upstream proxy behavior remain optional feature-pack work. The final disposition for this subset is therefore future feature-pack ownership, not shipped core runtime ownership.

## Ownership and disposition

- `references/typescript/src/assistant/AssistantSessionChooser.tsx` — future feature-pack ownership under the declared `assistant-session-history` family; archived session-chooser UI is not part of the current shipped TypeScript proving slice.
- `references/typescript/src/assistant/sessionHistory.ts` — future feature-pack ownership under the declared `assistant-session-history` family; archived assistant session history remains outside shipped core resume/session ownership.
- `references/typescript/src/upstreamproxy/upstreamproxy.ts` — future feature-pack ownership under the declared `upstream-proxy` family; archived upstream proxy behavior is not shipped as part of the core TypeScript runtime.
- `references/typescript/src/upstreamproxy/relay.ts` — future feature-pack ownership under the declared `upstream-proxy` family; relay behavior remains optional remote runtime infrastructure rather than shipped core ownership.
- `references/typescript/src/remote/remotePermissionBridge.ts` — future feature-pack ownership under the declared `bridge-remote-control` family; remote permission bridging remains optional advanced runtime behavior.
- `references/typescript/src/remote/RemoteSessionManager.ts` — future feature-pack ownership under the declared `bridge-remote-control` family; archived remote session management is not part of the shipped proving slice.
- `references/typescript/src/remote/sdkMessageAdapter.ts` — future feature-pack ownership under the declared `structured-remote-transport` family; SDK message adaptation remains optional remote transport infrastructure.
- `references/typescript/src/remote/SessionsWebSocket.ts` — future feature-pack ownership under the declared `structured-remote-transport` family; WebSocket-backed remote sessions are not part of the shipped TypeScript core runtime.
- `references/typescript/src/server/createDirectConnectSession.ts` — future feature-pack ownership under the declared `direct-connect-server` family; direct-connect session creation remains optional server/runtime behavior.
- `references/typescript/src/server/types.ts` — future feature-pack ownership under the declared `direct-connect-server` family; archived direct-connect server types are not preserved as shipped core runtime types.
- `references/typescript/src/server/directConnectManager.ts` — future feature-pack ownership under the declared `direct-connect-server` family; direct-connect management remains outside the current TypeScript proving slice.

## Shipped-language-pack rule

This subset is complete when each archived assistant, remote, server, and upstream-proxy row has a clear future feature-pack owner or an explicit archive-only rationale. These archived special-surface files must not be mistaken for already-shipped TypeScript runtime ownership just because the repository preserves their feature-pack taxonomy.
