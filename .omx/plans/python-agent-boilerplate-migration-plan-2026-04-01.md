# Python Coding-Agent Boilerplate Migration Plan

_Date: 2026-04-01_

## Assumptions

- I am treating `python/` as the **Python boilerplate target**. Your note `python/ # Rust boilerplate` appears to be a typo because `install/` is the Rust CLI area and `python/` already contains Python source.
- I am treating the current `python/` codebase as a **porting/mirroring workspace**, not a production-ready agent runtime. The migration should extract the reusable seams and replace the mirror-only pieces.
- I am aligning the Python contract with the TypeScript boilerplate direction already documented in `typescript/docs/config-reference.md:1-19` and `typescript/docs/architecture/core-runtime-boundaries.md:1-23`.

---

## Requirements Summary

You want to turn the existing `python/` source into a reusable Python coding-agent boilerplate inside this mono-repo, keep it customizable, support an agents/skills system so users can extend their generated instance, and make a Rust `install/` CLI the canonical scaffolder for both Python and TypeScript outputs.

The current Python source is useful, but it is still a **mirror-oriented workspace**:

- the CLI is centered on migration/reporting commands like `summary`, `manifest`, `parity-audit`, `command-graph`, `tool-pool`, `bootstrap`, `turn-loop`, and `exec-tool` rather than end-user agent commands (`python/src/main.py:21-91`)
- commands and tools are loaded from static JSON snapshots (`python/src/commands.py:22-36`, `python/src/tools.py:23-37`)
- command/tool execution is simulated with placeholder strings, not real runtime behavior (`python/src/commands.py:75-80`, `python/src/tools.py:81-86`)
- the query engine currently returns prompt/tool summary text instead of a real model/tool orchestration loop (`python/src/query_engine.py:61-104`)
- the runtime still adds migration-only reporting like parity audits, setup reports, and mirrored execution registries (`python/src/runtime.py:109-152`, `python/src/parity_audit.py:121-138`)
- tests validate mirror coverage and CLI smoke paths, and they currently pass from `python/` via `python -m unittest discover -s tests -v` (`python/tests/test_porting_workspace.py:15-244`)

That means the right move is **not** “ship the current tree as boilerplate.” The right move is to:

1. keep the reusable runtime seams,
2. delete or quarantine mirror-only artifacts,
3. externalize config/prompts/agents/skills/features,
4. freeze a stable template contract,
5. then build the Rust installer against that frozen contract.

---

## Current Source: What It Already Proves

The current Python tree already proves several important design ideas worth preserving:

### 1) CLI entrypoint and subcommand framing
`python/src/main.py` already cleanly centralizes argument parsing and subcommand dispatch (`python/src/main.py:21-91`, `94-209`).

### 2) Runtime/session seam
`PortRuntime.bootstrap_session()` already composes context, setup, routing, execution registry, transcripting, and persisted sessions into a single reportable unit (`python/src/runtime.py:109-152`).

### 3) Session state and persistence
`QueryEnginePort`, `TranscriptStore`, and `StoredSession` already define a useful state boundary for message history, compaction, and persistence (`python/src/query_engine.py:35-150`, `python/src/transcript.py:7-22`, `python/src/session_store.py:8-26`).

### 4) Permission filtering
The deny-by-name / deny-by-prefix pattern is a good minimal policy primitive to keep (`python/src/permissions.py:7-18`, `python/src/tools.py:56-72`).

### 5) Registry-driven commands/tools
Even though the current registries are snapshot-backed, they already show the right overall abstraction: commands/tools should come from a registry, not be hardcoded across the entire runtime (`python/src/commands.py:44-90`, `python/src/tools.py:40-96`).

### 6) Alignment with TypeScript extraction boundaries
The TypeScript side already documents the right reusable cuts: `core/engine`, `core/prompts`, `core/config`, `core/registry`, and `core/providers` (`typescript/docs/architecture/core-runtime-boundaries.md:5-23`). Python should mirror that layout conceptually.

---

## Recommended Target Architecture

### Decision

Build the Python boilerplate in **two layers**:

1. **Reusable runtime kernel** inside `python/src/agent_boilerplate/`
2. **Generated-project customization surface** via `agentkit.toml` + `.agent/` directories

### Why this cut is correct

- The current Python source is small enough to refactor safely, but much of it is migration-only or placeholder behavior.
- The TypeScript side already expects explicit config/prompts/tools/agents/skills/features surfaces (`typescript/docs/config-reference.md:5-19`).
- The installer will be far easier to implement if both languages share a mostly identical project contract.

### Migration rule

Follow the same rule already written for TypeScript: introduce new boundaries first and move logic behind compatibility shims rather than doing a flag-day rewrite (`typescript/docs/architecture/core-runtime-boundaries.md:22-23`).

---

## Final Repository Structure

## 1) Authoring repo structure

```text
my-project/
├── typescript/
│   └── ...
├── python/
│   ├── pyproject.toml
│   ├── README.md
│   ├── docs/
│   │   ├── architecture.md
│   │   ├── migration.md
│   │   ├── config-reference.md
│   │   ├── extension-model.md
│   │   └── feature-packs.md
│   ├── src/
│   │   └── agent_boilerplate/
│   │       ├── __init__.py
│   │       ├── entrypoints/
│   │       │   ├── cli.py
│   │       │   ├── repl.py
│   │       │   └── headless.py
│   │       ├── core/
│   │       │   ├── engine.py
│   │       │   ├── runtime.py
│   │       │   ├── router.py
│   │       │   ├── session.py
│   │       │   ├── transcript.py
│   │       │   └── types.py
│   │       ├── config/
│   │       │   ├── models.py
│   │       │   ├── loader.py
│   │       │   ├── defaults.py
│   │       │   └── schema.py
│   │       ├── prompts/
│   │       │   ├── composer.py
│   │       │   ├── system.md
│   │       │   └── sections/
│   │       ├── registry/
│   │       │   ├── commands.py
│   │       │   ├── tools.py
│   │       │   ├── agents.py
│   │       │   ├── skills.py
│   │       │   └── features.py
│   │       ├── providers/
│   │       │   ├── base.py
│   │       │   ├── openai.py
│   │       │   ├── anthropic.py
│   │       │   └── registry.py
│   │       ├── commands/
│   │       │   ├── chat.py
│   │       │   ├── run.py
│   │       │   ├── doctor.py
│   │       │   ├── agents.py
│   │       │   ├── skills.py
│   │       │   ├── feature.py
│   │       │   └── config.py
│   │       ├── tools/
│   │       │   ├── file_read.py
│   │       │   ├── file_edit.py
│   │       │   ├── file_write.py
│   │       │   ├── bash.py
│   │       │   ├── web_fetch.py
│   │       │   ├── web_search.py
│   │       │   ├── mcp.py
│   │       │   └── shared.py
│   │       ├── permissions/
│   │       │   └── policy.py
│   │       ├── state/
│   │       │   ├── session_store.py
│   │       │   ├── memory_store.py
│   │       │   └── usage.py
│   │       ├── extensions/
│   │       │   ├── builtin_agents/
│   │       │   ├── builtin_skills/
│   │       │   ├── builtin_prompts/
│   │       │   └── builtin_features/
│   │       └── utils/
│   ├── templates/
│   │   └── base/
│   │       ├── agentkit.toml
│   │       ├── .agent/
│   │       │   ├── prompts/
│   │       │   │   ├── system.md
│   │       │   │   └── sections/
│   │       │   ├── agents/
│   │       │   │   ├── planner.agent.json
│   │       │   │   ├── executor.agent.json
│   │       │   │   └── reviewer.agent.json
│   │       │   ├── skills/
│   │       │   │   ├── plan/SKILL.md
│   │       │   │   ├── add-feature/SKILL.md
│   │       │   │   └── verify/SKILL.md
│   │       │   └── features/
│   │       │       ├── registry.json
│   │       │       └── github-pr-review/
│   │       │           ├── feature.json
│   │       │           ├── files/
│   │       │           └── patches/
│   │       ├── src/
│   │       │   └── app/
│   │       └── tests/
│   ├── examples/
│   │   ├── minimal/
│   │   └── coding-agent-with-mcp/
│   ├── tests/
│   │   ├── unit/
│   │   ├── integration/
│   │   ├── scaffold/
│   │   └── fixtures/
│   └── migration/
│       ├── audit_snapshot.py
│       ├── snapshots/
│       └── archive_notes.md
└── install/
    ├── Cargo.toml
    ├── src/
    │   ├── main.rs
    │   ├── cli.rs
    │   ├── catalog.rs
    │   ├── manifest.rs
    │   ├── renderer.rs
    │   ├── patch.rs
    │   ├── fs_ops.rs
    │   └── commands/
    │       ├── init.rs
    │       ├── feature.rs
    │       ├── doctor.rs
    │       ├── list.rs
    │       └── upgrade.rs
    └── tests/
```

## 2) Generated Python instance structure

```text
<user-project>/
├── pyproject.toml
├── agentkit.toml
├── README.md
├── .agent/
│   ├── prompts/
│   │   ├── system.md
│   │   └── sections/
│   ├── agents/
│   │   ├── planner.agent.json
│   │   ├── executor.agent.json
│   │   └── reviewer.agent.json
│   ├── skills/
│   │   ├── plan/SKILL.md
│   │   ├── add-feature/SKILL.md
│   │   └── verify/SKILL.md
│   ├── features/
│   │   ├── registry.json
│   │   └── <feature-id>/
│   └── memory/
├── src/
│   └── <package_name>/
│       ├── __init__.py
│       ├── app.py
│       ├── agents/
│       ├── skills/
│       └── prompts/
└── tests/
```

### Design rule

The authoring repo owns the reusable runtime and template assets. Generated projects own only:

- config
- local prompts
- local agents
- local skills
- local feature packs
- project app code

---

## What to Keep / Remove / Generalize from the Current Source

| Current file/module | Keep | Remove | Generalize into |
|---|---|---|---|
| `python/src/main.py:21-209` | argparse subcommand pattern | migration-only command surface | `entrypoints/cli.py` with `chat`, `run`, `doctor`, `agents`, `skills`, `feature`, `config` |
| `python/src/runtime.py:89-192` | routing/session orchestration idea | report-heavy bootstrap output shape | `core/runtime.py` + `core/router.py` |
| `python/src/query_engine.py:15-193` | session config, turn state, usage totals, transcript compaction | summary-only fake model output | real `core/engine.py` executing provider + tools |
| `python/src/session_store.py:8-26` | persisted session storage seam | `.port_sessions` naming | `state/session_store.py` with project-local storage policy |
| `python/src/transcript.py:7-22` | transcript append/compact/flush behavior | none | `core/transcript.py` |
| `python/src/permissions.py:7-18` | minimal deny-name / deny-prefix policy | none | `permissions/policy.py` |
| `python/src/models.py:6-40` | dataclass-first modeling | porting-specific names like `PortingModule` | `config/models.py`, `core/types.py`, `registry/types.py` |
| `python/src/commands.py:22-90` | registry loading pattern | JSON snapshot dependency | `registry/commands.py` + real command registration |
| `python/src/tools.py:23-96` | registry loading + permission filtering | JSON snapshot dependency | `registry/tools.py` + tool implementations |
| `python/src/execution_registry.py:8-42` | registry facade concept | mirrored string execution | real command/tool registry objects |
| `python/src/setup.py:12-77` | environment/setup reporting boundary | default prefetch side effects like keychain/MDM/project scan | `commands/doctor.py` + optional startup hooks |
| `python/src/system_init.py:7-20` | startup summary composition idea | static mirrored counts | `prompts/composer.py` + `commands/doctor.py` |
| `python/src/history.py:7-20` | simple event log | markdown-only history formatting | `state/history.py` or merge into session events |
| `python/src/parity_audit.py:7-138` | useful during migration only | should not ship in generated boilerplate | move to `python/migration/audit_snapshot.py` |
| `python/src/port_manifest.py:12-52` | useful during migration only | should not ship in generated boilerplate | move to `python/migration/` docs/tooling |
| `python/src/reference_data/*` | archive facts useful during migration | runtime dependency in v1 boilerplate | move to `python/migration/snapshots/` |
| placeholder subsystem packages like `python/src/assistant/__init__.py` and `python/src/utils/__init__.py` | none for runtime | placeholder-only archive metadata packages | delete after migration facts are preserved elsewhere |
| `python/src/QueryEngine.py:1-16` and `python/src/Tool.py:1-14` | only as short-lived compatibility shims | long-term duplicate naming surface | remove after new package layout is stable |
| `python/src/direct_modes.py:1-20`, `python/src/remote_runtime.py:1-24` | concepts only | placeholder runtime modes in default template | optional later feature pack (`remote`, `ssh`, `teleport`) |
| `python/tests/test_porting_workspace.py:15-244` | regression baseline and smoke intent | mirror-specific coverage assertions in final boilerplate | split into `tests/unit`, `tests/integration`, `tests/scaffold` |

### Immediate removals from the default boilerplate runtime

These should **not** be part of the shipped Python boilerplate v1:

1. `parity-audit` command (`python/src/main.py:24-30`, `104-106`)
2. `manifest`, `command-graph`, `tool-pool`, `bootstrap-graph` reporting commands (`python/src/main.py:24-30`, `110-118`)
3. snapshot-backed `commands`/`tools` commands (`python/src/main.py:34-46`, `123-141`) as the primary registry source
4. placeholder remote/direct mode commands (`python/src/main.py:68-77`, `171-185`)
5. archive metadata packages and `reference_data/` as runtime dependencies

### What should remain available only for maintainers

Move these under `python/migration/` rather than deleting immediately:

- snapshot JSON files
- parity audit logic
- archive notes
- one-off inventory scripts

That preserves migration traceability without polluting the runtime kernel.

---

## Customization Points

## 1) Canonical project config

Use a language-neutral root config file for generated instances:

`agentkit.toml`

This keeps the Rust installer simple and gives both Python and TypeScript a shared contract. The top-level sections should match the TypeScript config reference: `app`, `prompts`, `tools`, `providers`, `agents`, `skills`, `features` (`typescript/docs/config-reference.md:5-19`).

### `agentkit.toml` example

```toml
[app]
name = "my-agent"
package_name = "my_agent"
version = "0.1.0"
default_provider = "openai"
default_mode = "interactive"
working_directory_policy = "project-root"

[prompts]
system = ".agent/prompts/system.md"
sections = [
  ".agent/prompts/sections/coding-style.md",
  ".agent/prompts/sections/verification.md",
]
append = []

[tools]
enabled = ["file_read", "file_edit", "file_write", "bash", "web_fetch"]
approval_mode = "default"
deny = []
bash_timeout_ms = 15000
web_fetch_timeout_ms = 10000
max_tool_calls_per_turn = 12

[providers.openai]
api_key_env = "OPENAI_API_KEY"
model = "gpt-5.4"
base_url = ""

[providers.anthropic]
api_key_env = "ANTHROPIC_API_KEY"
model = "claude-sonnet-4-6"
base_url = ""

[agents]
directories = [".agent/agents"]
enabled = ["planner", "executor", "reviewer"]
default = "executor"

[skills]
directories = [".agent/skills"]
enabled = ["plan", "add-feature", "verify"]
auto_discover = true

[features]
enabled = []
registry = ".agent/features/registry.json"
```

### Python loader shape

File: `python/src/agent_boilerplate/config/models.py`

```python
from dataclasses import dataclass, field
from typing import Literal

ApprovalMode = Literal["default", "always-ask", "never-ask"]
ProviderId = Literal["openai", "anthropic", "local"]

@dataclass(frozen=True)
class ProviderSettings:
    api_key_env: str
    model: str
    base_url: str | None = None

@dataclass(frozen=True)
class AppConfig:
    name: str
    package_name: str
    version: str
    default_provider: ProviderId
    default_mode: Literal["interactive", "headless"]
    working_directory_policy: Literal["project-root", "cwd"] = "project-root"

@dataclass(frozen=True)
class PromptConfig:
    system: str
    sections: tuple[str, ...] = ()
    append: tuple[str, ...] = ()

@dataclass(frozen=True)
class ToolConfig:
    enabled: tuple[str, ...]
    approval_mode: ApprovalMode = "default"
    deny: tuple[str, ...] = ()
    bash_timeout_ms: int = 15000
    web_fetch_timeout_ms: int = 10000
    max_tool_calls_per_turn: int = 12

@dataclass(frozen=True)
class RegistryConfig:
    directories: tuple[str, ...]
    enabled: tuple[str, ...] = ()
    auto_discover: bool = True
    default: str | None = None

@dataclass(frozen=True)
class FeatureConfig:
    enabled: tuple[str, ...] = ()
    registry: str = ".agent/features/registry.json"

@dataclass(frozen=True)
class BoilerplateConfig:
    app: AppConfig
    prompts: PromptConfig
    tools: ToolConfig
    providers: dict[str, ProviderSettings]
    agents: RegistryConfig
    skills: RegistryConfig
    features: FeatureConfig = field(default_factory=FeatureConfig)
```

## 2) Prompt customization

Generated instances should modify prompts by editing files, not runtime source.

Files:

- `.agent/prompts/system.md`
- `.agent/prompts/sections/coding-style.md`
- `.agent/prompts/sections/verification.md`
- `.agent/prompts/sections/security.md`
- `.agent/prompts/sections/project-context.md`

Prompt composition order:

1. runtime default prompt bundled in `python/src/agent_boilerplate/prompts/system.md`
2. project `.agent/prompts/system.md`
3. section files from `agentkit.toml`
4. agent-specific prompt file
5. one-turn append prompt from CLI/API

## 3) Tool customization

Users should be able to change:

- enabled tools
- tool approval mode
- deny list / deny prefixes
- per-tool timeout
- max tool calls per turn
- default MCP server set
- feature-pack-added tools

## 4) Provider customization

Users should be able to change:

- default provider
- model per provider
- base URL per provider
- required env var names
- whether multi-provider fallback is enabled

## 5) Branding customization

Users should be able to change:

- app/binary name
- package name
- README template title
- default banner text
- docs/support URLs
- default system prompt tone

## 6) Project-policy customization

Users should be able to change:

- approval policy
- working directory policy
- whether web tools are enabled
- whether MCP is enabled by default
- whether background agents are allowed

---

## Agents & Skills Architecture

## Core principle

A generated project should support requests like:

- “add GitHub PR review support”
- “add a docs-writer agent”
- “add a planning skill”
- “turn on MCP tools”

without editing the runtime kernel.

## Recommended architecture: Local extensions + Feature packs

### A. Local agents

Location:

- `.agent/agents/*.agent.json`

Example manifest:

```json
{
  "id": "executor",
  "description": "Implements product features in this project",
  "prompt": ".agent/prompts/agents/executor.md",
  "tools": ["file_read", "file_edit", "file_write", "bash"],
  "disallowedTools": ["web_search"],
  "model": "inherit",
  "effort": "medium",
  "permissionMode": "default",
  "mcpServers": ["github"],
  "skills": ["plan", "verify"],
  "initialPrompt": "Read relevant files before editing.",
  "memory": "project",
  "background": false,
  "isolation": "same-worktree"
}
```

Python interface shape (`registry/agents.py`):

```python
@dataclass(frozen=True)
class AgentManifest:
    id: str
    description: str
    prompt: str
    tools: tuple[str, ...] = ()
    disallowed_tools: tuple[str, ...] = ()
    skills: tuple[str, ...] = ()
    model: str = "inherit"
    effort: str = "medium"
    permission_mode: str = "default"
    mcp_servers: tuple[str, ...] = ()
    initial_prompt: str | None = None
    memory: str = "project"
    background: bool = False
    isolation: str = "same-worktree"
```

### B. Local skills

Location:

- `.agent/skills/<skill-name>/SKILL.md`

Use Markdown + YAML frontmatter so both languages can parse the same skill format.

Example:

```yaml
---
name: add-feature
description: Add a feature pack to this project
whenToUse: Use when the user asks to add a new capability to this boilerplate instance
allowed-tools: [Read, Edit, Write, Bash]
argument-hint: <feature-id>
---

<Workflow instructions>
```

Python interface shape (`registry/skills.py`):

```python
@dataclass(frozen=True)
class SkillManifest:
    name: str
    description: str
    when_to_use: str | None = None
    allowed_tools: tuple[str, ...] = ()
    argument_hint: str | None = None
    body_markdown: str = ""
```

### C. Feature packs

Location:

- `.agent/features/registry.json`
- `.agent/features/<feature-id>/feature.json`

Example `feature.json`:

```json
{
  "id": "github-pr-review",
  "name": "GitHub PR Review",
  "description": "Adds GitHub review prompts, skills, and MCP bindings",
  "version": "0.1.0",
  "dependsOn": ["git-core"],
  "adds": {
    "agents": ["reviewer.agent.json"],
    "skills": ["review-pr"],
    "prompts": ["github-review.md"],
    "tools": ["mcp"],
    "env": ["GITHUB_TOKEN"]
  },
  "patches": [
    {
      "target": "agentkit.toml",
      "op": "merge",
      "path": "features.enabled",
      "value": ["github-pr-review"]
    }
  ]
}
```

Python interface shape (`registry/features.py`):

```python
@dataclass(frozen=True)
class FeaturePatch:
    target: str
    op: Literal["merge", "replace", "append"]
    path: str | None = None
    value: object | None = None

@dataclass(frozen=True)
class FeaturePackManifest:
    id: str
    name: str
    description: str
    version: str
    depends_on: tuple[str, ...] = ()
    adds: dict[str, tuple[str, ...]] = field(default_factory=dict)
    patches: tuple[FeaturePatch, ...] = ()
```

### D. Built-in `add-feature` skill

Ship one built-in skill that does this:

1. read `.agent/features/registry.json`
2. match requested feature IDs/aliases
3. copy template files from the selected feature pack
4. patch `agentkit.toml`
5. run validation (`python -m pytest` or `python -m unittest` depending on template)
6. report exactly what changed

### E. Loader precedence

Define one clear precedence order:

1. runtime built-ins
2. project-local `.agent/*`
3. enabled feature packs
4. CLI overrides

Do not add marketplace/network/plugin discovery in v1. Local packs are enough.

---

## install CLI (Rust) Commands and Flags

The Rust CLI should be the only canonical scaffolder. `install/` is currently empty, so treat it as a clean-slate implementation lane.

## CLI name

Use `aicd` unless you already have another public-facing brand chosen.

## Command set

### 1) Initialize a new project

```bash
aicd init <project-name> --language python [OPTIONS]
aicd init <project-name> --language typescript [OPTIONS]
```

### Required grammar

- `project-name` is positional
- `--language` is required
- v1 implementation target: **Python first**
- TypeScript support can share the CLI grammar even if implemented by a separate lane

### Recommended flags

- `--template <template-id>` — default: `coding-agent`
- `--provider <openai|anthropic|multi>` — default: `multi`
- `--tools <csv>` — e.g. `file_read,file_edit,file_write,bash,web_fetch,mcp`
- `--feature <feature-id>` — repeatable
- `--prompt-pack <default|strict|teaching|minimal>`
- `--output <dir>` — default: `./<project-name>`
- `--package-name <python_package>`
- `--binary-name <command_name>`
- `--with-example` — repeatable
- `--yes` — non-interactive
- `--dry-run`

### 2) Add a feature pack to an existing project

```bash
aicd feature add <feature-id> --project <dir> [--language auto|python|typescript] [--dry-run]
```

### 3) Remove a feature pack

```bash
aicd feature remove <feature-id> --project <dir> [--language auto|python|typescript] [--dry-run]
```

### 4) List supported catalog items

```bash
aicd list languages
aicd list templates [--language python|typescript]
aicd list features [--language python|typescript]
aicd list prompt-packs [--language python|typescript]
```

### 5) Validate a generated project

```bash
aicd doctor --project <dir> [--language auto|python|typescript]
```

Doctor checks:

- `agentkit.toml` exists and parses
- enabled agents exist
- enabled skills exist
- prompt files exist
- feature manifests exist
- required env vars are documented
- Python project installs and boots
- generated test command runs

### 6) Upgrade template metadata later

```bash
aicd upgrade --project <dir> [--to <version>] [--dry-run]
```

## Rust interface shapes

File: `install/src/cli.rs`

```rust
#[derive(Clone, Debug)]
pub enum Language {
    Python,
    Typescript,
}

#[derive(Clone, Debug)]
pub struct InitArgs {
    pub project_name: String,
    pub language: Language,
    pub template: String,
    pub provider: String,
    pub tools: Vec<String>,
    pub features: Vec<String>,
    pub prompt_pack: Option<String>,
    pub output: std::path::PathBuf,
    pub package_name: Option<String>,
    pub binary_name: Option<String>,
    pub with_examples: Vec<String>,
    pub yes: bool,
    pub dry_run: bool,
}

#[derive(Clone, Debug)]
pub struct FeatureArgs {
    pub feature_id: String,
    pub project: std::path::PathBuf,
    pub language: Option<Language>,
    pub dry_run: bool,
}

#[derive(Clone, Debug)]
pub struct DoctorArgs {
    pub project: std::path::PathBuf,
    pub language: Option<Language>,
}
```

## Installer module responsibilities

- `install/src/catalog.rs` — template/feature catalog
- `install/src/manifest.rs` — parse/render `agentkit.toml`, `feature.json`, `registry.json`
- `install/src/renderer.rs` — copy templates + render placeholders
- `install/src/patch.rs` — apply merge/replace/append operations
- `install/src/fs_ops.rs` — safe file creation, overwrite policy, dry-run diff output
- `install/src/commands/init.rs` — project initialization flow
- `install/src/commands/feature.rs` — add/remove feature pack flow
- `install/src/commands/doctor.rs` — validation flow

---

## Migration Steps Ordered by Priority

## Phase 0 — Freeze baseline and capture current behavior

**Goal:** preserve the parts that already work before reorganizing files.

1. Add `python/docs/migration.md` with:
   - current runtime entrypoints
   - keep/remove/generalize table
   - migration invariants
2. Split the current smoke coverage in `python/tests/test_porting_workspace.py:15-244` into clearer groups:
   - CLI smoke
   - session persistence
   - permission filtering
   - generated scaffold tests (future)
3. Keep the current test command green from `python/`:
   - `python -m unittest discover -s tests -v`
4. Tag or branch the current porting snapshot before moving files.

**Exit criteria**

- current smoke behavior is preserved
- maintainers can still inspect snapshot/mirror data
- there is a written migration map

## Phase 1 — Create the new Python runtime skeleton with no behavior change

**Goal:** introduce the final directory boundaries before moving logic.

1. Add `python/src/agent_boilerplate/entrypoints/`
2. Add `python/src/agent_boilerplate/core/`
3. Add `python/src/agent_boilerplate/config/`
4. Add `python/src/agent_boilerplate/prompts/`
5. Add `python/src/agent_boilerplate/registry/`
6. Add `python/src/agent_boilerplate/providers/`
7. Add `python/src/agent_boilerplate/commands/`
8. Add `python/src/agent_boilerplate/tools/`
9. Add temporary compatibility imports from the old files to the new modules.

**Exit criteria**

- package skeleton exists
- old imports still work through shims
- test suite still passes

## Phase 2 — Extract the actual reusable runtime kernel

**Goal:** move the code that is worth keeping.

Recommended move order:

1. `query_engine.py` → `core/engine.py`
2. `runtime.py` → `core/runtime.py`
3. `session_store.py` + `transcript.py` → `state/` + `core/transcript.py`
4. `permissions.py` → `permissions/policy.py`
5. `models.py` → split into `config/models.py`, `core/types.py`, `registry/types.py`
6. `main.py` dispatch → `entrypoints/cli.py`

Important rule: keep compatibility wrappers until the new tree is green.

**Exit criteria**

- the runtime kernel no longer imports `reference_data/`
- the new package can expose a minimal CLI and session lifecycle

## Phase 3 — Replace snapshot-backed registries with real registries

**Goal:** stop depending on mirror JSON files.

1. Replace `commands.py` snapshot loading (`python/src/commands.py:22-36`) with direct registration of actual Python command objects.
2. Replace `tools.py` snapshot loading (`python/src/tools.py:23-37`) with direct registration of actual Python tool objects.
3. Replace placeholder execution strings (`python/src/commands.py:75-80`, `python/src/tools.py:81-86`) with actual handlers.
4. Keep a migration-only export of snapshot data under `python/migration/` if needed for documentation.

**Exit criteria**

- `commands` and `tools` are no longer JSON mirrors
- the runtime can load only the enabled command/tool set from config

## Phase 4 — Externalize config and prompts

**Goal:** make the boilerplate customizable without touching kernel code.

1. Implement `agentkit.toml` parsing via `tomllib`
2. Implement config model validation
3. Move default system prompt into files
4. Implement prompt layering
5. Add per-project `.agent/prompts/` overrides
6. Add CLI/config overrides for model, provider, tools, and prompt pack

**Exit criteria**

- prompt text is file-based
- config changes alter runtime behavior without code edits

## Phase 5 — Implement agents, skills, and feature packs

**Goal:** let generated projects extend themselves.

1. Implement `.agent/agents/*.agent.json` loading
2. Implement `.agent/skills/*/SKILL.md` loading
3. Implement `.agent/features/registry.json` + `feature.json` loading
4. Ship a built-in `add-feature` skill
5. Add `feature add/remove` command flow in both Python runtime and Rust installer where appropriate

**Exit criteria**

- generated project can add a new agent by adding a manifest file
- generated project can add a new skill by adding a SKILL.md file
- generated project can apply a feature pack

## Phase 6 — Build the Python template and generated-project fixtures

**Goal:** freeze the scaffold contract before writing much installer logic.

1. Create `python/templates/base/`
2. Create example generated fixture under `python/tests/fixtures/generated-minimal/`
3. Add scaffold verification tests that instantiate a temp project from the template
4. Confirm the generated project boots, loads config, and finds agents/skills

**Exit criteria**

- a generated Python project works without manual surgery
- file structure is stable enough for the Rust installer

## Phase 7 — Implement the Rust installer against the frozen contract

**Goal:** make scaffolding reproducible.

1. Implement `aicd init --language python`
2. Implement `aicd list` catalog commands
3. Implement `aicd feature add/remove`
4. Implement `aicd doctor`
5. Add installer snapshot tests against known-good generated fixtures

**Important dependency rule**

Do not let the Rust installer invent the Python contract. The template contract must be frozen first.

## Phase 8 — Cross-language contract alignment

**Goal:** keep Python and TypeScript consistent enough that one installer can serve both.

1. Confirm the same top-level config sections exist in both languages
2. Confirm agent manifest format is the same
3. Confirm skill file format is the same
4. Confirm feature-pack manifest shape is the same
5. Confirm `aicd doctor` can validate both with mostly shared logic

---

## Recommended Minimal v1 Scope

Ship v1 Python boilerplate with:

- interactive/headless CLI entrypoint
- one or two providers max
- file read/edit/write + bash tools
- prompt layering
- local agent manifests
- local skill discovery
- local feature packs
- Rust installer support for `init`, `list`, `feature add/remove`, and `doctor`

Do **not** ship in v1:

- plugin marketplace
- remote/SSH/teleport modes
- background swarm/team runtime by default
- voice/desktop/mobile surfaces
- migration snapshot reporting as end-user commands

---

## Acceptance Criteria

- [ ] `python/` builds/runs as a reusable runtime package rather than a mirror workspace.
- [ ] The default runtime does not depend on `python/src/reference_data/*`.
- [ ] A generated Python project can change its system prompt by editing `.agent/prompts/system.md`.
- [ ] A generated Python project can add a skill by creating `.agent/skills/<name>/SKILL.md`.
- [ ] A generated Python project can add an agent by creating `.agent/agents/<name>.agent.json`.
- [ ] A generated Python project can add a feature pack without editing runtime internals.
- [ ] `agentkit.toml` / `aicd` config can enable/disable tools and providers.
- [ ] `aicd init --language python` generates a bootable project.
- [ ] `aicd doctor` can validate a generated Python project.
- [ ] Python and TypeScript share the same extension contract at the manifest level.

---

## Risks and Mitigations

### Risk 1: treating placeholder mirror behavior as real runtime behavior

**Mitigation:** explicitly replace snapshot-backed registries and placeholder execution strings before calling the output “boilerplate.”

### Risk 2: installer and runtime drifting apart

**Mitigation:** freeze `agentkit.toml`, agent manifests, skill format, and feature-pack format before deep installer work.

### Risk 3: cross-language config divergence

**Mitigation:** keep the top-level config sections identical across Python and TypeScript (`typescript/docs/config-reference.md:5-19`).

### Risk 4: too many extension mechanisms in v1

**Mitigation:** keep only three first-class extension surfaces in v1: local prompts, local agents/skills, local feature packs.

### Risk 5: migration tools leaking into the product

**Mitigation:** quarantine parity audit / manifest / snapshot tools under `python/migration/` and keep them out of the generated template.

---

## Verification Steps

### Current baseline verification

Run from `python/`:

```bash
python -m unittest discover -s tests -v
```

This currently passes and should remain your first regression gate during extraction.

### New verification matrix to add

1. **Unit**
   - config parser loads `agentkit.toml`
   - prompt composer resolves files in the right order
   - permission policy blocks denied tools
   - agent manifest parser validates required fields
   - skill frontmatter parser validates required fields

2. **Integration**
   - runtime boots with a minimal generated config
   - runtime discovers local agents and skills
   - `add-feature` applies files and patches config
   - provider registry resolves the selected provider/model

3. **Scaffold / installer**
   - `aicd init --language python demo-agent --dry-run`
   - `aicd init --language python demo-agent --yes`
   - boot generated project
   - run generated project smoke tests
   - `aicd doctor --project ./demo-agent`

4. **Cross-language contract**
   - same feature pack manifest applies to both Python and TypeScript
   - same agent manifest schema works in both
   - same skill format works in both

---

## Open Questions / Decisions You Need to Make

1. **Config contract:** do you want one canonical cross-language config file (`agentkit.toml`, recommended) or separate language-native configs?
2. **Provider scope:** should Python v1 ship with OpenAI only, Anthropic only, or both?
3. **Package tooling:** do you want plain `pyproject.toml` + stdlib only, or `uv`/`hatch`/`poetry` conventions?
4. **Default command surface:** do you want a terminal-first REPL, a headless `run` mode, or both in v1?
5. **Feature-pack format:** JSON vs TOML for feature manifests? I recommended JSON for pack manifests and TOML for root config.
6. **Installer naming:** should the Rust binary be `aicd`, `agentkit`, or another public name?
7. **Remote/MCP scope:** should MCP be built-in by default, optional by config, or a feature pack?
8. **Generated-project philosophy:** should users mostly edit `.agent/*` and app code, or should they also be expected to edit runtime internals?
9. **TypeScript alignment:** is the TS lane willing to adopt the same manifest formats, or do you need compatibility adapters?
10. **Licensing/provenance:** are there any redistribution constraints around turning the current source into a public boilerplate?

---

## Recommended First Milestone

If you want the fastest path to a usable result, make milestone 1:

> Generate a minimal Python coding-agent project with configurable prompts, configurable tools, local agents, local skills, and one sample feature pack, all scaffolded by `aicd init --language python`.

That milestone should explicitly exclude:

- remote/SSH/teleport runtime
- marketplace/plugins
- large UI surfaces
- swarm/team features by default
- deep TypeScript/Python symmetry beyond shared manifests

Once that milestone works, the TypeScript lane and the Rust installer can converge around a stable contract instead of guessing at a moving target.
