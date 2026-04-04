---
name: add-feature
description: Add a feature pack to this project.
whenToUse: Use when the user asks to add a packaged capability.
allowed-tools: [Read, Edit, Write, Bash]
argument-hint: <feature-id>
---

1. Use `aicd feature add <feature-id> --project .`.
2. Confirm the feature materialized its assets under project code and `.agents/`.
3. Run verification.
