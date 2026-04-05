# TypeScript claude-in-chrome and secure-storage ownership

The archived helper files in bead `aicd-3ix.4.16.1.2` are not preserved as a one-for-one shipped runtime subtree in the current AICD TypeScript proving slice.

The shipped TypeScript language pack does not currently expose a `claudeInChrome` browser-control boundary or a dedicated `secureStorage` runtime package under `languages/typescript/`. These archived helpers therefore remain archive-only unless future browser-control, auth-storage, or plugin/runtime feature work explicitly adopts them.

## Ownership and disposition

- `references/typescript/src/utils/secureStorage/keychainPrefetch.ts` — archive-only secure-storage prefetch helper retained for source fidelity; the shipped proving slice does not expose a dedicated secure-storage runtime boundary.
- `references/typescript/src/utils/secureStorage/fallbackStorage.ts` — archive-only fallback-storage helper retained for source fidelity; browser/auth storage fallback behavior is not part of the shipped TypeScript proving slice.
- `references/typescript/src/utils/secureStorage/plainTextStorage.ts` — archive-only plain-text storage helper retained for source fidelity; the shipped proving slice does not ship this storage helper as a runtime module.
- `references/typescript/src/utils/secureStorage/index.ts` — archive-only secure-storage package entry retained for source fidelity; secure-storage runtime packaging is outside the shipped proving slice.
- `references/typescript/src/utils/secureStorage/macOsKeychainStorage.ts` — archive-only macOS keychain storage helper retained for source fidelity; native keychain integration is not part of the shipped TypeScript proving slice.
- `references/typescript/src/utils/secureStorage/macOsKeychainHelpers.ts` — archive-only macOS keychain helper retained for source fidelity; native secure-storage helpers remain outside the shipped proving slice.
- `references/typescript/src/utils/claudeInChrome/package.ts` — archive-only claude-in-chrome packaging helper retained for source fidelity; the shipped proving slice does not expose a browser-control runtime package.
- `references/typescript/src/utils/claudeInChrome/setup.ts` — archive-only browser-control setup helper retained for source fidelity; Chrome-host setup behavior is not part of the shipped proving slice.
- `references/typescript/src/utils/claudeInChrome/toolRendering.tsx` — archive-only browser-control rendering helper retained for source fidelity; the shipped proving slice does not expose this UI/runtime helper as a standalone module.
- `references/typescript/src/utils/claudeInChrome/prompt.ts` — archive-only claude-in-chrome prompt helper retained for source fidelity; browser-control prompt behavior is not part of the shipped proving slice.
- `references/typescript/src/utils/claudeInChrome/setupPortable.ts` — archive-only portable browser-control setup helper retained for source fidelity; portable Chrome-host setup remains outside the shipped proving slice.
- `references/typescript/src/utils/claudeInChrome/common.ts` — archive-only shared browser-control helper retained for source fidelity; the proving slice does not ship a dedicated claude-in-chrome helper boundary.
- `references/typescript/src/utils/claudeInChrome/mcpServer.ts` — archive-only browser-control MCP helper retained for source fidelity; advanced browser/MCP integration work remains outside the shipped proving slice.
- `references/typescript/src/utils/claudeInChrome/chromeNativeHost.ts` — archive-only Chrome native-host helper retained for source fidelity; native browser-host integration is not part of the shipped TypeScript proving slice.

## Shipped-language-pack rule

This subset is complete when each archived browser/auth-storage row has an explicit archive-only or shipped-owner rationale. These snapshot-era helpers must not be mistaken for already-shipped TypeScript runtime modules.
