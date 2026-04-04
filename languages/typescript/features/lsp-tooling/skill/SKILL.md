---
name: use-lsp-tooling
description: Use the optional LSP feature-pack workflow for code-intelligence tasks.
whenToUse: Use when the user wants symbol, diagnostics, or reference-aware investigation in a TypeScript generated project.
allowed-tools: [lsp, file_read]
argument-hint: <code-task>
---

1. Use LSP-backed diagnostics and symbol lookup before editing code.
2. Prefer reference-aware navigation over raw text search when the feature is enabled.
3. Validate the affected files after edits with the same tooling flow.
