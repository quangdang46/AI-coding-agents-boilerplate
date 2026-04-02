# PRD — Python Agent Boilerplate Migration

## Problem
The current `python/` tree is a mirror-oriented porting workspace. It proves useful abstractions, but it is not a reusable coding-agent boilerplate because it depends on snapshot metadata, placeholder execution, and migration-only reporting commands.

## Goal
Create a reusable Python coding-agent boilerplate that:
- exposes a stable runtime kernel
- is customizable through config, prompts, tools, agents, skills, and feature packs
- can be scaffolded by the Rust `install/` CLI
- aligns with the TypeScript boilerplate contract where practical

## Non-goals (v1)
- plugin marketplace
- remote/ssh/teleport runtime by default
- voice/mobile/desktop surfaces
- swarm/team runtime as a default feature
- public feature-pack registry

## Primary users
1. Boilerplate maintainer extracting and versioning the shared Python runtime
2. Project owner generating a new Python coding-agent instance
3. Agent user extending a generated instance via local agents, skills, and feature packs

## User stories
### US-001 — Config-driven runtime
As a project owner, I want to configure provider, prompts, tools, and enabled extensions through files so I can adapt the boilerplate without editing runtime internals.

**Acceptance criteria**
- Runtime loads `agentkit.toml`
- Prompt files are resolved from config
- Tool enablement is config-driven
- Provider defaults are config-driven

### US-002 — Local extension seams
As a user, I want to add agents and skills by adding files under `.agent/` so that the generated project is self-extensible.

**Acceptance criteria**
- Agent manifests load from `.agent/agents/*.agent.json`
- Skills load from `.agent/skills/*/SKILL.md`
- Missing files produce clear validation errors

### US-003 — Feature-pack based growth
As a user, I want to add reusable capabilities to my generated project without editing the kernel by hand.

**Acceptance criteria**
- Feature packs are discovered from `.agent/features/registry.json`
- A feature pack declares file additions and config patches
- The installer can add a feature pack to an existing Python project

### US-004 — Scaffoldable Python template
As a maintainer, I want the boilerplate contract to be frozen enough that the Rust CLI can generate a working Python project.

**Acceptance criteria**
- `python/templates/base/` contains a minimal working template
- `aicd init --language python` renders that template
- `aicd doctor` validates a generated project

### US-005 — Migration safety rails
As a maintainer, I want regression tests that protect the current baseline and the new extension seams so that extraction work does not silently regress.

**Acceptance criteria**
- Existing Python baseline tests stay green
- Config loading has tests
- Prompt composition has tests
- Agent loading has tests
- Skill loading has tests
- Scaffold generation has tests

## Release slice for Ralph iteration 1
- Python PRD
- Python test spec
- `python/docs/migration.md`
- `python/src/agent_boilerplate/` runtime skeleton
- config/prompt/agent/skill/feature loaders
- `python/templates/base/`
- Rust `aicd` CLI with `init --language python`, `list`, and `doctor`
- tests covering the new Python extension seams and scaffold flow
