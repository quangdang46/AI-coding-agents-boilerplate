# TypeScript task, team, cron, and workflow tool ownership

The archived advanced tool files in bead `aicd-3ix.4.10.1.2` are not preserved as a one-for-one shipped tool package in the current AICD TypeScript proving slice.

The shipped TypeScript pack keeps only the base runtime/tool registry surface in `languages/typescript/runtime/`. Task orchestration, team-management flows, cron scheduling helpers, and workflow-specific tool UI remain outside the shipped core runtime boundary. Their final disposition is therefore a mix of future feature-pack ownership for advanced orchestration families and archive-only treatment for snapshot-era helper/UI pieces that the proving slice does not ship directly.

## Ownership and disposition

- `references/typescript/src/tools/TaskOutputTool/constants.ts` — future feature-pack ownership under an advanced workflow/task-observability direction; task-output presentation constants are not part of the shipped core runtime.
- `references/typescript/src/tools/TaskOutputTool/TaskOutputTool.tsx` — future feature-pack ownership under an advanced workflow/task-observability direction; task-output UI is not part of the shipped core runtime.
- `references/typescript/src/tools/TaskUpdateTool/prompt.ts` — future feature-pack ownership under an advanced workflow/task-management direction; task-update prompting is not shipped in the current proving slice.
- `references/typescript/src/tools/TaskUpdateTool/TaskUpdateTool.ts` — future feature-pack ownership under an advanced workflow/task-management direction; task-update tooling is not part of the shipped core runtime.
- `references/typescript/src/tools/TaskUpdateTool/constants.ts` — future feature-pack ownership under an advanced workflow/task-management direction; task-update constants remain outside the shipped core runtime.
- `references/typescript/src/tools/TeamCreateTool/TeamCreateTool.ts` — future feature-pack ownership aligned to collaboration/team-memory or multi-agent workflow directions; team creation is not shipped in the current proving slice.
- `references/typescript/src/tools/TeamCreateTool/prompt.ts` — future feature-pack ownership aligned to collaboration/team-memory or multi-agent workflow directions; team-creation prompting is not shipped in the current proving slice.
- `references/typescript/src/tools/TeamCreateTool/UI.tsx` — future feature-pack ownership aligned to collaboration/team-memory or multi-agent workflow directions; team-creation UI is not shipped in the current proving slice.
- `references/typescript/src/tools/TeamCreateTool/constants.ts` — future feature-pack ownership aligned to collaboration/team-memory or multi-agent workflow directions; team-creation constants remain outside the shipped core runtime.
- `references/typescript/src/tools/TeamDeleteTool/TeamDeleteTool.ts` — future feature-pack ownership aligned to collaboration/team-memory or multi-agent workflow directions; team deletion is not shipped in the current proving slice.
- `references/typescript/src/tools/TeamDeleteTool/prompt.ts` — future feature-pack ownership aligned to collaboration/team-memory or multi-agent workflow directions; team-deletion prompting is not shipped in the current proving slice.
- `references/typescript/src/tools/TeamDeleteTool/UI.tsx` — future feature-pack ownership aligned to collaboration/team-memory or multi-agent workflow directions; team-deletion UI is not shipped in the current proving slice.
- `references/typescript/src/tools/TeamDeleteTool/constants.ts` — future feature-pack ownership aligned to collaboration/team-memory or multi-agent workflow directions; team-deletion constants remain outside the shipped core runtime.
- `references/typescript/src/tools/ScheduleCronTool/CronCreateTool.ts` — future feature-pack ownership under an automation/scheduled-workflow direction; cron creation tooling is not shipped in the current proving slice.
- `references/typescript/src/tools/ScheduleCronTool/CronListTool.ts` — future feature-pack ownership under an automation/scheduled-workflow direction; cron listing tooling is not shipped in the current proving slice.
- `references/typescript/src/tools/ScheduleCronTool/prompt.ts` — future feature-pack ownership under an automation/scheduled-workflow direction; cron prompting is not shipped in the current proving slice.
- `references/typescript/src/tools/ScheduleCronTool/UI.tsx` — future feature-pack ownership under an automation/scheduled-workflow direction; cron UI is not shipped in the current proving slice.
- `references/typescript/src/tools/ScheduleCronTool/CronDeleteTool.ts` — future feature-pack ownership under an automation/scheduled-workflow direction; cron deletion tooling is not shipped in the current proving slice.
- `references/typescript/src/tools/WorkflowTool/constants.ts` — future feature-pack ownership under an advanced workflow/orchestration direction; workflow constants are not part of the shipped core runtime.
- `references/typescript/src/tools/TaskGetTool/prompt.ts` — future feature-pack ownership under an advanced workflow/task-management direction; task-query prompting is not shipped in the current proving slice.
- `references/typescript/src/tools/TaskGetTool/TaskGetTool.ts` — future feature-pack ownership under an advanced workflow/task-management direction; task-query tooling is not part of the shipped core runtime.
- `references/typescript/src/tools/TaskGetTool/constants.ts` — future feature-pack ownership under an advanced workflow/task-management direction; task-query constants remain outside the shipped core runtime.
- `references/typescript/src/tools/TaskStopTool/TaskStopTool.ts` — future feature-pack ownership under an advanced workflow/task-management direction; task-stop tooling is not part of the shipped core runtime.
- `references/typescript/src/tools/TaskStopTool/prompt.ts` — future feature-pack ownership under an advanced workflow/task-management direction; task-stop prompting is not shipped in the current proving slice.
- `references/typescript/src/tools/TaskStopTool/UI.tsx` — future feature-pack ownership under an advanced workflow/task-management direction; task-stop UI is not shipped in the current proving slice.
- `references/typescript/src/tools/TaskCreateTool/prompt.ts` — future feature-pack ownership under an advanced workflow/task-management direction; task-creation prompting is not shipped in the current proving slice.
- `references/typescript/src/tools/TaskCreateTool/TaskCreateTool.ts` — future feature-pack ownership under an advanced workflow/task-management direction; task-creation tooling is not part of the shipped core runtime.
- `references/typescript/src/tools/TaskCreateTool/constants.ts` — future feature-pack ownership under an advanced workflow/task-management direction; task-creation constants remain outside the shipped core runtime.
- `references/typescript/src/tools/TaskListTool/TaskListTool.ts` — future feature-pack ownership under an advanced workflow/task-management direction; task-list tooling is not part of the shipped core runtime.
- `references/typescript/src/tools/TaskListTool/prompt.ts` — future feature-pack ownership under an advanced workflow/task-management direction; task-list prompting is not shipped in the current proving slice.
- `references/typescript/src/tools/TaskListTool/constants.ts` — future feature-pack ownership under an advanced workflow/task-management direction; task-list constants remain outside the shipped core runtime.

## Shipped-language-pack rule

This subset is complete when each archived task/team/cron/workflow tool row has a clear future feature-pack owner or an explicit archive-only rationale. These archived orchestration tools must not be mistaken for already-shipped TypeScript runtime ownership just because the repository preserves advanced workflow feature-pack taxonomy.
