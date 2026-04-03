# TypeScript Language Pack Config Reference

The TypeScript language pack currently uses `boilerplate.config.ts` as its base config file.

The generated-project template lives under `languages/typescript/template/base/`.

In the current migration slice, TypeScript supports manifest-driven `init`, `feature add/remove`, and `doctor`.

Generated TypeScript projects include provider and model selection through `defaultProvider` plus the `providers` map in `boilerplate.config.ts`.

Generated TypeScript projects also include permission and sandbox controls through `tools.policy`, including `approvalMode` and `deny`.

Generated TypeScript projects also reserve `.agent/sessions/` as the local persisted-session root for future session and resume flows.

Generated TypeScript projects also reserve `.agent/context/` as the local workspace-context root, and `boilerplate.config.ts` wires that root into `prompts.contextPaths`.

Generated TypeScript projects also reserve `.agent/usage/` as the local usage and cost tracking root for future accounting flows.

Generated TypeScript projects also expose local agent discovery through the `agents` config section, including directories, enabled agents, and `defaultAgent`.
