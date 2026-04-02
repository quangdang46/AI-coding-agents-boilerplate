# Python Boilerplate Config Reference

Generated Python projects are configured through `agentkit.toml`.

## Top-level sections
- `app` — project identity and default runtime mode
- `prompts` — system prompt path and prompt sections
- `tools` — enabled tools, approval mode, timeouts
- `providers` — provider/model defaults
- `agents` — agent directories and enabled agent IDs
- `skills` — skill directories and enabled skill names
- `features` — enabled feature packs and feature registry path

## Default project asset paths
- `.agent/prompts/system.md`
- `.agent/prompts/sections/*.md`
- `.agent/agents/*.agent.json`
- `.agent/skills/*/SKILL.md`
- `.agent/features/registry.json`
