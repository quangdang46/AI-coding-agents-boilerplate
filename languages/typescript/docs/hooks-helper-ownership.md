# TypeScript hook-helper ownership

The archived helper files in bead `aicd-3ix.4.14.1.6` are not preserved as a one-for-one shipped runtime subtree in the current AICD TypeScript proving slice.

The shipped TypeScript language pack currently exposes only a summary-level `hooks` command through `languages/typescript/runtime/registry/coreCommands.ts`; it does not ship the archived `src/utils/hooks/*` helper stack as a dedicated `languages/typescript/runtime/hooks/` boundary. Advanced hook runtime behavior therefore remains outside the proving slice unless future hook-runtime feature work explicitly adopts these files.

## Ownership and disposition

- `references/typescript/src/utils/hooks/apiQueryHookHelper.ts` — archive-only hook query helper retained for source fidelity; the shipped proving slice does not expose this helper as a standalone runtime module.
- `references/typescript/src/utils/hooks/hooksConfigSnapshot.ts` — archive-only hook-config snapshot helper retained for source fidelity; snapshot-era hook configuration state is outside the shipped proving slice.
- `references/typescript/src/utils/hooks/sessionHooks.ts` — archive-only session-hook helper retained for source fidelity; session hook orchestration remains outside the shipped TypeScript proving slice.
- `references/typescript/src/utils/hooks/fileChangedWatcher.ts` — archive-only hook file-watcher helper retained for source fidelity; hook-driven file watching is not part of the shipped proving slice.
- `references/typescript/src/utils/hooks/hooksConfigManager.ts` — archive-only hook-config manager retained for source fidelity; the shipped proving slice does not ship a dedicated hook configuration boundary.
- `references/typescript/src/utils/hooks/hookEvents.ts` — archive-only hook event helper retained for source fidelity; hook event plumbing remains outside the shipped proving slice.
- `references/typescript/src/utils/hooks/execPromptHook.ts` — archive-only prompt-hook execution helper retained for source fidelity; advanced prompt-hook runtime behavior is not part of the shipped proving slice.
- `references/typescript/src/utils/hooks/execHttpHook.ts` — archive-only HTTP-hook execution helper retained for source fidelity; HTTP hook integrations remain outside the shipped proving slice.
- `references/typescript/src/utils/hooks/ssrfGuard.ts` — archive-only hook SSRF guard retained for source fidelity; the shipped proving slice does not ship this hook-specific network guard as a runtime module.
- `references/typescript/src/utils/hooks/postSamplingHooks.ts` — archive-only post-sampling hook helper retained for source fidelity; post-sampling hook orchestration remains outside the shipped proving slice.
- `references/typescript/src/utils/hooks/hookHelpers.ts` — archive-only shared hook helper retained for source fidelity; the shipped TypeScript pack does not expose this helper boundary.
- `references/typescript/src/utils/hooks/AsyncHookRegistry.ts` — archive-only async hook registry retained for source fidelity; hook registry/runtime coordination remains outside the shipped proving slice.
- `references/typescript/src/utils/hooks/hooksSettings.ts` — archive-only hook settings helper retained for source fidelity; snapshot-era hook settings are not part of the shipped proving slice.
- `references/typescript/src/utils/hooks/skillImprovement.ts` — archive-only hook-driven skill-improvement helper retained for source fidelity; skill-improvement hook workflows remain outside the shipped proving slice.
- `references/typescript/src/utils/hooks/registerFrontmatterHooks.ts` — archive-only frontmatter-hook registration helper retained for source fidelity; hook registration flows remain outside the shipped proving slice.
- `references/typescript/src/utils/hooks/registerSkillHooks.ts` — archive-only skill-hook registration helper retained for source fidelity; the shipped proving slice does not ship archived skill-hook registration logic as a runtime boundary.
- `references/typescript/src/utils/hooks/execAgentHook.ts` — archive-only agent-hook execution helper retained for source fidelity; agent hook execution remains outside the shipped proving slice.

## Shipped-language-pack rule

This subset is complete when each archived hook-helper row has an explicit archive-only or shipped-owner rationale. These snapshot-era hook helper files must not be mistaken for already-shipped TypeScript runtime modules.
