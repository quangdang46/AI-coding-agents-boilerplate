# Boilerplate Config Reference

The generated TypeScript boilerplate is configured through `boilerplate.config.ts`.

## Top-level sections
- `app` — branding and default runtime behavior
- `prompts` — system and append prompt file locations
- `tools` — enabled tools, defaults, and approval policy
- `providers` — provider-specific auth/model defaults
- `agents` — agent directories and default enablement
- `skills` — skill directories and auto-discovery policy
- `features` — enabled feature packs

## Default template asset locations
- `.agent/prompts/system.md`
- `.agent/prompts/sections/*.md`
- `.agent/agents/*.agent.json`
- `.agent/skills/*/SKILL.md`
- `.agent/features/registry.json`
