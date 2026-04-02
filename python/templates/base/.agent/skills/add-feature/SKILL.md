---
name: add-feature
description: Add a feature pack to this project.
whenToUse: Use when the user asks to add a packaged capability.
allowed-tools: [Read, Edit, Write, Bash]
argument-hint: <feature-id>
---

1. Read `.agent/features/registry.json`.
2. Match the requested feature.
3. Apply feature files and config changes.
4. Run verification.
