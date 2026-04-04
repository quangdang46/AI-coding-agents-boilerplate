---
name: use-team-memory
description: Use the optional team-memory feature-pack workflow for shared-memory and collaboration-oriented retrieval tasks.
whenToUse: Use when a TypeScript generated project needs team or shared-memory behavior that should stay outside the base local runtime.
allowed-tools: [file_read]
argument-hint: <team-memory-task>
---

1. Keep team/shared memory semantics separate from local session memory.
2. Use this boundary for collaboration-oriented memory workflows rather than core prompt/context behavior.
3. Update only the shared-memory workflow requested by the user.
