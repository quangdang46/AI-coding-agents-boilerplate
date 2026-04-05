# TypeScript standalone utils ownership A

This visible standalone utility slice under bead `aicd-3ix.4.14.1.9` covers archived helpers that are not preserved as a one-for-one shipped utility package in the AICD TypeScript proving slice.

## Ownership and disposition

- `references/typescript/src/utils/xml.ts` — archive-only XML escaping helper retained for source fidelity; the shipped proving slice does not expose a dedicated XML utility boundary.
- `references/typescript/src/utils/sessionUrl.ts` — archive-only session resume parsing helper retained for source fidelity; remote/session resume behavior remains outside the current proving slice.
- `references/typescript/src/utils/detectRepository.ts` — archive-only repository detection helper retained for source fidelity; Git/GitHub/runtime integration is covered separately and not by this standalone slice.
- `references/typescript/src/utils/errorLogSink.ts` — archive-only error-log sink helper retained for source fidelity; heavy logging/analytics sinks are not part of the shipped proving slice.
- `references/typescript/src/utils/treeify.ts` — archive-only terminal tree rendering helper retained for source fidelity; UI/rendering helpers are not part of the shipped proving slice.
- `references/typescript/src/utils/generators.ts` — archive-only async generator helper retained for source fidelity; the shipped proving slice does not expose this utility as a dedicated runtime boundary.
- `references/typescript/src/utils/sliceAnsi.ts` — archive-only ANSI slicing helper retained for source fidelity; terminal formatting internals are not part of the shipped proving slice.
- `references/typescript/src/utils/handlePromptSubmit.ts` — archive-only REPL prompt submission orchestrator retained for source fidelity; the shipped proving slice does not carry the archived interactive submission stack.
- `references/typescript/src/utils/sessionActivity.ts` — archive-only session keepalive/activity helper retained for source fidelity; remote keepalive behavior remains outside the shipped proving slice.

## Shipped-language-pack rule

This subset is complete when each archived standalone utility row has an explicit archive-only or shipped-owner rationale. These helpers must not be mistaken for already-shipped TypeScript runtime modules.
