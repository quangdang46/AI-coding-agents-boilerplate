# TypeScript utils runtime-helper ownership

This standalone utility slice under bead `aicd-3ix.4.14.1.9` covers archived runtime-helper modules that are not preserved as a one-for-one shipped utility package in the AICD TypeScript proving slice.

## Ownership and disposition

- `references/typescript/src/utils/exampleCommands.ts` — archive-only interactive suggestion helper retained for source fidelity; the shipped proving slice does not expose example-command generation as a standalone runtime helper.
- `references/typescript/src/utils/memoize.ts` — archive-only general-purpose caching helper retained for source fidelity; the shipped proving slice does not currently expose this utility as a dedicated runtime boundary.
- `references/typescript/src/utils/set.ts` — archive-only collection helper retained for source fidelity; the shipped proving slice does not expose this utility as a dedicated runtime boundary.
- `references/typescript/src/utils/memory/versions.ts` — archive-only memory/git integration helper retained for source fidelity; shipped memory feature work remains outside the current proving slice.
- `references/typescript/src/utils/memory/types.ts` — archive-only memory type helper retained for source fidelity; shipped memory feature work remains outside the current proving slice.
- `references/typescript/src/utils/cronScheduler.ts` — archive-only scheduler helper retained for source fidelity; shipped cron behavior belongs to optional scheduling/feature-pack work rather than the proving slice.
- `references/typescript/src/utils/ripgrep.ts` — archive-only ripgrep runtime helper retained for source fidelity; vendored ripgrep/runtime-search behavior is not part of the shipped proving slice.

## Shipped-language-pack rule

This subset is complete when each archived runtime-helper row has an explicit archive-only or shipped-owner rationale. These standalone helpers must not be mistaken for already-shipped TypeScript runtime modules.
