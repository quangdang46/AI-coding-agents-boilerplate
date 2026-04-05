# TypeScript standalone utils ownership B

This standalone utility slice under bead `aicd-3ix.4.14.1.9` covers archived helpers that are not preserved as a one-for-one shipped utility package in the AICD TypeScript proving slice.

## Ownership and disposition

- `references/typescript/src/utils/queryHelpers.ts` — archive-only query orchestration helper retained for source fidelity; the shipped proving slice does not expose this internal query helper boundary.
- `references/typescript/src/utils/completionCache.ts` — archive-only shell-completion cache/install helper retained for source fidelity; shell completion installation is not part of the shipped proving slice.
- `references/typescript/src/utils/filePersistence/filePersistence.ts` — archive-only BYOC/cloud file-persistence orchestrator retained for source fidelity; remote persistence remains outside the shipped proving slice.
- `references/typescript/src/utils/filePersistence/outputsScanner.ts` — archive-only file-persistence scanner retained for source fidelity; remote persistence remains outside the shipped proving slice.
- `references/typescript/src/utils/filePersistence/types.ts` — archive-only file-persistence type surface retained for source fidelity; remote persistence remains outside the shipped proving slice.
- `references/typescript/src/utils/collapseBackgroundBashNotifications.ts` — archive-only UI notification-collapsing helper retained for source fidelity; fullscreen/background notification rendering is not part of the shipped proving slice.
- `references/typescript/src/utils/advisor.ts` — archive-only advisor-tool/growthbook helper retained for source fidelity; advisor integration is not part of the shipped proving slice.

## Shipped-language-pack rule

This subset is complete when each archived standalone utility row has an explicit archive-only or shipped-owner rationale. These helpers must not be mistaken for already-shipped TypeScript runtime modules.
