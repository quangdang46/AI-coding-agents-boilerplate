# TypeScript task-runtime ownership

The archived TypeScript `src/tasks/*` package in bead `aicd-3ix.4.16.1.3` is not preserved as a one-for-one shipped runtime package in the current AICD TypeScript proving slice.

The shipped TypeScript pack currently exposes task-oriented behavior only through the summary-level `tasks` command in `languages/typescript/runtime/registry/coreCommands.ts`; it does not ship the archived task runtime classes and UI task executors as a dedicated `languages/typescript/runtime/tasks/` boundary.

## Ownership and disposition

- `references/typescript/src/tasks/stopTask.ts` — archive-only task-stop helper retained for source fidelity; the shipped proving slice does not expose the archived task runtime stop flow as a dedicated runtime module.
- `references/typescript/src/tasks/LocalMainSessionTask.ts` — archive-only local-main-session task class retained for source fidelity; the shipped proving slice does not ship the archived task runtime class hierarchy.
- `references/typescript/src/tasks/DreamTask/DreamTask.ts` — archive-only dream-task executor retained for source fidelity; dream-style task execution remains outside the shipped TypeScript proving slice.
- `references/typescript/src/tasks/RemoteAgentTask/RemoteAgentTask.tsx` — archive-only remote-agent task UI/runtime helper retained for source fidelity; remote-agent task execution is not part of the shipped TypeScript runtime boundary.
- `references/typescript/src/tasks/InProcessTeammateTask/types.ts` — archive-only in-process teammate task types retained for source fidelity; teammate-task orchestration remains outside the shipped proving slice.
- `references/typescript/src/tasks/InProcessTeammateTask/InProcessTeammateTask.tsx` — archive-only in-process teammate task executor retained for source fidelity; teammate-task orchestration remains outside the shipped proving slice.
- `references/typescript/src/tasks/types.ts` — archive-only task package type surface retained for source fidelity; the shipped proving slice does not expose a standalone task-runtime type boundary.
- `references/typescript/src/tasks/pillLabel.ts` — archive-only task-label rendering helper retained for source fidelity; snapshot-era task UI affordances are not part of the shipped proving slice.
- `references/typescript/src/tasks/LocalAgentTask/LocalAgentTask.tsx` — archive-only local-agent task executor retained for source fidelity; local-agent task runtime execution is not part of the shipped TypeScript runtime boundary.
- `references/typescript/src/tasks/LocalShellTask/LocalShellTask.tsx` — archive-only local-shell task executor retained for source fidelity; shell-task runtime execution is not shipped as a dedicated TypeScript task package.
- `references/typescript/src/tasks/LocalShellTask/killShellTasks.ts` — archive-only shell-task termination helper retained for source fidelity; the shipped proving slice does not expose this task-control helper as a runtime module.
- `references/typescript/src/tasks/LocalShellTask/guards.ts` — archive-only shell-task guard helper retained for source fidelity; task-runtime enforcement helpers remain outside the shipped proving slice.

## Shipped-language-pack rule

This subset is complete when each archived task-runtime row has an explicit archive-only or shipped-owner rationale. These snapshot-era task runtime files must not be mistaken for already-shipped TypeScript runtime modules.
