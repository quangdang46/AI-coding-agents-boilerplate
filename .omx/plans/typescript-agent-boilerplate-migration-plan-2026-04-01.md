# TypeScript Coding-Agent Boilerplate Migration Plan

_Date: 2026-04-01_

## Requirements Summary

You want to turn the current TypeScript coding-agent source into a reusable boilerplate under `typescript/`, preserve the parts that make the agent extensible, and prepare the repo for a future `rust/` boilerplate plus a Rust-based `install/` scaffolding CLI.

The current TypeScript source is a very large monolith: `src/utils` alone has 569 files, `src/components` 392, `src/commands` 208, `src/tools` 189, and `src/services` 139, which is far too large to present as a clean starter boilerplate without an extraction pass (`typescript/src/*` inventory). The runtime is centered around:

- CLI bootstrap: `typescript/src/entrypoints/cli.tsx:38-60`
- Query/session engine: `typescript/src/QueryEngine.ts:130-173`, `184-207`
- Command registry: `typescript/src/commands.ts:255-346`, `476-517`
- Tool registry: `typescript/src/tools.ts:193-250`, `271-327`
- Skill loading: `typescript/src/skills/loadSkillsDir.ts:67-94`, `185-206`
- Agent loading/schema: `typescript/src/tools/AgentTool/loadAgentsDir.ts:73-98`, `105-143`
- Prompt layering/custom prompts: `typescript/src/utils/systemPrompt.ts:28-40`, `41-123`; `typescript/src/QueryEngine.ts:284-325`
- Plugin loading during runtime init: `typescript/src/QueryEngine.ts:529-550`

The current repo also contains obvious non-boilerplate artifacts that should not survive into v1 of a clean template:

- product branding and snapshot docs (`typescript/README.md:1-31`, `52-75`)
- many telemetry/growthbook references (`typescript/src/bootstrap/state.ts:89-109`; `typescript/src/utils/config.ts:48-52`; `typescript/src/constants/prompts.ts:49-57`)
- native installer command specific to Claude Code (`typescript/src/commands/install.tsx:1-20`, `89-173`)
- duplicated snapshot mirror at `typescript/free-code-main/` (directory inventory)

---

## Recommended Target Architecture

### Decision

Do **not** try to ship the current TypeScript tree as the boilerplate with light renaming.

Instead, migrate in two layers:

1. **Extract a minimal reusable runtime kernel** from the current monolith.
2. **Wrap that kernel in a template/feature-pack system** so each generated project is customizable and self-extendable by agents/skills.

### Why this is the right cut

The current codebase already proves the important extension concepts:

- commands are registry-driven (`typescript/src/commands.ts:460-468`, `476-517`)
- tools are registry-driven (`typescript/src/tools.ts:193-250`)
- agents have a typed manifest (`typescript/src/tools/AgentTool/loadAgentsDir.ts:73-98`, `105-143`)
- skills are file-loaded and frontmatter-driven (`typescript/src/skills/loadSkillsDir.ts:67-94`, `185-206`)
- system prompts are layered (`typescript/src/utils/systemPrompt.ts:41-123`; `typescript/src/QueryEngine.ts:321-325`)

So the reusable product is **not** “all current files”; it is the **extension model plus the smallest runtime needed to execute it**.

---

## Final Repository Structure

## 1) Authoring repo structure (`my-project/`)

```text
my-project/
├── typescript/
│   ├── package.json
│   ├── tsconfig.json
│   ├── README.md
│   ├── docs/
│   │   ├── architecture.md
│   │   ├── migration.md
│   │   ├── feature-packs.md
│   │   └── config-reference.md
│   ├── src/
│   │   ├── entrypoints/
│   │   │   ├── cli.ts
│   │   │   ├── sdk.ts
│   │   │   └── mcp.ts
│   │   ├── core/
│   │   │   ├── engine/
│   │   │   │   ├── QueryEngine.ts
│   │   │   │   ├── Session.ts
│   │   │   │   ├── MessageBus.ts
│   │   │   │   └── types.ts
│   │   │   ├── prompts/
│   │   │   │   ├── PromptComposer.ts
│   │   │   │   ├── default-system.md
│   │   │   │   └── sections/
│   │   │   ├── config/
│   │   │   │   ├── schema.ts
│   │   │   │   ├── loadConfig.ts
│   │   │   │   └── defaults.ts
│   │   │   ├── registry/
│   │   │   │   ├── commands.ts
│   │   │   │   ├── tools.ts
│   │   │   │   ├── agents.ts
│   │   │   │   ├── skills.ts
│   │   │   │   └── features.ts
│   │   │   ├── providers/
│   │   │   │   ├── base.ts
│   │   │   │   ├── anthropic.ts
│   │   │   │   ├── openai.ts
│   │   │   │   └── registry.ts
│   │   │   ├── permissions/
│   │   │   ├── state/
│   │   │   └── telemetry/
│   │   │       ├── logger.ts
│   │   │       └── noopTelemetry.ts
│   │   ├── tools/
│   │   │   ├── file-read/
│   │   │   ├── file-edit/
│   │   │   ├── bash/
│   │   │   ├── web-fetch/
│   │   │   ├── web-search/
│   │   │   ├── mcp/
│   │   │   └── registry.ts
│   │   ├── commands/
│   │   │   ├── chat.ts
│   │   │   ├── run.ts
│   │   │   ├── init.ts
│   │   │   ├── doctor.ts
│   │   │   ├── agents.ts
│   │   │   ├── skills.ts
│   │   │   ├── feature.ts
│   │   │   └── config.ts
│   │   ├── ui/
│   │   │   ├── terminal/
│   │   │   └── headless/
│   │   ├── extensions/
│   │   │   ├── builtin-agents/
│   │   │   ├── builtin-skills/
│   │   │   ├── builtin-prompts/
│   │   │   └── builtin-features/
│   │   └── templates/
│   │       └── base/
│   │           ├── boilerplate.config.ts
│   │           ├── prompts/
│   │           ├── agents/
│   │           ├── skills/
│   │           ├── features/
│   │           └── src/
│   ├── examples/
│   │   ├── minimal/
│   │   └── coding-agent-with-mcp/
│   └── tests/
│       ├── unit/
│       ├── integration/
│       └── fixtures/
├── rust/
│   ├── README.md
│   ├── docs/
│   └── templates/
└── install/
    ├── Cargo.toml
    ├── src/
    │   ├── main.rs
    │   ├── cli.rs
    │   ├── manifest.rs
    │   ├── render.rs
    │   ├── fs_ops.rs
    │   └── commands/
    │       ├── init.rs
    │       ├── feature.rs
    │       ├── list.rs
    │       ├── doctor.rs
    │       └── upgrade.rs
    └── tests/
```

## 2) Generated TypeScript instance structure (what `install` should create)

```text
<user-project>/
├── package.json
├── tsconfig.json
├── README.md
├── boilerplate.config.ts
├── .agent/
│   ├── prompts/
│   │   ├── system.md
│   │   └── sections/
│   ├── agents/
│   │   ├── planner.agent.json
│   │   ├── executor.agent.json
│   │   └── reviewer.agent.json
│   ├── skills/
│   │   ├── plan/
│   │   │   └── SKILL.md
│   │   ├── add-feature/
│   │   │   └── SKILL.md
│   │   └── verify/
│   │       └── SKILL.md
│   ├── features/
│   │   ├── registry.json
│   │   └── github-pr-review/
│   │       ├── feature.json
│   │       ├── files/
│   │       └── patches/
│   └── memory/
├── src/
│   ├── app/
│   ├── providers/
│   ├── tools/
│   └── index.ts
└── tests/
```

**Key design rule:** the authoring repo owns the runtime and template assets; generated projects own only configuration, prompts, skills, agents, and optional feature packs.

---

## What to Keep / Remove / Generalize

| Current area | Keep | Remove | Generalize into |
|---|---|---|---|
| `typescript/src/entrypoints/cli.tsx` | CLI bootstrap pattern and fast-path dispatch | Claude-specific version/branding paths, proprietary fast paths | `src/entrypoints/cli.ts` with generic `chat`, `run`, `doctor`, `feature` subcommands |
| `typescript/src/QueryEngine.ts` | conversation lifecycle, tool execution loop, prompt assembly seam | direct coupling to current monolith state and legacy feature flags | `src/core/engine/QueryEngine.ts` + `src/core/engine/Session.ts` |
| `typescript/src/tools.ts` | registry-based tool composition | one-off/internal tools, product-specific gating | `src/core/registry/tools.ts` + `src/tools/*` |
| `typescript/src/commands.ts` | registry-based command loading + merge of builtin/skills/plugins | current huge slash-command surface | `src/core/registry/commands.ts` with a minimal stable command set |
| `typescript/src/skills/loadSkillsDir.ts` | file-based skill loading and frontmatter parsing | legacy `/commands` compatibility paths if not needed | `.agent/skills/*/SKILL.md` loader |
| `typescript/src/tools/AgentTool/loadAgentsDir.ts` | typed agent manifest shape | ant-only isolation modes and legacy compatibility unless needed | `.agent/agents/*.agent.json` loader |
| `typescript/src/utils/systemPrompt.ts` and `typescript/src/constants/prompts.ts` | layered prompt composition | hardcoded Claude/Anthropic copy and cyber-risk overlays | `PromptComposer` + user-editable prompt sections |
| `typescript/src/plugins/builtinPlugins.ts` | concept of feature bundles shipping skills/hooks/MCP together | marketplace-specific behaviors for v1 | local `feature-packs/` and `builtin-features/` |
| `typescript/src/utils/config.ts` | typed config model and settings separation | Claude Code legacy settings fields | `boilerplate.config.ts` + minimal runtime state config |
| `typescript/scripts/build.ts` | build entry and compile-time feature grouping | snapshot-specific defines and branding | generic build pipeline + template variants |
| `typescript/free-code-main/` | nothing | duplicate snapshot | delete immediately |
| `typescript/README.md`, `FEATURES.md`, `install.sh`, `changes.md` | only reusable docs ideas | free-code branding, snapshot history, one-off installer | rewrite as boilerplate docs and template install docs |
| telemetry / analytics / GrowthBook references | optional logging hooks only | all default telemetry/event reporting | `telemetry/noopTelemetry.ts` + optional adapter interface |
| `/install` command inside TS CLI | maybe as future convenience wrapper | current native Claude installer semantics | Rust `install` crate becomes the source of truth |

### Immediate deletions in the TypeScript source tree

1. `typescript/free-code-main/` — duplicate tree, guaranteed maintenance trap.
2. `typescript/assets/screenshot.png` — replace with boilerplate-neutral screenshot later.
3. `typescript/install.sh` — superseded by Rust `install` CLI.
4. snapshot-specific docs: `README.md`, `FEATURES.md`, `changes.md` — rewrite rather than mutate in place.

---

## Customization Points

## 1) Runtime config surface

Create `boilerplate.config.ts` in each generated project:

```ts
export interface BoilerplateConfig {
  app: {
    name: string
    version: string
    defaultProvider: 'anthropic' | 'openai'
    defaultMode: 'interactive' | 'headless'
    workingDirectoryPolicy: 'project-root' | 'cwd'
  }
  prompts: {
    systemPath: string
    appendPaths: string[]
    contextPaths: string[]
  }
  tools: {
    enabled: Array<
      'bash' | 'file_read' | 'file_edit' | 'file_write' | 'web_fetch' | 'web_search' | 'mcp'
    >
    defaults: {
      bashTimeoutMs: number
      webFetchTimeoutMs: number
      maxToolCallsPerTurn: number
    }
    policy: {
      approvalMode: 'default' | 'always-ask' | 'never-ask'
      deny: string[]
    }
  }
  providers: {
    anthropic?: {
      apiKeyEnv: string
      model: string
      baseUrl?: string
    }
    openai?: {
      apiKeyEnv: string
      model: string
      baseUrl?: string
    }
  }
  agents: {
    directories: string[]
    enabled: string[]
    defaultAgent?: string
  }
  skills: {
    directories: string[]
    enabled: string[]
    autoDiscover: boolean
  }
  features: {
    enabled: string[]
  }
}
```

## 2) Agent manifest surface

Reuse the shape already proven in `loadAgentsDir` (`typescript/src/tools/AgentTool/loadAgentsDir.ts:73-98`, `105-143`) and standardize generated-project files as `.agent/agents/*.agent.json`:

```json
{
  "description": "Implements product features in this project",
  "prompt": "@.agent/prompts/executor.md",
  "tools": ["file_read", "file_edit", "bash"],
  "disallowedTools": ["web_search"],
  "model": "inherit",
  "effort": "medium",
  "permissionMode": "default",
  "mcpServers": ["github"],
  "skills": ["plan", "verify"],
  "initialPrompt": "Start by reading the relevant files before editing.",
  "memory": "project",
  "background": false,
  "isolation": "worktree"
}
```

## 3) Skill surface

Keep Markdown-based skills because the current loader already supports file-based metadata well (`typescript/src/skills/loadSkillsDir.ts:185-206`). Standardize generated-project skills as `.agent/skills/<name>/SKILL.md`:

```yaml
---
name: add-feature
description: Add a feature pack to this project
whenToUse: Use when the user asks to add a capability to this boilerplate instance
allowed-tools: Read,Edit,Write,Bash
argument-hint: <feature-id>
---

<Instructions for the agent>
```

## 4) Prompt surface

Prompts should no longer be hardcoded product text. Replace hardcoded prompt sections with editable files:

- `.agent/prompts/system.md`
- `.agent/prompts/sections/coding-style.md`
- `.agent/prompts/sections/verification.md`
- `.agent/prompts/sections/security.md`

Prompt resolution order:

1. runtime default prompt shipped by template
2. project `system.md`
3. additional section files from config
4. agent-specific prompt
5. one-turn `appendSystemPrompt`

This preserves the layering model already visible in `typescript/src/utils/systemPrompt.ts:28-40`, `41-123` and `typescript/src/QueryEngine.ts:321-325`.

## 5) Tool selection surface

Expose these knobs in config and CLI:

- enabled tool set
- default tool presets
- per-tool timeout
- per-tool approval policy
- provider-specific tool availability
- feature-pack-added tools

## 6) Branding surface

Make these injectable instead of hardcoded:

- binary name
- package name
- welcome banner
- default system prompt title text
- docs links
- support links

---

## Agents & Skills Architecture

## Core principle

Users should be able to ask the generated agent things like:

- “add GitHub review support”
- “add a planning skill”
- “enable browser automation”
- “add a docs-writer agent”

without manually editing runtime internals.

### Recommended architecture: Feature Packs + Local Extensions

#### A. Local extensions

These live inside the generated project:

- `.agent/agents/*.agent.json`
- `.agent/skills/*/SKILL.md`
- `.agent/prompts/**/*.md`

These are the first-class user customization surfaces.

#### B. Feature packs

Each feature pack is an installable/addable bundle.

```json
{
  "id": "github-pr-review",
  "name": "GitHub PR Review",
  "description": "Adds GitHub review commands, a review skill, and a GitHub MCP binding",
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
      "target": "boilerplate.config.ts",
      "op": "merge",
      "path": "features.enabled",
      "value": ["github-pr-review"]
    }
  ]
}
```

#### C. Built-in `add-feature` skill

Ship a default skill that:

1. reads `.agent/features/registry.json`
2. identifies matching feature pack(s)
3. applies file templates/patches
4. updates `boilerplate.config.ts`
5. runs validation (`build`, optional tests)
6. reports what changed

That is the mechanism that enables “users can ask an agent to ADD features.”

## Recommended registry interfaces

```ts
export interface AgentManifest {
  agentType: string
  description: string
  prompt: string
  tools?: string[]
  disallowedTools?: string[]
  skills?: string[]
  model?: string
  effort?: 'low' | 'medium' | 'high'
  permissionMode?: 'default' | 'always-ask' | 'accept-edits'
  mcpServers?: Array<string | Record<string, unknown>>
  initialPrompt?: string
  memory?: 'user' | 'project' | 'local'
  background?: boolean
  isolation?: 'worktree'
}

export interface SkillManifest {
  name: string
  description: string
  whenToUse?: string
  allowedTools?: string[]
  argumentHint?: string
  disableModelInvocation?: boolean
}

export interface FeaturePackManifest {
  id: string
  name: string
  description: string
  version: string
  dependsOn?: string[]
  adds: {
    agents?: string[]
    skills?: string[]
    prompts?: string[]
    tools?: string[]
    env?: string[]
  }
  patches?: Array<{
    target: string
    op: 'merge' | 'replace' | 'append'
    path?: string
    value?: unknown
  }>
}
```

## What to support in v1 vs later

### v1
- local agents
- local skills
- local prompt sections
- local feature packs
- provider selection via config
- basic MCP integration

### Later
- marketplace/plugin ecosystem
- remote install/update of feature packs
- agent pack publishing
- visual pack browser

This reduces initial scope while preserving the extension path already hinted at by bundled skills and built-in plugins (`typescript/src/skills/bundledSkills.ts:15-41`, `75-99`; `typescript/src/plugins/builtinPlugins.ts:1-18`, `104-121`).

---

## install CLI (Rust) Commands and Flags

The Rust CLI should become the only canonical installer/scaffolder. The current TypeScript `/install` flow is product-specific and should not be reused directly (`typescript/src/commands/install.tsx:89-173`).

## Command set

### 1) Initialize a project

```bash
agentkit init typescript <project-name> [OPTIONS]
```

**Flags**

- `--template <template-id>` — default: `coding-agent`
- `--provider <anthropic|openai|multi>` — default: `multi`
- `--package-manager <bun|pnpm|npm>` — default: `bun`
- `--ui <terminal|headless>` — default: `terminal`
- `--tools <csv>` — e.g. `bash,file_read,file_edit,web_fetch,mcp`
- `--feature <feature-id>` — repeatable
- `--prompt-pack <pack-id>` — e.g. `default`, `strict`, `teaching`
- `--output <dir>` — default: `./<project-name>`
- `--binary-name <name>`
- `--yes` — non-interactive
- `--dry-run`

### 2) Add a feature pack to an existing project

```bash
agentkit feature add <feature-id> [--project <dir>] [--dry-run]
```

### 3) Remove a feature pack

```bash
agentkit feature remove <feature-id> [--project <dir>] [--dry-run]
```

### 4) List templates/features/providers

```bash
agentkit list templates
agentkit list features
agentkit list providers
```

### 5) Validate / doctor

```bash
agentkit doctor [--project <dir>]
```

Checks:
- config file exists and parses
- required env vars are documented
- selected tools exist
- referenced agent/skill/prompt files exist
- template-generated project builds

### 6) Upgrade template metadata

```bash
agentkit upgrade [--project <dir>] [--to <version>] [--dry-run]
```

## Rust module layout

```rust
pub struct InitArgs {
    pub language: String,
    pub project_name: String,
    pub template: String,
    pub provider: String,
    pub package_manager: String,
    pub ui: String,
    pub tools: Vec<String>,
    pub features: Vec<String>,
    pub prompt_pack: Option<String>,
    pub output: PathBuf,
    pub yes: bool,
    pub dry_run: bool,
}

pub struct FeatureAddArgs {
    pub feature_id: String,
    pub project: PathBuf,
    pub dry_run: bool,
}
```

## Important constraint

Implement **only `init typescript` first**. Define the CLI grammar so `init rust` can be added later, but do not let future Rust support complicate the TypeScript migration.

---

## Migration Steps Ordered by Priority

## Phase 0 — Freeze and inventory the current TypeScript source

**Goal:** prevent accidental migration of junk.

1. Create `typescript/docs/migration.md` with a source inventory and a “keep/remove/generalize” table.
2. Delete the duplicate `typescript/free-code-main/` snapshot.
3. Tag the current snapshot as `snapshot/pre-boilerplate-extraction`.
4. Add smoke tests for current “must keep” behaviors:
   - CLI starts
   - one-shot prompt works
   - custom system prompt can be injected
   - skill loading from disk works
   - agent manifest loading works

**Deliverable:** a stable baseline before structural extraction.

## Phase 1 — Carve out the runtime kernel

**Goal:** isolate the reusable engine from the product-specific monolith.

1. Create new runtime boundaries under `typescript/src/core/`:
   - `core/engine`
   - `core/config`
   - `core/registry`
   - `core/providers`
2. Move or wrap these first:
   - `QueryEngine`
   - prompt composition
   - config loading
   - command/tool loading
   - agent/skill loading
3. Keep compatibility shims temporarily so old imports still compile while the tree is being reorganized.

**Priority order inside Phase 1**

1. `QueryEngine`
2. prompt builder
3. tool registry
4. command registry
5. skill loader
6. agent loader
7. provider adapters

## Phase 2 — Remove product-specific surfaces

**Goal:** strip anything that is not boilerplate-worthy.

Remove or defer:
- telemetry / analytics / GrowthBook
- Claude-specific install/update UX
- desktop/mobile/voice/chrome-specific features
- issue/share/product support commands
- remote-control and bridge features unless you explicitly want them in the template
- bundled experimental feature flags that are not part of the reusable kernel

At the end of this phase, the default binary should present only a minimal, documented surface.

## Phase 3 — Define and implement the boilerplate config contract

**Goal:** make customization explicit and file-based.

1. Create `typescript/src/core/config/schema.ts`
2. Create `typescript/src/core/config/loadConfig.ts`
3. Create `typescript/src/templates/base/boilerplate.config.ts`
4. Make these runtime behaviors config-driven:
   - provider selection
   - enabled tools
   - prompt file paths
   - enabled agents/skills
   - feature packs
   - binary/app branding

## Phase 4 — Externalize prompts, agents, and skills

**Goal:** make the template modifiable without touching runtime source.

1. Move built-in prompt text into files.
2. Create `.agent/prompts`, `.agent/agents`, `.agent/skills` in the base template.
3. Change the loader precedence to:
   - built-in runtime defaults
   - template files
   - project files
   - feature-pack additions
4. Ensure the runtime reports missing manifest/prompt files clearly.

## Phase 5 — Add feature-pack architecture

**Goal:** support “add a feature to this boilerplate instance.”

1. Create `feature.json` manifest schema.
2. Create `.agent/features/registry.json` in generated projects.
3. Add runtime feature-pack discovery.
4. Add built-in `add-feature` skill.
5. Add CLI support: `agentkit feature add/remove/list`.

## Phase 6 — Shrink and re-curate the command/tool surface

**Goal:** make the template feel intentional.

Recommended default commands:
- `chat`
- `run`
- `init`
- `doctor`
- `agents`
- `skills`
- `feature`
- `config`

Recommended default tools:
- file read
- file edit
- file write
- bash
- web fetch
- optional web search
- optional MCP

Everything else should be feature-gated or removed from v1.

## Phase 7 — Create the Rust installer

**Goal:** scaffold generated projects from the TypeScript template.

1. Create `install/Cargo.toml`
2. Implement `agentkit init typescript`
3. Implement feature-pack application in Rust
4. Implement `doctor`
5. Add snapshot tests that compare generated output to expected fixtures

## Phase 8 — Docs, examples, verification, and release process

**Goal:** make the boilerplate usable by other people.

1. Rewrite `typescript/README.md` for the boilerplate.
2. Add `typescript/examples/minimal` and `typescript/examples/coding-agent-with-mcp`.
3. Add `install` README with exact CLI examples.
4. Add migration notes for future `rust/` support.

---

## Recommended Implementation Sequence by File/Module

### First extraction batch

1. `typescript/src/QueryEngine.ts`
2. `typescript/src/utils/systemPrompt.ts`
3. `typescript/src/constants/prompts.ts`
4. `typescript/src/tools.ts`
5. `typescript/src/commands.ts`
6. `typescript/src/skills/loadSkillsDir.ts`
7. `typescript/src/tools/AgentTool/loadAgentsDir.ts`
8. `typescript/src/utils/config.ts`
9. `typescript/scripts/build.ts`

### Second extraction batch

- provider adapters
- minimal REPL/headless UI glue
- permission handling
- MCP integration

### Last batch / optional

- plugin system
- background teams/swarm
- remote bridge
- voice/desktop/mobile

---

## Acceptance Criteria

- [ ] `typescript/` builds as a clean boilerplate runtime with no duplicate snapshot tree.
- [ ] A generated TypeScript instance can change its system prompt without editing runtime source.
- [ ] A generated instance can add a skill by creating `.agent/skills/<name>/SKILL.md`.
- [ ] A generated instance can add an agent by creating `.agent/agents/<name>.agent.json`.
- [ ] A generated instance can add a feature pack via CLI or built-in skill.
- [ ] Runtime config selects providers, tools, and enabled extension directories.
- [ ] Rust `install` CLI can generate a working TypeScript project with `init typescript`.
- [ ] `agentkit doctor` validates a generated project successfully.

---

## Risks and Mitigations

### Risk 1: extracting from a huge monolith without breaking runtime seams

**Mitigation:** use compatibility wrappers for one phase; do not do a flag-day rename.

### Risk 2: carrying too much Claude/product-specific baggage into the boilerplate

**Mitigation:** require every retained module to justify itself as either runtime kernel, template asset, or feature pack.

### Risk 3: extension architecture becomes too complex for end users

**Mitigation:** keep v1 to three concepts only: config, local agents/skills, feature packs.

### Risk 4: Rust installer outruns TypeScript extraction

**Mitigation:** do not start `install/` implementation until the TypeScript template directory layout and config contract are frozen.

### Risk 5: legal/licensing ambiguity around redistributing extracted source

**Mitigation:** treat licensing and provenance as a release gate, not an afterthought.

---

## Open Questions / Decisions You Need to Make

1. **Runtime target:** Do you want to stay on **Bun** for v1, or migrate to Node + pnpm/npm? The current build is tightly coupled to Bun (`typescript/package.json:7`, `16-20`; `typescript/scripts/build.ts:158-187`).
2. **UI scope:** Do you want a full Ink terminal UI in v1, or is a minimal headless/REPL shell enough? The current UI surface is massive (`src/components` 392 files).
3. **Provider scope:** Should v1 ship with Anthropic only, OpenAI only, or both?
4. **Plugin strategy:** Do you need a full plugin marketplace, or are **local feature packs** enough for the first release?
5. **MCP scope:** Is MCP a required default feature, or an optional pack?
6. **Team/subagent features:** Should coordinated team/swarm behavior ship in v1, or be deferred as a later pack?
7. **Generated-project philosophy:** Should generated instances be **apps** users edit directly, or **framework consumers** that mostly customize config and extension folders?
8. **Backward compatibility:** Do you need to preserve any current command names or file formats from the existing source?
9. **Prompt policy:** Do you want the default template to ship with a strong opinionated coding-agent prompt, or a very thin prompt that users layer themselves?
10. **Licensing/provenance:** Are you comfortable shipping this as a public boilerplate based on the current source origin, or is this intended for private/internal use only?

---

## Verification Steps for the Migration Project

1. `bun install`
2. `bun run build`
3. `bun run dev -- --help` (or the equivalent new CLI help path)
4. fixture test: generate a project with the future Rust CLI and ensure it boots
5. fixture test: add a feature pack and confirm config + files update correctly
6. fixture test: add a local skill and local agent in the generated project and confirm both are discovered
7. integration test: custom prompt override changes runtime prompt composition

---

## Recommended First Milestone (what I would do first)

If you want the fastest path to value, define **one hard milestone**:

> “Generate a minimal TypeScript coding-agent project with configurable prompts, configurable tools, local skills, local agents, and one sample feature pack.”

That milestone excludes:
- marketplace plugins
- remote/team features
- voice/mobile/desktop
- telemetry
- native updater/install UX
- Rust template generation

Once that milestone works, the Rust `install` CLI becomes straightforward because it will only need to render a stable template contract rather than guess how to scaffold an unstable runtime.

---

## Appendix A — Phase-by-Phase Execution Checklist

This appendix turns the architecture plan into an execution checklist you can hand to an implementation team.

### Phase 0 — Snapshot cleanup and proof harness

#### Deliverables
- `typescript/docs/migration.md`
- `typescript/tests/smoke/cli-start.test.ts`
- `typescript/tests/smoke/prompt-override.test.ts`
- `typescript/tests/smoke/skill-load.test.ts`
- `typescript/tests/smoke/agent-load.test.ts`
- removal of `typescript/free-code-main/`

#### Tasks
- [ ] Create `typescript/docs/migration.md` with four sections:
  - current runtime entrypoints
  - extension points to preserve
  - product-specific surfaces to remove
  - migration invariants
- [ ] Delete `typescript/free-code-main/`
- [ ] Mark current snapshot with a git tag
- [ ] Add a minimal smoke-test harness around:
  - CLI boot
  - one-shot prompt execution
  - custom prompt layering
  - skill discovery from a temp `.agent/skills/`
  - agent discovery from a temp `.agent/agents/`
- [ ] Verify `bun run build` still works before any architectural moves

#### Exit criteria
- [ ] duplicate tree removed
- [ ] no runtime behavior changed yet
- [ ] core extension seams have tests

### Phase 1 — New directory skeleton without behavior change

#### Deliverables
- `typescript/src/core/engine/`
- `typescript/src/core/prompts/`
- `typescript/src/core/config/`
- `typescript/src/core/registry/`
- `typescript/src/core/providers/`

#### Tasks
- [ ] Create new target directories
- [ ] Add placeholder barrels:
  - `typescript/src/core/engine/index.ts`
  - `typescript/src/core/prompts/index.ts`
  - `typescript/src/core/config/index.ts`
  - `typescript/src/core/registry/index.ts`
  - `typescript/src/core/providers/index.ts`
- [ ] Keep old modules intact; only add re-export shims first
- [ ] Add architecture doc notes for each new boundary

#### Exit criteria
- [ ] tree exists
- [ ] build still green
- [ ] no logic moved yet beyond shims

### Phase 2 — Extract prompt/config/provider seams

#### Deliverables
- `typescript/src/core/prompts/PromptComposer.ts`
- `typescript/src/core/prompts/default-system.md`
- `typescript/src/core/config/schema.ts`
- `typescript/src/core/config/loadConfig.ts`
- `typescript/src/core/providers/base.ts`
- `typescript/src/core/providers/registry.ts`

#### Tasks
- [ ] Pull prompt layering out of current hardcoded utilities
- [ ] Replace hardcoded system prompt text with file-backed sections
- [ ] Define `BoilerplateConfig` schema in one place
- [ ] Build a config loader that merges:
  - template defaults
  - project config
  - env overrides
  - CLI overrides
- [ ] Create provider interface:

```ts
export interface ModelProvider {
  id: string
  createClient(config: ProviderConfig): Promise<ProviderClient>
  resolveModel(input: string): string
  validateAuth(): Promise<ProviderAuthStatus>
}
```

- [ ] Move Anthropic/OpenAI-specific logic behind provider adapters
- [ ] Defer Bedrock/Vertex/Foundry unless explicitly required for v1

#### Exit criteria
- [ ] prompt building works from files
- [ ] config parses from `boilerplate.config.ts`
- [ ] provider choice no longer leaks through the whole runtime

### Phase 3 — Extract registries

#### Deliverables
- `typescript/src/core/registry/commands.ts`
- `typescript/src/core/registry/tools.ts`
- `typescript/src/core/registry/skills.ts`
- `typescript/src/core/registry/agents.ts`

#### Tasks
- [ ] Split registry construction from command/tool implementations
- [ ] Normalize command registration shape
- [ ] Normalize tool registration shape
- [ ] Preserve disk-loaded skills
- [ ] Preserve JSON/manifest-loaded agents
- [ ] Define loader precedence explicitly

#### Loader precedence
1. built-in runtime defaults
2. template-provided extensions
3. project local extensions
4. feature-pack extensions

#### Exit criteria
- [ ] command list comes from new registry layer
- [ ] tool list comes from new registry layer
- [ ] skills and agents can be discovered from `.agent/`

### Phase 4 — Extract the engine kernel

#### Deliverables
- `typescript/src/core/engine/QueryEngine.ts`
- `typescript/src/core/engine/Session.ts`
- `typescript/src/core/engine/MessageBus.ts`
- `typescript/src/core/engine/types.ts`

#### Tasks
- [ ] Move conversation loop into `core/engine`
- [ ] Separate UI concerns from engine concerns
- [ ] Separate app-state access from engine contracts
- [ ] Introduce a thinner engine config shape

```ts
export interface EngineRuntime {
  config: BoilerplateConfig
  tools: ToolDefinition[]
  commands: CommandDefinition[]
  agents: AgentManifest[]
  skills: SkillManifest[]
  provider: ModelProvider
}
```

- [ ] Keep adapter shims from old imports until Phase 6

#### Exit criteria
- [ ] engine can run without the current full REPL stack
- [ ] headless mode can execute against the new kernel

### Phase 5 — Curate the default template surface

#### Deliverables
- `typescript/src/templates/base/`
- generated `.agent/` starter content
- minimal default commands and tools

#### Tasks
- [ ] Create base template contents
- [ ] Add starter agents:
  - planner
  - executor
  - reviewer
- [ ] Add starter skills:
  - plan
  - verify
  - add-feature
- [ ] Add starter prompts:
  - system
  - verification
  - style
- [ ] Remove product-only commands from default build
- [ ] Remove product-only tools from default build

#### Exit criteria
- [ ] generated project is understandable without the upstream monolith context
- [ ] default surface area is intentionally small

### Phase 6 — Feature-pack system

#### Deliverables
- `typescript/src/core/registry/features.ts`
- `typescript/src/extensions/builtin-features/`
- `.agent/features/registry.json`
- feature-pack manifest schema

#### Tasks
- [ ] Define `FeaturePackManifest`
- [ ] Implement local feature discovery
- [ ] Implement feature application pipeline
- [ ] Implement built-in `add-feature` skill
- [ ] Add at least one sample feature pack:
  - `github-pr-review`
  - or `mcp-browser-automation`

#### Exit criteria
- [ ] feature pack can add files and config
- [ ] feature pack can be requested by an agent/user workflow

### Phase 7 — Rust installer (`init typescript` only)

#### Deliverables
- `install/Cargo.toml`
- `install/src/main.rs`
- `install/src/cli.rs`
- `install/src/commands/init.rs`
- `install/src/commands/feature.rs`
- `install/tests/snapshots/`

#### Tasks
- [ ] implement `agentkit init typescript`
- [ ] render base template to disk
- [ ] apply selected feature packs
- [ ] write `boilerplate.config.ts`
- [ ] add `doctor` validation
- [ ] add snapshot tests for generation results

#### Exit criteria
- [ ] new project can be generated end-to-end
- [ ] generated project boots successfully

### Phase 8 — Final cleanup and docs

#### Deliverables
- final `typescript/README.md`
- `typescript/docs/config-reference.md`
- `typescript/docs/feature-packs.md`
- `install/README.md`

#### Tasks
- [ ] rewrite README for boilerplate users, not snapshot readers
- [ ] document config, prompt, agent, skill, feature-pack contracts
- [ ] add examples
- [ ] remove obsolete compatibility shims

#### Exit criteria
- [ ] repo reads like a boilerplate product
- [ ] old product/snapshot wording is gone

---

## Appendix B — Concrete Source → Target Migration Map

This is the recommended move/reshape map for the existing `typescript/` tree.

### 1) Entry points

| Current | Target | Action |
|---|---|---|
| `typescript/src/entrypoints/cli.tsx` | `typescript/src/entrypoints/cli.ts` | keep bootstrap idea, rewrite surface |
| `typescript/src/entrypoints/mcp.ts` | `typescript/src/entrypoints/mcp.ts` | keep if MCP remains first-class |
| `typescript/src/entrypoints/init.ts` | `typescript/src/commands/init.ts` or remove | re-evaluate; likely fold into command layer |
| `typescript/src/entrypoints/sdk/*` | `typescript/src/entrypoints/sdk.ts` + `src/core/engine/types.ts` | reduce surface and re-export only public SDK contracts |

### 2) Engine / orchestration

| Current | Target | Action |
|---|---|---|
| `typescript/src/QueryEngine.ts` | `typescript/src/core/engine/QueryEngine.ts` | move with minimal behavior change |
| `typescript/src/query.ts` | `typescript/src/core/engine/submitQuery.ts` | split orchestration helper from legacy wrapper |
| `typescript/src/context.ts` | `typescript/src/core/prompts/contextBuilder.ts` | move context assembly concerns |
| `typescript/src/history.ts` | `typescript/src/core/engine/history.ts` | keep if still needed in kernel |
| `typescript/src/projectOnboardingState.ts` | remove or template onboarding module | product UX, not kernel |

### 3) Prompt system

| Current | Target | Action |
|---|---|---|
| `typescript/src/utils/systemPrompt.ts` | `typescript/src/core/prompts/PromptComposer.ts` | preserve layering logic |
| `typescript/src/constants/prompts.ts` | `typescript/src/core/prompts/default-system.md` + `sections/*.md` | convert hardcoded text to files |
| `typescript/src/constants/systemPromptSections.ts` | `typescript/src/core/prompts/sections.ts` | keep only generic section machinery |
| `typescript/src/utils/queryContext.ts` | `typescript/src/core/prompts/queryContext.ts` | retain if still needed |

### 4) Config / settings

| Current | Target | Action |
|---|---|---|
| `typescript/src/utils/config.ts` | `typescript/src/core/config/loadConfig.ts` + `schema.ts` | split giant config object |
| `typescript/src/bootstrap/state.ts` | `typescript/src/core/state/sessionState.ts` | keep only runtime-session essentials |
| `typescript/src/state/*` | `typescript/src/ui/state/*` or `src/core/state/*` | split UI state from engine state |

### 5) Commands

| Current | Target | Action |
|---|---|---|
| `typescript/src/commands.ts` | `typescript/src/core/registry/commands.ts` | preserve merge logic, shrink default set |
| `typescript/src/commands/plan` | `typescript/src/extensions/builtin-skills/plan/` | likely better as skill-first |
| `typescript/src/commands/review*` | feature pack or builtin skill | not required for v1 core |
| `typescript/src/commands/install.tsx` | remove; replace with Rust CLI | do not port directly |
| `typescript/src/commands/skills/*` | keep minimal `skills` listing command | useful in template |
| `typescript/src/commands/agents/*` | keep minimal `agents` listing command | useful in template |
| `typescript/src/commands/plugin/*` | defer | replace with local feature packs in v1 |
| `typescript/src/commands/help`, `doctor`, `config` | keep simplified variants | good default admin surface |

### 6) Tools

| Current | Target | Action |
|---|---|---|
| `typescript/src/tools.ts` | `typescript/src/core/registry/tools.ts` | keep registry logic |
| `typescript/src/tools/FileReadTool/*` | `typescript/src/tools/file-read/*` | keep |
| `typescript/src/tools/FileEditTool/*` | `typescript/src/tools/file-edit/*` | keep |
| `typescript/src/tools/FileWriteTool/*` | `typescript/src/tools/file-write/*` | keep |
| `typescript/src/tools/BashTool/*` | `typescript/src/tools/bash/*` | keep |
| `typescript/src/tools/WebFetchTool/*` | `typescript/src/tools/web-fetch/*` | keep |
| `typescript/src/tools/WebSearchTool/*` | `typescript/src/tools/web-search/*` | optional in base template |
| `typescript/src/tools/SkillTool/*` | `typescript/src/core/registry/skills.ts` + `src/tools/skill-runner/*` | keep semantics, slim implementation |
| `typescript/src/tools/AgentTool/*` | `typescript/src/core/registry/agents.ts` + `src/tools/agent-runner/*` | keep semantics, slim implementation |
| `typescript/src/tools/Team*`, `Task*`, `Workflow*`, `Sleep*`, `RemoteTrigger*` | defer to future packs | too much v1 scope |
| `typescript/src/tools/ListMcpResourcesTool/*`, `ReadMcpResourceTool/*` | keep if MCP is in base |

### 7) Skills / agents / plugins

| Current | Target | Action |
|---|---|---|
| `typescript/src/skills/loadSkillsDir.ts` | `typescript/src/core/registry/skills.ts` | keep core loader behavior |
| `typescript/src/skills/bundledSkills.ts` | `typescript/src/extensions/builtin-skills/index.ts` | retain bundled-skill concept |
| `typescript/src/tools/AgentTool/loadAgentsDir.ts` | `typescript/src/core/registry/agents.ts` | keep manifest loading |
| `typescript/src/plugins/builtinPlugins.ts` | `typescript/src/extensions/builtin-features/index.ts` | reframe plugin bundles as feature packs |
| `typescript/src/plugins/*` | defer most of it | only keep what directly supports local feature bundles |

### 8) Providers / API layer

| Current | Target | Action |
|---|---|---|
| `typescript/src/services/api/claude.ts` | `typescript/src/core/providers/anthropic.ts` | extract provider-specific client |
| `typescript/src/services/api/codex-fetch-adapter.ts` | `typescript/src/core/providers/openai.ts` | extract OpenAI/Codex adapter |
| `typescript/src/services/api/client.ts` | `typescript/src/core/providers/httpClient.ts` | generic transport utility |
| `typescript/src/utils/model/providers.ts` | `typescript/src/core/providers/providerSelect.ts` | keep provider selection logic |
| `typescript/src/utils/model/model.ts` | `typescript/src/core/providers/modelResolver.ts` | split generic vs provider-specific model logic |
| `typescript/src/utils/auth.ts` | `typescript/src/core/providers/auth.ts` | extract only auth pieces needed for selected providers |
| `typescript/src/utils/model/bedrock.ts` | later feature pack or later provider | defer unless required |

### 9) UI

| Current | Target | Action |
|---|---|---|
| `typescript/src/screens/REPL.tsx` | `typescript/src/ui/terminal/REPL.tsx` | keep only if terminal UI is in v1 |
| `typescript/src/components/*` | `typescript/src/ui/terminal/components/*` | aggressively prune |
| `typescript/src/ink*` | keep only if Ink UI is retained |
| `typescript/src/dialogLaunchers.tsx`, `replLauncher.tsx`, `interactiveHelpers.tsx` | `typescript/src/ui/terminal/*` | keep only if needed |

### 10) Remove or defer wholesale for v1

These directories should be treated as **defer-by-default** unless you have a strong product reason:

- `typescript/src/bridge/`
- `typescript/src/buddy/`
- `typescript/src/voice/`
- `typescript/src/remote/`
- `typescript/src/upstreamproxy/`
- `typescript/src/coordinator/`
- `typescript/src/native-ts/`
- most of `typescript/src/components/`
- large parts of `typescript/src/hooks/`
- large parts of `typescript/src/services/analytics/`

---

## Appendix C — Suggested First Refactor PR Stack

To keep reviewable diffs, use this PR order.

### PR 1 — Snapshot cleanup
- delete `typescript/free-code-main/`
- add migration doc
- add smoke tests

### PR 2 — Core skeleton
- add `src/core/*`
- add shims
- no runtime behavior change

### PR 3 — Prompt externalization
- move prompt text into files
- add `PromptComposer`
- keep old entrypoints delegating to it

### PR 4 — Config contract
- add `BoilerplateConfig`
- add loader
- wire config into current startup path

### PR 5 — Registry extraction
- split `commands.ts` / `tools.ts` into registry modules
- preserve current behavior

### PR 6 — Agent + skill externalization
- introduce `.agent/` conventions
- support local project prompts/agents/skills

### PR 7 — Minimal template mode
- hide/remove non-v1 commands and tools
- add base template assets

### PR 8 — Feature packs
- add manifest, registry, and `add-feature`

### PR 9 — Rust installer bootstrap
- create `install/`
- implement `agentkit init typescript`

### PR 10 — Docs/examples/final cleanup
- rewrite README
- add examples
- remove old shims

---

## Appendix D — Non-Negotiable Invariants During Migration

- [ ] The engine must keep prompt layering support.
- [ ] Skills must remain file-backed and user-editable.
- [ ] Agents must remain manifest-backed and user-editable.
- [ ] Runtime config must be explicit and typed.
- [ ] Template-generated projects must not require editing internal runtime files for normal customization.
- [ ] The Rust CLI must render from stable template assets, not from ad hoc source-tree assumptions.
- [ ] The duplicate snapshot tree must not reappear.
- [ ] Telemetry must default to noop unless the user intentionally enables an adapter.
