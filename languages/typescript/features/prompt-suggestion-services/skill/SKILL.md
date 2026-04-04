---
name: use-prompt-suggestion-services
description: Use the optional prompt-suggestion service pack for Magic Docs, summarization, and suggestion-oriented workflows.
whenToUse: Use when the task depends on prompt suggestions, Magic Docs, summarization helpers, or adjacent non-core suggestion services.
allowed-tools: [file_read]
argument-hint: <prompt-task>
---

1. Treat prompt suggestions and Magic Docs helpers as optional services, not mandatory core runtime behavior.
2. Scope changes to the specific suggestion or summarization workflow being requested.
3. Keep the base TypeScript runtime lean by routing suggestion-heavy work through this feature boundary.
