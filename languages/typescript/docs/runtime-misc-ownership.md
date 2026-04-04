# TypeScript runtime miscellany ownership

The remaining uncovered top-level TypeScript support files in bead `aicd-3ix.4.18` are owned by the shipped TypeScript language-pack runtime and generated template surfaces, not by the archived source tree.

## Ownership map

- `references/typescript/src/tools/utils.ts` — owned by `languages/typescript/runtime/utils/toolExecution.ts` as the shipped core-tool execution helper for transient tool state and result shaping.
- `references/typescript/src/tools/testing/TestingPermissionTool.tsx` — explicitly excluded from shipped runtime ownership; it is a testing-only archive artifact and not part of the manifest-backed TypeScript pack contract.
- `references/typescript/src/ink.ts` — owned by `languages/typescript/runtime/entrypoints/main.ts` and the generated template `languages/typescript/template/base/src/index.ts` as the shipped TypeScript interactive/runtime shell surface.
- `references/typescript/src/outputStyles/loadOutputStylesDir.ts` — owned by `languages/typescript/runtime/context/loadContextState.ts` and template prompt/context loading behavior as the shipped markdown/config context ingestion seam.
- `references/typescript/src/coordinator/coordinatorMode.ts` — explicitly excluded from the shipped core slice; coordinator/swarm behavior belongs to optional feature-pack work rather than base runtime ownership.
- `references/typescript/src/vendor/ripgrep/x64-linux/rg` — explicitly excluded from shipped runtime ownership; vendored archive binaries are evidence-only and the generated TypeScript pack does not ship archived binaries.

## Shipped-language-pack rule

This cluster is complete when each archived row has either a concrete shipped TypeScript owner or an explicit exclusion rationale. Archive-only testing helpers, optional coordinator behavior, and vendored binaries must not be mistaken for required base-runtime extraction work.
