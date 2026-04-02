# Test Spec — TypeScript Agent Boilerplate Migration

## Scope
This test spec covers the first migration slice: safety rails and cleanup prerequisites.

## Test areas

### 1. CLI startup seam
**Objective:** verify the CLI bootstrap path remains callable.

**Test:**
- invoke the version fast path through Bun
- assert exit code is 0
- assert stdout contains `Claude Code`

### 2. Prompt layering seam
**Objective:** verify custom and appended system prompts can still be composed.

**Test:**
- call the exported prompt-composition helper
- assert a custom prompt replaces the default body when no agent override exists
- assert append prompt is preserved at the end

### 3. Skill loading seam
**Objective:** verify a project-local skill file is discoverable.

**Test:**
- create temp project with `.claude/skills/<name>/SKILL.md`
- load commands via skill loader
- assert the skill name is present

### 4. Agent loading seam
**Objective:** verify a project-local agent manifest is discoverable.

**Test:**
- create temp project with `.claude/agents/<name>.md`
- load agents via agent loader
- assert the custom agent is active

## Verification commands
- `cd typescript && bun test tests/smoke`
- `cd typescript && bun run build`

## Exit criteria
- all smoke tests pass
- build passes after duplicate-tree cleanup
