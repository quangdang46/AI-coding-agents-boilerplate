# TypeScript permissions, shell, and filesystem enforcement ownership

The archived permissions-and-shell helper files in bead `aicd-3ix.4.14.1.1` are not preserved as a one-for-one shipped utility subtree in the current AICD TypeScript proving slice.

The shipped TypeScript pack keeps the current safety-critical runtime boundary in `languages/typescript/runtime/utils/config.ts`, `languages/typescript/runtime/utils/toolExecution.ts`, and the permission-oriented command summaries surfaced through the core runtime. That means the archived permission, path-validation, and shell helper families need explicit ownership mapping rather than being implied as shipped one-for-one utilities.

## Ownership and disposition

- `references/typescript/src/utils/shell/powershellProvider.ts` — archive-only shell provider helper retained for source fidelity; the shipped proving slice does not expose a dedicated shell-provider utility subtree.
- `references/typescript/src/utils/shell/resolveDefaultShell.ts` — archive-only shell helper retained for source fidelity; default-shell resolution is not shipped as a standalone utility boundary in the proving slice.
- `references/typescript/src/utils/shell/prefix.ts` — archive-only shell helper retained for source fidelity; prefix handling is not shipped as a standalone utility boundary in the proving slice.
- `references/typescript/src/utils/shell/shellProvider.ts` — archive-only shell provider helper retained for source fidelity; the proving slice does not expose the archived provider helper package one-for-one.
- `references/typescript/src/utils/shell/readOnlyCommandValidation.ts` — owned conceptually by shipped runtime policy and tool-execution enforcement under `languages/typescript/runtime/utils/toolExecution.ts`, without preserving the archived helper one-for-one.
- `references/typescript/src/utils/shell/outputLimits.ts` — archive-only shell helper retained for source fidelity; output-limits logic is not shipped as a standalone utility boundary in the proving slice.
- `references/typescript/src/utils/shell/specPrefix.ts` — archive-only shell helper retained for source fidelity; spec-prefix handling is not shipped as a standalone utility boundary in the proving slice.
- `references/typescript/src/utils/shell/shellToolUtils.ts` — owned conceptually by shipped runtime tool execution under `languages/typescript/runtime/utils/toolExecution.ts`, without preserving the archived helper one-for-one.
- `references/typescript/src/utils/shell/bashProvider.ts` — archive-only shell provider helper retained for source fidelity; the shipped proving slice does not expose a dedicated bash-provider utility subtree.
- `references/typescript/src/utils/shell/powershellDetection.ts` — archive-only shell helper retained for source fidelity; PowerShell detection is not shipped as a standalone utility boundary in the proving slice.
- `references/typescript/src/utils/permissions/autoModeState.ts` — owned conceptually by shipped runtime config and approval-mode handling under `languages/typescript/runtime/utils/config.ts`, without preserving the archived helper one-for-one.
- `references/typescript/src/utils/permissions/dangerousPatterns.ts` — archive-only safety helper retained for source fidelity; dangerous-pattern matching is not shipped as a standalone utility boundary in the proving slice.
- `references/typescript/src/utils/permissions/PermissionRule.ts` — archive-only permission-rule type/helper retained for source fidelity; the proving slice does not expose the archived permission-rule package one-for-one.
- `references/typescript/src/utils/permissions/denialTracking.ts` — owned conceptually by shipped runtime policy/accounting boundaries under `languages/typescript/runtime/utils/toolExecution.ts`, without preserving the archived helper one-for-one.
- `references/typescript/src/utils/permissions/shadowedRuleDetection.ts` — archive-only permission helper retained for source fidelity; shadowed-rule detection is not shipped as a standalone utility boundary in the proving slice.
- `references/typescript/src/utils/permissions/permissions.ts` — owned conceptually by shipped runtime config and policy handling under `languages/typescript/runtime/utils/config.ts` and `languages/typescript/runtime/utils/toolExecution.ts`.
- `references/typescript/src/utils/permissions/permissionRuleParser.ts` — archive-only permission helper retained for source fidelity; rule parsing is not shipped as a standalone utility boundary in the proving slice.
- `references/typescript/src/utils/permissions/filesystem.ts` — owned conceptually by shipped runtime file/tool execution boundaries under `languages/typescript/runtime/utils/toolExecution.ts`, without preserving the archived helper one-for-one.
- `references/typescript/src/utils/permissions/getNextPermissionMode.ts` — owned conceptually by shipped runtime approval-mode handling under `languages/typescript/runtime/utils/config.ts`, without preserving the archived helper one-for-one.
- `references/typescript/src/utils/permissions/classifierShared.ts` — archive-only classifier helper retained for source fidelity; classifier scaffolding is not shipped as a standalone utility boundary in the proving slice.
- `references/typescript/src/utils/permissions/bypassPermissionsKillswitch.ts` — owned conceptually by shipped runtime config/settings handling under `languages/typescript/runtime/utils/config.ts`, without preserving the archived helper one-for-one.
- `references/typescript/src/utils/permissions/PermissionPromptToolResultSchema.ts` — archive-only permission schema helper retained for source fidelity; the proving slice does not expose the archived permission schema package one-for-one.
- `references/typescript/src/utils/permissions/bashClassifier.ts` — archive-only classifier helper retained for source fidelity; classifier scaffolding is not shipped as a standalone utility boundary in the proving slice.
- `references/typescript/src/utils/permissions/PermissionResult.ts` — archive-only permission result helper retained for source fidelity; the proving slice does not expose the archived permission result package one-for-one.
- `references/typescript/src/utils/permissions/permissionsLoader.ts` — owned conceptually by shipped runtime config handling under `languages/typescript/runtime/utils/config.ts`, without preserving the archived helper one-for-one.
- `references/typescript/src/utils/permissions/PermissionUpdateSchema.ts` — archive-only permission schema helper retained for source fidelity; the proving slice does not expose the archived permission schema package one-for-one.
- `references/typescript/src/utils/permissions/permissionExplainer.ts` — archive-only permission helper retained for source fidelity; explanation scaffolding is not shipped as a standalone utility boundary in the proving slice.
- `references/typescript/src/utils/permissions/classifierDecision.ts` — archive-only classifier helper retained for source fidelity; classifier scaffolding is not shipped as a standalone utility boundary in the proving slice.
- `references/typescript/src/utils/permissions/PermissionMode.ts` — archive-only permission-mode helper retained for source fidelity; the proving slice does not expose the archived permission-mode package one-for-one.
- `references/typescript/src/utils/permissions/PermissionUpdate.ts` — archive-only permission-update helper retained for source fidelity; the proving slice does not expose the archived permission-update package one-for-one.
- `references/typescript/src/utils/permissions/permissionSetup.ts` — owned conceptually by shipped runtime config and policy bootstrap handling under `languages/typescript/runtime/utils/config.ts`, without preserving the archived helper one-for-one.
- `references/typescript/src/utils/permissions/pathValidation.ts` — owned conceptually by shipped runtime file/tool execution boundaries under `languages/typescript/runtime/utils/toolExecution.ts`, without preserving the archived helper one-for-one.
- `references/typescript/src/utils/permissions/shellRuleMatching.ts` — owned conceptually by shipped runtime tool execution and policy handling under `languages/typescript/runtime/utils/toolExecution.ts`, without preserving the archived helper one-for-one.
- `references/typescript/src/utils/permissions/yoloClassifier.ts` — archive-only classifier helper retained for source fidelity; classifier scaffolding is not shipped as a standalone utility boundary in the proving slice.

## Shipped-language-pack rule

This subset is complete when each archived permission/shell/file-enforcement row has an explicit shipped-owner or archive-only rationale. These safety-critical helpers must not be mistaken for already-shipped one-for-one runtime modules just because the proving slice preserves their underlying policy behavior in decomposed runtime form.
