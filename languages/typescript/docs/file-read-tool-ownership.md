# TypeScript file-read tool ownership

The archived `FileReadTool` family in bead `aicd-3ix.4.9.1` is not shipped as a one-for-one preserved tool package, but the current TypeScript proving slice does preserve a narrower `file_read` core-tool contract in `languages/typescript/runtime/registry/coreTools.ts` and `languages/typescript/runtime/utils/toolExecution.ts`.

That shipped contract is intentionally smaller than the archived tool. The proving slice reads configured text context paths and summarizes them through runtime-owned helpers; it does not preserve the archived multimodal prompt/UI, PDF/image/notebook handling, or dynamic read-limit/image-processor machinery as direct shipped modules.

## Ownership and disposition

- `references/typescript/src/tools/FileReadTool/FileReadTool.ts` — owned conceptually by the shipped `file_read` runtime boundary under `languages/typescript/runtime/registry/coreTools.ts`, `languages/typescript/runtime/utils/toolExecution.ts`, and `languages/typescript/runtime/utils/files.ts`, which preserve the proving-slice read contract without shipping the archived monolithic tool implementation directly.
- `references/typescript/src/tools/FileReadTool/prompt.ts` — archive-only prompt module retained for source fidelity; the shipped proving slice preserves read behavior through runtime summaries and registry wiring rather than the archived tool prompt text.
- `references/typescript/src/tools/FileReadTool/UI.tsx` — archive-only UI/result-rendering layer retained for source fidelity; the shipped proving slice does not ship the archived interactive read-tool UI.
- `references/typescript/src/tools/FileReadTool/limits.ts` — archive-only limit-tuning module retained for source fidelity; the shipped proving slice preserves a narrower file-read contract and does not expose the archived token/byte-read limit subsystem as a direct runtime module.
- `references/typescript/src/tools/FileReadTool/imageProcessor.ts` — archive-only image-processing helper retained for source fidelity; the shipped proving slice does not preserve the archived multimodal image-read path as part of the runtime-owned `file_read` boundary.

## Shipped-language-pack rule

This subset is complete when each archived `FileReadTool` row has a concrete shipped owner or an explicit archive-only rationale. The archived multimodal implementation, prompt, UI, and helper modules must not be mistaken for the shipped runtime itself just because the proving slice still preserves a narrower `file_read` contract.
