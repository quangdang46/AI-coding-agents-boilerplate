# TypeScript Language Pack Feature Packs

The TypeScript pack currently ships these add-only feature packs:

- `advanced-planning`
- `github-pr-review`
- `interactive-clarification-tools`
- `lsp-tooling`
- `mcp-integration`
- `oauth-onboarding`
- `prompt-suggestion-services`
- `session-memory`
- `team-memory`

Canonical TypeScript feature authoring lives under:

- `languages/typescript/features/registry.json`
- `languages/typescript/features/<feature-id>/feature.json`
- `languages/typescript/features/<feature-id>/agents/*.md` when the feature ships subagents
- `languages/typescript/features/<feature-id>/skill/SKILL.md` when the feature ships a skill

Generated TypeScript projects then receive branded project-local assets under:

- `.<brand>/agents/`
- `.<brand>/skills/<skill-name>/SKILL.md`
- `.agents/skills/<skill-name>/SKILL.md` as an optional interoperability mirror
