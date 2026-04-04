---
name: clarify-requirements
description: Gather missing details before acting with the interactive clarification tool pack.
whenToUse: Use when a request is underspecified and the TypeScript project has the clarification tools feature enabled.
allowed-tools: [ask_user, brief, send_message, file_read]
argument-hint: <goal>
---

1. Read the relevant local context before asking questions.
2. Ask only for information that materially changes implementation.
3. Use brief or send-message when a short structured follow-up is better than a long prompt.
