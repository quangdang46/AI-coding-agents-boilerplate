# PRD — TypeScript Agent Boilerplate Migration

## Problem
The current `typescript/` tree is a large product snapshot, not a reusable starter. It mixes runtime kernel concerns with product-specific UI, installer, analytics, and duplicated source.

## Goal
Create a reusable TypeScript coding-agent boilerplate that preserves the strong extension seams (prompts, tools, skills, agents, provider selection) while removing snapshot-specific baggage.

## Non-goals (v1)
- Full plugin marketplace
- Voice/mobile/desktop features
- Team/swarm runtime as a default surface
- Rust boilerplate implementation
- Remote-control/bridge parity

## Primary users
1. Boilerplate author maintaining the shared TypeScript runtime
2. Project user generating a new boilerplate instance
3. Agent user extending an instance through local agents/skills/features

## User stories
### US-001 — Preserve extension seams
As a boilerplate author, I want skills, agents, prompts, and tools to remain explicit extension points so generated projects can customize behavior without editing the kernel.

**Acceptance criteria**
- Skills load from project files
- Agents load from project manifests
- Prompt layering is configurable
- Tool/command registration is registry-driven

### US-002 — Remove snapshot duplication
As a maintainer, I want duplicate and snapshot-only artifacts removed so the repo becomes understandable and maintainable.

**Acceptance criteria**
- `typescript/free-code-main/` is removed
- Migration docs explain what is kept vs removed
- Snapshot-specific docs are identified for rewrite/removal

### US-003 — Establish migration safety rails
As a maintainer, I want smoke tests for the critical runtime seams so cleanup does not silently break the migration.

**Acceptance criteria**
- CLI startup seam has a smoke test
- Prompt composition seam has a smoke test
- Skill loading seam has a smoke test
- Agent loading seam has a smoke test

### US-004 — Prepare for scaffolding
As a future user, I want the runtime contract to be stable enough that a Rust CLI can scaffold it reliably.

**Acceptance criteria**
- Template shape is documented
- Config contract is documented
- Feature-pack direction is documented

## Release slice for Ralph iteration 1
- Context snapshot
- PRD
- Test spec
- `typescript/docs/migration.md`
- smoke tests
- duplicate tree removal
