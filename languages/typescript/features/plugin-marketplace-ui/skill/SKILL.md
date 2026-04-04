---
name: use-plugin-marketplace-ui
description: Use the optional plugin marketplace UI feature-pack workflow for plugin discovery, install, update, and trust-management tasks.
whenToUse: Use when a TypeScript generated project needs marketplace-oriented plugin workflows such as browsing, installation, updates, trust warnings, or marketplace configuration.
allowed-tools: [file_read]
argument-hint: <plugin-marketplace-task>
---

1. Keep plugin browsing, installation, update, and marketplace configuration flows inside this optional marketplace boundary rather than the base TypeScript core.
2. Treat marketplace lifecycle UI separately from plugin manifest and plugin-loading mechanics.
3. Prefer the plugin-runtime feature when the request is about loaders, hooks, schemas, or bundled and builtin plugin sources.
