# Python Extension Model

The Python boilerplate exposes three first-class extension seams:

## 1. Prompts
Project-local prompt files live under `.agent/prompts/`.

## 2. Agents
Agent manifests live under `.agent/agents/*.agent.json`.
Each manifest declares prompt, allowed tools, and skill references.

## 3. Skills
Skills live under `.agent/skills/<name>/SKILL.md`.
Each skill uses Markdown plus YAML frontmatter.

## Loader precedence
1. runtime defaults
2. project-local `.agent/*` files
3. enabled feature-pack files (feature-added prompts, agents, and skills become active when the feature is enabled)
4. explicit CLI overrides
