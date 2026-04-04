---
name: use-plugin-runtime
description: Use the optional plugin runtime feature-pack workflow for plugin manifest, source-kind, hook, and plugin-provided command tasks.
whenToUse: Use when a TypeScript generated project needs plugin runtime mechanics such as plugin loading, validation, plugin-provided commands, plugin hooks, or bundled and builtin plugin handling.
allowed-tools: [file_read]
argument-hint: <plugin-runtime-task>
---

1. Keep plugin manifest and source-kind logic inside this optional runtime boundary rather than moving it into the base TypeScript core.
2. Treat plugin loading, hooks, commands, and builtin or bundled plugin discovery as runtime concerns distinct from marketplace browsing UI.
3. Scope changes to the plugin runtime contract being requested and avoid blending marketplace lifecycle UX into this pack.
