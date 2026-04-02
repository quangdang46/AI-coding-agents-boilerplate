# Test Spec — Python Agent Boilerplate Migration

## Scope
This test spec covers the first executable migration slice for the Python boilerplate and Rust scaffolder.

## Test areas

### 1. Existing Python baseline remains callable
**Objective:** verify the current mirror workspace still functions during extraction.

**Test:**
- run `python -m unittest discover -s tests -v` from `python/`
- assert exit code is 0

### 2. Config loading seam
**Objective:** verify `agentkit.toml` loads into typed config objects.

**Test:**
- load the base template config file
- assert app/provider/tool sections are parsed
- assert enabled agents/skills/features match the template defaults

### 3. Prompt layering seam
**Objective:** verify runtime default prompt + project prompt + section prompts compose in deterministic order.

**Test:**
- create a temp project from the base template
- call the prompt composer
- assert runtime default content is included
- assert project system prompt is included
- assert configured section prompts are appended in order

### 4. Agent loading seam
**Objective:** verify local agent manifests are discoverable.

**Test:**
- create temp project with `.agent/agents/*.agent.json`
- load agents via the registry loader
- assert expected agent IDs are present

### 5. Skill loading seam
**Objective:** verify local skill files are discoverable.

**Test:**
- create temp project with `.agent/skills/<name>/SKILL.md`
- load skills via the registry loader
- assert expected skill names are present

### 6. Feature-pack seam
**Objective:** verify local feature packs are discoverable.

**Test:**
- load `.agent/features/registry.json`
- resolve a feature manifest from `.agent/features/<feature-id>/feature.json`
- assert adds/patches metadata are parsed

### 7. Template scaffold seam
**Objective:** verify the Python base template can be copied into a working project layout.

**Test:**
- scaffold the base template into a temp directory
- assert `agentkit.toml`, `.agent/`, `src/`, and `tests/` exist

### 8. Rust installer init flow
**Objective:** verify the installer can render the Python template.

**Test:**
- run `cargo run -- init demo-agent --language python --output <tmp> --yes`
- assert exit code is 0
- assert rendered project contains required files

### 9. Rust installer doctor flow
**Objective:** verify the installer validates a generated Python project.

**Test:**
- run `cargo run -- doctor --project <tmp>`
- assert exit code is 0 for a valid scaffold
- assert missing required files causes non-zero exit for an invalid scaffold

## Verification commands
- `cd python && python -m unittest discover -s tests -v`
- `cd install && cargo test`
- `cd install && cargo run -- list languages`
- `cd install && cargo run -- init demo-agent --language python --output /tmp/demo-agent --yes`
- `cd install && cargo run -- doctor --project /tmp/demo-agent`

## Exit criteria
- Python baseline tests pass
- New Python boilerplate tests pass
- Rust installer tests pass
- Installer can scaffold and validate the Python template
