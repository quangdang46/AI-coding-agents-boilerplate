# TypeScript task-output ownership

The archived TypeScript `src/utils/task/*` files in this slice are task-runtime support internals from the source snapshot. They are not currently shipped as a standalone `languages/typescript/runtime/utils/task/` boundary in the AICD TypeScript proving slice.

## Ownership and disposition

- `references/typescript/src/utils/task/outputFormatting.ts` — archive-only task-output formatting helper retained for source fidelity; the shipped proving slice does not expose a dedicated task-output formatting module.
- `references/typescript/src/utils/task/TaskOutput.ts` — archive-only task-output buffering/polling helper retained for source fidelity; the shipped proving slice does not expose a dedicated task-output runtime module.
- `references/typescript/src/utils/task/sdkProgress.ts` — archive-only SDK progress helper retained for source fidelity; the shipped proving slice does not expose a dedicated task-progress event helper.
- `references/typescript/src/utils/task/framework.ts` — archive-only task framework helper retained for source fidelity; active task-runtime behavior is covered separately under task-specific coverage, not by this utility bead.
- `references/typescript/src/utils/task/diskOutput.ts` — archive-only task disk-output helper retained for source fidelity; the shipped proving slice does not expose a dedicated task-output persistence module.

## Shipped-language-pack rule

This subset is complete when each archived task-output row has an explicit archive-only or shipped-owner rationale. These snapshot-era task internals must not be mistaken for already-shipped TypeScript runtime modules.
