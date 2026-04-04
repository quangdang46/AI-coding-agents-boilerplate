---
name: use-session-memory
description: Use the optional session-memory feature-pack workflow for relevant-memory retrieval tasks.
whenToUse: Use when a TypeScript generated project needs memory-aware recall beyond the base saved-session/runtime summary behavior.
allowed-tools: [file_read]
argument-hint: <memory-task>
---

1. Separate session-memory retrieval from the base resume and persistence features.
2. Use this feature boundary for recall-oriented behavior, not for generic prompt/context loading.
3. Keep memory changes scoped to the requested retrieval or memory-management task.
