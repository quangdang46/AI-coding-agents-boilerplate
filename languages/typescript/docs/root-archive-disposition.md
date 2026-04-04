# TypeScript root archive disposition

The archived TypeScript root files below are migration evidence and historical build metadata, not default shipped ownership surfaces for the manifest-backed AICD TypeScript language pack.

## Final disposition

- `references/typescript/README.md` — archive-only product/history document retained for source context; superseded by current AICD repo and language-pack READMEs.
- `references/typescript/FEATURES.md` — archive-only feature-flag audit retained as migration evidence, not as the shipped TypeScript feature contract.
- `references/typescript/CLAUDE.md` — archive-only workflow guidance from the archived workspace, superseded by current repo rules and language-pack docs.
- `references/typescript/package.json` — archive-only package manifest retained for source fidelity; the shipped TypeScript template uses `languages/typescript/template/base/package.json`.
- `references/typescript/tsconfig.json` — archive-only TypeScript compiler snapshot retained for source fidelity; the shipped TypeScript template uses `languages/typescript/template/base/tsconfig.json`.
- `references/typescript/env.d.ts` — archive-only environment typing snapshot retained for source fidelity and migration analysis.
- `references/typescript/scripts/build.ts` — archive-only build pipeline artifact retained as historical evidence; not part of the shipped AICD TypeScript template build.
- `references/typescript/install.sh` — archive-only installer artifact retained for migration context; not part of the shipped AICD TypeScript pack.
- `references/typescript/changes.md` — archive-only changelog/history document retained for source context.
- `references/typescript/assets/screenshot.png` — archive-only screenshot retained as visual evidence, not as shipped runtime material.
- `references/typescript/bun.lock` — archive-only dependency lockfile retained for source fidelity.
- `references/typescript/.gitignore` — archive-only workspace ignore file retained for source fidelity.

## Shipped-language-pack rule

These files may inform migration audits and documentation, but the shipped TypeScript language pack owns its current build/runtime surfaces under `languages/typescript/` and must not depend on the archived root files at runtime.
