# TypeScript computer-use ownership

The archived helper files in bead `aicd-3ix.4.16.1.1` are not preserved as a one-for-one shipped runtime subtree in the current AICD TypeScript proving slice.

The shipped TypeScript language pack does not currently expose a dedicated `computerUse` runtime boundary under `languages/typescript/`. Browser-control and computer-use execution therefore remain outside the proving slice unless future browser-control, MCP, or remote-control feature work explicitly adopts these files.

## Ownership and disposition

- `references/typescript/src/utils/computerUse/setup.ts` — archive-only computer-use setup helper retained for source fidelity; the shipped proving slice does not expose a dedicated computer-use setup runtime boundary.
- `references/typescript/src/utils/computerUse/wrapper.tsx` — archive-only computer-use wrapper retained for source fidelity; interactive computer-use UI/runtime wrappers remain outside the shipped proving slice.
- `references/typescript/src/utils/computerUse/toolRendering.tsx` — archive-only computer-use rendering helper retained for source fidelity; the shipped proving slice does not expose this UI/runtime helper as a standalone module.
- `references/typescript/src/utils/computerUse/appNames.ts` — archive-only computer-use app-name helper retained for source fidelity; application-specific computer-use mapping is outside the shipped proving slice.
- `references/typescript/src/utils/computerUse/swiftLoader.ts` — archive-only native computer-use loader retained for source fidelity; native computer-use integration is not part of the shipped proving slice.
- `references/typescript/src/utils/computerUse/hostAdapter.ts` — archive-only computer-use host adapter retained for source fidelity; host-adapter wiring remains outside the shipped proving slice.
- `references/typescript/src/utils/computerUse/cleanup.ts` — archive-only computer-use cleanup helper retained for source fidelity; cleanup orchestration for computer-use sessions is outside the shipped proving slice.
- `references/typescript/src/utils/computerUse/inputLoader.ts` — archive-only computer-use input loader retained for source fidelity; input-loader behavior is not part of the shipped proving slice.
- `references/typescript/src/utils/computerUse/computerUseLock.ts` — archive-only computer-use lock helper retained for source fidelity; runtime locking for computer-use workflows remains outside the shipped proving slice.
- `references/typescript/src/utils/computerUse/gates.ts` — archive-only computer-use gating helper retained for source fidelity; gating logic for browser-control workflows is not part of the shipped proving slice.
- `references/typescript/src/utils/computerUse/common.ts` — archive-only shared computer-use helper retained for source fidelity; the proving slice does not ship a dedicated computer-use helper boundary.
- `references/typescript/src/utils/computerUse/executor.ts` — archive-only computer-use executor retained for source fidelity; computer-use execution remains outside the shipped TypeScript proving slice.
- `references/typescript/src/utils/computerUse/drainRunLoop.ts` — archive-only computer-use run-loop helper retained for source fidelity; run-loop draining for computer-use workflows is outside the shipped proving slice.
- `references/typescript/src/utils/computerUse/mcpServer.ts` — archive-only computer-use MCP helper retained for source fidelity; advanced MCP/browser-control integration remains outside the shipped proving slice.
- `references/typescript/src/utils/computerUse/escHotkey.ts` — archive-only computer-use hotkey helper retained for source fidelity; keyboard-control behavior for computer-use workflows is not part of the shipped proving slice.

## Shipped-language-pack rule

This subset is complete when each archived computer-use row has an explicit archive-only or shipped-owner rationale. These snapshot-era computer-use helper files must not be mistaken for already-shipped TypeScript runtime modules.
