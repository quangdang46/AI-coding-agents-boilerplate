---
name: use-oauth-onboarding
description: Use the optional OAuth onboarding feature-pack workflow for guided authentication and setup tasks.
whenToUse: Use when a TypeScript generated project needs onboarding, credential setup, or OAuth-style login guidance beyond the base runtime.
allowed-tools: [file_read]
argument-hint: <auth-task>
---

1. Confirm the auth or onboarding goal before changing setup flows.
2. Keep guided login and credential flows inside this optional feature boundary instead of treating them as core runtime behavior.
3. Update project-local docs or settings only when the requested auth workflow actually needs them.
