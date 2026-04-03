# AICD Boilerplate Port Plan

_Date: 2026-04-03_

## 1. Mission

Port the archived original system into the shipped AICD boilerplate so that:

- generated projects are real, usable coding-agent projects
- no core behavior from the original system is silently lost
- architecture stays recognizable relative to the original system
- the shipped repo no longer depends on the archive after parity is complete
- all three languages are brought under one coherent multi-language contract

This plan treats the archive as a temporary audit source only. The delivered system must stand on its own after the archive is removed.

## 2. What "Correct Port" Means

Correct porting does **not** mean copying the original source tree as-is into generated projects.

Correct porting **does** mean preserving:

- behavior
- architecture boundaries
- registry composition model
- extension seams
- command/tool/session lifecycle
- permission and safety model
- prompt composition model
- provider model
- feature toggling and optional capability growth

The port must preserve the **logical structure** of the original system even when the physical packaging is simplified.

### Required preservation rule

After `init`, a generated project should feel like:

- the same agent runtime family
- the same core loop
- the same extensibility model
- the same major capability groups

But it should be:

- cleaner
- smaller
- less product-specific
- easier to extend
- easier to verify

## 3. Final Product Definition

When the migration is complete, `aicd init --language <python|typescript|rust>` must generate a project that already includes:

- a real runtime kernel
- a real prompt/session/tool execution loop
- real config loading
- real provider selection
- real prompt layering
- real local agents
- real local skills
- real local feature packs
- real doctor validation
- real feature add/remove support
- real session save/resume foundations

Optional subsystems from the original system must be shipped as feature packs when they are too large or too specialized for core.

## 4. Non-Negotiable Constraints

- No runtime dependency on archive files or archive paths.
- No silent capability loss.
- No placeholder runtime in final shipped language packs.
- No monolithic copy-paste port of giant source clusters.
- No parity claims without tests.
- No language-specific hardcoding inside the installer beyond manifest-driven behavior.

## 5. Current Audit Summary

### 5.1 What is already solid

- The installer is real and working.
- Manifest-driven language discovery exists.
- `init`, `feature add`, `feature remove`, and `doctor` exist and are tested.
- All three language packs already expose config plus `.agent/*` seams.
- Reversible feature-pack mechanics already exist in the installer.

### 5.2 What is still incomplete

- language runtime directories are still mostly placeholder boundaries
- generated starter apps are smoke placeholders, not real runtime implementations
- most original capabilities are still only tracked as backlog, not ported
- current tests mostly validate shape and installer behavior, not runtime parity
- some migration docs and tests still reflect older archive naming/layout assumptions

### 5.3 Current parity posture

The repo currently has a scaffold contract, not full behavioral parity.

What is currently closest to done:

- installer contract
- config contract
- feature-pack plumbing
- editable agent/skill/prompt/feature seams

What is currently far from done:

- interactive runtime
- session lifecycle
- command runtime
- tool runtime depth
- MCP/LSP/plugin ecosystems
- workflow/task/team systems
- Git/GitHub operator flows

## 6. Architecture Preservation Policy

To avoid losing the original codebase structure during boilerplate extraction:

### 6.1 Preserve these major boundaries in all languages

- `entrypoints`
- `runtime/core`
- `config`
- `prompts`
- `providers`
- `permissions`
- `session`
- `registry`
- `commands`
- `tools`
- `agents`
- `skills`
- `features`
- `tasks` when task/team workflow is enabled

### 6.2 Preserve these logical flows

- startup/bootstrap
- config merge/load
- prompt assembly
- provider resolution
- command discovery
- tool discovery
- tool permission checks
- tool execution
- session persistence
- resume/export/compaction
- feature enablement
- doctor validation

### 6.3 Do not preserve these as raw monoliths

- giant catch-all `utils`
- giant catch-all `components`
- giant catch-all `services`
- giant catch-all `commands`
- giant catch-all `tools`

These must be decomposed into bounded owned modules with capability IDs and tests.

## 7. Core vs Feature-Pack Split

### 7.1 Core must include

- one-shot prompt execution
- interactive session loop
- core file/shell/web tools
- provider/model abstraction
- prompt layering
- local agent loading
- local skill loading
- local feature discovery
- permission/sandbox controls
- doctor
- save/resume foundations
- context building foundations

### 7.2 Feature packs should own

- advanced planning modes
- multi-agent task/team orchestration
- MCP integration
- LSP integration
- Git/GitHub workflows
- richer diagnostics and doctor UI
- onboarding/auth UX
- plugin marketplace and advanced plugin lifecycle
- remote transport / direct-connect / bridge / upstream proxy
- memory and team-memory extensions
- hook-driven advanced automation

### 7.3 Deferred or non-core families

- rich terminal UI system as a monolith
- companion/buddy UX
- broad analytics/telemetry surfaces
- native performance modules that are not required for correctness

## 8. What the Generated Project Must Look Like

### 8.1 Common generated shape

Every language must generate a project with:

- one main config file
- `.agent/prompts/`
- `.agent/agents/`
- `.agent/skills/`
- `.agent/features/`
- `.agent/context/`
- `.agent/sessions/`
- `.agent/usage/`
- `src/`
- `tests/`
- README

### 8.2 Python target shape

```text
<project>/
├─ agentkit.toml
├─ pyproject.toml
├─ README.md
├─ .agent/
│  ├─ prompts/
│  ├─ agents/
│  ├─ skills/
│  ├─ features/
│  ├─ context/
│  ├─ sessions/
│  └─ usage/
├─ src/
│  └─ <package_name>/
│     ├─ entrypoints/
│     ├─ runtime/
│     ├─ commands/
│     ├─ tools/
│     ├─ providers/
│     ├─ prompts/
│     ├─ permissions/
│     ├─ sessions/
│     └─ app.py
└─ tests/
```

### 8.3 TypeScript target shape

```text
<project>/
├─ boilerplate.config.ts
├─ package.json
├─ tsconfig.json
├─ README.md
├─ .agent/
│  ├─ prompts/
│  ├─ agents/
│  ├─ skills/
│  ├─ features/
│  ├─ context/
│  ├─ sessions/
│  └─ usage/
├─ src/
│  ├─ entrypoints/
│  ├─ runtime/
│  ├─ commands/
│  ├─ tools/
│  ├─ providers/
│  ├─ prompts/
│  ├─ permissions/
│  ├─ sessions/
│  └─ index.ts
└─ tests/
```

### 8.4 Rust target shape

```text
<project>/
├─ CLAW.md
├─ .claw.json
├─ Cargo.toml
├─ README.md
├─ .agent/
│  ├─ prompts/
│  ├─ agents/
│  ├─ skills/
│  ├─ features/
│  ├─ context/
│  ├─ sessions/
│  └─ usage/
├─ src/
│  ├─ entrypoints/
│  ├─ runtime/
│  ├─ commands/
│  ├─ tools/
│  ├─ providers/
│  ├─ prompts/
│  ├─ permissions/
│  ├─ sessions/
│  └─ main.rs
└─ tests/
```

## 9. Shared Parity Backlog

The port must cover these capability families in order.

### 9.1 Runtime loop

- interactive REPL and prompt-driven sessions
- one-shot prompt execution
- structured output path
- stop-reason handling
- turn loop budgeting
- stream events or equivalent output lifecycle

### 9.2 Session lifecycle

- session creation
- persisted session storage
- session load
- resume
- export
- compaction
- status
- usage and cost accounting

### 9.3 Command model

- command registry
- builtin command loading
- local extension command loading where applicable
- command help/indexing
- command filtering / enablement

### 9.4 Tool model

- tool registry
- core file tools
- shell tool
- search tools
- web fetch/search
- ask-user/message/brief family
- planning/todo family
- config inspection
- session/control tools

### 9.5 Safety model

- permission modes
- deny rules
- workspace boundary enforcement
- read-only/write policy
- shell escalation policy
- sandbox compatibility

### 9.6 Extensibility model

- prompt layering
- local agents
- local skills
- local feature packs
- reversible feature add/remove
- doctor validation of extension assets

### 9.7 Advanced ecosystems

- MCP
- LSP
- plugins
- hooks
- workflow/task/team execution
- Git/GitHub automation
- remote/structured transports

## 10. Exhaustive Port Matrix

This matrix is the mandatory no-loss checklist. No row may be silently dropped.

### 10.1 Interaction and UI

| Capability | Delivery | State now | Port requirement |
| --- | --- | --- | --- |
| `interactive-repl` | `core` | `declared` | Ship a real interactive session loop in all three languages. |
| `rich-terminal-ui` | `deferred` | `declared` | Do not port wholesale into core; preserve only required UI hooks and defer the large UI system. |
| `doctor-and-resume-screens` | `feature-pack` | `declared` | Implement richer doctor/resume UX as an optional pack after core runtime exists. |
| `onboarding-auth-dialogs` | `feature-pack` | `declared` | Port guided auth/onboarding flows as an optional pack, not as hardcoded core UI. |
| `companion-affordances` | `deferred` | `declared` | Keep out of core until primary parity and runtime correctness are complete. |

### 10.2 Agents, planning, and multi-agent workflows

| Capability | Delivery | State now | Port requirement |
| --- | --- | --- | --- |
| `local-agent-discovery` | `core` | `verified` | Keep and deepen existing local agent discovery without regressions. |
| `built-in-specialist-agents` | `feature-pack` | `declared` | Port built-in specialist presets as optional packs over the base agent seam. |
| `advanced-planning-modes` | `feature-pack` | `declared` | Port plan mode and deep-plan behaviors as an advanced planning pack. |
| `task-team-orchestration` | `feature-pack` | `declared` | Port task/team runtime as a reversible pack with real lifecycle tests. |
| `coordinator-permission-modes` | `feature-pack` | `declared` | Port coordinator/swarm permission policy separately from base permissions. |
| `assistant-session-history` | `feature-pack` | `declared` | Port assistant-specific history behavior after base session lifecycle is done. |

### 10.3 CLI, operator flows, and session control

| Capability | Delivery | State now | Port requirement |
| --- | --- | --- | --- |
| `slash-command-surface` | `reference-only` | `declared` | Do not clone the full command inventory; decompose and classify every command into core, feature-pack, deferred, or rejected. |
| `structured-remote-cli-transport` | `feature-pack` | `declared` | Port structured remote CLI transport as an optional advanced transport pack. |
| `session-management-and-export` | `core` | `declared` | Implement real status, export, resume, and session control behavior in core. |
| `git-and-github-workflow-assistance` | `feature-pack` | `declared` | Port Git/GitHub automation as a dedicated reversible pack. |
| `workspace-bootstrap-init` | `core` | `verified` | Preserve and extend current init behavior into real runtime scaffolding. |

### 10.4 Tool runtime and execution platform

| Capability | Delivery | State now | Port requirement |
| --- | --- | --- | --- |
| `workspace-file-shell-web-tools` | `core` | `verified` | Replace config-only seams with real runtime implementations while preserving existing config contract. |
| `lsp-code-intelligence` | `feature-pack` | `declared` | Port LSP diagnostics/symbol/reference flows as a feature-pack with real tests. |
| `mcp-resource-tooling` | `feature-pack` | `declared` | Port resource listing, resource read, tool call, and auth support as a dedicated MCP pack. |
| `web-fetch-and-search` | `core` | `verified` | Keep and deepen web fetch/search into actual execution behavior in all three languages. |
| `planning-and-todo-tools` | `feature-pack` | `declared` | Port planning/todo/tool-control family as an advanced planning pack, not base core. |
| `ask-user-and-brief-tools` | `feature-pack` | `declared` | Port ask-user/brief/message interaction tools as an optional interaction pack after core loop exists. |
| `tool-platform-breadth` | `reference-only` | `declared` | Decompose every original tool into explicit dispositions; no blanket “ported tools” claim is allowed. |
| `local-skill-discovery` | `core` | `verified` | Preserve and deepen existing local skill loading. |

### 10.5 Prompts, providers, session, memory, and safety

| Capability | Delivery | State now | Port requirement |
| --- | --- | --- | --- |
| `bundled-skill-pipelines` | `feature-pack` | `declared` | Port bundled/registry-style skills as a separate pack over local skill loading. |
| `workspace-instruction-memory` | `core` | `verified` | Preserve workspace instruction files and connect them to real prompt composition. |
| `system-prompt-layering` | `core` | `verified` | Keep prompt layering contract and implement real runtime prompt composition. |
| `provider-and-model-selection` | `core` | `verified` | Preserve config contract and wire to real provider adapters. |
| `oauth-login-flows` | `feature-pack` | `declared` | Port OAuth and guided auth flows as an optional onboarding/auth pack. |
| `streaming-provider-responses` | `feature-pack` | `declared` | Implement streaming model responses after the base provider abstraction is stable. |
| `usage-and-cost-tracking` | `core` | `implemented` | Upgrade reserved seams into real token/cost accounting with tests. |
| `model-migration-rules` | `deferred` | `declared` | Keep as a documented deferred migration policy until core runtime stabilizes. |
| `mcp-integration-subsystem` | `feature-pack` | `declared` | Port the broader MCP subsystem beyond bare resource tooling as an advanced pack. |
| `remote-cli-transport-stack` | `feature-pack` | `declared` | Port remote transport stack separately from local core runtime. |
| `bridge-remote-session-control` | `feature-pack` | `declared` | Port bridge/relay/direct-control behavior as a dedicated pack. |
| `upstream-proxy-support` | `feature-pack` | `declared` | Port upstream proxy support as a dedicated advanced runtime pack. |
| `server-direct-connect` | `feature-pack` | `declared` | Port direct-connect server behavior as a dedicated pack. |
| `external-app-integrations` | `feature-pack` | `declared` | Port external app/channel integrations separately from core. |
| `saved-sessions-and-resume` | `core` | `implemented` | Upgrade reserved session roots into real save/load/resume behavior. |
| `session-memory` | `feature-pack` | `declared` | Port retrieval/memory augmentation as a separate memory pack. |
| `team-memory` | `feature-pack` | `declared` | Port shared/team memory as a dedicated collaboration pack. |
| `workspace-context-building` | `core` | `implemented` | Upgrade reserved context roots into real context ingestion/building behavior. |
| `session-compaction` | `feature-pack` | `declared` | Port compaction logic as a tested capability, likely feature-packed first. |
| `permissions-and-sandbox-safety` | `core` | `verified` | Preserve and deepen into real permission enforcement and sandbox behavior. |
| `hook-runtime` | `feature-pack` | `declared` | Port hook-driven automation as a dedicated runtime extension pack. |

### 10.6 Plugins, services, and decomposition guards

| Capability | Delivery | State now | Port requirement |
| --- | --- | --- | --- |
| `plugin-marketplace-lifecycle` | `feature-pack` | `declared` | Port plugin install/enable/disable/update/uninstall as an optional plugin management pack. |
| `plugin-runtime-manifests` | `feature-pack` | `declared` | Port plugin runtime contracts, permissions, hooks, commands, and tools as a plugin runtime pack. |
| `plugin-source-kinds` | `feature-pack` | `declared` | Port bundled/builtin/external plugin source handling inside the plugin runtime pack. |
| `workspace-editable-extension-seams` | `core` | `verified` | Preserve local editability of agents, skills, and features. |
| `services-cluster-decomposition` | `reference-only` | `declared` | Decompose large service ecosystem into bounded capabilities before porting anything from it. |
| `analytics-and-telemetry` | `deferred` | `declared` | Keep out of core and do not restore default telemetry behavior. |
| `magic-docs-and-prompt-suggestions` | `feature-pack` | `declared` | Port prompt-suggestion and magic-doc helpers as a separate pack. |
| `native-performance-modules` | `deferred` | `declared` | Defer native/perf modules unless needed to preserve correctness. |
| `schema-backed-validation` | `core` | `verified` | Preserve and extend schema-backed validation for manifests and feature packs. |
| `components-cluster-decomposition` | `reference-only` | `declared` | Decompose original UI/component cluster before porting any large UI surface. |
| `hooks-cluster-decomposition` | `reference-only` | `declared` | Decompose original hook cluster before porting advanced hook behavior. |
| `command-inventory-decomposition` | `reference-only` | `declared` | Decompose the full original command inventory into bounded shipped decisions. |
| `tool-inventory-decomposition` | `reference-only` | `declared` | Decompose the full original tool inventory into bounded shipped decisions. |
| `utils-cluster-decomposition` | `reference-only` | `declared` | Decompose the original utility sprawl into owned modules instead of cloning it. |

### 10.7 Inventory closure rule

Before archive deletion:

- every original command must be mapped to a final disposition
- every original tool must be mapped to a final disposition
- every large source cluster must either be decomposed and ported or explicitly excluded
- no “implicit carry-over” is allowed
- no capability row may remain ambiguous

Required inventory artifacts before archive deletion:

- `docs/porting/command-port-table.md`
- `docs/porting/tool-port-table.md`
- `docs/porting/feature-pack-port-table.md`

Each artifact must classify every entry as one of:

- `core`
- `feature-pack`
- `deferred`
- `rejected`
- `reference-only`

And each row must also record:

- shipped name
- target module owner
- language coverage (`python`, `typescript`, `rust`)
- verification status

## 11. Mandatory Audit Workflow During Porting

Every implementation task must follow this loop:

1. identify one capability or one tightly related capability cluster
2. inspect current shipped code for that capability
3. compare behavior and architecture against the archived original system
4. record the gap
5. write failing tests for the missing behavior
6. implement or refactor until the tests pass
7. update capability state
8. move to the next capability

This is not optional. The current code must be continuously audited and corrected while porting.

## 12. Current Code Audit and Required Repair Tracks

### Track A — Scaffold correctness

Repair or complete:

- placeholder runtime directories in each language pack
- starter app placeholders
- minimal smoke tests into real runtime tests

### Track B — Governance correctness

Repair or complete:

- stale migration docs
- stale archive naming assumptions
- missing top-level archive boundary docs
- any tests that still encode pre-normalized archive structure

### Track C — Capability-state correctness

Repair or complete:

- capability matrix rows that overstate shipped depth
- missing decomposition of oversized capability families
- missing per-language notes where one language is ahead or behind

## 13. Implementation Phases

### Phase 0 — Baseline normalization

Deliver:

- normalized docs
- normalized archive-boundary docs
- normalized migration tests
- canonical generated-project contract stated in docs

Definition of done:

- repo documentation no longer depends on transient archive layout choices
- future work can proceed without ambiguity

### Phase 1 — Capability decomposition

Deliver:

- fully decomposed capability backlog
- explicit core vs feature-pack ownership
- explicit per-language target notes
- explicit verification rule per capability

Definition of done:

- no major subsystem remains described only as “we should port this somehow”

### Phase 2 — Runtime kernel extraction design

Deliver:

- target module map for Python
- target module map for TypeScript
- target module map for Rust
- registry and lifecycle contracts shared across languages

Definition of done:

- runtime extraction can proceed module by module without architectural churn

### Phase 3 — Core runtime parity slice

Deliver:

- one-shot execution
- interactive session loop
- core tool runtime
- prompt layering
- provider selection
- permissions
- context/session/usage foundations

Definition of done:

- fresh generated projects are actually usable as agent runtimes

### Phase 4 — Core installer parity hardening

Deliver:

- stronger doctor checks
- richer feature-pack validation
- fixture-driven generated-project verification
- per-language end-to-end init lifecycle tests

Definition of done:

- installer reliably scaffolds and validates the real runtime, not just files on disk

### Phase 5 — First advanced feature packs

Deliver:

- advanced planning pack
- MCP pack
- LSP pack
- git workflow pack
- richer diagnostics/resume pack

Definition of done:

- advanced capabilities can be added and removed cleanly

### Phase 6 — Language-deep parity passes

Deliver:

- Python replaces mirror behavior with true runtime behavior
- TypeScript decomposes the large original runtime into owned modules
- Rust extracts or adapts the richer reconstructed runtime into the language-pack contract

Definition of done:

- all three language packs implement the same core contract with language-native depth

### Phase 7 — Final archive independence

Deliver:

- no shipped tests/docs/runtime rely on archive paths
- parity specs are preserved in shipped docs and tests
- archive can be deleted without losing runtime truth

Definition of done:

- the repo stands alone as the new canonical product

## 14. Per-Language Detailed Plan

### 13.1 Python

#### Current state

- strongest on generated-project contract
- weakest on true runtime parity
- still shaped by mirror/inventory code

#### Required work

- replace snapshot-backed command registry with true command modules
- replace snapshot-backed tool registry with true tool modules
- replace simulated execution strings with real execution
- replace mirror runtime reporting with actual runtime kernel
- keep only useful abstractions from the current Python mirror workspace
- add provider adapters, prompt composer, session store, permission policy, and tool executors under the language-pack runtime

#### Acceptance criteria

- Python generated project no longer behaves like a porting workspace
- Python generated project behaves like a real agent runtime

### 13.2 TypeScript

#### Current state

- richest original capability source
- highest risk of over-porting
- current shipped pack is still template/config heavy and runtime-light

#### Required work

- decompose original runtime into bounded modules
- preserve registry-driven commands/tools
- preserve session/query loop
- preserve prompt composition model
- preserve extension seams
- move optional breadth into feature packs instead of shipping the monolith
- keep naming and logical boundaries close enough that the original architecture remains recognizable

#### Acceptance criteria

- TypeScript pack contains a real runtime kernel
- generated TypeScript projects preserve the original agent structure without carrying the full monolith

### 13.3 Rust

#### Current state

- deepest actual runtime reconstruction
- already includes many advanced runtime ideas
- current shipped pack still does not expose that depth in the language-pack runtime

#### Required work

- extract/adapt the reconstructed runtime into `languages/rust/runtime`
- preserve commands/tools/runtime/providers/permissions/session boundaries
- keep advanced subsystems optional when too large for core
- use Rust as the strongest proving lane for hard runtime behaviors where appropriate

#### Acceptance criteria

- Rust pack becomes a real first-class runtime, not only a scaffold
- generated Rust projects inherit the real runtime shape, not only config placeholders

## 15. Test Plan

### 14.1 Cross-language tests

- manifest validation
- generated-project shape
- init lifecycle
- feature add/remove lifecycle
- doctor lifecycle
- feature-pack reversibility

### 14.2 Runtime parity tests

- prompt execution
- session save/resume
- permission enforcement
- file read/write/edit
- shell execution
- web fetch
- prompt layering
- provider selection

### 14.3 Advanced capability tests

- MCP lifecycle
- LSP lifecycle
- planning mode
- task/team workflow
- git workflow automation
- plugin lifecycle

### 14.4 Archive independence tests

- no runtime path references to the archive
- no shipped docs that require the archive for correctness
- parity state retained in tests/specs after archive removal

## 16. Release Gates

No capability moves to `verified` until:

- behavior exists
- automated tests exist
- generated-project lifecycle is covered where relevant
- the capability works in the correct language-pack boundary
- the capability does not require archive files at runtime

No language pack is considered parity-ready until:

- core runtime is real
- installer lifecycle is real
- feature-pack lifecycle is real
- tests prove generated projects work without archive dependency

## 17. Execution Order

Recommended execution order:

1. baseline normalization
2. capability decomposition
3. runtime module maps
4. core runtime slice tests
5. core runtime implementation
6. installer hardening
7. first advanced feature packs
8. Python deep pass
9. TypeScript deep pass
10. Rust deep pass
11. archive independence cleanup
12. final parity verification pass

## 18. Immediate Next Steps

1. Add missing top-level parity and archive-boundary docs.
2. Rewrite stale migration docs so they no longer depend on transient archive naming.
3. Expand the capability backlog into concrete implementation units.
4. Write failing tests for the first core parity slice.
5. Start replacing placeholder runtime boundaries with real runtime modules in all three language packs.

## 19. Definition of Success

The plan is successful only when:

- a user can run `aicd init` in any of the three languages and receive a real coding-agent boilerplate
- the generated project still clearly reflects the original system’s architecture
- the runtime is smaller and cleaner than the original monolith
- optional complexity lives in feature packs
- the archive can be deleted without breaking the shipped product

## 20. Exact Source File Inventory

This appendix is generated from `files.txt`, using the current archived evidence layout under `references/<language>/...` for every tracked source path.

Migration artifacts may also use normalized source identities such as `references/source-python/`, `references/source-typescript/`, `references/source-rust/`, and `references/parity/` when the goal is to preserve original-source naming independent of the current archive directory names.

Rules:

- Every path below is a canonical migration evidence path under `references/`.
- Normalized source identities are planning aliases; the rows below remain the auditable current-layout evidence paths.
- No row may be dropped during migration planning.
- Archive deletion is blocked until every row has a final disposition in shipped docs/specs/tests.

- Total tracked archived source files: `2186`

### 20.1 Python Inventory

- Count: `117`

| Canonical archived source path | Link |
| --- | --- |
| `references/python/.claude/sessions/session-1775007622154.json` | [`references/python/.claude/sessions/session-1775007622154.json`](references/python/.claude/sessions/session-1775007622154.json) |
| `references/python/.claude/sessions/session-1775007846522.json` | [`references/python/.claude/sessions/session-1775007846522.json`](references/python/.claude/sessions/session-1775007846522.json) |
| `references/python/.claude/sessions/session-1775007632904.json` | [`references/python/.claude/sessions/session-1775007632904.json`](references/python/.claude/sessions/session-1775007632904.json) |
| `references/python/.claude/sessions/session-1775009583240.json` | [`references/python/.claude/sessions/session-1775009583240.json`](references/python/.claude/sessions/session-1775009583240.json) |
| `references/python/.claude/sessions/session-1774998994373.json` | [`references/python/.claude/sessions/session-1774998994373.json`](references/python/.claude/sessions/session-1774998994373.json) |
| `references/python/.claude/sessions/session-1775010002596.json` | [`references/python/.claude/sessions/session-1775010002596.json`](references/python/.claude/sessions/session-1775010002596.json) |
| `references/python/.claude/sessions/session-1775010237519.json` | [`references/python/.claude/sessions/session-1775010237519.json`](references/python/.claude/sessions/session-1775010237519.json) |
| `references/python/.claude/sessions/session-1775010229294.json` | [`references/python/.claude/sessions/session-1775010229294.json`](references/python/.claude/sessions/session-1775010229294.json) |
| `references/python/.claude/sessions/session-1775007533836.json` | [`references/python/.claude/sessions/session-1775007533836.json`](references/python/.claude/sessions/session-1775007533836.json) |
| `references/python/.claude/sessions/session-1775009651284.json` | [`references/python/.claude/sessions/session-1775009651284.json`](references/python/.claude/sessions/session-1775009651284.json) |
| `references/python/.claude/sessions/session-1774998936453.json` | [`references/python/.claude/sessions/session-1774998936453.json`](references/python/.claude/sessions/session-1774998936453.json) |
| `references/python/.claude/sessions/session-1775009126105.json` | [`references/python/.claude/sessions/session-1775009126105.json`](references/python/.claude/sessions/session-1775009126105.json) |
| `references/python/PARITY.md` | [`references/python/PARITY.md`](references/python/PARITY.md) |
| `references/python/tests/test_porting_workspace.py` | [`references/python/tests/test_porting_workspace.py`](references/python/tests/test_porting_workspace.py) |
| `references/python/CLAUDE.md` | [`references/python/CLAUDE.md`](references/python/CLAUDE.md) |
| `references/python/ROADMAP.md` | [`references/python/ROADMAP.md`](references/python/ROADMAP.md) |
| `references/python/.claude.json` | [`references/python/.claude.json`](references/python/.claude.json) |
| `references/python/.gitignore` | [`references/python/.gitignore`](references/python/.gitignore) |
| `references/python/src/system_init.py` | [`references/python/src/system_init.py`](references/python/src/system_init.py) |
| `references/python/src/setup.py` | [`references/python/src/setup.py`](references/python/src/setup.py) |
| `references/python/src/constants/__init__.py` | [`references/python/src/constants/__init__.py`](references/python/src/constants/__init__.py) |
| `references/python/src/command_graph.py` | [`references/python/src/command_graph.py`](references/python/src/command_graph.py) |
| `references/python/src/utils/__init__.py` | [`references/python/src/utils/__init__.py`](references/python/src/utils/__init__.py) |
| `references/python/src/projectOnboardingState.py` | [`references/python/src/projectOnboardingState.py`](references/python/src/projectOnboardingState.py) |
| `references/python/src/voice/__init__.py` | [`references/python/src/voice/__init__.py`](references/python/src/voice/__init__.py) |
| `references/python/src/query_engine.py` | [`references/python/src/query_engine.py`](references/python/src/query_engine.py) |
| `references/python/src/native_ts/__init__.py` | [`references/python/src/native_ts/__init__.py`](references/python/src/native_ts/__init__.py) |
| `references/python/src/session_store.py` | [`references/python/src/session_store.py`](references/python/src/session_store.py) |
| `references/python/src/entrypoints/__init__.py` | [`references/python/src/entrypoints/__init__.py`](references/python/src/entrypoints/__init__.py) |
| `references/python/src/schemas/__init__.py` | [`references/python/src/schemas/__init__.py`](references/python/src/schemas/__init__.py) |
| `references/python/src/bootstrap_graph.py` | [`references/python/src/bootstrap_graph.py`](references/python/src/bootstrap_graph.py) |
| `references/python/src/QueryEngine.py` | [`references/python/src/QueryEngine.py`](references/python/src/QueryEngine.py) |
| `references/python/src/cost_tracker.py` | [`references/python/src/cost_tracker.py`](references/python/src/cost_tracker.py) |
| `references/python/src/_archive_helper.py` | [`references/python/src/_archive_helper.py`](references/python/src/_archive_helper.py) |
| `references/python/src/bridge/__init__.py` | [`references/python/src/bridge/__init__.py`](references/python/src/bridge/__init__.py) |
| `references/python/src/tasks.py` | [`references/python/src/tasks.py`](references/python/src/tasks.py) |
| `references/python/src/vim/__init__.py` | [`references/python/src/vim/__init__.py`](references/python/src/vim/__init__.py) |
| `references/python/src/models.py` | [`references/python/src/models.py`](references/python/src/models.py) |
| `references/python/src/services/__init__.py` | [`references/python/src/services/__init__.py`](references/python/src/services/__init__.py) |
| `references/python/src/state/__init__.py` | [`references/python/src/state/__init__.py`](references/python/src/state/__init__.py) |
| `references/python/src/plugins/__init__.py` | [`references/python/src/plugins/__init__.py`](references/python/src/plugins/__init__.py) |
| `references/python/src/tool_pool.py` | [`references/python/src/tool_pool.py`](references/python/src/tool_pool.py) |
| `references/python/src/deferred_init.py` | [`references/python/src/deferred_init.py`](references/python/src/deferred_init.py) |
| `references/python/src/memdir/__init__.py` | [`references/python/src/memdir/__init__.py`](references/python/src/memdir/__init__.py) |
| `references/python/src/port_manifest.py` | [`references/python/src/port_manifest.py`](references/python/src/port_manifest.py) |
| `references/python/src/buddy/__init__.py` | [`references/python/src/buddy/__init__.py`](references/python/src/buddy/__init__.py) |
| `references/python/src/transcript.py` | [`references/python/src/transcript.py`](references/python/src/transcript.py) |
| `references/python/src/Tool.py` | [`references/python/src/Tool.py`](references/python/src/Tool.py) |
| `references/python/src/outputStyles/__init__.py` | [`references/python/src/outputStyles/__init__.py`](references/python/src/outputStyles/__init__.py) |
| `references/python/src/coordinator/__init__.py` | [`references/python/src/coordinator/__init__.py`](references/python/src/coordinator/__init__.py) |
| `references/python/src/keybindings/__init__.py` | [`references/python/src/keybindings/__init__.py`](references/python/src/keybindings/__init__.py) |
| `references/python/src/reference_data/archive_surface_snapshot.json` | [`references/python/src/reference_data/archive_surface_snapshot.json`](references/python/src/reference_data/archive_surface_snapshot.json) |
| `references/python/src/reference_data/commands_snapshot.json` | [`references/python/src/reference_data/commands_snapshot.json`](references/python/src/reference_data/commands_snapshot.json) |
| `references/python/src/reference_data/tools_snapshot.json` | [`references/python/src/reference_data/tools_snapshot.json`](references/python/src/reference_data/tools_snapshot.json) |
| `references/python/src/reference_data/subsystems/skills.json` | [`references/python/src/reference_data/subsystems/skills.json`](references/python/src/reference_data/subsystems/skills.json) |
| `references/python/src/reference_data/subsystems/voice.json` | [`references/python/src/reference_data/subsystems/voice.json`](references/python/src/reference_data/subsystems/voice.json) |
| `references/python/src/reference_data/subsystems/vim.json` | [`references/python/src/reference_data/subsystems/vim.json`](references/python/src/reference_data/subsystems/vim.json) |
| `references/python/src/reference_data/subsystems/state.json` | [`references/python/src/reference_data/subsystems/state.json`](references/python/src/reference_data/subsystems/state.json) |
| `references/python/src/reference_data/subsystems/types.json` | [`references/python/src/reference_data/subsystems/types.json`](references/python/src/reference_data/subsystems/types.json) |
| `references/python/src/reference_data/subsystems/server.json` | [`references/python/src/reference_data/subsystems/server.json`](references/python/src/reference_data/subsystems/server.json) |
| `references/python/src/reference_data/subsystems/constants.json` | [`references/python/src/reference_data/subsystems/constants.json`](references/python/src/reference_data/subsystems/constants.json) |
| `references/python/src/reference_data/subsystems/entrypoints.json` | [`references/python/src/reference_data/subsystems/entrypoints.json`](references/python/src/reference_data/subsystems/entrypoints.json) |
| `references/python/src/reference_data/subsystems/keybindings.json` | [`references/python/src/reference_data/subsystems/keybindings.json`](references/python/src/reference_data/subsystems/keybindings.json) |
| `references/python/src/reference_data/subsystems/bootstrap.json` | [`references/python/src/reference_data/subsystems/bootstrap.json`](references/python/src/reference_data/subsystems/bootstrap.json) |
| `references/python/src/reference_data/subsystems/assistant.json` | [`references/python/src/reference_data/subsystems/assistant.json`](references/python/src/reference_data/subsystems/assistant.json) |
| `references/python/src/reference_data/subsystems/outputStyles.json` | [`references/python/src/reference_data/subsystems/outputStyles.json`](references/python/src/reference_data/subsystems/outputStyles.json) |
| `references/python/src/reference_data/subsystems/plugins.json` | [`references/python/src/reference_data/subsystems/plugins.json`](references/python/src/reference_data/subsystems/plugins.json) |
| `references/python/src/reference_data/subsystems/hooks.json` | [`references/python/src/reference_data/subsystems/hooks.json`](references/python/src/reference_data/subsystems/hooks.json) |
| `references/python/src/reference_data/subsystems/memdir.json` | [`references/python/src/reference_data/subsystems/memdir.json`](references/python/src/reference_data/subsystems/memdir.json) |
| `references/python/src/reference_data/subsystems/utils.json` | [`references/python/src/reference_data/subsystems/utils.json`](references/python/src/reference_data/subsystems/utils.json) |
| `references/python/src/reference_data/subsystems/remote.json` | [`references/python/src/reference_data/subsystems/remote.json`](references/python/src/reference_data/subsystems/remote.json) |
| `references/python/src/reference_data/subsystems/upstreamproxy.json` | [`references/python/src/reference_data/subsystems/upstreamproxy.json`](references/python/src/reference_data/subsystems/upstreamproxy.json) |
| `references/python/src/reference_data/subsystems/schemas.json` | [`references/python/src/reference_data/subsystems/schemas.json`](references/python/src/reference_data/subsystems/schemas.json) |
| `references/python/src/reference_data/subsystems/coordinator.json` | [`references/python/src/reference_data/subsystems/coordinator.json`](references/python/src/reference_data/subsystems/coordinator.json) |
| `references/python/src/reference_data/subsystems/migrations.json` | [`references/python/src/reference_data/subsystems/migrations.json`](references/python/src/reference_data/subsystems/migrations.json) |
| `references/python/src/reference_data/subsystems/components.json` | [`references/python/src/reference_data/subsystems/components.json`](references/python/src/reference_data/subsystems/components.json) |
| `references/python/src/reference_data/subsystems/screens.json` | [`references/python/src/reference_data/subsystems/screens.json`](references/python/src/reference_data/subsystems/screens.json) |
| `references/python/src/reference_data/subsystems/native_ts.json` | [`references/python/src/reference_data/subsystems/native_ts.json`](references/python/src/reference_data/subsystems/native_ts.json) |
| `references/python/src/reference_data/subsystems/moreright.json` | [`references/python/src/reference_data/subsystems/moreright.json`](references/python/src/reference_data/subsystems/moreright.json) |
| `references/python/src/reference_data/subsystems/services.json` | [`references/python/src/reference_data/subsystems/services.json`](references/python/src/reference_data/subsystems/services.json) |
| `references/python/src/reference_data/subsystems/buddy.json` | [`references/python/src/reference_data/subsystems/buddy.json`](references/python/src/reference_data/subsystems/buddy.json) |
| `references/python/src/reference_data/subsystems/bridge.json` | [`references/python/src/reference_data/subsystems/bridge.json`](references/python/src/reference_data/subsystems/bridge.json) |
| `references/python/src/reference_data/subsystems/cli.json` | [`references/python/src/reference_data/subsystems/cli.json`](references/python/src/reference_data/subsystems/cli.json) |
| `references/python/src/reference_data/__init__.py` | [`references/python/src/reference_data/__init__.py`](references/python/src/reference_data/__init__.py) |
| `references/python/src/upstreamproxy/__init__.py` | [`references/python/src/upstreamproxy/__init__.py`](references/python/src/upstreamproxy/__init__.py) |
| `references/python/src/remote/__init__.py` | [`references/python/src/remote/__init__.py`](references/python/src/remote/__init__.py) |
| `references/python/src/interactiveHelpers.py` | [`references/python/src/interactiveHelpers.py`](references/python/src/interactiveHelpers.py) |
| `references/python/src/parity_audit.py` | [`references/python/src/parity_audit.py`](references/python/src/parity_audit.py) |
| `references/python/src/dialogLaunchers.py` | [`references/python/src/dialogLaunchers.py`](references/python/src/dialogLaunchers.py) |
| `references/python/src/runtime.py` | [`references/python/src/runtime.py`](references/python/src/runtime.py) |
| `references/python/src/execution_registry.py` | [`references/python/src/execution_registry.py`](references/python/src/execution_registry.py) |
| `references/python/src/history.py` | [`references/python/src/history.py`](references/python/src/history.py) |
| `references/python/src/ink.py` | [`references/python/src/ink.py`](references/python/src/ink.py) |
| `references/python/src/types/__init__.py` | [`references/python/src/types/__init__.py`](references/python/src/types/__init__.py) |
| `references/python/src/hooks/__init__.py` | [`references/python/src/hooks/__init__.py`](references/python/src/hooks/__init__.py) |
| `references/python/src/task.py` | [`references/python/src/task.py`](references/python/src/task.py) |
| `references/python/src/migrations/__init__.py` | [`references/python/src/migrations/__init__.py`](references/python/src/migrations/__init__.py) |
| `references/python/src/permissions.py` | [`references/python/src/permissions.py`](references/python/src/permissions.py) |
| `references/python/src/replLauncher.py` | [`references/python/src/replLauncher.py`](references/python/src/replLauncher.py) |
| `references/python/src/cli/__init__.py` | [`references/python/src/cli/__init__.py`](references/python/src/cli/__init__.py) |
| `references/python/src/assistant/__init__.py` | [`references/python/src/assistant/__init__.py`](references/python/src/assistant/__init__.py) |
| `references/python/src/tools.py` | [`references/python/src/tools.py`](references/python/src/tools.py) |
| `references/python/src/bootstrap/__init__.py` | [`references/python/src/bootstrap/__init__.py`](references/python/src/bootstrap/__init__.py) |
| `references/python/src/server/__init__.py` | [`references/python/src/server/__init__.py`](references/python/src/server/__init__.py) |
| `references/python/src/context.py` | [`references/python/src/context.py`](references/python/src/context.py) |
| `references/python/src/query.py` | [`references/python/src/query.py`](references/python/src/query.py) |
| `references/python/src/skills/__init__.py` | [`references/python/src/skills/__init__.py`](references/python/src/skills/__init__.py) |
| `references/python/src/components/__init__.py` | [`references/python/src/components/__init__.py`](references/python/src/components/__init__.py) |
| `references/python/src/commands.py` | [`references/python/src/commands.py`](references/python/src/commands.py) |
| `references/python/src/direct_modes.py` | [`references/python/src/direct_modes.py`](references/python/src/direct_modes.py) |
| `references/python/src/remote_runtime.py` | [`references/python/src/remote_runtime.py`](references/python/src/remote_runtime.py) |
| `references/python/src/screens/__init__.py` | [`references/python/src/screens/__init__.py`](references/python/src/screens/__init__.py) |
| `references/python/src/main.py` | [`references/python/src/main.py`](references/python/src/main.py) |
| `references/python/src/costHook.py` | [`references/python/src/costHook.py`](references/python/src/costHook.py) |
| `references/python/src/prefetch.py` | [`references/python/src/prefetch.py`](references/python/src/prefetch.py) |
| `references/python/src/moreright/__init__.py` | [`references/python/src/moreright/__init__.py`](references/python/src/moreright/__init__.py) |
| `references/python/src/__init__.py` | [`references/python/src/__init__.py`](references/python/src/__init__.py) |

### 20.2 TypeScript Inventory

- Count: `1946`

| Canonical archived source path | Link |
| --- | --- |
| `references/typescript/README.md` | [`references/typescript/README.md`](references/typescript/README.md) |
| `references/typescript/FEATURES.md` | [`references/typescript/FEATURES.md`](references/typescript/FEATURES.md) |
| `references/typescript/env.d.ts` | [`references/typescript/env.d.ts`](references/typescript/env.d.ts) |
| `references/typescript/scripts/build.ts` | [`references/typescript/scripts/build.ts`](references/typescript/scripts/build.ts) |
| `references/typescript/install.sh` | [`references/typescript/install.sh`](references/typescript/install.sh) |
| `references/typescript/CLAUDE.md` | [`references/typescript/CLAUDE.md`](references/typescript/CLAUDE.md) |
| `references/typescript/tsconfig.json` | [`references/typescript/tsconfig.json`](references/typescript/tsconfig.json) |
| `references/typescript/package.json` | [`references/typescript/package.json`](references/typescript/package.json) |
| `references/typescript/.gitignore` | [`references/typescript/.gitignore`](references/typescript/.gitignore) |
| `references/typescript/src/projectOnboardingState.ts` | [`references/typescript/src/projectOnboardingState.ts`](references/typescript/src/projectOnboardingState.ts) |
| `references/typescript/src/constants/outputStyles.ts` | [`references/typescript/src/constants/outputStyles.ts`](references/typescript/src/constants/outputStyles.ts) |
| `references/typescript/src/constants/errorIds.ts` | [`references/typescript/src/constants/errorIds.ts`](references/typescript/src/constants/errorIds.ts) |
| `references/typescript/src/constants/xml.ts` | [`references/typescript/src/constants/xml.ts`](references/typescript/src/constants/xml.ts) |
| `references/typescript/src/constants/keys.ts` | [`references/typescript/src/constants/keys.ts`](references/typescript/src/constants/keys.ts) |
| `references/typescript/src/constants/spinnerVerbs.ts` | [`references/typescript/src/constants/spinnerVerbs.ts`](references/typescript/src/constants/spinnerVerbs.ts) |
| `references/typescript/src/constants/apiLimits.ts` | [`references/typescript/src/constants/apiLimits.ts`](references/typescript/src/constants/apiLimits.ts) |
| `references/typescript/src/constants/figures.ts` | [`references/typescript/src/constants/figures.ts`](references/typescript/src/constants/figures.ts) |
| `references/typescript/src/constants/messages.ts` | [`references/typescript/src/constants/messages.ts`](references/typescript/src/constants/messages.ts) |
| `references/typescript/src/constants/cyberRiskInstruction.ts` | [`references/typescript/src/constants/cyberRiskInstruction.ts`](references/typescript/src/constants/cyberRiskInstruction.ts) |
| `references/typescript/src/constants/toolLimits.ts` | [`references/typescript/src/constants/toolLimits.ts`](references/typescript/src/constants/toolLimits.ts) |
| `references/typescript/src/constants/product.ts` | [`references/typescript/src/constants/product.ts`](references/typescript/src/constants/product.ts) |
| `references/typescript/src/constants/systemPromptSections.ts` | [`references/typescript/src/constants/systemPromptSections.ts`](references/typescript/src/constants/systemPromptSections.ts) |
| `references/typescript/src/constants/oauth.ts` | [`references/typescript/src/constants/oauth.ts`](references/typescript/src/constants/oauth.ts) |
| `references/typescript/src/constants/codex-oauth.ts` | [`references/typescript/src/constants/codex-oauth.ts`](references/typescript/src/constants/codex-oauth.ts) |
| `references/typescript/src/constants/betas.ts` | [`references/typescript/src/constants/betas.ts`](references/typescript/src/constants/betas.ts) |
| `references/typescript/src/constants/turnCompletionVerbs.ts` | [`references/typescript/src/constants/turnCompletionVerbs.ts`](references/typescript/src/constants/turnCompletionVerbs.ts) |
| `references/typescript/src/constants/system.ts` | [`references/typescript/src/constants/system.ts`](references/typescript/src/constants/system.ts) |
| `references/typescript/src/constants/common.ts` | [`references/typescript/src/constants/common.ts`](references/typescript/src/constants/common.ts) |
| `references/typescript/src/constants/files.ts` | [`references/typescript/src/constants/files.ts`](references/typescript/src/constants/files.ts) |
| `references/typescript/src/constants/prompts.ts` | [`references/typescript/src/constants/prompts.ts`](references/typescript/src/constants/prompts.ts) |
| `references/typescript/src/constants/tools.ts` | [`references/typescript/src/constants/tools.ts`](references/typescript/src/constants/tools.ts) |
| `references/typescript/src/constants/github-app.ts` | [`references/typescript/src/constants/github-app.ts`](references/typescript/src/constants/github-app.ts) |
| `references/typescript/src/utils/words.ts` | [`references/typescript/src/utils/words.ts`](references/typescript/src/utils/words.ts) |
| `references/typescript/src/utils/readFileInRange.ts` | [`references/typescript/src/utils/readFileInRange.ts`](references/typescript/src/utils/readFileInRange.ts) |
| `references/typescript/src/utils/authFileDescriptor.ts` | [`references/typescript/src/utils/authFileDescriptor.ts`](references/typescript/src/utils/authFileDescriptor.ts) |
| `references/typescript/src/utils/stringUtils.ts` | [`references/typescript/src/utils/stringUtils.ts`](references/typescript/src/utils/stringUtils.ts) |
| `references/typescript/src/utils/hyperlink.ts` | [`references/typescript/src/utils/hyperlink.ts`](references/typescript/src/utils/hyperlink.ts) |
| `references/typescript/src/utils/lazySchema.ts` | [`references/typescript/src/utils/lazySchema.ts`](references/typescript/src/utils/lazySchema.ts) |
| `references/typescript/src/utils/fingerprint.ts` | [`references/typescript/src/utils/fingerprint.ts`](references/typescript/src/utils/fingerprint.ts) |
| `references/typescript/src/utils/backgroundHousekeeping.ts` | [`references/typescript/src/utils/backgroundHousekeeping.ts`](references/typescript/src/utils/backgroundHousekeeping.ts) |
| `references/typescript/src/utils/mailbox.ts` | [`references/typescript/src/utils/mailbox.ts`](references/typescript/src/utils/mailbox.ts) |
| `references/typescript/src/utils/systemDirectories.ts` | [`references/typescript/src/utils/systemDirectories.ts`](references/typescript/src/utils/systemDirectories.ts) |
| `references/typescript/src/utils/envValidation.ts` | [`references/typescript/src/utils/envValidation.ts`](references/typescript/src/utils/envValidation.ts) |
| `references/typescript/src/utils/fileHistory.ts` | [`references/typescript/src/utils/fileHistory.ts`](references/typescript/src/utils/fileHistory.ts) |
| `references/typescript/src/utils/memoryFileDetection.ts` | [`references/typescript/src/utils/memoryFileDetection.ts`](references/typescript/src/utils/memoryFileDetection.ts) |
| `references/typescript/src/utils/toolResultStorage.ts` | [`references/typescript/src/utils/toolResultStorage.ts`](references/typescript/src/utils/toolResultStorage.ts) |
| `references/typescript/src/utils/shellConfig.ts` | [`references/typescript/src/utils/shellConfig.ts`](references/typescript/src/utils/shellConfig.ts) |
| `references/typescript/src/utils/Cursor.ts` | [`references/typescript/src/utils/Cursor.ts`](references/typescript/src/utils/Cursor.ts) |
| `references/typescript/src/utils/activityManager.ts` | [`references/typescript/src/utils/activityManager.ts`](references/typescript/src/utils/activityManager.ts) |
| `references/typescript/src/utils/screenshotClipboard.ts` | [`references/typescript/src/utils/screenshotClipboard.ts`](references/typescript/src/utils/screenshotClipboard.ts) |
| `references/typescript/src/utils/powershell/dangerousCmdlets.ts` | [`references/typescript/src/utils/powershell/dangerousCmdlets.ts`](references/typescript/src/utils/powershell/dangerousCmdlets.ts) |
| `references/typescript/src/utils/powershell/staticPrefix.ts` | [`references/typescript/src/utils/powershell/staticPrefix.ts`](references/typescript/src/utils/powershell/staticPrefix.ts) |
| `references/typescript/src/utils/powershell/parser.ts` | [`references/typescript/src/utils/powershell/parser.ts`](references/typescript/src/utils/powershell/parser.ts) |
| `references/typescript/src/utils/settings/validation.ts` | [`references/typescript/src/utils/settings/validation.ts`](references/typescript/src/utils/settings/validation.ts) |
| `references/typescript/src/utils/settings/pluginOnlyPolicy.ts` | [`references/typescript/src/utils/settings/pluginOnlyPolicy.ts`](references/typescript/src/utils/settings/pluginOnlyPolicy.ts) |
| `references/typescript/src/utils/settings/mdm/rawRead.ts` | [`references/typescript/src/utils/settings/mdm/rawRead.ts`](references/typescript/src/utils/settings/mdm/rawRead.ts) |
| `references/typescript/src/utils/settings/mdm/settings.ts` | [`references/typescript/src/utils/settings/mdm/settings.ts`](references/typescript/src/utils/settings/mdm/settings.ts) |
| `references/typescript/src/utils/settings/mdm/constants.ts` | [`references/typescript/src/utils/settings/mdm/constants.ts`](references/typescript/src/utils/settings/mdm/constants.ts) |
| `references/typescript/src/utils/settings/toolValidationConfig.ts` | [`references/typescript/src/utils/settings/toolValidationConfig.ts`](references/typescript/src/utils/settings/toolValidationConfig.ts) |
| `references/typescript/src/utils/settings/changeDetector.ts` | [`references/typescript/src/utils/settings/changeDetector.ts`](references/typescript/src/utils/settings/changeDetector.ts) |
| `references/typescript/src/utils/settings/types.ts` | [`references/typescript/src/utils/settings/types.ts`](references/typescript/src/utils/settings/types.ts) |
| `references/typescript/src/utils/settings/permissionValidation.ts` | [`references/typescript/src/utils/settings/permissionValidation.ts`](references/typescript/src/utils/settings/permissionValidation.ts) |
| `references/typescript/src/utils/settings/applySettingsChange.ts` | [`references/typescript/src/utils/settings/applySettingsChange.ts`](references/typescript/src/utils/settings/applySettingsChange.ts) |
| `references/typescript/src/utils/settings/internalWrites.ts` | [`references/typescript/src/utils/settings/internalWrites.ts`](references/typescript/src/utils/settings/internalWrites.ts) |
| `references/typescript/src/utils/settings/validationTips.ts` | [`references/typescript/src/utils/settings/validationTips.ts`](references/typescript/src/utils/settings/validationTips.ts) |
| `references/typescript/src/utils/settings/settingsCache.ts` | [`references/typescript/src/utils/settings/settingsCache.ts`](references/typescript/src/utils/settings/settingsCache.ts) |
| `references/typescript/src/utils/settings/allErrors.ts` | [`references/typescript/src/utils/settings/allErrors.ts`](references/typescript/src/utils/settings/allErrors.ts) |
| `references/typescript/src/utils/settings/managedPath.ts` | [`references/typescript/src/utils/settings/managedPath.ts`](references/typescript/src/utils/settings/managedPath.ts) |
| `references/typescript/src/utils/settings/settings.ts` | [`references/typescript/src/utils/settings/settings.ts`](references/typescript/src/utils/settings/settings.ts) |
| `references/typescript/src/utils/settings/constants.ts` | [`references/typescript/src/utils/settings/constants.ts`](references/typescript/src/utils/settings/constants.ts) |
| `references/typescript/src/utils/settings/schemaOutput.ts` | [`references/typescript/src/utils/settings/schemaOutput.ts`](references/typescript/src/utils/settings/schemaOutput.ts) |
| `references/typescript/src/utils/settings/validateEditTool.ts` | [`references/typescript/src/utils/settings/validateEditTool.ts`](references/typescript/src/utils/settings/validateEditTool.ts) |
| `references/typescript/src/utils/sandbox/sandbox-adapter.ts` | [`references/typescript/src/utils/sandbox/sandbox-adapter.ts`](references/typescript/src/utils/sandbox/sandbox-adapter.ts) |
| `references/typescript/src/utils/sandbox/sandbox-ui-utils.ts` | [`references/typescript/src/utils/sandbox/sandbox-ui-utils.ts`](references/typescript/src/utils/sandbox/sandbox-ui-utils.ts) |
| `references/typescript/src/utils/fileStateCache.ts` | [`references/typescript/src/utils/fileStateCache.ts`](references/typescript/src/utils/fileStateCache.ts) |
| `references/typescript/src/utils/messages/mappers.ts` | [`references/typescript/src/utils/messages/mappers.ts`](references/typescript/src/utils/messages/mappers.ts) |
| `references/typescript/src/utils/messages/systemInit.ts` | [`references/typescript/src/utils/messages/systemInit.ts`](references/typescript/src/utils/messages/systemInit.ts) |
| `references/typescript/src/utils/doctorContextWarnings.ts` | [`references/typescript/src/utils/doctorContextWarnings.ts`](references/typescript/src/utils/doctorContextWarnings.ts) |
| `references/typescript/src/utils/cliHighlight.ts` | [`references/typescript/src/utils/cliHighlight.ts`](references/typescript/src/utils/cliHighlight.ts) |
| `references/typescript/src/utils/processUserInput/processUserInput.ts` | [`references/typescript/src/utils/processUserInput/processUserInput.ts`](references/typescript/src/utils/processUserInput/processUserInput.ts) |
| `references/typescript/src/utils/processUserInput/processTextPrompt.ts` | [`references/typescript/src/utils/processUserInput/processTextPrompt.ts`](references/typescript/src/utils/processUserInput/processTextPrompt.ts) |
| `references/typescript/src/utils/processUserInput/processBashCommand.tsx` | [`references/typescript/src/utils/processUserInput/processBashCommand.tsx`](references/typescript/src/utils/processUserInput/processBashCommand.tsx) |
| `references/typescript/src/utils/processUserInput/processSlashCommand.tsx` | [`references/typescript/src/utils/processUserInput/processSlashCommand.tsx`](references/typescript/src/utils/processUserInput/processSlashCommand.tsx) |
| `references/typescript/src/utils/task/outputFormatting.ts` | [`references/typescript/src/utils/task/outputFormatting.ts`](references/typescript/src/utils/task/outputFormatting.ts) |
| `references/typescript/src/utils/task/TaskOutput.ts` | [`references/typescript/src/utils/task/TaskOutput.ts`](references/typescript/src/utils/task/TaskOutput.ts) |
| `references/typescript/src/utils/task/sdkProgress.ts` | [`references/typescript/src/utils/task/sdkProgress.ts`](references/typescript/src/utils/task/sdkProgress.ts) |
| `references/typescript/src/utils/task/framework.ts` | [`references/typescript/src/utils/task/framework.ts`](references/typescript/src/utils/task/framework.ts) |
| `references/typescript/src/utils/task/diskOutput.ts` | [`references/typescript/src/utils/task/diskOutput.ts`](references/typescript/src/utils/task/diskOutput.ts) |
| `references/typescript/src/utils/exampleCommands.ts` | [`references/typescript/src/utils/exampleCommands.ts`](references/typescript/src/utils/exampleCommands.ts) |
| `references/typescript/src/utils/memoize.ts` | [`references/typescript/src/utils/memoize.ts`](references/typescript/src/utils/memoize.ts) |
| `references/typescript/src/utils/set.ts` | [`references/typescript/src/utils/set.ts`](references/typescript/src/utils/set.ts) |
| `references/typescript/src/utils/memory/versions.ts` | [`references/typescript/src/utils/memory/versions.ts`](references/typescript/src/utils/memory/versions.ts) |
| `references/typescript/src/utils/memory/types.ts` | [`references/typescript/src/utils/memory/types.ts`](references/typescript/src/utils/memory/types.ts) |
| `references/typescript/src/utils/cronScheduler.ts` | [`references/typescript/src/utils/cronScheduler.ts`](references/typescript/src/utils/cronScheduler.ts) |
| `references/typescript/src/utils/ripgrep.ts` | [`references/typescript/src/utils/ripgrep.ts`](references/typescript/src/utils/ripgrep.ts) |
| `references/typescript/src/utils/xml.ts` | [`references/typescript/src/utils/xml.ts`](references/typescript/src/utils/xml.ts) |
| `references/typescript/src/utils/sessionUrl.ts` | [`references/typescript/src/utils/sessionUrl.ts`](references/typescript/src/utils/sessionUrl.ts) |
| `references/typescript/src/utils/detectRepository.ts` | [`references/typescript/src/utils/detectRepository.ts`](references/typescript/src/utils/detectRepository.ts) |
| `references/typescript/src/utils/errorLogSink.ts` | [`references/typescript/src/utils/errorLogSink.ts`](references/typescript/src/utils/errorLogSink.ts) |
| `references/typescript/src/utils/treeify.ts` | [`references/typescript/src/utils/treeify.ts`](references/typescript/src/utils/treeify.ts) |
| `references/typescript/src/utils/generators.ts` | [`references/typescript/src/utils/generators.ts`](references/typescript/src/utils/generators.ts) |
| `references/typescript/src/utils/sliceAnsi.ts` | [`references/typescript/src/utils/sliceAnsi.ts`](references/typescript/src/utils/sliceAnsi.ts) |
| `references/typescript/src/utils/handlePromptSubmit.ts` | [`references/typescript/src/utils/handlePromptSubmit.ts`](references/typescript/src/utils/handlePromptSubmit.ts) |
| `references/typescript/src/utils/sessionActivity.ts` | [`references/typescript/src/utils/sessionActivity.ts`](references/typescript/src/utils/sessionActivity.ts) |
| `references/typescript/src/utils/queryHelpers.ts` | [`references/typescript/src/utils/queryHelpers.ts`](references/typescript/src/utils/queryHelpers.ts) |
| `references/typescript/src/utils/completionCache.ts` | [`references/typescript/src/utils/completionCache.ts`](references/typescript/src/utils/completionCache.ts) |
| `references/typescript/src/utils/filePersistence/filePersistence.ts` | [`references/typescript/src/utils/filePersistence/filePersistence.ts`](references/typescript/src/utils/filePersistence/filePersistence.ts) |
| `references/typescript/src/utils/filePersistence/outputsScanner.ts` | [`references/typescript/src/utils/filePersistence/outputsScanner.ts`](references/typescript/src/utils/filePersistence/outputsScanner.ts) |
| `references/typescript/src/utils/filePersistence/types.ts` | [`references/typescript/src/utils/filePersistence/types.ts`](references/typescript/src/utils/filePersistence/types.ts) |
| `references/typescript/src/utils/collapseBackgroundBashNotifications.ts` | [`references/typescript/src/utils/collapseBackgroundBashNotifications.ts`](references/typescript/src/utils/collapseBackgroundBashNotifications.ts) |
| `references/typescript/src/utils/advisor.ts` | [`references/typescript/src/utils/advisor.ts`](references/typescript/src/utils/advisor.ts) |
| `references/typescript/src/utils/fullscreen.ts` | [`references/typescript/src/utils/fullscreen.ts`](references/typescript/src/utils/fullscreen.ts) |
| `references/typescript/src/utils/idePathConversion.ts` | [`references/typescript/src/utils/idePathConversion.ts`](references/typescript/src/utils/idePathConversion.ts) |
| `references/typescript/src/utils/caCerts.ts` | [`references/typescript/src/utils/caCerts.ts`](references/typescript/src/utils/caCerts.ts) |
| `references/typescript/src/utils/diagLogs.ts` | [`references/typescript/src/utils/diagLogs.ts`](references/typescript/src/utils/diagLogs.ts) |
| `references/typescript/src/utils/markdownConfigLoader.ts` | [`references/typescript/src/utils/markdownConfigLoader.ts`](references/typescript/src/utils/markdownConfigLoader.ts) |
| `references/typescript/src/utils/swarm/teammateLayoutManager.ts` | [`references/typescript/src/utils/swarm/teammateLayoutManager.ts`](references/typescript/src/utils/swarm/teammateLayoutManager.ts) |
| `references/typescript/src/utils/swarm/teammateModel.ts` | [`references/typescript/src/utils/swarm/teammateModel.ts`](references/typescript/src/utils/swarm/teammateModel.ts) |
| `references/typescript/src/utils/swarm/It2SetupPrompt.tsx` | [`references/typescript/src/utils/swarm/It2SetupPrompt.tsx`](references/typescript/src/utils/swarm/It2SetupPrompt.tsx) |
| `references/typescript/src/utils/swarm/teammatePromptAddendum.ts` | [`references/typescript/src/utils/swarm/teammatePromptAddendum.ts`](references/typescript/src/utils/swarm/teammatePromptAddendum.ts) |
| `references/typescript/src/utils/swarm/teammateInit.ts` | [`references/typescript/src/utils/swarm/teammateInit.ts`](references/typescript/src/utils/swarm/teammateInit.ts) |
| `references/typescript/src/utils/swarm/reconnection.ts` | [`references/typescript/src/utils/swarm/reconnection.ts`](references/typescript/src/utils/swarm/reconnection.ts) |
| `references/typescript/src/utils/swarm/spawnInProcess.ts` | [`references/typescript/src/utils/swarm/spawnInProcess.ts`](references/typescript/src/utils/swarm/spawnInProcess.ts) |
| `references/typescript/src/utils/swarm/leaderPermissionBridge.ts` | [`references/typescript/src/utils/swarm/leaderPermissionBridge.ts`](references/typescript/src/utils/swarm/leaderPermissionBridge.ts) |
| `references/typescript/src/utils/swarm/inProcessRunner.ts` | [`references/typescript/src/utils/swarm/inProcessRunner.ts`](references/typescript/src/utils/swarm/inProcessRunner.ts) |
| `references/typescript/src/utils/swarm/permissionSync.ts` | [`references/typescript/src/utils/swarm/permissionSync.ts`](references/typescript/src/utils/swarm/permissionSync.ts) |
| `references/typescript/src/utils/swarm/spawnUtils.ts` | [`references/typescript/src/utils/swarm/spawnUtils.ts`](references/typescript/src/utils/swarm/spawnUtils.ts) |
| `references/typescript/src/utils/swarm/teamHelpers.ts` | [`references/typescript/src/utils/swarm/teamHelpers.ts`](references/typescript/src/utils/swarm/teamHelpers.ts) |
| `references/typescript/src/utils/swarm/constants.ts` | [`references/typescript/src/utils/swarm/constants.ts`](references/typescript/src/utils/swarm/constants.ts) |
| `references/typescript/src/utils/swarm/backends/detection.ts` | [`references/typescript/src/utils/swarm/backends/detection.ts`](references/typescript/src/utils/swarm/backends/detection.ts) |
| `references/typescript/src/utils/swarm/backends/PaneBackendExecutor.ts` | [`references/typescript/src/utils/swarm/backends/PaneBackendExecutor.ts`](references/typescript/src/utils/swarm/backends/PaneBackendExecutor.ts) |
| `references/typescript/src/utils/swarm/backends/ITermBackend.ts` | [`references/typescript/src/utils/swarm/backends/ITermBackend.ts`](references/typescript/src/utils/swarm/backends/ITermBackend.ts) |
| `references/typescript/src/utils/swarm/backends/types.ts` | [`references/typescript/src/utils/swarm/backends/types.ts`](references/typescript/src/utils/swarm/backends/types.ts) |
| `references/typescript/src/utils/swarm/backends/registry.ts` | [`references/typescript/src/utils/swarm/backends/registry.ts`](references/typescript/src/utils/swarm/backends/registry.ts) |
| `references/typescript/src/utils/swarm/backends/it2Setup.ts` | [`references/typescript/src/utils/swarm/backends/it2Setup.ts`](references/typescript/src/utils/swarm/backends/it2Setup.ts) |
| `references/typescript/src/utils/swarm/backends/teammateModeSnapshot.ts` | [`references/typescript/src/utils/swarm/backends/teammateModeSnapshot.ts`](references/typescript/src/utils/swarm/backends/teammateModeSnapshot.ts) |
| `references/typescript/src/utils/swarm/backends/InProcessBackend.ts` | [`references/typescript/src/utils/swarm/backends/InProcessBackend.ts`](references/typescript/src/utils/swarm/backends/InProcessBackend.ts) |
| `references/typescript/src/utils/swarm/backends/TmuxBackend.ts` | [`references/typescript/src/utils/swarm/backends/TmuxBackend.ts`](references/typescript/src/utils/swarm/backends/TmuxBackend.ts) |
| `references/typescript/src/utils/profilerBase.ts` | [`references/typescript/src/utils/profilerBase.ts`](references/typescript/src/utils/profilerBase.ts) |
| `references/typescript/src/utils/messageQueueManager.ts` | [`references/typescript/src/utils/messageQueueManager.ts`](references/typescript/src/utils/messageQueueManager.ts) |
| `references/typescript/src/utils/iTermBackup.ts` | [`references/typescript/src/utils/iTermBackup.ts`](references/typescript/src/utils/iTermBackup.ts) |
| `references/typescript/src/utils/subprocessEnv.ts` | [`references/typescript/src/utils/subprocessEnv.ts`](references/typescript/src/utils/subprocessEnv.ts) |
| `references/typescript/src/utils/stream.ts` | [`references/typescript/src/utils/stream.ts`](references/typescript/src/utils/stream.ts) |
| `references/typescript/src/utils/sessionState.ts` | [`references/typescript/src/utils/sessionState.ts`](references/typescript/src/utils/sessionState.ts) |
| `references/typescript/src/utils/theme.ts` | [`references/typescript/src/utils/theme.ts`](references/typescript/src/utils/theme.ts) |
| `references/typescript/src/utils/config.ts` | [`references/typescript/src/utils/config.ts`](references/typescript/src/utils/config.ts) |
| `references/typescript/src/utils/systemTheme.ts` | [`references/typescript/src/utils/systemTheme.ts`](references/typescript/src/utils/systemTheme.ts) |
| `references/typescript/src/utils/systemPrompt.ts` | [`references/typescript/src/utils/systemPrompt.ts`](references/typescript/src/utils/systemPrompt.ts) |
| `references/typescript/src/utils/dxt/zip.ts` | [`references/typescript/src/utils/dxt/zip.ts`](references/typescript/src/utils/dxt/zip.ts) |
| `references/typescript/src/utils/dxt/helpers.ts` | [`references/typescript/src/utils/dxt/helpers.ts`](references/typescript/src/utils/dxt/helpers.ts) |
| `references/typescript/src/utils/collapseHookSummaries.ts` | [`references/typescript/src/utils/collapseHookSummaries.ts`](references/typescript/src/utils/collapseHookSummaries.ts) |
| `references/typescript/src/utils/toolSchemaCache.ts` | [`references/typescript/src/utils/toolSchemaCache.ts`](references/typescript/src/utils/toolSchemaCache.ts) |
| `references/typescript/src/utils/teamMemoryOps.ts` | [`references/typescript/src/utils/teamMemoryOps.ts`](references/typescript/src/utils/teamMemoryOps.ts) |
| `references/typescript/src/utils/suggestions/slackChannelSuggestions.ts` | [`references/typescript/src/utils/suggestions/slackChannelSuggestions.ts`](references/typescript/src/utils/suggestions/slackChannelSuggestions.ts) |
| `references/typescript/src/utils/suggestions/skillUsageTracking.ts` | [`references/typescript/src/utils/suggestions/skillUsageTracking.ts`](references/typescript/src/utils/suggestions/skillUsageTracking.ts) |
| `references/typescript/src/utils/suggestions/commandSuggestions.ts` | [`references/typescript/src/utils/suggestions/commandSuggestions.ts`](references/typescript/src/utils/suggestions/commandSuggestions.ts) |
| `references/typescript/src/utils/suggestions/shellHistoryCompletion.ts` | [`references/typescript/src/utils/suggestions/shellHistoryCompletion.ts`](references/typescript/src/utils/suggestions/shellHistoryCompletion.ts) |
| `references/typescript/src/utils/suggestions/directoryCompletion.ts` | [`references/typescript/src/utils/suggestions/directoryCompletion.ts`](references/typescript/src/utils/suggestions/directoryCompletion.ts) |
| `references/typescript/src/utils/getWorktreePaths.ts` | [`references/typescript/src/utils/getWorktreePaths.ts`](references/typescript/src/utils/getWorktreePaths.ts) |
| `references/typescript/src/utils/peerAddress.ts` | [`references/typescript/src/utils/peerAddress.ts`](references/typescript/src/utils/peerAddress.ts) |
| `references/typescript/src/utils/lockfile.ts` | [`references/typescript/src/utils/lockfile.ts`](references/typescript/src/utils/lockfile.ts) |
| `references/typescript/src/utils/authPortable.ts` | [`references/typescript/src/utils/authPortable.ts`](references/typescript/src/utils/authPortable.ts) |
| `references/typescript/src/utils/teammate.ts` | [`references/typescript/src/utils/teammate.ts`](references/typescript/src/utils/teammate.ts) |
| `references/typescript/src/utils/concurrentSessions.ts` | [`references/typescript/src/utils/concurrentSessions.ts`](references/typescript/src/utils/concurrentSessions.ts) |
| `references/typescript/src/utils/log.ts` | [`references/typescript/src/utils/log.ts`](references/typescript/src/utils/log.ts) |
| `references/typescript/src/utils/mtls.ts` | [`references/typescript/src/utils/mtls.ts`](references/typescript/src/utils/mtls.ts) |
| `references/typescript/src/utils/binaryCheck.ts` | [`references/typescript/src/utils/binaryCheck.ts`](references/typescript/src/utils/binaryCheck.ts) |
| `references/typescript/src/utils/keyboardShortcuts.ts` | [`references/typescript/src/utils/keyboardShortcuts.ts`](references/typescript/src/utils/keyboardShortcuts.ts) |
| `references/typescript/src/utils/argumentSubstitution.ts` | [`references/typescript/src/utils/argumentSubstitution.ts`](references/typescript/src/utils/argumentSubstitution.ts) |
| `references/typescript/src/utils/forkedAgent.ts` | [`references/typescript/src/utils/forkedAgent.ts`](references/typescript/src/utils/forkedAgent.ts) |
| `references/typescript/src/utils/file.ts` | [`references/typescript/src/utils/file.ts`](references/typescript/src/utils/file.ts) |
| `references/typescript/src/utils/classifierApprovals.ts` | [`references/typescript/src/utils/classifierApprovals.ts`](references/typescript/src/utils/classifierApprovals.ts) |
| `references/typescript/src/utils/analyzeContext.ts` | [`references/typescript/src/utils/analyzeContext.ts`](references/typescript/src/utils/analyzeContext.ts) |
| `references/typescript/src/utils/textHighlighting.ts` | [`references/typescript/src/utils/textHighlighting.ts`](references/typescript/src/utils/textHighlighting.ts) |
| `references/typescript/src/utils/sessionEnvironment.ts` | [`references/typescript/src/utils/sessionEnvironment.ts`](references/typescript/src/utils/sessionEnvironment.ts) |
| `references/typescript/src/utils/diff.ts` | [`references/typescript/src/utils/diff.ts`](references/typescript/src/utils/diff.ts) |
| `references/typescript/src/utils/agenticSessionSearch.ts` | [`references/typescript/src/utils/agenticSessionSearch.ts`](references/typescript/src/utils/agenticSessionSearch.ts) |
| `references/typescript/src/utils/sdkEventQueue.ts` | [`references/typescript/src/utils/sdkEventQueue.ts`](references/typescript/src/utils/sdkEventQueue.ts) |
| `references/typescript/src/utils/notebook.ts` | [`references/typescript/src/utils/notebook.ts`](references/typescript/src/utils/notebook.ts) |
| `references/typescript/src/utils/classifierApprovalsHook.ts` | [`references/typescript/src/utils/classifierApprovalsHook.ts`](references/typescript/src/utils/classifierApprovalsHook.ts) |
| `references/typescript/src/utils/imagePaste.ts` | [`references/typescript/src/utils/imagePaste.ts`](references/typescript/src/utils/imagePaste.ts) |
| `references/typescript/src/utils/process.ts` | [`references/typescript/src/utils/process.ts`](references/typescript/src/utils/process.ts) |
| `references/typescript/src/utils/glob.ts` | [`references/typescript/src/utils/glob.ts`](references/typescript/src/utils/glob.ts) |
| `references/typescript/src/utils/preflightChecks.tsx` | [`references/typescript/src/utils/preflightChecks.tsx`](references/typescript/src/utils/preflightChecks.tsx) |
| `references/typescript/src/utils/context.ts` | [`references/typescript/src/utils/context.ts`](references/typescript/src/utils/context.ts) |
| `references/typescript/src/utils/sessionIngressAuth.ts` | [`references/typescript/src/utils/sessionIngressAuth.ts`](references/typescript/src/utils/sessionIngressAuth.ts) |
| `references/typescript/src/utils/git/gitFilesystem.ts` | [`references/typescript/src/utils/git/gitFilesystem.ts`](references/typescript/src/utils/git/gitFilesystem.ts) |
| `references/typescript/src/utils/git/gitignore.ts` | [`references/typescript/src/utils/git/gitignore.ts`](references/typescript/src/utils/git/gitignore.ts) |
| `references/typescript/src/utils/git/gitConfigParser.ts` | [`references/typescript/src/utils/git/gitConfigParser.ts`](references/typescript/src/utils/git/gitConfigParser.ts) |
| `references/typescript/src/utils/extraUsage.ts` | [`references/typescript/src/utils/extraUsage.ts`](references/typescript/src/utils/extraUsage.ts) |
| `references/typescript/src/utils/ansiToSvg.ts` | [`references/typescript/src/utils/ansiToSvg.ts`](references/typescript/src/utils/ansiToSvg.ts) |
| `references/typescript/src/utils/asciicast.ts` | [`references/typescript/src/utils/asciicast.ts`](references/typescript/src/utils/asciicast.ts) |
| `references/typescript/src/utils/caCertsConfig.ts` | [`references/typescript/src/utils/caCertsConfig.ts`](references/typescript/src/utils/caCertsConfig.ts) |
| `references/typescript/src/utils/managedEnv.ts` | [`references/typescript/src/utils/managedEnv.ts`](references/typescript/src/utils/managedEnv.ts) |
| `references/typescript/src/utils/slashCommandParsing.ts` | [`references/typescript/src/utils/slashCommandParsing.ts`](references/typescript/src/utils/slashCommandParsing.ts) |
| `references/typescript/src/utils/mcp/elicitationValidation.ts` | [`references/typescript/src/utils/mcp/elicitationValidation.ts`](references/typescript/src/utils/mcp/elicitationValidation.ts) |
| `references/typescript/src/utils/mcp/dateTimeParser.ts` | [`references/typescript/src/utils/mcp/dateTimeParser.ts`](references/typescript/src/utils/mcp/dateTimeParser.ts) |
| `references/typescript/src/utils/managedEnvConstants.ts` | [`references/typescript/src/utils/managedEnvConstants.ts`](references/typescript/src/utils/managedEnvConstants.ts) |
| `references/typescript/src/utils/http.ts` | [`references/typescript/src/utils/http.ts`](references/typescript/src/utils/http.ts) |
| `references/typescript/src/utils/heapDumpService.ts` | [`references/typescript/src/utils/heapDumpService.ts`](references/typescript/src/utils/heapDumpService.ts) |
| `references/typescript/src/utils/editor.ts` | [`references/typescript/src/utils/editor.ts`](references/typescript/src/utils/editor.ts) |
| `references/typescript/src/utils/promptEditor.ts` | [`references/typescript/src/utils/promptEditor.ts`](references/typescript/src/utils/promptEditor.ts) |
| `references/typescript/src/utils/execFileNoThrow.ts` | [`references/typescript/src/utils/execFileNoThrow.ts`](references/typescript/src/utils/execFileNoThrow.ts) |
| `references/typescript/src/utils/collapseTeammateShutdowns.ts` | [`references/typescript/src/utils/collapseTeammateShutdowns.ts`](references/typescript/src/utils/collapseTeammateShutdowns.ts) |
| `references/typescript/src/utils/exportRenderer.tsx` | [`references/typescript/src/utils/exportRenderer.tsx`](references/typescript/src/utils/exportRenderer.tsx) |
| `references/typescript/src/utils/startupProfiler.ts` | [`references/typescript/src/utils/startupProfiler.ts`](references/typescript/src/utils/startupProfiler.ts) |
| `references/typescript/src/utils/platform.ts` | [`references/typescript/src/utils/platform.ts`](references/typescript/src/utils/platform.ts) |
| `references/typescript/src/utils/desktopDeepLink.ts` | [`references/typescript/src/utils/desktopDeepLink.ts`](references/typescript/src/utils/desktopDeepLink.ts) |
| `references/typescript/src/utils/aws.ts` | [`references/typescript/src/utils/aws.ts`](references/typescript/src/utils/aws.ts) |
| `references/typescript/src/utils/ansiToPng.ts` | [`references/typescript/src/utils/ansiToPng.ts`](references/typescript/src/utils/ansiToPng.ts) |
| `references/typescript/src/utils/collapseReadSearch.ts` | [`references/typescript/src/utils/collapseReadSearch.ts`](references/typescript/src/utils/collapseReadSearch.ts) |
| `references/typescript/src/utils/proxy.ts` | [`references/typescript/src/utils/proxy.ts`](references/typescript/src/utils/proxy.ts) |
| `references/typescript/src/utils/claudeCodeHints.ts` | [`references/typescript/src/utils/claudeCodeHints.ts`](references/typescript/src/utils/claudeCodeHints.ts) |
| `references/typescript/src/utils/commitAttribution.ts` | [`references/typescript/src/utils/commitAttribution.ts`](references/typescript/src/utils/commitAttribution.ts) |
| `references/typescript/src/utils/execFileNoThrowPortable.ts` | [`references/typescript/src/utils/execFileNoThrowPortable.ts`](references/typescript/src/utils/execFileNoThrowPortable.ts) |
| `references/typescript/src/utils/mcpValidation.ts` | [`references/typescript/src/utils/mcpValidation.ts`](references/typescript/src/utils/mcpValidation.ts) |
| `references/typescript/src/utils/cleanup.ts` | [`references/typescript/src/utils/cleanup.ts`](references/typescript/src/utils/cleanup.ts) |
| `references/typescript/src/utils/secureStorage/keychainPrefetch.ts` | [`references/typescript/src/utils/secureStorage/keychainPrefetch.ts`](references/typescript/src/utils/secureStorage/keychainPrefetch.ts) |
| `references/typescript/src/utils/secureStorage/fallbackStorage.ts` | [`references/typescript/src/utils/secureStorage/fallbackStorage.ts`](references/typescript/src/utils/secureStorage/fallbackStorage.ts) |
| `references/typescript/src/utils/secureStorage/plainTextStorage.ts` | [`references/typescript/src/utils/secureStorage/plainTextStorage.ts`](references/typescript/src/utils/secureStorage/plainTextStorage.ts) |
| `references/typescript/src/utils/secureStorage/index.ts` | [`references/typescript/src/utils/secureStorage/index.ts`](references/typescript/src/utils/secureStorage/index.ts) |
| `references/typescript/src/utils/secureStorage/macOsKeychainStorage.ts` | [`references/typescript/src/utils/secureStorage/macOsKeychainStorage.ts`](references/typescript/src/utils/secureStorage/macOsKeychainStorage.ts) |
| `references/typescript/src/utils/secureStorage/macOsKeychainHelpers.ts` | [`references/typescript/src/utils/secureStorage/macOsKeychainHelpers.ts`](references/typescript/src/utils/secureStorage/macOsKeychainHelpers.ts) |
| `references/typescript/src/utils/telemetryAttributes.ts` | [`references/typescript/src/utils/telemetryAttributes.ts`](references/typescript/src/utils/telemetryAttributes.ts) |
| `references/typescript/src/utils/userPromptKeywords.ts` | [`references/typescript/src/utils/userPromptKeywords.ts`](references/typescript/src/utils/userPromptKeywords.ts) |
| `references/typescript/src/utils/standaloneAgent.ts` | [`references/typescript/src/utils/standaloneAgent.ts`](references/typescript/src/utils/standaloneAgent.ts) |
| `references/typescript/src/utils/sequential.ts` | [`references/typescript/src/utils/sequential.ts`](references/typescript/src/utils/sequential.ts) |
| `references/typescript/src/utils/attribution.ts` | [`references/typescript/src/utils/attribution.ts`](references/typescript/src/utils/attribution.ts) |
| `references/typescript/src/utils/slowOperations.ts` | [`references/typescript/src/utils/slowOperations.ts`](references/typescript/src/utils/slowOperations.ts) |
| `references/typescript/src/utils/highlightMatch.tsx` | [`references/typescript/src/utils/highlightMatch.tsx`](references/typescript/src/utils/highlightMatch.tsx) |
| `references/typescript/src/utils/messages.ts` | [`references/typescript/src/utils/messages.ts`](references/typescript/src/utils/messages.ts) |
| `references/typescript/src/utils/plugins/pluginInstallationHelpers.ts` | [`references/typescript/src/utils/plugins/pluginInstallationHelpers.ts`](references/typescript/src/utils/plugins/pluginInstallationHelpers.ts) |
| `references/typescript/src/utils/plugins/walkPluginMarkdown.ts` | [`references/typescript/src/utils/plugins/walkPluginMarkdown.ts`](references/typescript/src/utils/plugins/walkPluginMarkdown.ts) |
| `references/typescript/src/utils/plugins/pluginVersioning.ts` | [`references/typescript/src/utils/plugins/pluginVersioning.ts`](references/typescript/src/utils/plugins/pluginVersioning.ts) |
| `references/typescript/src/utils/plugins/zipCache.ts` | [`references/typescript/src/utils/plugins/zipCache.ts`](references/typescript/src/utils/plugins/zipCache.ts) |
| `references/typescript/src/utils/plugins/gitAvailability.ts` | [`references/typescript/src/utils/plugins/gitAvailability.ts`](references/typescript/src/utils/plugins/gitAvailability.ts) |
| `references/typescript/src/utils/plugins/mcpPluginIntegration.ts` | [`references/typescript/src/utils/plugins/mcpPluginIntegration.ts`](references/typescript/src/utils/plugins/mcpPluginIntegration.ts) |
| `references/typescript/src/utils/plugins/installCounts.ts` | [`references/typescript/src/utils/plugins/installCounts.ts`](references/typescript/src/utils/plugins/installCounts.ts) |
| `references/typescript/src/utils/plugins/pluginIdentifier.ts` | [`references/typescript/src/utils/plugins/pluginIdentifier.ts`](references/typescript/src/utils/plugins/pluginIdentifier.ts) |
| `references/typescript/src/utils/plugins/officialMarketplace.ts` | [`references/typescript/src/utils/plugins/officialMarketplace.ts`](references/typescript/src/utils/plugins/officialMarketplace.ts) |
| `references/typescript/src/utils/plugins/dependencyResolver.ts` | [`references/typescript/src/utils/plugins/dependencyResolver.ts`](references/typescript/src/utils/plugins/dependencyResolver.ts) |
| `references/typescript/src/utils/plugins/installedPluginsManager.ts` | [`references/typescript/src/utils/plugins/installedPluginsManager.ts`](references/typescript/src/utils/plugins/installedPluginsManager.ts) |
| `references/typescript/src/utils/plugins/cacheUtils.ts` | [`references/typescript/src/utils/plugins/cacheUtils.ts`](references/typescript/src/utils/plugins/cacheUtils.ts) |
| `references/typescript/src/utils/plugins/pluginFlagging.ts` | [`references/typescript/src/utils/plugins/pluginFlagging.ts`](references/typescript/src/utils/plugins/pluginFlagging.ts) |
| `references/typescript/src/utils/plugins/zipCacheAdapters.ts` | [`references/typescript/src/utils/plugins/zipCacheAdapters.ts`](references/typescript/src/utils/plugins/zipCacheAdapters.ts) |
| `references/typescript/src/utils/plugins/parseMarketplaceInput.ts` | [`references/typescript/src/utils/plugins/parseMarketplaceInput.ts`](references/typescript/src/utils/plugins/parseMarketplaceInput.ts) |
| `references/typescript/src/utils/plugins/loadPluginAgents.ts` | [`references/typescript/src/utils/plugins/loadPluginAgents.ts`](references/typescript/src/utils/plugins/loadPluginAgents.ts) |
| `references/typescript/src/utils/plugins/hintRecommendation.ts` | [`references/typescript/src/utils/plugins/hintRecommendation.ts`](references/typescript/src/utils/plugins/hintRecommendation.ts) |
| `references/typescript/src/utils/plugins/mcpbHandler.ts` | [`references/typescript/src/utils/plugins/mcpbHandler.ts`](references/typescript/src/utils/plugins/mcpbHandler.ts) |
| `references/typescript/src/utils/plugins/officialMarketplaceStartupCheck.ts` | [`references/typescript/src/utils/plugins/officialMarketplaceStartupCheck.ts`](references/typescript/src/utils/plugins/officialMarketplaceStartupCheck.ts) |
| `references/typescript/src/utils/plugins/reconciler.ts` | [`references/typescript/src/utils/plugins/reconciler.ts`](references/typescript/src/utils/plugins/reconciler.ts) |
| `references/typescript/src/utils/plugins/lspPluginIntegration.ts` | [`references/typescript/src/utils/plugins/lspPluginIntegration.ts`](references/typescript/src/utils/plugins/lspPluginIntegration.ts) |
| `references/typescript/src/utils/plugins/pluginStartupCheck.ts` | [`references/typescript/src/utils/plugins/pluginStartupCheck.ts`](references/typescript/src/utils/plugins/pluginStartupCheck.ts) |
| `references/typescript/src/utils/plugins/addDirPluginSettings.ts` | [`references/typescript/src/utils/plugins/addDirPluginSettings.ts`](references/typescript/src/utils/plugins/addDirPluginSettings.ts) |
| `references/typescript/src/utils/plugins/headlessPluginInstall.ts` | [`references/typescript/src/utils/plugins/headlessPluginInstall.ts`](references/typescript/src/utils/plugins/headlessPluginInstall.ts) |
| `references/typescript/src/utils/plugins/pluginPolicy.ts` | [`references/typescript/src/utils/plugins/pluginPolicy.ts`](references/typescript/src/utils/plugins/pluginPolicy.ts) |
| `references/typescript/src/utils/plugins/lspRecommendation.ts` | [`references/typescript/src/utils/plugins/lspRecommendation.ts`](references/typescript/src/utils/plugins/lspRecommendation.ts) |
| `references/typescript/src/utils/plugins/loadPluginHooks.ts` | [`references/typescript/src/utils/plugins/loadPluginHooks.ts`](references/typescript/src/utils/plugins/loadPluginHooks.ts) |
| `references/typescript/src/utils/plugins/validatePlugin.ts` | [`references/typescript/src/utils/plugins/validatePlugin.ts`](references/typescript/src/utils/plugins/validatePlugin.ts) |
| `references/typescript/src/utils/plugins/pluginLoader.ts` | [`references/typescript/src/utils/plugins/pluginLoader.ts`](references/typescript/src/utils/plugins/pluginLoader.ts) |
| `references/typescript/src/utils/plugins/marketplaceManager.ts` | [`references/typescript/src/utils/plugins/marketplaceManager.ts`](references/typescript/src/utils/plugins/marketplaceManager.ts) |
| `references/typescript/src/utils/plugins/loadPluginCommands.ts` | [`references/typescript/src/utils/plugins/loadPluginCommands.ts`](references/typescript/src/utils/plugins/loadPluginCommands.ts) |
| `references/typescript/src/utils/plugins/pluginAutoupdate.ts` | [`references/typescript/src/utils/plugins/pluginAutoupdate.ts`](references/typescript/src/utils/plugins/pluginAutoupdate.ts) |
| `references/typescript/src/utils/plugins/managedPlugins.ts` | [`references/typescript/src/utils/plugins/managedPlugins.ts`](references/typescript/src/utils/plugins/managedPlugins.ts) |
| `references/typescript/src/utils/plugins/marketplaceHelpers.ts` | [`references/typescript/src/utils/plugins/marketplaceHelpers.ts`](references/typescript/src/utils/plugins/marketplaceHelpers.ts) |
| `references/typescript/src/utils/plugins/pluginOptionsStorage.ts` | [`references/typescript/src/utils/plugins/pluginOptionsStorage.ts`](references/typescript/src/utils/plugins/pluginOptionsStorage.ts) |
| `references/typescript/src/utils/plugins/pluginBlocklist.ts` | [`references/typescript/src/utils/plugins/pluginBlocklist.ts`](references/typescript/src/utils/plugins/pluginBlocklist.ts) |
| `references/typescript/src/utils/plugins/refresh.ts` | [`references/typescript/src/utils/plugins/refresh.ts`](references/typescript/src/utils/plugins/refresh.ts) |
| `references/typescript/src/utils/plugins/fetchTelemetry.ts` | [`references/typescript/src/utils/plugins/fetchTelemetry.ts`](references/typescript/src/utils/plugins/fetchTelemetry.ts) |
| `references/typescript/src/utils/plugins/schemas.ts` | [`references/typescript/src/utils/plugins/schemas.ts`](references/typescript/src/utils/plugins/schemas.ts) |
| `references/typescript/src/utils/plugins/loadPluginOutputStyles.ts` | [`references/typescript/src/utils/plugins/loadPluginOutputStyles.ts`](references/typescript/src/utils/plugins/loadPluginOutputStyles.ts) |
| `references/typescript/src/utils/plugins/pluginDirectories.ts` | [`references/typescript/src/utils/plugins/pluginDirectories.ts`](references/typescript/src/utils/plugins/pluginDirectories.ts) |
| `references/typescript/src/utils/plugins/officialMarketplaceGcs.ts` | [`references/typescript/src/utils/plugins/officialMarketplaceGcs.ts`](references/typescript/src/utils/plugins/officialMarketplaceGcs.ts) |
| `references/typescript/src/utils/plugins/orphanedPluginFilter.ts` | [`references/typescript/src/utils/plugins/orphanedPluginFilter.ts`](references/typescript/src/utils/plugins/orphanedPluginFilter.ts) |
| `references/typescript/src/utils/plugins/performStartupChecks.tsx` | [`references/typescript/src/utils/plugins/performStartupChecks.tsx`](references/typescript/src/utils/plugins/performStartupChecks.tsx) |
| `references/typescript/src/utils/json.ts` | [`references/typescript/src/utils/json.ts`](references/typescript/src/utils/json.ts) |
| `references/typescript/src/utils/github/ghAuthStatus.ts` | [`references/typescript/src/utils/github/ghAuthStatus.ts`](references/typescript/src/utils/github/ghAuthStatus.ts) |
| `references/typescript/src/utils/tokenBudget.ts` | [`references/typescript/src/utils/tokenBudget.ts`](references/typescript/src/utils/tokenBudget.ts) |
| `references/typescript/src/utils/objectGroupBy.ts` | [`references/typescript/src/utils/objectGroupBy.ts`](references/typescript/src/utils/objectGroupBy.ts) |
| `references/typescript/src/utils/modelCost.ts` | [`references/typescript/src/utils/modelCost.ts`](references/typescript/src/utils/modelCost.ts) |
| `references/typescript/src/utils/imageResizer.ts` | [`references/typescript/src/utils/imageResizer.ts`](references/typescript/src/utils/imageResizer.ts) |
| `references/typescript/src/utils/immediateCommand.ts` | [`references/typescript/src/utils/immediateCommand.ts`](references/typescript/src/utils/immediateCommand.ts) |
| `references/typescript/src/utils/mcpInstructionsDelta.ts` | [`references/typescript/src/utils/mcpInstructionsDelta.ts`](references/typescript/src/utils/mcpInstructionsDelta.ts) |
| `references/typescript/src/utils/cch.ts` | [`references/typescript/src/utils/cch.ts`](references/typescript/src/utils/cch.ts) |
| `references/typescript/src/utils/execSyncWrapper.ts` | [`references/typescript/src/utils/execSyncWrapper.ts`](references/typescript/src/utils/execSyncWrapper.ts) |
| `references/typescript/src/utils/fileOperationAnalytics.ts` | [`references/typescript/src/utils/fileOperationAnalytics.ts`](references/typescript/src/utils/fileOperationAnalytics.ts) |
| `references/typescript/src/utils/doctorDiagnostic.ts` | [`references/typescript/src/utils/doctorDiagnostic.ts`](references/typescript/src/utils/doctorDiagnostic.ts) |
| `references/typescript/src/utils/cachePaths.ts` | [`references/typescript/src/utils/cachePaths.ts`](references/typescript/src/utils/cachePaths.ts) |
| `references/typescript/src/utils/todo/types.ts` | [`references/typescript/src/utils/todo/types.ts`](references/typescript/src/utils/todo/types.ts) |
| `references/typescript/src/utils/tmuxSocket.ts` | [`references/typescript/src/utils/tmuxSocket.ts`](references/typescript/src/utils/tmuxSocket.ts) |
| `references/typescript/src/utils/agentSwarmsEnabled.ts` | [`references/typescript/src/utils/agentSwarmsEnabled.ts`](references/typescript/src/utils/agentSwarmsEnabled.ts) |
| `references/typescript/src/utils/ink.ts` | [`references/typescript/src/utils/ink.ts`](references/typescript/src/utils/ink.ts) |
| `references/typescript/src/utils/localInstaller.ts` | [`references/typescript/src/utils/localInstaller.ts`](references/typescript/src/utils/localInstaller.ts) |
| `references/typescript/src/utils/semver.ts` | [`references/typescript/src/utils/semver.ts`](references/typescript/src/utils/semver.ts) |
| `references/typescript/src/utils/user.ts` | [`references/typescript/src/utils/user.ts`](references/typescript/src/utils/user.ts) |
| `references/typescript/src/utils/crypto.ts` | [`references/typescript/src/utils/crypto.ts`](references/typescript/src/utils/crypto.ts) |
| `references/typescript/src/utils/codeIndexing.ts` | [`references/typescript/src/utils/codeIndexing.ts`](references/typescript/src/utils/codeIndexing.ts) |
| `references/typescript/src/utils/awsAuthStatusManager.ts` | [`references/typescript/src/utils/awsAuthStatusManager.ts`](references/typescript/src/utils/awsAuthStatusManager.ts) |
| `references/typescript/src/utils/queryProfiler.ts` | [`references/typescript/src/utils/queryProfiler.ts`](references/typescript/src/utils/queryProfiler.ts) |
| `references/typescript/src/utils/cronTasksLock.ts` | [`references/typescript/src/utils/cronTasksLock.ts`](references/typescript/src/utils/cronTasksLock.ts) |
| `references/typescript/src/utils/CircularBuffer.ts` | [`references/typescript/src/utils/CircularBuffer.ts`](references/typescript/src/utils/CircularBuffer.ts) |
| `references/typescript/src/utils/inProcessTeammateHelpers.ts` | [`references/typescript/src/utils/inProcessTeammateHelpers.ts`](references/typescript/src/utils/inProcessTeammateHelpers.ts) |
| `references/typescript/src/utils/displayTags.ts` | [`references/typescript/src/utils/displayTags.ts`](references/typescript/src/utils/displayTags.ts) |
| `references/typescript/src/utils/timeouts.ts` | [`references/typescript/src/utils/timeouts.ts`](references/typescript/src/utils/timeouts.ts) |
| `references/typescript/src/utils/truncate.ts` | [`references/typescript/src/utils/truncate.ts`](references/typescript/src/utils/truncate.ts) |
| `references/typescript/src/utils/queryContext.ts` | [`references/typescript/src/utils/queryContext.ts`](references/typescript/src/utils/queryContext.ts) |
| `references/typescript/src/utils/statusNoticeDefinitions.tsx` | [`references/typescript/src/utils/statusNoticeDefinitions.tsx`](references/typescript/src/utils/statusNoticeDefinitions.tsx) |
| `references/typescript/src/utils/apiPreconnect.ts` | [`references/typescript/src/utils/apiPreconnect.ts`](references/typescript/src/utils/apiPreconnect.ts) |
| `references/typescript/src/utils/claudeInChrome/package.ts` | [`references/typescript/src/utils/claudeInChrome/package.ts`](references/typescript/src/utils/claudeInChrome/package.ts) |
| `references/typescript/src/utils/claudeInChrome/setup.ts` | [`references/typescript/src/utils/claudeInChrome/setup.ts`](references/typescript/src/utils/claudeInChrome/setup.ts) |
| `references/typescript/src/utils/claudeInChrome/toolRendering.tsx` | [`references/typescript/src/utils/claudeInChrome/toolRendering.tsx`](references/typescript/src/utils/claudeInChrome/toolRendering.tsx) |
| `references/typescript/src/utils/claudeInChrome/prompt.ts` | [`references/typescript/src/utils/claudeInChrome/prompt.ts`](references/typescript/src/utils/claudeInChrome/prompt.ts) |
| `references/typescript/src/utils/claudeInChrome/setupPortable.ts` | [`references/typescript/src/utils/claudeInChrome/setupPortable.ts`](references/typescript/src/utils/claudeInChrome/setupPortable.ts) |
| `references/typescript/src/utils/claudeInChrome/common.ts` | [`references/typescript/src/utils/claudeInChrome/common.ts`](references/typescript/src/utils/claudeInChrome/common.ts) |
| `references/typescript/src/utils/claudeInChrome/mcpServer.ts` | [`references/typescript/src/utils/claudeInChrome/mcpServer.ts`](references/typescript/src/utils/claudeInChrome/mcpServer.ts) |
| `references/typescript/src/utils/claudeInChrome/chromeNativeHost.ts` | [`references/typescript/src/utils/claudeInChrome/chromeNativeHost.ts`](references/typescript/src/utils/claudeInChrome/chromeNativeHost.ts) |
| `references/typescript/src/utils/unaryLogging.ts` | [`references/typescript/src/utils/unaryLogging.ts`](references/typescript/src/utils/unaryLogging.ts) |
| `references/typescript/src/utils/controlMessageCompat.ts` | [`references/typescript/src/utils/controlMessageCompat.ts`](references/typescript/src/utils/controlMessageCompat.ts) |
| `references/typescript/src/utils/terminalPanel.ts` | [`references/typescript/src/utils/terminalPanel.ts`](references/typescript/src/utils/terminalPanel.ts) |
| `references/typescript/src/utils/privacyLevel.ts` | [`references/typescript/src/utils/privacyLevel.ts`](references/typescript/src/utils/privacyLevel.ts) |
| `references/typescript/src/utils/contextAnalysis.ts` | [`references/typescript/src/utils/contextAnalysis.ts`](references/typescript/src/utils/contextAnalysis.ts) |
| `references/typescript/src/utils/teamDiscovery.ts` | [`references/typescript/src/utils/teamDiscovery.ts`](references/typescript/src/utils/teamDiscovery.ts) |
| `references/typescript/src/utils/agentContext.ts` | [`references/typescript/src/utils/agentContext.ts`](references/typescript/src/utils/agentContext.ts) |
| `references/typescript/src/utils/streamJsonStdoutGuard.ts` | [`references/typescript/src/utils/streamJsonStdoutGuard.ts`](references/typescript/src/utils/streamJsonStdoutGuard.ts) |
| `references/typescript/src/utils/markdown.ts` | [`references/typescript/src/utils/markdown.ts`](references/typescript/src/utils/markdown.ts) |
| `references/typescript/src/utils/appleTerminalBackup.ts` | [`references/typescript/src/utils/appleTerminalBackup.ts`](references/typescript/src/utils/appleTerminalBackup.ts) |
| `references/typescript/src/utils/findExecutable.ts` | [`references/typescript/src/utils/findExecutable.ts`](references/typescript/src/utils/findExecutable.ts) |
| `references/typescript/src/utils/model/modelCapabilities.ts` | [`references/typescript/src/utils/model/modelCapabilities.ts`](references/typescript/src/utils/model/modelCapabilities.ts) |
| `references/typescript/src/utils/model/bedrock.ts` | [`references/typescript/src/utils/model/bedrock.ts`](references/typescript/src/utils/model/bedrock.ts) |
| `references/typescript/src/utils/model/modelSupportOverrides.ts` | [`references/typescript/src/utils/model/modelSupportOverrides.ts`](references/typescript/src/utils/model/modelSupportOverrides.ts) |
| `references/typescript/src/utils/model/modelOptions.ts` | [`references/typescript/src/utils/model/modelOptions.ts`](references/typescript/src/utils/model/modelOptions.ts) |
| `references/typescript/src/utils/model/check1mAccess.ts` | [`references/typescript/src/utils/model/check1mAccess.ts`](references/typescript/src/utils/model/check1mAccess.ts) |
| `references/typescript/src/utils/model/modelStrings.ts` | [`references/typescript/src/utils/model/modelStrings.ts`](references/typescript/src/utils/model/modelStrings.ts) |
| `references/typescript/src/utils/model/aliases.ts` | [`references/typescript/src/utils/model/aliases.ts`](references/typescript/src/utils/model/aliases.ts) |
| `references/typescript/src/utils/model/model.ts` | [`references/typescript/src/utils/model/model.ts`](references/typescript/src/utils/model/model.ts) |
| `references/typescript/src/utils/model/configs.ts` | [`references/typescript/src/utils/model/configs.ts`](references/typescript/src/utils/model/configs.ts) |
| `references/typescript/src/utils/model/validateModel.ts` | [`references/typescript/src/utils/model/validateModel.ts`](references/typescript/src/utils/model/validateModel.ts) |
| `references/typescript/src/utils/model/agent.ts` | [`references/typescript/src/utils/model/agent.ts`](references/typescript/src/utils/model/agent.ts) |
| `references/typescript/src/utils/model/antModels.ts` | [`references/typescript/src/utils/model/antModels.ts`](references/typescript/src/utils/model/antModels.ts) |
| `references/typescript/src/utils/model/contextWindowUpgradeCheck.ts` | [`references/typescript/src/utils/model/contextWindowUpgradeCheck.ts`](references/typescript/src/utils/model/contextWindowUpgradeCheck.ts) |
| `references/typescript/src/utils/model/deprecation.ts` | [`references/typescript/src/utils/model/deprecation.ts`](references/typescript/src/utils/model/deprecation.ts) |
| `references/typescript/src/utils/model/modelAllowlist.ts` | [`references/typescript/src/utils/model/modelAllowlist.ts`](references/typescript/src/utils/model/modelAllowlist.ts) |
| `references/typescript/src/utils/model/providers.ts` | [`references/typescript/src/utils/model/providers.ts`](references/typescript/src/utils/model/providers.ts) |
| `references/typescript/src/utils/headlessProfiler.ts` | [`references/typescript/src/utils/headlessProfiler.ts`](references/typescript/src/utils/headlessProfiler.ts) |
| `references/typescript/src/utils/promptShellExecution.ts` | [`references/typescript/src/utils/promptShellExecution.ts`](references/typescript/src/utils/promptShellExecution.ts) |
| `references/typescript/src/utils/fpsTracker.ts` | [`references/typescript/src/utils/fpsTracker.ts`](references/typescript/src/utils/fpsTracker.ts) |
| `references/typescript/src/utils/embeddedTools.ts` | [`references/typescript/src/utils/embeddedTools.ts`](references/typescript/src/utils/embeddedTools.ts) |
| `references/typescript/src/utils/imageStore.ts` | [`references/typescript/src/utils/imageStore.ts`](references/typescript/src/utils/imageStore.ts) |
| `references/typescript/src/utils/promptCategory.ts` | [`references/typescript/src/utils/promptCategory.ts`](references/typescript/src/utils/promptCategory.ts) |
| `references/typescript/src/utils/hash.ts` | [`references/typescript/src/utils/hash.ts`](references/typescript/src/utils/hash.ts) |
| `references/typescript/src/utils/pasteStore.ts` | [`references/typescript/src/utils/pasteStore.ts`](references/typescript/src/utils/pasteStore.ts) |
| `references/typescript/src/utils/hooks.ts` | [`references/typescript/src/utils/hooks.ts`](references/typescript/src/utils/hooks.ts) |
| `references/typescript/src/utils/betas.ts` | [`references/typescript/src/utils/betas.ts`](references/typescript/src/utils/betas.ts) |
| `references/typescript/src/utils/mcpWebSocketTransport.ts` | [`references/typescript/src/utils/mcpWebSocketTransport.ts`](references/typescript/src/utils/mcpWebSocketTransport.ts) |
| `references/typescript/src/utils/mcpOutputStorage.ts` | [`references/typescript/src/utils/mcpOutputStorage.ts`](references/typescript/src/utils/mcpOutputStorage.ts) |
| `references/typescript/src/utils/streamlinedTransform.ts` | [`references/typescript/src/utils/streamlinedTransform.ts`](references/typescript/src/utils/streamlinedTransform.ts) |
| `references/typescript/src/utils/QueryGuard.ts` | [`references/typescript/src/utils/QueryGuard.ts`](references/typescript/src/utils/QueryGuard.ts) |
| `references/typescript/src/utils/codex-fetch-adapter.ts` | [`references/typescript/src/utils/codex-fetch-adapter.ts`](references/typescript/src/utils/codex-fetch-adapter.ts) |
| `references/typescript/src/utils/signal.ts` | [`references/typescript/src/utils/signal.ts`](references/typescript/src/utils/signal.ts) |
| `references/typescript/src/utils/planModeV2.ts` | [`references/typescript/src/utils/planModeV2.ts`](references/typescript/src/utils/planModeV2.ts) |
| `references/typescript/src/utils/computerUse/setup.ts` | [`references/typescript/src/utils/computerUse/setup.ts`](references/typescript/src/utils/computerUse/setup.ts) |
| `references/typescript/src/utils/computerUse/wrapper.tsx` | [`references/typescript/src/utils/computerUse/wrapper.tsx`](references/typescript/src/utils/computerUse/wrapper.tsx) |
| `references/typescript/src/utils/computerUse/toolRendering.tsx` | [`references/typescript/src/utils/computerUse/toolRendering.tsx`](references/typescript/src/utils/computerUse/toolRendering.tsx) |
| `references/typescript/src/utils/computerUse/appNames.ts` | [`references/typescript/src/utils/computerUse/appNames.ts`](references/typescript/src/utils/computerUse/appNames.ts) |
| `references/typescript/src/utils/computerUse/swiftLoader.ts` | [`references/typescript/src/utils/computerUse/swiftLoader.ts`](references/typescript/src/utils/computerUse/swiftLoader.ts) |
| `references/typescript/src/utils/computerUse/hostAdapter.ts` | [`references/typescript/src/utils/computerUse/hostAdapter.ts`](references/typescript/src/utils/computerUse/hostAdapter.ts) |
| `references/typescript/src/utils/computerUse/cleanup.ts` | [`references/typescript/src/utils/computerUse/cleanup.ts`](references/typescript/src/utils/computerUse/cleanup.ts) |
| `references/typescript/src/utils/computerUse/inputLoader.ts` | [`references/typescript/src/utils/computerUse/inputLoader.ts`](references/typescript/src/utils/computerUse/inputLoader.ts) |
| `references/typescript/src/utils/computerUse/computerUseLock.ts` | [`references/typescript/src/utils/computerUse/computerUseLock.ts`](references/typescript/src/utils/computerUse/computerUseLock.ts) |
| `references/typescript/src/utils/computerUse/gates.ts` | [`references/typescript/src/utils/computerUse/gates.ts`](references/typescript/src/utils/computerUse/gates.ts) |
| `references/typescript/src/utils/computerUse/common.ts` | [`references/typescript/src/utils/computerUse/common.ts`](references/typescript/src/utils/computerUse/common.ts) |
| `references/typescript/src/utils/computerUse/executor.ts` | [`references/typescript/src/utils/computerUse/executor.ts`](references/typescript/src/utils/computerUse/executor.ts) |
| `references/typescript/src/utils/computerUse/drainRunLoop.ts` | [`references/typescript/src/utils/computerUse/drainRunLoop.ts`](references/typescript/src/utils/computerUse/drainRunLoop.ts) |
| `references/typescript/src/utils/computerUse/mcpServer.ts` | [`references/typescript/src/utils/computerUse/mcpServer.ts`](references/typescript/src/utils/computerUse/mcpServer.ts) |
| `references/typescript/src/utils/computerUse/escHotkey.ts` | [`references/typescript/src/utils/computerUse/escHotkey.ts`](references/typescript/src/utils/computerUse/escHotkey.ts) |
| `references/typescript/src/utils/effort.ts` | [`references/typescript/src/utils/effort.ts`](references/typescript/src/utils/effort.ts) |
| `references/typescript/src/utils/terminal.ts` | [`references/typescript/src/utils/terminal.ts`](references/typescript/src/utils/terminal.ts) |
| `references/typescript/src/utils/toolSearch.ts` | [`references/typescript/src/utils/toolSearch.ts`](references/typescript/src/utils/toolSearch.ts) |
| `references/typescript/src/utils/deepLink/registerProtocol.ts` | [`references/typescript/src/utils/deepLink/registerProtocol.ts`](references/typescript/src/utils/deepLink/registerProtocol.ts) |
| `references/typescript/src/utils/deepLink/banner.ts` | [`references/typescript/src/utils/deepLink/banner.ts`](references/typescript/src/utils/deepLink/banner.ts) |
| `references/typescript/src/utils/deepLink/protocolHandler.ts` | [`references/typescript/src/utils/deepLink/protocolHandler.ts`](references/typescript/src/utils/deepLink/protocolHandler.ts) |
| `references/typescript/src/utils/deepLink/terminalLauncher.ts` | [`references/typescript/src/utils/deepLink/terminalLauncher.ts`](references/typescript/src/utils/deepLink/terminalLauncher.ts) |
| `references/typescript/src/utils/deepLink/parseDeepLink.ts` | [`references/typescript/src/utils/deepLink/parseDeepLink.ts`](references/typescript/src/utils/deepLink/parseDeepLink.ts) |
| `references/typescript/src/utils/deepLink/terminalPreference.ts` | [`references/typescript/src/utils/deepLink/terminalPreference.ts`](references/typescript/src/utils/deepLink/terminalPreference.ts) |
| `references/typescript/src/utils/genericProcessUtils.ts` | [`references/typescript/src/utils/genericProcessUtils.ts`](references/typescript/src/utils/genericProcessUtils.ts) |
| `references/typescript/src/utils/tokens.ts` | [`references/typescript/src/utils/tokens.ts`](references/typescript/src/utils/tokens.ts) |
| `references/typescript/src/utils/conversationRecovery.ts` | [`references/typescript/src/utils/conversationRecovery.ts`](references/typescript/src/utils/conversationRecovery.ts) |
| `references/typescript/src/utils/agentId.ts` | [`references/typescript/src/utils/agentId.ts`](references/typescript/src/utils/agentId.ts) |
| `references/typescript/src/utils/bufferedWriter.ts` | [`references/typescript/src/utils/bufferedWriter.ts`](references/typescript/src/utils/bufferedWriter.ts) |
| `references/typescript/src/utils/idleTimeout.ts` | [`references/typescript/src/utils/idleTimeout.ts`](references/typescript/src/utils/idleTimeout.ts) |
| `references/typescript/src/utils/sideQuestion.ts` | [`references/typescript/src/utils/sideQuestion.ts`](references/typescript/src/utils/sideQuestion.ts) |
| `references/typescript/src/utils/formatBriefTimestamp.ts` | [`references/typescript/src/utils/formatBriefTimestamp.ts`](references/typescript/src/utils/formatBriefTimestamp.ts) |
| `references/typescript/src/utils/bash/shellCompletion.ts` | [`references/typescript/src/utils/bash/shellCompletion.ts`](references/typescript/src/utils/bash/shellCompletion.ts) |
| `references/typescript/src/utils/bash/prefix.ts` | [`references/typescript/src/utils/bash/prefix.ts`](references/typescript/src/utils/bash/prefix.ts) |
| `references/typescript/src/utils/bash/bashParser.ts` | [`references/typescript/src/utils/bash/bashParser.ts`](references/typescript/src/utils/bash/bashParser.ts) |
| `references/typescript/src/utils/bash/treeSitterAnalysis.ts` | [`references/typescript/src/utils/bash/treeSitterAnalysis.ts`](references/typescript/src/utils/bash/treeSitterAnalysis.ts) |
| `references/typescript/src/utils/bash/shellQuote.ts` | [`references/typescript/src/utils/bash/shellQuote.ts`](references/typescript/src/utils/bash/shellQuote.ts) |
| `references/typescript/src/utils/bash/registry.ts` | [`references/typescript/src/utils/bash/registry.ts`](references/typescript/src/utils/bash/registry.ts) |
| `references/typescript/src/utils/bash/ast.ts` | [`references/typescript/src/utils/bash/ast.ts`](references/typescript/src/utils/bash/ast.ts) |
| `references/typescript/src/utils/bash/specs/alias.ts` | [`references/typescript/src/utils/bash/specs/alias.ts`](references/typescript/src/utils/bash/specs/alias.ts) |
| `references/typescript/src/utils/bash/specs/nohup.ts` | [`references/typescript/src/utils/bash/specs/nohup.ts`](references/typescript/src/utils/bash/specs/nohup.ts) |
| `references/typescript/src/utils/bash/specs/time.ts` | [`references/typescript/src/utils/bash/specs/time.ts`](references/typescript/src/utils/bash/specs/time.ts) |
| `references/typescript/src/utils/bash/specs/timeout.ts` | [`references/typescript/src/utils/bash/specs/timeout.ts`](references/typescript/src/utils/bash/specs/timeout.ts) |
| `references/typescript/src/utils/bash/specs/index.ts` | [`references/typescript/src/utils/bash/specs/index.ts`](references/typescript/src/utils/bash/specs/index.ts) |
| `references/typescript/src/utils/bash/specs/srun.ts` | [`references/typescript/src/utils/bash/specs/srun.ts`](references/typescript/src/utils/bash/specs/srun.ts) |
| `references/typescript/src/utils/bash/specs/pyright.ts` | [`references/typescript/src/utils/bash/specs/pyright.ts`](references/typescript/src/utils/bash/specs/pyright.ts) |
| `references/typescript/src/utils/bash/specs/sleep.ts` | [`references/typescript/src/utils/bash/specs/sleep.ts`](references/typescript/src/utils/bash/specs/sleep.ts) |
| `references/typescript/src/utils/bash/shellQuoting.ts` | [`references/typescript/src/utils/bash/shellQuoting.ts`](references/typescript/src/utils/bash/shellQuoting.ts) |
| `references/typescript/src/utils/bash/heredoc.ts` | [`references/typescript/src/utils/bash/heredoc.ts`](references/typescript/src/utils/bash/heredoc.ts) |
| `references/typescript/src/utils/bash/ParsedCommand.ts` | [`references/typescript/src/utils/bash/ParsedCommand.ts`](references/typescript/src/utils/bash/ParsedCommand.ts) |
| `references/typescript/src/utils/bash/parser.ts` | [`references/typescript/src/utils/bash/parser.ts`](references/typescript/src/utils/bash/parser.ts) |
| `references/typescript/src/utils/bash/commands.ts` | [`references/typescript/src/utils/bash/commands.ts`](references/typescript/src/utils/bash/commands.ts) |
| `references/typescript/src/utils/bash/bashPipeCommand.ts` | [`references/typescript/src/utils/bash/bashPipeCommand.ts`](references/typescript/src/utils/bash/bashPipeCommand.ts) |
| `references/typescript/src/utils/bash/ShellSnapshot.ts` | [`references/typescript/src/utils/bash/ShellSnapshot.ts`](references/typescript/src/utils/bash/ShellSnapshot.ts) |
| `references/typescript/src/utils/bash/shellPrefix.ts` | [`references/typescript/src/utils/bash/shellPrefix.ts`](references/typescript/src/utils/bash/shellPrefix.ts) |
| `references/typescript/src/utils/teammateContext.ts` | [`references/typescript/src/utils/teammateContext.ts`](references/typescript/src/utils/teammateContext.ts) |
| `references/typescript/src/utils/cwd.ts` | [`references/typescript/src/utils/cwd.ts`](references/typescript/src/utils/cwd.ts) |
| `references/typescript/src/utils/errors.ts` | [`references/typescript/src/utils/errors.ts`](references/typescript/src/utils/errors.ts) |
| `references/typescript/src/utils/workloadContext.ts` | [`references/typescript/src/utils/workloadContext.ts`](references/typescript/src/utils/workloadContext.ts) |
| `references/typescript/src/utils/heatmap.ts` | [`references/typescript/src/utils/heatmap.ts`](references/typescript/src/utils/heatmap.ts) |
| `references/typescript/src/utils/intl.ts` | [`references/typescript/src/utils/intl.ts`](references/typescript/src/utils/intl.ts) |
| `references/typescript/src/utils/hooks/apiQueryHookHelper.ts` | [`references/typescript/src/utils/hooks/apiQueryHookHelper.ts`](references/typescript/src/utils/hooks/apiQueryHookHelper.ts) |
| `references/typescript/src/utils/hooks/hooksConfigSnapshot.ts` | [`references/typescript/src/utils/hooks/hooksConfigSnapshot.ts`](references/typescript/src/utils/hooks/hooksConfigSnapshot.ts) |
| `references/typescript/src/utils/hooks/sessionHooks.ts` | [`references/typescript/src/utils/hooks/sessionHooks.ts`](references/typescript/src/utils/hooks/sessionHooks.ts) |
| `references/typescript/src/utils/hooks/fileChangedWatcher.ts` | [`references/typescript/src/utils/hooks/fileChangedWatcher.ts`](references/typescript/src/utils/hooks/fileChangedWatcher.ts) |
| `references/typescript/src/utils/hooks/hooksConfigManager.ts` | [`references/typescript/src/utils/hooks/hooksConfigManager.ts`](references/typescript/src/utils/hooks/hooksConfigManager.ts) |
| `references/typescript/src/utils/hooks/hookEvents.ts` | [`references/typescript/src/utils/hooks/hookEvents.ts`](references/typescript/src/utils/hooks/hookEvents.ts) |
| `references/typescript/src/utils/hooks/execPromptHook.ts` | [`references/typescript/src/utils/hooks/execPromptHook.ts`](references/typescript/src/utils/hooks/execPromptHook.ts) |
| `references/typescript/src/utils/hooks/execHttpHook.ts` | [`references/typescript/src/utils/hooks/execHttpHook.ts`](references/typescript/src/utils/hooks/execHttpHook.ts) |
| `references/typescript/src/utils/hooks/ssrfGuard.ts` | [`references/typescript/src/utils/hooks/ssrfGuard.ts`](references/typescript/src/utils/hooks/ssrfGuard.ts) |
| `references/typescript/src/utils/hooks/postSamplingHooks.ts` | [`references/typescript/src/utils/hooks/postSamplingHooks.ts`](references/typescript/src/utils/hooks/postSamplingHooks.ts) |
| `references/typescript/src/utils/hooks/hookHelpers.ts` | [`references/typescript/src/utils/hooks/hookHelpers.ts`](references/typescript/src/utils/hooks/hookHelpers.ts) |
| `references/typescript/src/utils/hooks/AsyncHookRegistry.ts` | [`references/typescript/src/utils/hooks/AsyncHookRegistry.ts`](references/typescript/src/utils/hooks/AsyncHookRegistry.ts) |
| `references/typescript/src/utils/hooks/hooksSettings.ts` | [`references/typescript/src/utils/hooks/hooksSettings.ts`](references/typescript/src/utils/hooks/hooksSettings.ts) |
| `references/typescript/src/utils/hooks/skillImprovement.ts` | [`references/typescript/src/utils/hooks/skillImprovement.ts`](references/typescript/src/utils/hooks/skillImprovement.ts) |
| `references/typescript/src/utils/hooks/registerFrontmatterHooks.ts` | [`references/typescript/src/utils/hooks/registerFrontmatterHooks.ts`](references/typescript/src/utils/hooks/registerFrontmatterHooks.ts) |
| `references/typescript/src/utils/hooks/registerSkillHooks.ts` | [`references/typescript/src/utils/hooks/registerSkillHooks.ts`](references/typescript/src/utils/hooks/registerSkillHooks.ts) |
| `references/typescript/src/utils/hooks/execAgentHook.ts` | [`references/typescript/src/utils/hooks/execAgentHook.ts`](references/typescript/src/utils/hooks/execAgentHook.ts) |
| `references/typescript/src/utils/thinking.ts` | [`references/typescript/src/utils/thinking.ts`](references/typescript/src/utils/thinking.ts) |
| `references/typescript/src/utils/sessionTitle.ts` | [`references/typescript/src/utils/sessionTitle.ts`](references/typescript/src/utils/sessionTitle.ts) |
| `references/typescript/src/utils/tasks.ts` | [`references/typescript/src/utils/tasks.ts`](references/typescript/src/utils/tasks.ts) |
| `references/typescript/src/utils/ide.ts` | [`references/typescript/src/utils/ide.ts`](references/typescript/src/utils/ide.ts) |
| `references/typescript/src/utils/semanticBoolean.ts` | [`references/typescript/src/utils/semanticBoolean.ts`](references/typescript/src/utils/semanticBoolean.ts) |
| `references/typescript/src/utils/teleport.tsx` | [`references/typescript/src/utils/teleport.tsx`](references/typescript/src/utils/teleport.tsx) |
| `references/typescript/src/utils/staticRender.tsx` | [`references/typescript/src/utils/staticRender.tsx`](references/typescript/src/utils/staticRender.tsx) |
| `references/typescript/src/utils/plans.ts` | [`references/typescript/src/utils/plans.ts`](references/typescript/src/utils/plans.ts) |
| `references/typescript/src/utils/uuid.ts` | [`references/typescript/src/utils/uuid.ts`](references/typescript/src/utils/uuid.ts) |
| `references/typescript/src/utils/fileReadCache.ts` | [`references/typescript/src/utils/fileReadCache.ts`](references/typescript/src/utils/fileReadCache.ts) |
| `references/typescript/src/utils/toolErrors.ts` | [`references/typescript/src/utils/toolErrors.ts`](references/typescript/src/utils/toolErrors.ts) |
| `references/typescript/src/utils/bundledMode.ts` | [`references/typescript/src/utils/bundledMode.ts`](references/typescript/src/utils/bundledMode.ts) |
| `references/typescript/src/utils/cleanupRegistry.ts` | [`references/typescript/src/utils/cleanupRegistry.ts`](references/typescript/src/utils/cleanupRegistry.ts) |
| `references/typescript/src/utils/frontmatterParser.ts` | [`references/typescript/src/utils/frontmatterParser.ts`](references/typescript/src/utils/frontmatterParser.ts) |
| `references/typescript/src/utils/Shell.ts` | [`references/typescript/src/utils/Shell.ts`](references/typescript/src/utils/Shell.ts) |
| `references/typescript/src/utils/combinedAbortSignal.ts` | [`references/typescript/src/utils/combinedAbortSignal.ts`](references/typescript/src/utils/combinedAbortSignal.ts) |
| `references/typescript/src/utils/modifiers.ts` | [`references/typescript/src/utils/modifiers.ts`](references/typescript/src/utils/modifiers.ts) |
| `references/typescript/src/utils/auth.ts` | [`references/typescript/src/utils/auth.ts`](references/typescript/src/utils/auth.ts) |
| `references/typescript/src/utils/nativeInstaller/pidLock.ts` | [`references/typescript/src/utils/nativeInstaller/pidLock.ts`](references/typescript/src/utils/nativeInstaller/pidLock.ts) |
| `references/typescript/src/utils/nativeInstaller/installer.ts` | [`references/typescript/src/utils/nativeInstaller/installer.ts`](references/typescript/src/utils/nativeInstaller/installer.ts) |
| `references/typescript/src/utils/nativeInstaller/packageManagers.ts` | [`references/typescript/src/utils/nativeInstaller/packageManagers.ts`](references/typescript/src/utils/nativeInstaller/packageManagers.ts) |
| `references/typescript/src/utils/nativeInstaller/download.ts` | [`references/typescript/src/utils/nativeInstaller/download.ts`](references/typescript/src/utils/nativeInstaller/download.ts) |
| `references/typescript/src/utils/nativeInstaller/index.ts` | [`references/typescript/src/utils/nativeInstaller/index.ts`](references/typescript/src/utils/nativeInstaller/index.ts) |
| `references/typescript/src/utils/earlyInput.ts` | [`references/typescript/src/utils/earlyInput.ts`](references/typescript/src/utils/earlyInput.ts) |
| `references/typescript/src/utils/withResolvers.ts` | [`references/typescript/src/utils/withResolvers.ts`](references/typescript/src/utils/withResolvers.ts) |
| `references/typescript/src/utils/envUtils.ts` | [`references/typescript/src/utils/envUtils.ts`](references/typescript/src/utils/envUtils.ts) |
| `references/typescript/src/utils/cronJitterConfig.ts` | [`references/typescript/src/utils/cronJitterConfig.ts`](references/typescript/src/utils/cronJitterConfig.ts) |
| `references/typescript/src/utils/yaml.ts` | [`references/typescript/src/utils/yaml.ts`](references/typescript/src/utils/yaml.ts) |
| `references/typescript/src/utils/readEditContext.ts` | [`references/typescript/src/utils/readEditContext.ts`](references/typescript/src/utils/readEditContext.ts) |
| `references/typescript/src/utils/ShellCommand.ts` | [`references/typescript/src/utils/ShellCommand.ts`](references/typescript/src/utils/ShellCommand.ts) |
| `references/typescript/src/utils/fileRead.ts` | [`references/typescript/src/utils/fileRead.ts`](references/typescript/src/utils/fileRead.ts) |
| `references/typescript/src/utils/jetbrains.ts` | [`references/typescript/src/utils/jetbrains.ts`](references/typescript/src/utils/jetbrains.ts) |
| `references/typescript/src/utils/messagePredicates.ts` | [`references/typescript/src/utils/messagePredicates.ts`](references/typescript/src/utils/messagePredicates.ts) |
| `references/typescript/src/utils/stats.ts` | [`references/typescript/src/utils/stats.ts`](references/typescript/src/utils/stats.ts) |
| `references/typescript/src/utils/taggedId.ts` | [`references/typescript/src/utils/taggedId.ts`](references/typescript/src/utils/taggedId.ts) |
| `references/typescript/src/utils/sessionStart.ts` | [`references/typescript/src/utils/sessionStart.ts`](references/typescript/src/utils/sessionStart.ts) |
| `references/typescript/src/utils/tempfile.ts` | [`references/typescript/src/utils/tempfile.ts`](references/typescript/src/utils/tempfile.ts) |
| `references/typescript/src/utils/contentArray.ts` | [`references/typescript/src/utils/contentArray.ts`](references/typescript/src/utils/contentArray.ts) |
| `references/typescript/src/utils/status.tsx` | [`references/typescript/src/utils/status.tsx`](references/typescript/src/utils/status.tsx) |
| `references/typescript/src/utils/contextSuggestions.ts` | [`references/typescript/src/utils/contextSuggestions.ts`](references/typescript/src/utils/contextSuggestions.ts) |
| `references/typescript/src/utils/claudemd.ts` | [`references/typescript/src/utils/claudemd.ts`](references/typescript/src/utils/claudemd.ts) |
| `references/typescript/src/utils/teammateMailbox.ts` | [`references/typescript/src/utils/teammateMailbox.ts`](references/typescript/src/utils/teammateMailbox.ts) |
| `references/typescript/src/utils/sessionStoragePortable.ts` | [`references/typescript/src/utils/sessionStoragePortable.ts`](references/typescript/src/utils/sessionStoragePortable.ts) |
| `references/typescript/src/utils/gitSettings.ts` | [`references/typescript/src/utils/gitSettings.ts`](references/typescript/src/utils/gitSettings.ts) |
| `references/typescript/src/utils/semanticNumber.ts` | [`references/typescript/src/utils/semanticNumber.ts`](references/typescript/src/utils/semanticNumber.ts) |
| `references/typescript/src/utils/sessionEnvVars.ts` | [`references/typescript/src/utils/sessionEnvVars.ts`](references/typescript/src/utils/sessionEnvVars.ts) |
| `references/typescript/src/utils/billing.ts` | [`references/typescript/src/utils/billing.ts`](references/typescript/src/utils/billing.ts) |
| `references/typescript/src/utils/releaseNotes.ts` | [`references/typescript/src/utils/releaseNotes.ts`](references/typescript/src/utils/releaseNotes.ts) |
| `references/typescript/src/utils/debug.ts` | [`references/typescript/src/utils/debug.ts`](references/typescript/src/utils/debug.ts) |
| `references/typescript/src/utils/claudeDesktop.ts` | [`references/typescript/src/utils/claudeDesktop.ts`](references/typescript/src/utils/claudeDesktop.ts) |
| `references/typescript/src/utils/crossProjectResume.ts` | [`references/typescript/src/utils/crossProjectResume.ts`](references/typescript/src/utils/crossProjectResume.ts) |
| `references/typescript/src/utils/worktreeModeEnabled.ts` | [`references/typescript/src/utils/worktreeModeEnabled.ts`](references/typescript/src/utils/worktreeModeEnabled.ts) |
| `references/typescript/src/utils/commandLifecycle.ts` | [`references/typescript/src/utils/commandLifecycle.ts`](references/typescript/src/utils/commandLifecycle.ts) |
| `references/typescript/src/utils/background/remote/preconditions.ts` | [`references/typescript/src/utils/background/remote/preconditions.ts`](references/typescript/src/utils/background/remote/preconditions.ts) |
| `references/typescript/src/utils/background/remote/remoteSession.ts` | [`references/typescript/src/utils/background/remote/remoteSession.ts`](references/typescript/src/utils/background/remote/remoteSession.ts) |
| `references/typescript/src/utils/sinks.ts` | [`references/typescript/src/utils/sinks.ts`](references/typescript/src/utils/sinks.ts) |
| `references/typescript/src/utils/path.ts` | [`references/typescript/src/utils/path.ts`](references/typescript/src/utils/path.ts) |
| `references/typescript/src/utils/pdfUtils.ts` | [`references/typescript/src/utils/pdfUtils.ts`](references/typescript/src/utils/pdfUtils.ts) |
| `references/typescript/src/utils/undercover.ts` | [`references/typescript/src/utils/undercover.ts`](references/typescript/src/utils/undercover.ts) |
| `references/typescript/src/utils/cronTasks.ts` | [`references/typescript/src/utils/cronTasks.ts`](references/typescript/src/utils/cronTasks.ts) |
| `references/typescript/src/utils/gracefulShutdown.ts` | [`references/typescript/src/utils/gracefulShutdown.ts`](references/typescript/src/utils/gracefulShutdown.ts) |
| `references/typescript/src/utils/xdg.ts` | [`references/typescript/src/utils/xdg.ts`](references/typescript/src/utils/xdg.ts) |
| `references/typescript/src/utils/git.ts` | [`references/typescript/src/utils/git.ts`](references/typescript/src/utils/git.ts) |
| `references/typescript/src/utils/sessionStorage.ts` | [`references/typescript/src/utils/sessionStorage.ts`](references/typescript/src/utils/sessionStorage.ts) |
| `references/typescript/src/utils/cliArgs.ts` | [`references/typescript/src/utils/cliArgs.ts`](references/typescript/src/utils/cliArgs.ts) |
| `references/typescript/src/utils/directMemberMessage.ts` | [`references/typescript/src/utils/directMemberMessage.ts`](references/typescript/src/utils/directMemberMessage.ts) |
| `references/typescript/src/utils/teleport/environmentSelection.ts` | [`references/typescript/src/utils/teleport/environmentSelection.ts`](references/typescript/src/utils/teleport/environmentSelection.ts) |
| `references/typescript/src/utils/teleport/gitBundle.ts` | [`references/typescript/src/utils/teleport/gitBundle.ts`](references/typescript/src/utils/teleport/gitBundle.ts) |
| `references/typescript/src/utils/teleport/environments.ts` | [`references/typescript/src/utils/teleport/environments.ts`](references/typescript/src/utils/teleport/environments.ts) |
| `references/typescript/src/utils/teleport/api.ts` | [`references/typescript/src/utils/teleport/api.ts`](references/typescript/src/utils/teleport/api.ts) |
| `references/typescript/src/utils/sessionFileAccessHooks.ts` | [`references/typescript/src/utils/sessionFileAccessHooks.ts`](references/typescript/src/utils/sessionFileAccessHooks.ts) |
| `references/typescript/src/utils/skills/skillChangeDetector.ts` | [`references/typescript/src/utils/skills/skillChangeDetector.ts`](references/typescript/src/utils/skills/skillChangeDetector.ts) |
| `references/typescript/src/utils/horizontalScroll.ts` | [`references/typescript/src/utils/horizontalScroll.ts`](references/typescript/src/utils/horizontalScroll.ts) |
| `references/typescript/src/utils/logoV2Utils.ts` | [`references/typescript/src/utils/logoV2Utils.ts`](references/typescript/src/utils/logoV2Utils.ts) |
| `references/typescript/src/utils/jsonRead.ts` | [`references/typescript/src/utils/jsonRead.ts`](references/typescript/src/utils/jsonRead.ts) |
| `references/typescript/src/utils/sideQuery.ts` | [`references/typescript/src/utils/sideQuery.ts`](references/typescript/src/utils/sideQuery.ts) |
| `references/typescript/src/utils/abortController.ts` | [`references/typescript/src/utils/abortController.ts`](references/typescript/src/utils/abortController.ts) |
| `references/typescript/src/utils/autoModeDenials.ts` | [`references/typescript/src/utils/autoModeDenials.ts`](references/typescript/src/utils/autoModeDenials.ts) |
| `references/typescript/src/utils/listSessionsImpl.ts` | [`references/typescript/src/utils/listSessionsImpl.ts`](references/typescript/src/utils/listSessionsImpl.ts) |
| `references/typescript/src/utils/zodToJsonSchema.ts` | [`references/typescript/src/utils/zodToJsonSchema.ts`](references/typescript/src/utils/zodToJsonSchema.ts) |
| `references/typescript/src/utils/statsCache.ts` | [`references/typescript/src/utils/statsCache.ts`](references/typescript/src/utils/statsCache.ts) |
| `references/typescript/src/utils/pdf.ts` | [`references/typescript/src/utils/pdf.ts`](references/typescript/src/utils/pdf.ts) |
| `references/typescript/src/utils/format.ts` | [`references/typescript/src/utils/format.ts`](references/typescript/src/utils/format.ts) |
| `references/typescript/src/utils/configConstants.ts` | [`references/typescript/src/utils/configConstants.ts`](references/typescript/src/utils/configConstants.ts) |
| `references/typescript/src/utils/ghPrStatus.ts` | [`references/typescript/src/utils/ghPrStatus.ts`](references/typescript/src/utils/ghPrStatus.ts) |
| `references/typescript/src/utils/userAgent.ts` | [`references/typescript/src/utils/userAgent.ts`](references/typescript/src/utils/userAgent.ts) |
| `references/typescript/src/utils/queueProcessor.ts` | [`references/typescript/src/utils/queueProcessor.ts`](references/typescript/src/utils/queueProcessor.ts) |
| `references/typescript/src/utils/statusNoticeHelpers.ts` | [`references/typescript/src/utils/statusNoticeHelpers.ts`](references/typescript/src/utils/statusNoticeHelpers.ts) |
| `references/typescript/src/utils/telemetry/logger.ts` | [`references/typescript/src/utils/telemetry/logger.ts`](references/typescript/src/utils/telemetry/logger.ts) |
| `references/typescript/src/utils/telemetry/instrumentation.ts` | [`references/typescript/src/utils/telemetry/instrumentation.ts`](references/typescript/src/utils/telemetry/instrumentation.ts) |
| `references/typescript/src/utils/telemetry/events.ts` | [`references/typescript/src/utils/telemetry/events.ts`](references/typescript/src/utils/telemetry/events.ts) |
| `references/typescript/src/utils/telemetry/perfettoTracing.ts` | [`references/typescript/src/utils/telemetry/perfettoTracing.ts`](references/typescript/src/utils/telemetry/perfettoTracing.ts) |
| `references/typescript/src/utils/telemetry/skillLoadedEvent.ts` | [`references/typescript/src/utils/telemetry/skillLoadedEvent.ts`](references/typescript/src/utils/telemetry/skillLoadedEvent.ts) |
| `references/typescript/src/utils/telemetry/bigqueryExporter.ts` | [`references/typescript/src/utils/telemetry/bigqueryExporter.ts`](references/typescript/src/utils/telemetry/bigqueryExporter.ts) |
| `references/typescript/src/utils/telemetry/betaSessionTracing.ts` | [`references/typescript/src/utils/telemetry/betaSessionTracing.ts`](references/typescript/src/utils/telemetry/betaSessionTracing.ts) |
| `references/typescript/src/utils/telemetry/pluginTelemetry.ts` | [`references/typescript/src/utils/telemetry/pluginTelemetry.ts`](references/typescript/src/utils/telemetry/pluginTelemetry.ts) |
| `references/typescript/src/utils/telemetry/sessionTracing.ts` | [`references/typescript/src/utils/telemetry/sessionTracing.ts`](references/typescript/src/utils/telemetry/sessionTracing.ts) |
| `references/typescript/src/utils/sessionRestore.ts` | [`references/typescript/src/utils/sessionRestore.ts`](references/typescript/src/utils/sessionRestore.ts) |
| `references/typescript/src/utils/sleep.ts` | [`references/typescript/src/utils/sleep.ts`](references/typescript/src/utils/sleep.ts) |
| `references/typescript/src/utils/fastMode.ts` | [`references/typescript/src/utils/fastMode.ts`](references/typescript/src/utils/fastMode.ts) |
| `references/typescript/src/utils/env.ts` | [`references/typescript/src/utils/env.ts`](references/typescript/src/utils/env.ts) |
| `references/typescript/src/utils/debugFilter.ts` | [`references/typescript/src/utils/debugFilter.ts`](references/typescript/src/utils/debugFilter.ts) |
| `references/typescript/src/utils/renderOptions.ts` | [`references/typescript/src/utils/renderOptions.ts`](references/typescript/src/utils/renderOptions.ts) |
| `references/typescript/src/utils/array.ts` | [`references/typescript/src/utils/array.ts`](references/typescript/src/utils/array.ts) |
| `references/typescript/src/utils/which.ts` | [`references/typescript/src/utils/which.ts`](references/typescript/src/utils/which.ts) |
| `references/typescript/src/utils/groupToolUses.ts` | [`references/typescript/src/utils/groupToolUses.ts`](references/typescript/src/utils/groupToolUses.ts) |
| `references/typescript/src/utils/ultraplan/ccrSession.ts` | [`references/typescript/src/utils/ultraplan/ccrSession.ts`](references/typescript/src/utils/ultraplan/ccrSession.ts) |
| `references/typescript/src/utils/ultraplan/prompt.txt` | [`references/typescript/src/utils/ultraplan/prompt.txt`](references/typescript/src/utils/ultraplan/prompt.txt) |
| `references/typescript/src/utils/ultraplan/keyword.ts` | [`references/typescript/src/utils/ultraplan/keyword.ts`](references/typescript/src/utils/ultraplan/keyword.ts) |
| `references/typescript/src/utils/browser.ts` | [`references/typescript/src/utils/browser.ts`](references/typescript/src/utils/browser.ts) |
| `references/typescript/src/utils/worktree.ts` | [`references/typescript/src/utils/worktree.ts`](references/typescript/src/utils/worktree.ts) |
| `references/typescript/src/utils/attachments.ts` | [`references/typescript/src/utils/attachments.ts`](references/typescript/src/utils/attachments.ts) |
| `references/typescript/src/utils/fsOperations.ts` | [`references/typescript/src/utils/fsOperations.ts`](references/typescript/src/utils/fsOperations.ts) |
| `references/typescript/src/utils/api.ts` | [`references/typescript/src/utils/api.ts`](references/typescript/src/utils/api.ts) |
| `references/typescript/src/utils/sanitization.ts` | [`references/typescript/src/utils/sanitization.ts`](references/typescript/src/utils/sanitization.ts) |
| `references/typescript/src/utils/toolPool.ts` | [`references/typescript/src/utils/toolPool.ts`](references/typescript/src/utils/toolPool.ts) |
| `references/typescript/src/utils/generatedFiles.ts` | [`references/typescript/src/utils/generatedFiles.ts`](references/typescript/src/utils/generatedFiles.ts) |
| `references/typescript/src/utils/githubRepoPathMapping.ts` | [`references/typescript/src/utils/githubRepoPathMapping.ts`](references/typescript/src/utils/githubRepoPathMapping.ts) |
| `references/typescript/src/utils/warningHandler.ts` | [`references/typescript/src/utils/warningHandler.ts`](references/typescript/src/utils/warningHandler.ts) |
| `references/typescript/src/utils/gitDiff.ts` | [`references/typescript/src/utils/gitDiff.ts`](references/typescript/src/utils/gitDiff.ts) |
| `references/typescript/src/utils/windowsPaths.ts` | [`references/typescript/src/utils/windowsPaths.ts`](references/typescript/src/utils/windowsPaths.ts) |
| `references/typescript/src/utils/transcriptSearch.ts` | [`references/typescript/src/utils/transcriptSearch.ts`](references/typescript/src/utils/transcriptSearch.ts) |
| `references/typescript/src/utils/cron.ts` | [`references/typescript/src/utils/cron.ts`](references/typescript/src/utils/cron.ts) |
| `references/typescript/src/utils/getWorktreePathsPortable.ts` | [`references/typescript/src/utils/getWorktreePathsPortable.ts`](references/typescript/src/utils/getWorktreePathsPortable.ts) |
| `references/typescript/src/utils/systemPromptType.ts` | [`references/typescript/src/utils/systemPromptType.ts`](references/typescript/src/utils/systemPromptType.ts) |
| `references/typescript/src/utils/shell/powershellProvider.ts` | [`references/typescript/src/utils/shell/powershellProvider.ts`](references/typescript/src/utils/shell/powershellProvider.ts) |
| `references/typescript/src/utils/shell/resolveDefaultShell.ts` | [`references/typescript/src/utils/shell/resolveDefaultShell.ts`](references/typescript/src/utils/shell/resolveDefaultShell.ts) |
| `references/typescript/src/utils/shell/prefix.ts` | [`references/typescript/src/utils/shell/prefix.ts`](references/typescript/src/utils/shell/prefix.ts) |
| `references/typescript/src/utils/shell/shellProvider.ts` | [`references/typescript/src/utils/shell/shellProvider.ts`](references/typescript/src/utils/shell/shellProvider.ts) |
| `references/typescript/src/utils/shell/readOnlyCommandValidation.ts` | [`references/typescript/src/utils/shell/readOnlyCommandValidation.ts`](references/typescript/src/utils/shell/readOnlyCommandValidation.ts) |
| `references/typescript/src/utils/shell/outputLimits.ts` | [`references/typescript/src/utils/shell/outputLimits.ts`](references/typescript/src/utils/shell/outputLimits.ts) |
| `references/typescript/src/utils/shell/specPrefix.ts` | [`references/typescript/src/utils/shell/specPrefix.ts`](references/typescript/src/utils/shell/specPrefix.ts) |
| `references/typescript/src/utils/shell/shellToolUtils.ts` | [`references/typescript/src/utils/shell/shellToolUtils.ts`](references/typescript/src/utils/shell/shellToolUtils.ts) |
| `references/typescript/src/utils/shell/bashProvider.ts` | [`references/typescript/src/utils/shell/bashProvider.ts`](references/typescript/src/utils/shell/bashProvider.ts) |
| `references/typescript/src/utils/shell/powershellDetection.ts` | [`references/typescript/src/utils/shell/powershellDetection.ts`](references/typescript/src/utils/shell/powershellDetection.ts) |
| `references/typescript/src/utils/permissions/autoModeState.ts` | [`references/typescript/src/utils/permissions/autoModeState.ts`](references/typescript/src/utils/permissions/autoModeState.ts) |
| `references/typescript/src/utils/permissions/dangerousPatterns.ts` | [`references/typescript/src/utils/permissions/dangerousPatterns.ts`](references/typescript/src/utils/permissions/dangerousPatterns.ts) |
| `references/typescript/src/utils/permissions/PermissionRule.ts` | [`references/typescript/src/utils/permissions/PermissionRule.ts`](references/typescript/src/utils/permissions/PermissionRule.ts) |
| `references/typescript/src/utils/permissions/denialTracking.ts` | [`references/typescript/src/utils/permissions/denialTracking.ts`](references/typescript/src/utils/permissions/denialTracking.ts) |
| `references/typescript/src/utils/permissions/shadowedRuleDetection.ts` | [`references/typescript/src/utils/permissions/shadowedRuleDetection.ts`](references/typescript/src/utils/permissions/shadowedRuleDetection.ts) |
| `references/typescript/src/utils/permissions/permissions.ts` | [`references/typescript/src/utils/permissions/permissions.ts`](references/typescript/src/utils/permissions/permissions.ts) |
| `references/typescript/src/utils/permissions/permissionRuleParser.ts` | [`references/typescript/src/utils/permissions/permissionRuleParser.ts`](references/typescript/src/utils/permissions/permissionRuleParser.ts) |
| `references/typescript/src/utils/permissions/filesystem.ts` | [`references/typescript/src/utils/permissions/filesystem.ts`](references/typescript/src/utils/permissions/filesystem.ts) |
| `references/typescript/src/utils/permissions/getNextPermissionMode.ts` | [`references/typescript/src/utils/permissions/getNextPermissionMode.ts`](references/typescript/src/utils/permissions/getNextPermissionMode.ts) |
| `references/typescript/src/utils/permissions/classifierShared.ts` | [`references/typescript/src/utils/permissions/classifierShared.ts`](references/typescript/src/utils/permissions/classifierShared.ts) |
| `references/typescript/src/utils/permissions/bypassPermissionsKillswitch.ts` | [`references/typescript/src/utils/permissions/bypassPermissionsKillswitch.ts`](references/typescript/src/utils/permissions/bypassPermissionsKillswitch.ts) |
| `references/typescript/src/utils/permissions/PermissionPromptToolResultSchema.ts` | [`references/typescript/src/utils/permissions/PermissionPromptToolResultSchema.ts`](references/typescript/src/utils/permissions/PermissionPromptToolResultSchema.ts) |
| `references/typescript/src/utils/permissions/bashClassifier.ts` | [`references/typescript/src/utils/permissions/bashClassifier.ts`](references/typescript/src/utils/permissions/bashClassifier.ts) |
| `references/typescript/src/utils/permissions/PermissionResult.ts` | [`references/typescript/src/utils/permissions/PermissionResult.ts`](references/typescript/src/utils/permissions/PermissionResult.ts) |
| `references/typescript/src/utils/permissions/permissionsLoader.ts` | [`references/typescript/src/utils/permissions/permissionsLoader.ts`](references/typescript/src/utils/permissions/permissionsLoader.ts) |
| `references/typescript/src/utils/permissions/PermissionUpdateSchema.ts` | [`references/typescript/src/utils/permissions/PermissionUpdateSchema.ts`](references/typescript/src/utils/permissions/PermissionUpdateSchema.ts) |
| `references/typescript/src/utils/permissions/permissionExplainer.ts` | [`references/typescript/src/utils/permissions/permissionExplainer.ts`](references/typescript/src/utils/permissions/permissionExplainer.ts) |
| `references/typescript/src/utils/permissions/classifierDecision.ts` | [`references/typescript/src/utils/permissions/classifierDecision.ts`](references/typescript/src/utils/permissions/classifierDecision.ts) |
| `references/typescript/src/utils/permissions/PermissionMode.ts` | [`references/typescript/src/utils/permissions/PermissionMode.ts`](references/typescript/src/utils/permissions/PermissionMode.ts) |
| `references/typescript/src/utils/permissions/PermissionUpdate.ts` | [`references/typescript/src/utils/permissions/PermissionUpdate.ts`](references/typescript/src/utils/permissions/PermissionUpdate.ts) |
| `references/typescript/src/utils/permissions/permissionSetup.ts` | [`references/typescript/src/utils/permissions/permissionSetup.ts`](references/typescript/src/utils/permissions/permissionSetup.ts) |
| `references/typescript/src/utils/permissions/pathValidation.ts` | [`references/typescript/src/utils/permissions/pathValidation.ts`](references/typescript/src/utils/permissions/pathValidation.ts) |
| `references/typescript/src/utils/permissions/shellRuleMatching.ts` | [`references/typescript/src/utils/permissions/shellRuleMatching.ts`](references/typescript/src/utils/permissions/shellRuleMatching.ts) |
| `references/typescript/src/utils/permissions/yoloClassifier.ts` | [`references/typescript/src/utils/permissions/yoloClassifier.ts`](references/typescript/src/utils/permissions/yoloClassifier.ts) |
| `references/typescript/src/utils/envDynamic.ts` | [`references/typescript/src/utils/envDynamic.ts`](references/typescript/src/utils/envDynamic.ts) |
| `references/typescript/src/utils/autoRunIssue.tsx` | [`references/typescript/src/utils/autoRunIssue.tsx`](references/typescript/src/utils/autoRunIssue.tsx) |
| `references/typescript/src/utils/autoUpdater.ts` | [`references/typescript/src/utils/autoUpdater.ts`](references/typescript/src/utils/autoUpdater.ts) |
| `references/typescript/src/utils/imageValidation.ts` | [`references/typescript/src/utils/imageValidation.ts`](references/typescript/src/utils/imageValidation.ts) |
| `references/typescript/src/Tool.ts` | [`references/typescript/src/Tool.ts`](references/typescript/src/Tool.ts) |
| `references/typescript/src/history.ts` | [`references/typescript/src/history.ts`](references/typescript/src/history.ts) |
| `references/typescript/src/voice/voiceModeEnabled.ts` | [`references/typescript/src/voice/voiceModeEnabled.ts`](references/typescript/src/voice/voiceModeEnabled.ts) |
| `references/typescript/src/setup.ts` | [`references/typescript/src/setup.ts`](references/typescript/src/setup.ts) |
| `references/typescript/src/query.ts` | [`references/typescript/src/query.ts`](references/typescript/src/query.ts) |
| `references/typescript/src/entrypoints/mcp.ts` | [`references/typescript/src/entrypoints/mcp.ts`](references/typescript/src/entrypoints/mcp.ts) |
| `references/typescript/src/entrypoints/init.ts` | [`references/typescript/src/entrypoints/init.ts`](references/typescript/src/entrypoints/init.ts) |
| `references/typescript/src/entrypoints/agentSdkTypes.ts` | [`references/typescript/src/entrypoints/agentSdkTypes.ts`](references/typescript/src/entrypoints/agentSdkTypes.ts) |
| `references/typescript/src/entrypoints/cli.tsx` | [`references/typescript/src/entrypoints/cli.tsx`](references/typescript/src/entrypoints/cli.tsx) |
| `references/typescript/src/entrypoints/sdk/coreTypes.ts` | [`references/typescript/src/entrypoints/sdk/coreTypes.ts`](references/typescript/src/entrypoints/sdk/coreTypes.ts) |
| `references/typescript/src/entrypoints/sdk/coreTypes.generated.ts` | [`references/typescript/src/entrypoints/sdk/coreTypes.generated.ts`](references/typescript/src/entrypoints/sdk/coreTypes.generated.ts) |
| `references/typescript/src/entrypoints/sdk/controlSchemas.ts` | [`references/typescript/src/entrypoints/sdk/controlSchemas.ts`](references/typescript/src/entrypoints/sdk/controlSchemas.ts) |
| `references/typescript/src/entrypoints/sdk/toolTypes.ts` | [`references/typescript/src/entrypoints/sdk/toolTypes.ts`](references/typescript/src/entrypoints/sdk/toolTypes.ts) |
| `references/typescript/src/entrypoints/sdk/runtimeTypes.ts` | [`references/typescript/src/entrypoints/sdk/runtimeTypes.ts`](references/typescript/src/entrypoints/sdk/runtimeTypes.ts) |
| `references/typescript/src/entrypoints/sdk/coreSchemas.ts` | [`references/typescript/src/entrypoints/sdk/coreSchemas.ts`](references/typescript/src/entrypoints/sdk/coreSchemas.ts) |
| `references/typescript/src/entrypoints/sandboxTypes.ts` | [`references/typescript/src/entrypoints/sandboxTypes.ts`](references/typescript/src/entrypoints/sandboxTypes.ts) |
| `references/typescript/src/replLauncher.tsx` | [`references/typescript/src/replLauncher.tsx`](references/typescript/src/replLauncher.tsx) |
| `references/typescript/src/cost-tracker.ts` | [`references/typescript/src/cost-tracker.ts`](references/typescript/src/cost-tracker.ts) |
| `references/typescript/src/schemas/hooks.ts` | [`references/typescript/src/schemas/hooks.ts`](references/typescript/src/schemas/hooks.ts) |
| `references/typescript/src/context/fpsMetrics.tsx` | [`references/typescript/src/context/fpsMetrics.tsx`](references/typescript/src/context/fpsMetrics.tsx) |
| `references/typescript/src/context/mailbox.tsx` | [`references/typescript/src/context/mailbox.tsx`](references/typescript/src/context/mailbox.tsx) |
| `references/typescript/src/context/overlayContext.tsx` | [`references/typescript/src/context/overlayContext.tsx`](references/typescript/src/context/overlayContext.tsx) |
| `references/typescript/src/context/stats.tsx` | [`references/typescript/src/context/stats.tsx`](references/typescript/src/context/stats.tsx) |
| `references/typescript/src/context/promptOverlayContext.tsx` | [`references/typescript/src/context/promptOverlayContext.tsx`](references/typescript/src/context/promptOverlayContext.tsx) |
| `references/typescript/src/context/QueuedMessageContext.tsx` | [`references/typescript/src/context/QueuedMessageContext.tsx`](references/typescript/src/context/QueuedMessageContext.tsx) |
| `references/typescript/src/context/notifications.tsx` | [`references/typescript/src/context/notifications.tsx`](references/typescript/src/context/notifications.tsx) |
| `references/typescript/src/context/modalContext.tsx` | [`references/typescript/src/context/modalContext.tsx`](references/typescript/src/context/modalContext.tsx) |
| `references/typescript/src/context/voice.tsx` | [`references/typescript/src/context/voice.tsx`](references/typescript/src/context/voice.tsx) |
| `references/typescript/src/tools/TaskOutputTool/constants.ts` | [`references/typescript/src/tools/TaskOutputTool/constants.ts`](references/typescript/src/tools/TaskOutputTool/constants.ts) |
| `references/typescript/src/tools/TaskOutputTool/TaskOutputTool.tsx` | [`references/typescript/src/tools/TaskOutputTool/TaskOutputTool.tsx`](references/typescript/src/tools/TaskOutputTool/TaskOutputTool.tsx) |
| `references/typescript/src/tools/VerifyPlanExecutionTool/VerifyPlanExecutionTool.ts` | [`references/typescript/src/tools/VerifyPlanExecutionTool/VerifyPlanExecutionTool.ts`](references/typescript/src/tools/VerifyPlanExecutionTool/VerifyPlanExecutionTool.ts) |
| `references/typescript/src/tools/VerifyPlanExecutionTool/constants.ts` | [`references/typescript/src/tools/VerifyPlanExecutionTool/constants.ts`](references/typescript/src/tools/VerifyPlanExecutionTool/constants.ts) |
| `references/typescript/src/tools/TaskUpdateTool/prompt.ts` | [`references/typescript/src/tools/TaskUpdateTool/prompt.ts`](references/typescript/src/tools/TaskUpdateTool/prompt.ts) |
| `references/typescript/src/tools/TaskUpdateTool/TaskUpdateTool.ts` | [`references/typescript/src/tools/TaskUpdateTool/TaskUpdateTool.ts`](references/typescript/src/tools/TaskUpdateTool/TaskUpdateTool.ts) |
| `references/typescript/src/tools/TaskUpdateTool/constants.ts` | [`references/typescript/src/tools/TaskUpdateTool/constants.ts`](references/typescript/src/tools/TaskUpdateTool/constants.ts) |
| `references/typescript/src/tools/REPLTool/primitiveTools.ts` | [`references/typescript/src/tools/REPLTool/primitiveTools.ts`](references/typescript/src/tools/REPLTool/primitiveTools.ts) |
| `references/typescript/src/tools/REPLTool/constants.ts` | [`references/typescript/src/tools/REPLTool/constants.ts`](references/typescript/src/tools/REPLTool/constants.ts) |
| `references/typescript/src/tools/TeamCreateTool/TeamCreateTool.ts` | [`references/typescript/src/tools/TeamCreateTool/TeamCreateTool.ts`](references/typescript/src/tools/TeamCreateTool/TeamCreateTool.ts) |
| `references/typescript/src/tools/TeamCreateTool/prompt.ts` | [`references/typescript/src/tools/TeamCreateTool/prompt.ts`](references/typescript/src/tools/TeamCreateTool/prompt.ts) |
| `references/typescript/src/tools/TeamCreateTool/UI.tsx` | [`references/typescript/src/tools/TeamCreateTool/UI.tsx`](references/typescript/src/tools/TeamCreateTool/UI.tsx) |
| `references/typescript/src/tools/TeamCreateTool/constants.ts` | [`references/typescript/src/tools/TeamCreateTool/constants.ts`](references/typescript/src/tools/TeamCreateTool/constants.ts) |
| `references/typescript/src/tools/WebFetchTool/preapproved.ts` | [`references/typescript/src/tools/WebFetchTool/preapproved.ts`](references/typescript/src/tools/WebFetchTool/preapproved.ts) |
| `references/typescript/src/tools/WebFetchTool/prompt.ts` | [`references/typescript/src/tools/WebFetchTool/prompt.ts`](references/typescript/src/tools/WebFetchTool/prompt.ts) |
| `references/typescript/src/tools/WebFetchTool/UI.tsx` | [`references/typescript/src/tools/WebFetchTool/UI.tsx`](references/typescript/src/tools/WebFetchTool/UI.tsx) |
| `references/typescript/src/tools/WebFetchTool/WebFetchTool.ts` | [`references/typescript/src/tools/WebFetchTool/WebFetchTool.ts`](references/typescript/src/tools/WebFetchTool/WebFetchTool.ts) |
| `references/typescript/src/tools/WebFetchTool/utils.ts` | [`references/typescript/src/tools/WebFetchTool/utils.ts`](references/typescript/src/tools/WebFetchTool/utils.ts) |
| `references/typescript/src/tools/TeamDeleteTool/TeamDeleteTool.ts` | [`references/typescript/src/tools/TeamDeleteTool/TeamDeleteTool.ts`](references/typescript/src/tools/TeamDeleteTool/TeamDeleteTool.ts) |
| `references/typescript/src/tools/TeamDeleteTool/prompt.ts` | [`references/typescript/src/tools/TeamDeleteTool/prompt.ts`](references/typescript/src/tools/TeamDeleteTool/prompt.ts) |
| `references/typescript/src/tools/TeamDeleteTool/UI.tsx` | [`references/typescript/src/tools/TeamDeleteTool/UI.tsx`](references/typescript/src/tools/TeamDeleteTool/UI.tsx) |
| `references/typescript/src/tools/TeamDeleteTool/constants.ts` | [`references/typescript/src/tools/TeamDeleteTool/constants.ts`](references/typescript/src/tools/TeamDeleteTool/constants.ts) |
| `references/typescript/src/tools/MCPTool/prompt.ts` | [`references/typescript/src/tools/MCPTool/prompt.ts`](references/typescript/src/tools/MCPTool/prompt.ts) |
| `references/typescript/src/tools/MCPTool/UI.tsx` | [`references/typescript/src/tools/MCPTool/UI.tsx`](references/typescript/src/tools/MCPTool/UI.tsx) |
| `references/typescript/src/tools/MCPTool/classifyForCollapse.ts` | [`references/typescript/src/tools/MCPTool/classifyForCollapse.ts`](references/typescript/src/tools/MCPTool/classifyForCollapse.ts) |
| `references/typescript/src/tools/MCPTool/MCPTool.ts` | [`references/typescript/src/tools/MCPTool/MCPTool.ts`](references/typescript/src/tools/MCPTool/MCPTool.ts) |
| `references/typescript/src/tools/SendMessageTool/SendMessageTool.ts` | [`references/typescript/src/tools/SendMessageTool/SendMessageTool.ts`](references/typescript/src/tools/SendMessageTool/SendMessageTool.ts) |
| `references/typescript/src/tools/SendMessageTool/prompt.ts` | [`references/typescript/src/tools/SendMessageTool/prompt.ts`](references/typescript/src/tools/SendMessageTool/prompt.ts) |
| `references/typescript/src/tools/SendMessageTool/UI.tsx` | [`references/typescript/src/tools/SendMessageTool/UI.tsx`](references/typescript/src/tools/SendMessageTool/UI.tsx) |
| `references/typescript/src/tools/SendMessageTool/constants.ts` | [`references/typescript/src/tools/SendMessageTool/constants.ts`](references/typescript/src/tools/SendMessageTool/constants.ts) |
| `references/typescript/src/tools/ExitWorktreeTool/prompt.ts` | [`references/typescript/src/tools/ExitWorktreeTool/prompt.ts`](references/typescript/src/tools/ExitWorktreeTool/prompt.ts) |
| `references/typescript/src/tools/ExitWorktreeTool/UI.tsx` | [`references/typescript/src/tools/ExitWorktreeTool/UI.tsx`](references/typescript/src/tools/ExitWorktreeTool/UI.tsx) |
| `references/typescript/src/tools/ExitWorktreeTool/ExitWorktreeTool.ts` | [`references/typescript/src/tools/ExitWorktreeTool/ExitWorktreeTool.ts`](references/typescript/src/tools/ExitWorktreeTool/ExitWorktreeTool.ts) |
| `references/typescript/src/tools/ExitWorktreeTool/constants.ts` | [`references/typescript/src/tools/ExitWorktreeTool/constants.ts`](references/typescript/src/tools/ExitWorktreeTool/constants.ts) |
| `references/typescript/src/tools/LSPTool/LSPTool.ts` | [`references/typescript/src/tools/LSPTool/LSPTool.ts`](references/typescript/src/tools/LSPTool/LSPTool.ts) |
| `references/typescript/src/tools/LSPTool/prompt.ts` | [`references/typescript/src/tools/LSPTool/prompt.ts`](references/typescript/src/tools/LSPTool/prompt.ts) |
| `references/typescript/src/tools/LSPTool/UI.tsx` | [`references/typescript/src/tools/LSPTool/UI.tsx`](references/typescript/src/tools/LSPTool/UI.tsx) |
| `references/typescript/src/tools/LSPTool/formatters.ts` | [`references/typescript/src/tools/LSPTool/formatters.ts`](references/typescript/src/tools/LSPTool/formatters.ts) |
| `references/typescript/src/tools/LSPTool/symbolContext.ts` | [`references/typescript/src/tools/LSPTool/symbolContext.ts`](references/typescript/src/tools/LSPTool/symbolContext.ts) |
| `references/typescript/src/tools/LSPTool/schemas.ts` | [`references/typescript/src/tools/LSPTool/schemas.ts`](references/typescript/src/tools/LSPTool/schemas.ts) |
| `references/typescript/src/tools/FileWriteTool/FileWriteTool.ts` | [`references/typescript/src/tools/FileWriteTool/FileWriteTool.ts`](references/typescript/src/tools/FileWriteTool/FileWriteTool.ts) |
| `references/typescript/src/tools/FileWriteTool/prompt.ts` | [`references/typescript/src/tools/FileWriteTool/prompt.ts`](references/typescript/src/tools/FileWriteTool/prompt.ts) |
| `references/typescript/src/tools/FileWriteTool/UI.tsx` | [`references/typescript/src/tools/FileWriteTool/UI.tsx`](references/typescript/src/tools/FileWriteTool/UI.tsx) |
| `references/typescript/src/tools/WebSearchTool/WebSearchTool.ts` | [`references/typescript/src/tools/WebSearchTool/WebSearchTool.ts`](references/typescript/src/tools/WebSearchTool/WebSearchTool.ts) |
| `references/typescript/src/tools/WebSearchTool/prompt.ts` | [`references/typescript/src/tools/WebSearchTool/prompt.ts`](references/typescript/src/tools/WebSearchTool/prompt.ts) |
| `references/typescript/src/tools/WebSearchTool/UI.tsx` | [`references/typescript/src/tools/WebSearchTool/UI.tsx`](references/typescript/src/tools/WebSearchTool/UI.tsx) |
| `references/typescript/src/tools/ReadMcpResourceTool/prompt.ts` | [`references/typescript/src/tools/ReadMcpResourceTool/prompt.ts`](references/typescript/src/tools/ReadMcpResourceTool/prompt.ts) |
| `references/typescript/src/tools/ReadMcpResourceTool/ReadMcpResourceTool.ts` | [`references/typescript/src/tools/ReadMcpResourceTool/ReadMcpResourceTool.ts`](references/typescript/src/tools/ReadMcpResourceTool/ReadMcpResourceTool.ts) |
| `references/typescript/src/tools/ReadMcpResourceTool/UI.tsx` | [`references/typescript/src/tools/ReadMcpResourceTool/UI.tsx`](references/typescript/src/tools/ReadMcpResourceTool/UI.tsx) |
| `references/typescript/src/tools/ExitPlanModeTool/ExitPlanModeV2Tool.ts` | [`references/typescript/src/tools/ExitPlanModeTool/ExitPlanModeV2Tool.ts`](references/typescript/src/tools/ExitPlanModeTool/ExitPlanModeV2Tool.ts) |
| `references/typescript/src/tools/ExitPlanModeTool/prompt.ts` | [`references/typescript/src/tools/ExitPlanModeTool/prompt.ts`](references/typescript/src/tools/ExitPlanModeTool/prompt.ts) |
| `references/typescript/src/tools/ExitPlanModeTool/UI.tsx` | [`references/typescript/src/tools/ExitPlanModeTool/UI.tsx`](references/typescript/src/tools/ExitPlanModeTool/UI.tsx) |
| `references/typescript/src/tools/ExitPlanModeTool/constants.ts` | [`references/typescript/src/tools/ExitPlanModeTool/constants.ts`](references/typescript/src/tools/ExitPlanModeTool/constants.ts) |
| `references/typescript/src/tools/TungstenTool/TungstenLiveMonitor.tsx` | [`references/typescript/src/tools/TungstenTool/TungstenLiveMonitor.tsx`](references/typescript/src/tools/TungstenTool/TungstenLiveMonitor.tsx) |
| `references/typescript/src/tools/TungstenTool/TungstenTool.ts` | [`references/typescript/src/tools/TungstenTool/TungstenTool.ts`](references/typescript/src/tools/TungstenTool/TungstenTool.ts) |
| `references/typescript/src/tools/AgentTool/resumeAgent.ts` | [`references/typescript/src/tools/AgentTool/resumeAgent.ts`](references/typescript/src/tools/AgentTool/resumeAgent.ts) |
| `references/typescript/src/tools/AgentTool/forkSubagent.ts` | [`references/typescript/src/tools/AgentTool/forkSubagent.ts`](references/typescript/src/tools/AgentTool/forkSubagent.ts) |
| `references/typescript/src/tools/AgentTool/agentToolUtils.ts` | [`references/typescript/src/tools/AgentTool/agentToolUtils.ts`](references/typescript/src/tools/AgentTool/agentToolUtils.ts) |
| `references/typescript/src/tools/AgentTool/agentMemory.ts` | [`references/typescript/src/tools/AgentTool/agentMemory.ts`](references/typescript/src/tools/AgentTool/agentMemory.ts) |
| `references/typescript/src/tools/AgentTool/builtInAgents.ts` | [`references/typescript/src/tools/AgentTool/builtInAgents.ts`](references/typescript/src/tools/AgentTool/builtInAgents.ts) |
| `references/typescript/src/tools/AgentTool/agentDisplay.ts` | [`references/typescript/src/tools/AgentTool/agentDisplay.ts`](references/typescript/src/tools/AgentTool/agentDisplay.ts) |
| `references/typescript/src/tools/AgentTool/prompt.ts` | [`references/typescript/src/tools/AgentTool/prompt.ts`](references/typescript/src/tools/AgentTool/prompt.ts) |
| `references/typescript/src/tools/AgentTool/AgentTool.tsx` | [`references/typescript/src/tools/AgentTool/AgentTool.tsx`](references/typescript/src/tools/AgentTool/AgentTool.tsx) |
| `references/typescript/src/tools/AgentTool/UI.tsx` | [`references/typescript/src/tools/AgentTool/UI.tsx`](references/typescript/src/tools/AgentTool/UI.tsx) |
| `references/typescript/src/tools/AgentTool/agentMemorySnapshot.ts` | [`references/typescript/src/tools/AgentTool/agentMemorySnapshot.ts`](references/typescript/src/tools/AgentTool/agentMemorySnapshot.ts) |
| `references/typescript/src/tools/AgentTool/built-in/claudeCodeGuideAgent.ts` | [`references/typescript/src/tools/AgentTool/built-in/claudeCodeGuideAgent.ts`](references/typescript/src/tools/AgentTool/built-in/claudeCodeGuideAgent.ts) |
| `references/typescript/src/tools/AgentTool/built-in/exploreAgent.ts` | [`references/typescript/src/tools/AgentTool/built-in/exploreAgent.ts`](references/typescript/src/tools/AgentTool/built-in/exploreAgent.ts) |
| `references/typescript/src/tools/AgentTool/built-in/statuslineSetup.ts` | [`references/typescript/src/tools/AgentTool/built-in/statuslineSetup.ts`](references/typescript/src/tools/AgentTool/built-in/statuslineSetup.ts) |
| `references/typescript/src/tools/AgentTool/built-in/verificationAgent.ts` | [`references/typescript/src/tools/AgentTool/built-in/verificationAgent.ts`](references/typescript/src/tools/AgentTool/built-in/verificationAgent.ts) |
| `references/typescript/src/tools/AgentTool/built-in/planAgent.ts` | [`references/typescript/src/tools/AgentTool/built-in/planAgent.ts`](references/typescript/src/tools/AgentTool/built-in/planAgent.ts) |
| `references/typescript/src/tools/AgentTool/built-in/generalPurposeAgent.ts` | [`references/typescript/src/tools/AgentTool/built-in/generalPurposeAgent.ts`](references/typescript/src/tools/AgentTool/built-in/generalPurposeAgent.ts) |
| `references/typescript/src/tools/AgentTool/loadAgentsDir.ts` | [`references/typescript/src/tools/AgentTool/loadAgentsDir.ts`](references/typescript/src/tools/AgentTool/loadAgentsDir.ts) |
| `references/typescript/src/tools/AgentTool/runAgent.ts` | [`references/typescript/src/tools/AgentTool/runAgent.ts`](references/typescript/src/tools/AgentTool/runAgent.ts) |
| `references/typescript/src/tools/AgentTool/constants.ts` | [`references/typescript/src/tools/AgentTool/constants.ts`](references/typescript/src/tools/AgentTool/constants.ts) |
| `references/typescript/src/tools/AgentTool/agentColorManager.ts` | [`references/typescript/src/tools/AgentTool/agentColorManager.ts`](references/typescript/src/tools/AgentTool/agentColorManager.ts) |
| `references/typescript/src/tools/PowerShellTool/powershellPermissions.ts` | [`references/typescript/src/tools/PowerShellTool/powershellPermissions.ts`](references/typescript/src/tools/PowerShellTool/powershellPermissions.ts) |
| `references/typescript/src/tools/PowerShellTool/PowerShellTool.tsx` | [`references/typescript/src/tools/PowerShellTool/PowerShellTool.tsx`](references/typescript/src/tools/PowerShellTool/PowerShellTool.tsx) |
| `references/typescript/src/tools/PowerShellTool/gitSafety.ts` | [`references/typescript/src/tools/PowerShellTool/gitSafety.ts`](references/typescript/src/tools/PowerShellTool/gitSafety.ts) |
| `references/typescript/src/tools/PowerShellTool/toolName.ts` | [`references/typescript/src/tools/PowerShellTool/toolName.ts`](references/typescript/src/tools/PowerShellTool/toolName.ts) |
| `references/typescript/src/tools/PowerShellTool/modeValidation.ts` | [`references/typescript/src/tools/PowerShellTool/modeValidation.ts`](references/typescript/src/tools/PowerShellTool/modeValidation.ts) |
| `references/typescript/src/tools/PowerShellTool/prompt.ts` | [`references/typescript/src/tools/PowerShellTool/prompt.ts`](references/typescript/src/tools/PowerShellTool/prompt.ts) |
| `references/typescript/src/tools/PowerShellTool/commonParameters.ts` | [`references/typescript/src/tools/PowerShellTool/commonParameters.ts`](references/typescript/src/tools/PowerShellTool/commonParameters.ts) |
| `references/typescript/src/tools/PowerShellTool/powershellSecurity.ts` | [`references/typescript/src/tools/PowerShellTool/powershellSecurity.ts`](references/typescript/src/tools/PowerShellTool/powershellSecurity.ts) |
| `references/typescript/src/tools/PowerShellTool/UI.tsx` | [`references/typescript/src/tools/PowerShellTool/UI.tsx`](references/typescript/src/tools/PowerShellTool/UI.tsx) |
| `references/typescript/src/tools/PowerShellTool/clmTypes.ts` | [`references/typescript/src/tools/PowerShellTool/clmTypes.ts`](references/typescript/src/tools/PowerShellTool/clmTypes.ts) |
| `references/typescript/src/tools/PowerShellTool/readOnlyValidation.ts` | [`references/typescript/src/tools/PowerShellTool/readOnlyValidation.ts`](references/typescript/src/tools/PowerShellTool/readOnlyValidation.ts) |
| `references/typescript/src/tools/PowerShellTool/destructiveCommandWarning.ts` | [`references/typescript/src/tools/PowerShellTool/destructiveCommandWarning.ts`](references/typescript/src/tools/PowerShellTool/destructiveCommandWarning.ts) |
| `references/typescript/src/tools/PowerShellTool/pathValidation.ts` | [`references/typescript/src/tools/PowerShellTool/pathValidation.ts`](references/typescript/src/tools/PowerShellTool/pathValidation.ts) |
| `references/typescript/src/tools/PowerShellTool/commandSemantics.ts` | [`references/typescript/src/tools/PowerShellTool/commandSemantics.ts`](references/typescript/src/tools/PowerShellTool/commandSemantics.ts) |
| `references/typescript/src/tools/EnterWorktreeTool/prompt.ts` | [`references/typescript/src/tools/EnterWorktreeTool/prompt.ts`](references/typescript/src/tools/EnterWorktreeTool/prompt.ts) |
| `references/typescript/src/tools/EnterWorktreeTool/UI.tsx` | [`references/typescript/src/tools/EnterWorktreeTool/UI.tsx`](references/typescript/src/tools/EnterWorktreeTool/UI.tsx) |
| `references/typescript/src/tools/EnterWorktreeTool/EnterWorktreeTool.ts` | [`references/typescript/src/tools/EnterWorktreeTool/EnterWorktreeTool.ts`](references/typescript/src/tools/EnterWorktreeTool/EnterWorktreeTool.ts) |
| `references/typescript/src/tools/EnterWorktreeTool/constants.ts` | [`references/typescript/src/tools/EnterWorktreeTool/constants.ts`](references/typescript/src/tools/EnterWorktreeTool/constants.ts) |
| `references/typescript/src/tools/ScheduleCronTool/CronCreateTool.ts` | [`references/typescript/src/tools/ScheduleCronTool/CronCreateTool.ts`](references/typescript/src/tools/ScheduleCronTool/CronCreateTool.ts) |
| `references/typescript/src/tools/ScheduleCronTool/CronListTool.ts` | [`references/typescript/src/tools/ScheduleCronTool/CronListTool.ts`](references/typescript/src/tools/ScheduleCronTool/CronListTool.ts) |
| `references/typescript/src/tools/ScheduleCronTool/prompt.ts` | [`references/typescript/src/tools/ScheduleCronTool/prompt.ts`](references/typescript/src/tools/ScheduleCronTool/prompt.ts) |
| `references/typescript/src/tools/ScheduleCronTool/UI.tsx` | [`references/typescript/src/tools/ScheduleCronTool/UI.tsx`](references/typescript/src/tools/ScheduleCronTool/UI.tsx) |
| `references/typescript/src/tools/ScheduleCronTool/CronDeleteTool.ts` | [`references/typescript/src/tools/ScheduleCronTool/CronDeleteTool.ts`](references/typescript/src/tools/ScheduleCronTool/CronDeleteTool.ts) |
| `references/typescript/src/tools/ConfigTool/ConfigTool.ts` | [`references/typescript/src/tools/ConfigTool/ConfigTool.ts`](references/typescript/src/tools/ConfigTool/ConfigTool.ts) |
| `references/typescript/src/tools/ConfigTool/prompt.ts` | [`references/typescript/src/tools/ConfigTool/prompt.ts`](references/typescript/src/tools/ConfigTool/prompt.ts) |
| `references/typescript/src/tools/ConfigTool/UI.tsx` | [`references/typescript/src/tools/ConfigTool/UI.tsx`](references/typescript/src/tools/ConfigTool/UI.tsx) |
| `references/typescript/src/tools/ConfigTool/supportedSettings.ts` | [`references/typescript/src/tools/ConfigTool/supportedSettings.ts`](references/typescript/src/tools/ConfigTool/supportedSettings.ts) |
| `references/typescript/src/tools/ConfigTool/constants.ts` | [`references/typescript/src/tools/ConfigTool/constants.ts`](references/typescript/src/tools/ConfigTool/constants.ts) |
| `references/typescript/src/tools/ToolSearchTool/prompt.ts` | [`references/typescript/src/tools/ToolSearchTool/prompt.ts`](references/typescript/src/tools/ToolSearchTool/prompt.ts) |
| `references/typescript/src/tools/ToolSearchTool/ToolSearchTool.ts` | [`references/typescript/src/tools/ToolSearchTool/ToolSearchTool.ts`](references/typescript/src/tools/ToolSearchTool/ToolSearchTool.ts) |
| `references/typescript/src/tools/ToolSearchTool/constants.ts` | [`references/typescript/src/tools/ToolSearchTool/constants.ts`](references/typescript/src/tools/ToolSearchTool/constants.ts) |
| `references/typescript/src/tools/WorkflowTool/constants.ts` | [`references/typescript/src/tools/WorkflowTool/constants.ts`](references/typescript/src/tools/WorkflowTool/constants.ts) |
| `references/typescript/src/tools/NotebookEditTool/NotebookEditTool.ts` | [`references/typescript/src/tools/NotebookEditTool/NotebookEditTool.ts`](references/typescript/src/tools/NotebookEditTool/NotebookEditTool.ts) |
| `references/typescript/src/tools/NotebookEditTool/prompt.ts` | [`references/typescript/src/tools/NotebookEditTool/prompt.ts`](references/typescript/src/tools/NotebookEditTool/prompt.ts) |
| `references/typescript/src/tools/NotebookEditTool/UI.tsx` | [`references/typescript/src/tools/NotebookEditTool/UI.tsx`](references/typescript/src/tools/NotebookEditTool/UI.tsx) |
| `references/typescript/src/tools/NotebookEditTool/constants.ts` | [`references/typescript/src/tools/NotebookEditTool/constants.ts`](references/typescript/src/tools/NotebookEditTool/constants.ts) |
| `references/typescript/src/tools/ListMcpResourcesTool/prompt.ts` | [`references/typescript/src/tools/ListMcpResourcesTool/prompt.ts`](references/typescript/src/tools/ListMcpResourcesTool/prompt.ts) |
| `references/typescript/src/tools/ListMcpResourcesTool/UI.tsx` | [`references/typescript/src/tools/ListMcpResourcesTool/UI.tsx`](references/typescript/src/tools/ListMcpResourcesTool/UI.tsx) |
| `references/typescript/src/tools/ListMcpResourcesTool/ListMcpResourcesTool.ts` | [`references/typescript/src/tools/ListMcpResourcesTool/ListMcpResourcesTool.ts`](references/typescript/src/tools/ListMcpResourcesTool/ListMcpResourcesTool.ts) |
| `references/typescript/src/tools/shared/spawnMultiAgent.ts` | [`references/typescript/src/tools/shared/spawnMultiAgent.ts`](references/typescript/src/tools/shared/spawnMultiAgent.ts) |
| `references/typescript/src/tools/shared/gitOperationTracking.ts` | [`references/typescript/src/tools/shared/gitOperationTracking.ts`](references/typescript/src/tools/shared/gitOperationTracking.ts) |
| `references/typescript/src/tools/TaskGetTool/prompt.ts` | [`references/typescript/src/tools/TaskGetTool/prompt.ts`](references/typescript/src/tools/TaskGetTool/prompt.ts) |
| `references/typescript/src/tools/TaskGetTool/TaskGetTool.ts` | [`references/typescript/src/tools/TaskGetTool/TaskGetTool.ts`](references/typescript/src/tools/TaskGetTool/TaskGetTool.ts) |
| `references/typescript/src/tools/TaskGetTool/constants.ts` | [`references/typescript/src/tools/TaskGetTool/constants.ts`](references/typescript/src/tools/TaskGetTool/constants.ts) |
| `references/typescript/src/tools/GlobTool/prompt.ts` | [`references/typescript/src/tools/GlobTool/prompt.ts`](references/typescript/src/tools/GlobTool/prompt.ts) |
| `references/typescript/src/tools/GlobTool/UI.tsx` | [`references/typescript/src/tools/GlobTool/UI.tsx`](references/typescript/src/tools/GlobTool/UI.tsx) |
| `references/typescript/src/tools/GlobTool/GlobTool.ts` | [`references/typescript/src/tools/GlobTool/GlobTool.ts`](references/typescript/src/tools/GlobTool/GlobTool.ts) |
| `references/typescript/src/tools/FileEditTool/prompt.ts` | [`references/typescript/src/tools/FileEditTool/prompt.ts`](references/typescript/src/tools/FileEditTool/prompt.ts) |
| `references/typescript/src/tools/FileEditTool/types.ts` | [`references/typescript/src/tools/FileEditTool/types.ts`](references/typescript/src/tools/FileEditTool/types.ts) |
| `references/typescript/src/tools/FileEditTool/UI.tsx` | [`references/typescript/src/tools/FileEditTool/UI.tsx`](references/typescript/src/tools/FileEditTool/UI.tsx) |
| `references/typescript/src/tools/FileEditTool/utils.ts` | [`references/typescript/src/tools/FileEditTool/utils.ts`](references/typescript/src/tools/FileEditTool/utils.ts) |
| `references/typescript/src/tools/FileEditTool/FileEditTool.ts` | [`references/typescript/src/tools/FileEditTool/FileEditTool.ts`](references/typescript/src/tools/FileEditTool/FileEditTool.ts) |
| `references/typescript/src/tools/FileEditTool/constants.ts` | [`references/typescript/src/tools/FileEditTool/constants.ts`](references/typescript/src/tools/FileEditTool/constants.ts) |
| `references/typescript/src/tools/utils.ts` | [`references/typescript/src/tools/utils.ts`](references/typescript/src/tools/utils.ts) |
| `references/typescript/src/tools/TodoWriteTool/prompt.ts` | [`references/typescript/src/tools/TodoWriteTool/prompt.ts`](references/typescript/src/tools/TodoWriteTool/prompt.ts) |
| `references/typescript/src/tools/TodoWriteTool/TodoWriteTool.ts` | [`references/typescript/src/tools/TodoWriteTool/TodoWriteTool.ts`](references/typescript/src/tools/TodoWriteTool/TodoWriteTool.ts) |
| `references/typescript/src/tools/TodoWriteTool/constants.ts` | [`references/typescript/src/tools/TodoWriteTool/constants.ts`](references/typescript/src/tools/TodoWriteTool/constants.ts) |
| `references/typescript/src/tools/TaskStopTool/TaskStopTool.ts` | [`references/typescript/src/tools/TaskStopTool/TaskStopTool.ts`](references/typescript/src/tools/TaskStopTool/TaskStopTool.ts) |
| `references/typescript/src/tools/TaskStopTool/prompt.ts` | [`references/typescript/src/tools/TaskStopTool/prompt.ts`](references/typescript/src/tools/TaskStopTool/prompt.ts) |
| `references/typescript/src/tools/TaskStopTool/UI.tsx` | [`references/typescript/src/tools/TaskStopTool/UI.tsx`](references/typescript/src/tools/TaskStopTool/UI.tsx) |
| `references/typescript/src/tools/RemoteTriggerTool/prompt.ts` | [`references/typescript/src/tools/RemoteTriggerTool/prompt.ts`](references/typescript/src/tools/RemoteTriggerTool/prompt.ts) |
| `references/typescript/src/tools/RemoteTriggerTool/UI.tsx` | [`references/typescript/src/tools/RemoteTriggerTool/UI.tsx`](references/typescript/src/tools/RemoteTriggerTool/UI.tsx) |
| `references/typescript/src/tools/RemoteTriggerTool/RemoteTriggerTool.ts` | [`references/typescript/src/tools/RemoteTriggerTool/RemoteTriggerTool.ts`](references/typescript/src/tools/RemoteTriggerTool/RemoteTriggerTool.ts) |
| `references/typescript/src/tools/McpAuthTool/McpAuthTool.ts` | [`references/typescript/src/tools/McpAuthTool/McpAuthTool.ts`](references/typescript/src/tools/McpAuthTool/McpAuthTool.ts) |
| `references/typescript/src/tools/SleepTool/prompt.ts` | [`references/typescript/src/tools/SleepTool/prompt.ts`](references/typescript/src/tools/SleepTool/prompt.ts) |
| `references/typescript/src/tools/SyntheticOutputTool/SyntheticOutputTool.ts` | [`references/typescript/src/tools/SyntheticOutputTool/SyntheticOutputTool.ts`](references/typescript/src/tools/SyntheticOutputTool/SyntheticOutputTool.ts) |
| `references/typescript/src/tools/AskUserQuestionTool/prompt.ts` | [`references/typescript/src/tools/AskUserQuestionTool/prompt.ts`](references/typescript/src/tools/AskUserQuestionTool/prompt.ts) |
| `references/typescript/src/tools/AskUserQuestionTool/AskUserQuestionTool.tsx` | [`references/typescript/src/tools/AskUserQuestionTool/AskUserQuestionTool.tsx`](references/typescript/src/tools/AskUserQuestionTool/AskUserQuestionTool.tsx) |
| `references/typescript/src/tools/TaskCreateTool/prompt.ts` | [`references/typescript/src/tools/TaskCreateTool/prompt.ts`](references/typescript/src/tools/TaskCreateTool/prompt.ts) |
| `references/typescript/src/tools/TaskCreateTool/TaskCreateTool.ts` | [`references/typescript/src/tools/TaskCreateTool/TaskCreateTool.ts`](references/typescript/src/tools/TaskCreateTool/TaskCreateTool.ts) |
| `references/typescript/src/tools/TaskCreateTool/constants.ts` | [`references/typescript/src/tools/TaskCreateTool/constants.ts`](references/typescript/src/tools/TaskCreateTool/constants.ts) |
| `references/typescript/src/tools/BashTool/bashCommandHelpers.ts` | [`references/typescript/src/tools/BashTool/bashCommandHelpers.ts`](references/typescript/src/tools/BashTool/bashCommandHelpers.ts) |
| `references/typescript/src/tools/BashTool/sedEditParser.ts` | [`references/typescript/src/tools/BashTool/sedEditParser.ts`](references/typescript/src/tools/BashTool/sedEditParser.ts) |
| `references/typescript/src/tools/BashTool/commentLabel.ts` | [`references/typescript/src/tools/BashTool/commentLabel.ts`](references/typescript/src/tools/BashTool/commentLabel.ts) |
| `references/typescript/src/tools/BashTool/bashSecurity.ts` | [`references/typescript/src/tools/BashTool/bashSecurity.ts`](references/typescript/src/tools/BashTool/bashSecurity.ts) |
| `references/typescript/src/tools/BashTool/BashToolResultMessage.tsx` | [`references/typescript/src/tools/BashTool/BashToolResultMessage.tsx`](references/typescript/src/tools/BashTool/BashToolResultMessage.tsx) |
| `references/typescript/src/tools/BashTool/toolName.ts` | [`references/typescript/src/tools/BashTool/toolName.ts`](references/typescript/src/tools/BashTool/toolName.ts) |
| `references/typescript/src/tools/BashTool/modeValidation.ts` | [`references/typescript/src/tools/BashTool/modeValidation.ts`](references/typescript/src/tools/BashTool/modeValidation.ts) |
| `references/typescript/src/tools/BashTool/prompt.ts` | [`references/typescript/src/tools/BashTool/prompt.ts`](references/typescript/src/tools/BashTool/prompt.ts) |
| `references/typescript/src/tools/BashTool/UI.tsx` | [`references/typescript/src/tools/BashTool/UI.tsx`](references/typescript/src/tools/BashTool/UI.tsx) |
| `references/typescript/src/tools/BashTool/shouldUseSandbox.ts` | [`references/typescript/src/tools/BashTool/shouldUseSandbox.ts`](references/typescript/src/tools/BashTool/shouldUseSandbox.ts) |
| `references/typescript/src/tools/BashTool/utils.ts` | [`references/typescript/src/tools/BashTool/utils.ts`](references/typescript/src/tools/BashTool/utils.ts) |
| `references/typescript/src/tools/BashTool/BashTool.tsx` | [`references/typescript/src/tools/BashTool/BashTool.tsx`](references/typescript/src/tools/BashTool/BashTool.tsx) |
| `references/typescript/src/tools/BashTool/bashPermissions.ts` | [`references/typescript/src/tools/BashTool/bashPermissions.ts`](references/typescript/src/tools/BashTool/bashPermissions.ts) |
| `references/typescript/src/tools/BashTool/readOnlyValidation.ts` | [`references/typescript/src/tools/BashTool/readOnlyValidation.ts`](references/typescript/src/tools/BashTool/readOnlyValidation.ts) |
| `references/typescript/src/tools/BashTool/destructiveCommandWarning.ts` | [`references/typescript/src/tools/BashTool/destructiveCommandWarning.ts`](references/typescript/src/tools/BashTool/destructiveCommandWarning.ts) |
| `references/typescript/src/tools/BashTool/pathValidation.ts` | [`references/typescript/src/tools/BashTool/pathValidation.ts`](references/typescript/src/tools/BashTool/pathValidation.ts) |
| `references/typescript/src/tools/BashTool/commandSemantics.ts` | [`references/typescript/src/tools/BashTool/commandSemantics.ts`](references/typescript/src/tools/BashTool/commandSemantics.ts) |
| `references/typescript/src/tools/BashTool/sedValidation.ts` | [`references/typescript/src/tools/BashTool/sedValidation.ts`](references/typescript/src/tools/BashTool/sedValidation.ts) |
| `references/typescript/src/tools/SkillTool/SkillTool.ts` | [`references/typescript/src/tools/SkillTool/SkillTool.ts`](references/typescript/src/tools/SkillTool/SkillTool.ts) |
| `references/typescript/src/tools/SkillTool/prompt.ts` | [`references/typescript/src/tools/SkillTool/prompt.ts`](references/typescript/src/tools/SkillTool/prompt.ts) |
| `references/typescript/src/tools/SkillTool/UI.tsx` | [`references/typescript/src/tools/SkillTool/UI.tsx`](references/typescript/src/tools/SkillTool/UI.tsx) |
| `references/typescript/src/tools/SkillTool/constants.ts` | [`references/typescript/src/tools/SkillTool/constants.ts`](references/typescript/src/tools/SkillTool/constants.ts) |
| `references/typescript/src/tools/FileReadTool/prompt.ts` | [`references/typescript/src/tools/FileReadTool/prompt.ts`](references/typescript/src/tools/FileReadTool/prompt.ts) |
| `references/typescript/src/tools/FileReadTool/UI.tsx` | [`references/typescript/src/tools/FileReadTool/UI.tsx`](references/typescript/src/tools/FileReadTool/UI.tsx) |
| `references/typescript/src/tools/FileReadTool/FileReadTool.ts` | [`references/typescript/src/tools/FileReadTool/FileReadTool.ts`](references/typescript/src/tools/FileReadTool/FileReadTool.ts) |
| `references/typescript/src/tools/FileReadTool/limits.ts` | [`references/typescript/src/tools/FileReadTool/limits.ts`](references/typescript/src/tools/FileReadTool/limits.ts) |
| `references/typescript/src/tools/FileReadTool/imageProcessor.ts` | [`references/typescript/src/tools/FileReadTool/imageProcessor.ts`](references/typescript/src/tools/FileReadTool/imageProcessor.ts) |
| `references/typescript/src/tools/EnterPlanModeTool/prompt.ts` | [`references/typescript/src/tools/EnterPlanModeTool/prompt.ts`](references/typescript/src/tools/EnterPlanModeTool/prompt.ts) |
| `references/typescript/src/tools/EnterPlanModeTool/UI.tsx` | [`references/typescript/src/tools/EnterPlanModeTool/UI.tsx`](references/typescript/src/tools/EnterPlanModeTool/UI.tsx) |
| `references/typescript/src/tools/EnterPlanModeTool/EnterPlanModeTool.ts` | [`references/typescript/src/tools/EnterPlanModeTool/EnterPlanModeTool.ts`](references/typescript/src/tools/EnterPlanModeTool/EnterPlanModeTool.ts) |
| `references/typescript/src/tools/EnterPlanModeTool/constants.ts` | [`references/typescript/src/tools/EnterPlanModeTool/constants.ts`](references/typescript/src/tools/EnterPlanModeTool/constants.ts) |
| `references/typescript/src/tools/BriefTool/BriefTool.ts` | [`references/typescript/src/tools/BriefTool/BriefTool.ts`](references/typescript/src/tools/BriefTool/BriefTool.ts) |
| `references/typescript/src/tools/BriefTool/prompt.ts` | [`references/typescript/src/tools/BriefTool/prompt.ts`](references/typescript/src/tools/BriefTool/prompt.ts) |
| `references/typescript/src/tools/BriefTool/UI.tsx` | [`references/typescript/src/tools/BriefTool/UI.tsx`](references/typescript/src/tools/BriefTool/UI.tsx) |
| `references/typescript/src/tools/BriefTool/attachments.ts` | [`references/typescript/src/tools/BriefTool/attachments.ts`](references/typescript/src/tools/BriefTool/attachments.ts) |
| `references/typescript/src/tools/BriefTool/upload.ts` | [`references/typescript/src/tools/BriefTool/upload.ts`](references/typescript/src/tools/BriefTool/upload.ts) |
| `references/typescript/src/tools/testing/TestingPermissionTool.tsx` | [`references/typescript/src/tools/testing/TestingPermissionTool.tsx`](references/typescript/src/tools/testing/TestingPermissionTool.tsx) |
| `references/typescript/src/tools/GrepTool/GrepTool.ts` | [`references/typescript/src/tools/GrepTool/GrepTool.ts`](references/typescript/src/tools/GrepTool/GrepTool.ts) |
| `references/typescript/src/tools/GrepTool/prompt.ts` | [`references/typescript/src/tools/GrepTool/prompt.ts`](references/typescript/src/tools/GrepTool/prompt.ts) |
| `references/typescript/src/tools/GrepTool/UI.tsx` | [`references/typescript/src/tools/GrepTool/UI.tsx`](references/typescript/src/tools/GrepTool/UI.tsx) |
| `references/typescript/src/tools/TaskListTool/TaskListTool.ts` | [`references/typescript/src/tools/TaskListTool/TaskListTool.ts`](references/typescript/src/tools/TaskListTool/TaskListTool.ts) |
| `references/typescript/src/tools/TaskListTool/prompt.ts` | [`references/typescript/src/tools/TaskListTool/prompt.ts`](references/typescript/src/tools/TaskListTool/prompt.ts) |
| `references/typescript/src/tools/TaskListTool/constants.ts` | [`references/typescript/src/tools/TaskListTool/constants.ts`](references/typescript/src/tools/TaskListTool/constants.ts) |
| `references/typescript/src/context.ts` | [`references/typescript/src/context.ts`](references/typescript/src/context.ts) |
| `references/typescript/src/interactiveHelpers.tsx` | [`references/typescript/src/interactiveHelpers.tsx`](references/typescript/src/interactiveHelpers.tsx) |
| `references/typescript/src/bridge/debugUtils.ts` | [`references/typescript/src/bridge/debugUtils.ts`](references/typescript/src/bridge/debugUtils.ts) |
| `references/typescript/src/bridge/bridgeStatusUtil.ts` | [`references/typescript/src/bridge/bridgeStatusUtil.ts`](references/typescript/src/bridge/bridgeStatusUtil.ts) |
| `references/typescript/src/bridge/pollConfigDefaults.ts` | [`references/typescript/src/bridge/pollConfigDefaults.ts`](references/typescript/src/bridge/pollConfigDefaults.ts) |
| `references/typescript/src/bridge/remoteBridgeCore.ts` | [`references/typescript/src/bridge/remoteBridgeCore.ts`](references/typescript/src/bridge/remoteBridgeCore.ts) |
| `references/typescript/src/bridge/bridgeMessaging.ts` | [`references/typescript/src/bridge/bridgeMessaging.ts`](references/typescript/src/bridge/bridgeMessaging.ts) |
| `references/typescript/src/bridge/bridgeUI.ts` | [`references/typescript/src/bridge/bridgeUI.ts`](references/typescript/src/bridge/bridgeUI.ts) |
| `references/typescript/src/bridge/workSecret.ts` | [`references/typescript/src/bridge/workSecret.ts`](references/typescript/src/bridge/workSecret.ts) |
| `references/typescript/src/bridge/jwtUtils.ts` | [`references/typescript/src/bridge/jwtUtils.ts`](references/typescript/src/bridge/jwtUtils.ts) |
| `references/typescript/src/bridge/replBridge.ts` | [`references/typescript/src/bridge/replBridge.ts`](references/typescript/src/bridge/replBridge.ts) |
| `references/typescript/src/bridge/trustedDevice.ts` | [`references/typescript/src/bridge/trustedDevice.ts`](references/typescript/src/bridge/trustedDevice.ts) |
| `references/typescript/src/bridge/sessionRunner.ts` | [`references/typescript/src/bridge/sessionRunner.ts`](references/typescript/src/bridge/sessionRunner.ts) |
| `references/typescript/src/bridge/sessionIdCompat.ts` | [`references/typescript/src/bridge/sessionIdCompat.ts`](references/typescript/src/bridge/sessionIdCompat.ts) |
| `references/typescript/src/bridge/codeSessionApi.ts` | [`references/typescript/src/bridge/codeSessionApi.ts`](references/typescript/src/bridge/codeSessionApi.ts) |
| `references/typescript/src/bridge/bridgeEnabled.ts` | [`references/typescript/src/bridge/bridgeEnabled.ts`](references/typescript/src/bridge/bridgeEnabled.ts) |
| `references/typescript/src/bridge/createSession.ts` | [`references/typescript/src/bridge/createSession.ts`](references/typescript/src/bridge/createSession.ts) |
| `references/typescript/src/bridge/pollConfig.ts` | [`references/typescript/src/bridge/pollConfig.ts`](references/typescript/src/bridge/pollConfig.ts) |
| `references/typescript/src/bridge/types.ts` | [`references/typescript/src/bridge/types.ts`](references/typescript/src/bridge/types.ts) |
| `references/typescript/src/bridge/replBridgeHandle.ts` | [`references/typescript/src/bridge/replBridgeHandle.ts`](references/typescript/src/bridge/replBridgeHandle.ts) |
| `references/typescript/src/bridge/initReplBridge.ts` | [`references/typescript/src/bridge/initReplBridge.ts`](references/typescript/src/bridge/initReplBridge.ts) |
| `references/typescript/src/bridge/inboundMessages.ts` | [`references/typescript/src/bridge/inboundMessages.ts`](references/typescript/src/bridge/inboundMessages.ts) |
| `references/typescript/src/bridge/flushGate.ts` | [`references/typescript/src/bridge/flushGate.ts`](references/typescript/src/bridge/flushGate.ts) |
| `references/typescript/src/bridge/bridgePointer.ts` | [`references/typescript/src/bridge/bridgePointer.ts`](references/typescript/src/bridge/bridgePointer.ts) |
| `references/typescript/src/bridge/bridgeConfig.ts` | [`references/typescript/src/bridge/bridgeConfig.ts`](references/typescript/src/bridge/bridgeConfig.ts) |
| `references/typescript/src/bridge/bridgeMain.ts` | [`references/typescript/src/bridge/bridgeMain.ts`](references/typescript/src/bridge/bridgeMain.ts) |
| `references/typescript/src/bridge/envLessBridgeConfig.ts` | [`references/typescript/src/bridge/envLessBridgeConfig.ts`](references/typescript/src/bridge/envLessBridgeConfig.ts) |
| `references/typescript/src/bridge/bridgeApi.ts` | [`references/typescript/src/bridge/bridgeApi.ts`](references/typescript/src/bridge/bridgeApi.ts) |
| `references/typescript/src/bridge/bridgePermissionCallbacks.ts` | [`references/typescript/src/bridge/bridgePermissionCallbacks.ts`](references/typescript/src/bridge/bridgePermissionCallbacks.ts) |
| `references/typescript/src/bridge/inboundAttachments.ts` | [`references/typescript/src/bridge/inboundAttachments.ts`](references/typescript/src/bridge/inboundAttachments.ts) |
| `references/typescript/src/bridge/capacityWake.ts` | [`references/typescript/src/bridge/capacityWake.ts`](references/typescript/src/bridge/capacityWake.ts) |
| `references/typescript/src/bridge/replBridgeTransport.ts` | [`references/typescript/src/bridge/replBridgeTransport.ts`](references/typescript/src/bridge/replBridgeTransport.ts) |
| `references/typescript/src/bridge/bridgeDebug.ts` | [`references/typescript/src/bridge/bridgeDebug.ts`](references/typescript/src/bridge/bridgeDebug.ts) |
| `references/typescript/src/vim/transitions.ts` | [`references/typescript/src/vim/transitions.ts`](references/typescript/src/vim/transitions.ts) |
| `references/typescript/src/vim/motions.ts` | [`references/typescript/src/vim/motions.ts`](references/typescript/src/vim/motions.ts) |
| `references/typescript/src/vim/operators.ts` | [`references/typescript/src/vim/operators.ts`](references/typescript/src/vim/operators.ts) |
| `references/typescript/src/vim/textObjects.ts` | [`references/typescript/src/vim/textObjects.ts`](references/typescript/src/vim/textObjects.ts) |
| `references/typescript/src/vim/types.ts` | [`references/typescript/src/vim/types.ts`](references/typescript/src/vim/types.ts) |
| `references/typescript/src/services/autoDream/config.ts` | [`references/typescript/src/services/autoDream/config.ts`](references/typescript/src/services/autoDream/config.ts) |
| `references/typescript/src/services/autoDream/autoDream.ts` | [`references/typescript/src/services/autoDream/autoDream.ts`](references/typescript/src/services/autoDream/autoDream.ts) |
| `references/typescript/src/services/autoDream/consolidationLock.ts` | [`references/typescript/src/services/autoDream/consolidationLock.ts`](references/typescript/src/services/autoDream/consolidationLock.ts) |
| `references/typescript/src/services/autoDream/consolidationPrompt.ts` | [`references/typescript/src/services/autoDream/consolidationPrompt.ts`](references/typescript/src/services/autoDream/consolidationPrompt.ts) |
| `references/typescript/src/services/voiceKeyterms.ts` | [`references/typescript/src/services/voiceKeyterms.ts`](references/typescript/src/services/voiceKeyterms.ts) |
| `references/typescript/src/services/rateLimitMocking.ts` | [`references/typescript/src/services/rateLimitMocking.ts`](references/typescript/src/services/rateLimitMocking.ts) |
| `references/typescript/src/services/MagicDocs/magicDocs.ts` | [`references/typescript/src/services/MagicDocs/magicDocs.ts`](references/typescript/src/services/MagicDocs/magicDocs.ts) |
| `references/typescript/src/services/MagicDocs/prompts.ts` | [`references/typescript/src/services/MagicDocs/prompts.ts`](references/typescript/src/services/MagicDocs/prompts.ts) |
| `references/typescript/src/services/diagnosticTracking.ts` | [`references/typescript/src/services/diagnosticTracking.ts`](references/typescript/src/services/diagnosticTracking.ts) |
| `references/typescript/src/services/policyLimits/types.ts` | [`references/typescript/src/services/policyLimits/types.ts`](references/typescript/src/services/policyLimits/types.ts) |
| `references/typescript/src/services/policyLimits/index.ts` | [`references/typescript/src/services/policyLimits/index.ts`](references/typescript/src/services/policyLimits/index.ts) |
| `references/typescript/src/services/vcr.ts` | [`references/typescript/src/services/vcr.ts`](references/typescript/src/services/vcr.ts) |
| `references/typescript/src/services/mcpServerApproval.tsx` | [`references/typescript/src/services/mcpServerApproval.tsx`](references/typescript/src/services/mcpServerApproval.tsx) |
| `references/typescript/src/services/voiceStreamSTT.ts` | [`references/typescript/src/services/voiceStreamSTT.ts`](references/typescript/src/services/voiceStreamSTT.ts) |
| `references/typescript/src/services/rateLimitMessages.ts` | [`references/typescript/src/services/rateLimitMessages.ts`](references/typescript/src/services/rateLimitMessages.ts) |
| `references/typescript/src/services/teamMemorySync/teamMemSecretGuard.ts` | [`references/typescript/src/services/teamMemorySync/teamMemSecretGuard.ts`](references/typescript/src/services/teamMemorySync/teamMemSecretGuard.ts) |
| `references/typescript/src/services/teamMemorySync/types.ts` | [`references/typescript/src/services/teamMemorySync/types.ts`](references/typescript/src/services/teamMemorySync/types.ts) |
| `references/typescript/src/services/teamMemorySync/secretScanner.ts` | [`references/typescript/src/services/teamMemorySync/secretScanner.ts`](references/typescript/src/services/teamMemorySync/secretScanner.ts) |
| `references/typescript/src/services/teamMemorySync/index.ts` | [`references/typescript/src/services/teamMemorySync/index.ts`](references/typescript/src/services/teamMemorySync/index.ts) |
| `references/typescript/src/services/teamMemorySync/watcher.ts` | [`references/typescript/src/services/teamMemorySync/watcher.ts`](references/typescript/src/services/teamMemorySync/watcher.ts) |
| `references/typescript/src/services/tips/tipScheduler.ts` | [`references/typescript/src/services/tips/tipScheduler.ts`](references/typescript/src/services/tips/tipScheduler.ts) |
| `references/typescript/src/services/tips/tipRegistry.ts` | [`references/typescript/src/services/tips/tipRegistry.ts`](references/typescript/src/services/tips/tipRegistry.ts) |
| `references/typescript/src/services/tips/tipHistory.ts` | [`references/typescript/src/services/tips/tipHistory.ts`](references/typescript/src/services/tips/tipHistory.ts) |
| `references/typescript/src/services/tokenEstimation.ts` | [`references/typescript/src/services/tokenEstimation.ts`](references/typescript/src/services/tokenEstimation.ts) |
| `references/typescript/src/services/api/logging.ts` | [`references/typescript/src/services/api/logging.ts`](references/typescript/src/services/api/logging.ts) |
| `references/typescript/src/services/api/filesApi.ts` | [`references/typescript/src/services/api/filesApi.ts`](references/typescript/src/services/api/filesApi.ts) |
| `references/typescript/src/services/api/dumpPrompts.ts` | [`references/typescript/src/services/api/dumpPrompts.ts`](references/typescript/src/services/api/dumpPrompts.ts) |
| `references/typescript/src/services/api/client.ts` | [`references/typescript/src/services/api/client.ts`](references/typescript/src/services/api/client.ts) |
| `references/typescript/src/services/api/ultrareviewQuota.ts` | [`references/typescript/src/services/api/ultrareviewQuota.ts`](references/typescript/src/services/api/ultrareviewQuota.ts) |
| `references/typescript/src/services/api/claude.ts` | [`references/typescript/src/services/api/claude.ts`](references/typescript/src/services/api/claude.ts) |
| `references/typescript/src/services/api/sessionIngress.ts` | [`references/typescript/src/services/api/sessionIngress.ts`](references/typescript/src/services/api/sessionIngress.ts) |
| `references/typescript/src/services/api/promptCacheBreakDetection.ts` | [`references/typescript/src/services/api/promptCacheBreakDetection.ts`](references/typescript/src/services/api/promptCacheBreakDetection.ts) |
| `references/typescript/src/services/api/emptyUsage.ts` | [`references/typescript/src/services/api/emptyUsage.ts`](references/typescript/src/services/api/emptyUsage.ts) |
| `references/typescript/src/services/api/errorUtils.ts` | [`references/typescript/src/services/api/errorUtils.ts`](references/typescript/src/services/api/errorUtils.ts) |
| `references/typescript/src/services/api/adminRequests.ts` | [`references/typescript/src/services/api/adminRequests.ts`](references/typescript/src/services/api/adminRequests.ts) |
| `references/typescript/src/services/api/withRetry.ts` | [`references/typescript/src/services/api/withRetry.ts`](references/typescript/src/services/api/withRetry.ts) |
| `references/typescript/src/services/api/codex-fetch-adapter.ts` | [`references/typescript/src/services/api/codex-fetch-adapter.ts`](references/typescript/src/services/api/codex-fetch-adapter.ts) |
| `references/typescript/src/services/api/errors.ts` | [`references/typescript/src/services/api/errors.ts`](references/typescript/src/services/api/errors.ts) |
| `references/typescript/src/services/api/grove.ts` | [`references/typescript/src/services/api/grove.ts`](references/typescript/src/services/api/grove.ts) |
| `references/typescript/src/services/api/bootstrap.ts` | [`references/typescript/src/services/api/bootstrap.ts`](references/typescript/src/services/api/bootstrap.ts) |
| `references/typescript/src/services/api/firstTokenDate.ts` | [`references/typescript/src/services/api/firstTokenDate.ts`](references/typescript/src/services/api/firstTokenDate.ts) |
| `references/typescript/src/services/api/referral.ts` | [`references/typescript/src/services/api/referral.ts`](references/typescript/src/services/api/referral.ts) |
| `references/typescript/src/services/api/metricsOptOut.ts` | [`references/typescript/src/services/api/metricsOptOut.ts`](references/typescript/src/services/api/metricsOptOut.ts) |
| `references/typescript/src/services/api/overageCreditGrant.ts` | [`references/typescript/src/services/api/overageCreditGrant.ts`](references/typescript/src/services/api/overageCreditGrant.ts) |
| `references/typescript/src/services/api/usage.ts` | [`references/typescript/src/services/api/usage.ts`](references/typescript/src/services/api/usage.ts) |
| `references/typescript/src/services/mockRateLimits.ts` | [`references/typescript/src/services/mockRateLimits.ts`](references/typescript/src/services/mockRateLimits.ts) |
| `references/typescript/src/services/tools/toolHooks.ts` | [`references/typescript/src/services/tools/toolHooks.ts`](references/typescript/src/services/tools/toolHooks.ts) |
| `references/typescript/src/services/tools/toolExecution.ts` | [`references/typescript/src/services/tools/toolExecution.ts`](references/typescript/src/services/tools/toolExecution.ts) |
| `references/typescript/src/services/tools/toolOrchestration.ts` | [`references/typescript/src/services/tools/toolOrchestration.ts`](references/typescript/src/services/tools/toolOrchestration.ts) |
| `references/typescript/src/services/tools/StreamingToolExecutor.ts` | [`references/typescript/src/services/tools/StreamingToolExecutor.ts`](references/typescript/src/services/tools/StreamingToolExecutor.ts) |
| `references/typescript/src/services/oauth/codex-client.ts` | [`references/typescript/src/services/oauth/codex-client.ts`](references/typescript/src/services/oauth/codex-client.ts) |
| `references/typescript/src/services/oauth/auth-code-listener.ts` | [`references/typescript/src/services/oauth/auth-code-listener.ts`](references/typescript/src/services/oauth/auth-code-listener.ts) |
| `references/typescript/src/services/oauth/client.ts` | [`references/typescript/src/services/oauth/client.ts`](references/typescript/src/services/oauth/client.ts) |
| `references/typescript/src/services/oauth/crypto.ts` | [`references/typescript/src/services/oauth/crypto.ts`](references/typescript/src/services/oauth/crypto.ts) |
| `references/typescript/src/services/oauth/index.ts` | [`references/typescript/src/services/oauth/index.ts`](references/typescript/src/services/oauth/index.ts) |
| `references/typescript/src/services/oauth/getOauthProfile.ts` | [`references/typescript/src/services/oauth/getOauthProfile.ts`](references/typescript/src/services/oauth/getOauthProfile.ts) |
| `references/typescript/src/services/compact/compact.ts` | [`references/typescript/src/services/compact/compact.ts`](references/typescript/src/services/compact/compact.ts) |
| `references/typescript/src/services/compact/cachedMicrocompact.ts` | [`references/typescript/src/services/compact/cachedMicrocompact.ts`](references/typescript/src/services/compact/cachedMicrocompact.ts) |
| `references/typescript/src/services/compact/compactWarningHook.ts` | [`references/typescript/src/services/compact/compactWarningHook.ts`](references/typescript/src/services/compact/compactWarningHook.ts) |
| `references/typescript/src/services/compact/compactWarningState.ts` | [`references/typescript/src/services/compact/compactWarningState.ts`](references/typescript/src/services/compact/compactWarningState.ts) |
| `references/typescript/src/services/compact/grouping.ts` | [`references/typescript/src/services/compact/grouping.ts`](references/typescript/src/services/compact/grouping.ts) |
| `references/typescript/src/services/compact/autoCompact.ts` | [`references/typescript/src/services/compact/autoCompact.ts`](references/typescript/src/services/compact/autoCompact.ts) |
| `references/typescript/src/services/compact/microCompact.ts` | [`references/typescript/src/services/compact/microCompact.ts`](references/typescript/src/services/compact/microCompact.ts) |
| `references/typescript/src/services/compact/prompt.ts` | [`references/typescript/src/services/compact/prompt.ts`](references/typescript/src/services/compact/prompt.ts) |
| `references/typescript/src/services/compact/timeBasedMCConfig.ts` | [`references/typescript/src/services/compact/timeBasedMCConfig.ts`](references/typescript/src/services/compact/timeBasedMCConfig.ts) |
| `references/typescript/src/services/compact/apiMicrocompact.ts` | [`references/typescript/src/services/compact/apiMicrocompact.ts`](references/typescript/src/services/compact/apiMicrocompact.ts) |
| `references/typescript/src/services/compact/sessionMemoryCompact.ts` | [`references/typescript/src/services/compact/sessionMemoryCompact.ts`](references/typescript/src/services/compact/sessionMemoryCompact.ts) |
| `references/typescript/src/services/compact/snipProjection.ts` | [`references/typescript/src/services/compact/snipProjection.ts`](references/typescript/src/services/compact/snipProjection.ts) |
| `references/typescript/src/services/compact/postCompactCleanup.ts` | [`references/typescript/src/services/compact/postCompactCleanup.ts`](references/typescript/src/services/compact/postCompactCleanup.ts) |
| `references/typescript/src/services/compact/snipCompact.ts` | [`references/typescript/src/services/compact/snipCompact.ts`](references/typescript/src/services/compact/snipCompact.ts) |
| `references/typescript/src/services/compact/cachedMCConfig.ts` | [`references/typescript/src/services/compact/cachedMCConfig.ts`](references/typescript/src/services/compact/cachedMCConfig.ts) |
| `references/typescript/src/services/mcp/xaaIdpLogin.ts` | [`references/typescript/src/services/mcp/xaaIdpLogin.ts`](references/typescript/src/services/mcp/xaaIdpLogin.ts) |
| `references/typescript/src/services/mcp/headersHelper.ts` | [`references/typescript/src/services/mcp/headersHelper.ts`](references/typescript/src/services/mcp/headersHelper.ts) |
| `references/typescript/src/services/mcp/channelNotification.ts` | [`references/typescript/src/services/mcp/channelNotification.ts`](references/typescript/src/services/mcp/channelNotification.ts) |
| `references/typescript/src/services/mcp/xaa.ts` | [`references/typescript/src/services/mcp/xaa.ts`](references/typescript/src/services/mcp/xaa.ts) |
| `references/typescript/src/services/mcp/config.ts` | [`references/typescript/src/services/mcp/config.ts`](references/typescript/src/services/mcp/config.ts) |
| `references/typescript/src/services/mcp/InProcessTransport.ts` | [`references/typescript/src/services/mcp/InProcessTransport.ts`](references/typescript/src/services/mcp/InProcessTransport.ts) |
| `references/typescript/src/services/mcp/client.ts` | [`references/typescript/src/services/mcp/client.ts`](references/typescript/src/services/mcp/client.ts) |
| `references/typescript/src/services/mcp/normalization.ts` | [`references/typescript/src/services/mcp/normalization.ts`](references/typescript/src/services/mcp/normalization.ts) |
| `references/typescript/src/services/mcp/envExpansion.ts` | [`references/typescript/src/services/mcp/envExpansion.ts`](references/typescript/src/services/mcp/envExpansion.ts) |
| `references/typescript/src/services/mcp/vscodeSdkMcp.ts` | [`references/typescript/src/services/mcp/vscodeSdkMcp.ts`](references/typescript/src/services/mcp/vscodeSdkMcp.ts) |
| `references/typescript/src/services/mcp/useManageMCPConnections.ts` | [`references/typescript/src/services/mcp/useManageMCPConnections.ts`](references/typescript/src/services/mcp/useManageMCPConnections.ts) |
| `references/typescript/src/services/mcp/SdkControlTransport.ts` | [`references/typescript/src/services/mcp/SdkControlTransport.ts`](references/typescript/src/services/mcp/SdkControlTransport.ts) |
| `references/typescript/src/services/mcp/elicitationHandler.ts` | [`references/typescript/src/services/mcp/elicitationHandler.ts`](references/typescript/src/services/mcp/elicitationHandler.ts) |
| `references/typescript/src/services/mcp/channelPermissions.ts` | [`references/typescript/src/services/mcp/channelPermissions.ts`](references/typescript/src/services/mcp/channelPermissions.ts) |
| `references/typescript/src/services/mcp/types.ts` | [`references/typescript/src/services/mcp/types.ts`](references/typescript/src/services/mcp/types.ts) |
| `references/typescript/src/services/mcp/claudeai.ts` | [`references/typescript/src/services/mcp/claudeai.ts`](references/typescript/src/services/mcp/claudeai.ts) |
| `references/typescript/src/services/mcp/utils.ts` | [`references/typescript/src/services/mcp/utils.ts`](references/typescript/src/services/mcp/utils.ts) |
| `references/typescript/src/services/mcp/officialRegistry.ts` | [`references/typescript/src/services/mcp/officialRegistry.ts`](references/typescript/src/services/mcp/officialRegistry.ts) |
| `references/typescript/src/services/mcp/auth.ts` | [`references/typescript/src/services/mcp/auth.ts`](references/typescript/src/services/mcp/auth.ts) |
| `references/typescript/src/services/mcp/oauthPort.ts` | [`references/typescript/src/services/mcp/oauthPort.ts`](references/typescript/src/services/mcp/oauthPort.ts) |
| `references/typescript/src/services/mcp/MCPConnectionManager.tsx` | [`references/typescript/src/services/mcp/MCPConnectionManager.tsx`](references/typescript/src/services/mcp/MCPConnectionManager.tsx) |
| `references/typescript/src/services/mcp/mcpStringUtils.ts` | [`references/typescript/src/services/mcp/mcpStringUtils.ts`](references/typescript/src/services/mcp/mcpStringUtils.ts) |
| `references/typescript/src/services/mcp/channelAllowlist.ts` | [`references/typescript/src/services/mcp/channelAllowlist.ts`](references/typescript/src/services/mcp/channelAllowlist.ts) |
| `references/typescript/src/services/remoteManagedSettings/types.ts` | [`references/typescript/src/services/remoteManagedSettings/types.ts`](references/typescript/src/services/remoteManagedSettings/types.ts) |
| `references/typescript/src/services/remoteManagedSettings/securityCheck.tsx` | [`references/typescript/src/services/remoteManagedSettings/securityCheck.tsx`](references/typescript/src/services/remoteManagedSettings/securityCheck.tsx) |
| `references/typescript/src/services/remoteManagedSettings/syncCache.ts` | [`references/typescript/src/services/remoteManagedSettings/syncCache.ts`](references/typescript/src/services/remoteManagedSettings/syncCache.ts) |
| `references/typescript/src/services/remoteManagedSettings/index.ts` | [`references/typescript/src/services/remoteManagedSettings/index.ts`](references/typescript/src/services/remoteManagedSettings/index.ts) |
| `references/typescript/src/services/remoteManagedSettings/syncCacheState.ts` | [`references/typescript/src/services/remoteManagedSettings/syncCacheState.ts`](references/typescript/src/services/remoteManagedSettings/syncCacheState.ts) |
| `references/typescript/src/services/preventSleep.ts` | [`references/typescript/src/services/preventSleep.ts`](references/typescript/src/services/preventSleep.ts) |
| `references/typescript/src/services/plugins/pluginCliCommands.ts` | [`references/typescript/src/services/plugins/pluginCliCommands.ts`](references/typescript/src/services/plugins/pluginCliCommands.ts) |
| `references/typescript/src/services/plugins/pluginOperations.ts` | [`references/typescript/src/services/plugins/pluginOperations.ts`](references/typescript/src/services/plugins/pluginOperations.ts) |
| `references/typescript/src/services/plugins/PluginInstallationManager.ts` | [`references/typescript/src/services/plugins/PluginInstallationManager.ts`](references/typescript/src/services/plugins/PluginInstallationManager.ts) |
| `references/typescript/src/services/contextCollapse/operations.ts` | [`references/typescript/src/services/contextCollapse/operations.ts`](references/typescript/src/services/contextCollapse/operations.ts) |
| `references/typescript/src/services/contextCollapse/index.ts` | [`references/typescript/src/services/contextCollapse/index.ts`](references/typescript/src/services/contextCollapse/index.ts) |
| `references/typescript/src/services/contextCollapse/persist.ts` | [`references/typescript/src/services/contextCollapse/persist.ts`](references/typescript/src/services/contextCollapse/persist.ts) |
| `references/typescript/src/services/AgentSummary/agentSummary.ts` | [`references/typescript/src/services/AgentSummary/agentSummary.ts`](references/typescript/src/services/AgentSummary/agentSummary.ts) |
| `references/typescript/src/services/awaySummary.ts` | [`references/typescript/src/services/awaySummary.ts`](references/typescript/src/services/awaySummary.ts) |
| `references/typescript/src/services/toolUseSummary/toolUseSummaryGenerator.ts` | [`references/typescript/src/services/toolUseSummary/toolUseSummaryGenerator.ts`](references/typescript/src/services/toolUseSummary/toolUseSummaryGenerator.ts) |
| `references/typescript/src/services/extractMemories/prompts.ts` | [`references/typescript/src/services/extractMemories/prompts.ts`](references/typescript/src/services/extractMemories/prompts.ts) |
| `references/typescript/src/services/extractMemories/extractMemories.ts` | [`references/typescript/src/services/extractMemories/extractMemories.ts`](references/typescript/src/services/extractMemories/extractMemories.ts) |
| `references/typescript/src/services/settingsSync/types.ts` | [`references/typescript/src/services/settingsSync/types.ts`](references/typescript/src/services/settingsSync/types.ts) |
| `references/typescript/src/services/settingsSync/index.ts` | [`references/typescript/src/services/settingsSync/index.ts`](references/typescript/src/services/settingsSync/index.ts) |
| `references/typescript/src/services/lsp/LSPDiagnosticRegistry.ts` | [`references/typescript/src/services/lsp/LSPDiagnosticRegistry.ts`](references/typescript/src/services/lsp/LSPDiagnosticRegistry.ts) |
| `references/typescript/src/services/lsp/config.ts` | [`references/typescript/src/services/lsp/config.ts`](references/typescript/src/services/lsp/config.ts) |
| `references/typescript/src/services/lsp/LSPServerManager.ts` | [`references/typescript/src/services/lsp/LSPServerManager.ts`](references/typescript/src/services/lsp/LSPServerManager.ts) |
| `references/typescript/src/services/lsp/passiveFeedback.ts` | [`references/typescript/src/services/lsp/passiveFeedback.ts`](references/typescript/src/services/lsp/passiveFeedback.ts) |
| `references/typescript/src/services/lsp/LSPServerInstance.ts` | [`references/typescript/src/services/lsp/LSPServerInstance.ts`](references/typescript/src/services/lsp/LSPServerInstance.ts) |
| `references/typescript/src/services/lsp/manager.ts` | [`references/typescript/src/services/lsp/manager.ts`](references/typescript/src/services/lsp/manager.ts) |
| `references/typescript/src/services/lsp/LSPClient.ts` | [`references/typescript/src/services/lsp/LSPClient.ts`](references/typescript/src/services/lsp/LSPClient.ts) |
| `references/typescript/src/services/internalLogging.ts` | [`references/typescript/src/services/internalLogging.ts`](references/typescript/src/services/internalLogging.ts) |
| `references/typescript/src/services/notifier.ts` | [`references/typescript/src/services/notifier.ts`](references/typescript/src/services/notifier.ts) |
| `references/typescript/src/services/SessionMemory/sessionMemoryUtils.ts` | [`references/typescript/src/services/SessionMemory/sessionMemoryUtils.ts`](references/typescript/src/services/SessionMemory/sessionMemoryUtils.ts) |
| `references/typescript/src/services/SessionMemory/sessionMemory.ts` | [`references/typescript/src/services/SessionMemory/sessionMemory.ts`](references/typescript/src/services/SessionMemory/sessionMemory.ts) |
| `references/typescript/src/services/SessionMemory/prompts.ts` | [`references/typescript/src/services/SessionMemory/prompts.ts`](references/typescript/src/services/SessionMemory/prompts.ts) |
| `references/typescript/src/services/analytics/firstPartyEventLogger.ts` | [`references/typescript/src/services/analytics/firstPartyEventLogger.ts`](references/typescript/src/services/analytics/firstPartyEventLogger.ts) |
| `references/typescript/src/services/analytics/config.ts` | [`references/typescript/src/services/analytics/config.ts`](references/typescript/src/services/analytics/config.ts) |
| `references/typescript/src/services/analytics/firstPartyEventLoggingExporter.ts` | [`references/typescript/src/services/analytics/firstPartyEventLoggingExporter.ts`](references/typescript/src/services/analytics/firstPartyEventLoggingExporter.ts) |
| `references/typescript/src/services/analytics/datadog.ts` | [`references/typescript/src/services/analytics/datadog.ts`](references/typescript/src/services/analytics/datadog.ts) |
| `references/typescript/src/services/analytics/growthbook.ts` | [`references/typescript/src/services/analytics/growthbook.ts`](references/typescript/src/services/analytics/growthbook.ts) |
| `references/typescript/src/services/analytics/metadata.ts` | [`references/typescript/src/services/analytics/metadata.ts`](references/typescript/src/services/analytics/metadata.ts) |
| `references/typescript/src/services/analytics/sink.ts` | [`references/typescript/src/services/analytics/sink.ts`](references/typescript/src/services/analytics/sink.ts) |
| `references/typescript/src/services/analytics/index.ts` | [`references/typescript/src/services/analytics/index.ts`](references/typescript/src/services/analytics/index.ts) |
| `references/typescript/src/services/analytics/sinkKillswitch.ts` | [`references/typescript/src/services/analytics/sinkKillswitch.ts`](references/typescript/src/services/analytics/sinkKillswitch.ts) |
| `references/typescript/src/services/voice.ts` | [`references/typescript/src/services/voice.ts`](references/typescript/src/services/voice.ts) |
| `references/typescript/src/services/claudeAiLimits.ts` | [`references/typescript/src/services/claudeAiLimits.ts`](references/typescript/src/services/claudeAiLimits.ts) |
| `references/typescript/src/services/claudeAiLimitsHook.ts` | [`references/typescript/src/services/claudeAiLimitsHook.ts`](references/typescript/src/services/claudeAiLimitsHook.ts) |
| `references/typescript/src/services/PromptSuggestion/promptSuggestion.ts` | [`references/typescript/src/services/PromptSuggestion/promptSuggestion.ts`](references/typescript/src/services/PromptSuggestion/promptSuggestion.ts) |
| `references/typescript/src/services/PromptSuggestion/speculation.ts` | [`references/typescript/src/services/PromptSuggestion/speculation.ts`](references/typescript/src/services/PromptSuggestion/speculation.ts) |
| `references/typescript/src/state/AppState.tsx` | [`references/typescript/src/state/AppState.tsx`](references/typescript/src/state/AppState.tsx) |
| `references/typescript/src/state/store.ts` | [`references/typescript/src/state/store.ts`](references/typescript/src/state/store.ts) |
| `references/typescript/src/state/onChangeAppState.ts` | [`references/typescript/src/state/onChangeAppState.ts`](references/typescript/src/state/onChangeAppState.ts) |
| `references/typescript/src/state/teammateViewHelpers.ts` | [`references/typescript/src/state/teammateViewHelpers.ts`](references/typescript/src/state/teammateViewHelpers.ts) |
| `references/typescript/src/state/AppStateStore.ts` | [`references/typescript/src/state/AppStateStore.ts`](references/typescript/src/state/AppStateStore.ts) |
| `references/typescript/src/state/selectors.ts` | [`references/typescript/src/state/selectors.ts`](references/typescript/src/state/selectors.ts) |
| `references/typescript/src/plugins/bundled/index.ts` | [`references/typescript/src/plugins/bundled/index.ts`](references/typescript/src/plugins/bundled/index.ts) |
| `references/typescript/src/plugins/builtinPlugins.ts` | [`references/typescript/src/plugins/builtinPlugins.ts`](references/typescript/src/plugins/builtinPlugins.ts) |
| `references/typescript/src/Task.ts` | [`references/typescript/src/Task.ts`](references/typescript/src/Task.ts) |
| `references/typescript/src/tasks/stopTask.ts` | [`references/typescript/src/tasks/stopTask.ts`](references/typescript/src/tasks/stopTask.ts) |
| `references/typescript/src/tasks/LocalMainSessionTask.ts` | [`references/typescript/src/tasks/LocalMainSessionTask.ts`](references/typescript/src/tasks/LocalMainSessionTask.ts) |
| `references/typescript/src/tasks/DreamTask/DreamTask.ts` | [`references/typescript/src/tasks/DreamTask/DreamTask.ts`](references/typescript/src/tasks/DreamTask/DreamTask.ts) |
| `references/typescript/src/tasks/RemoteAgentTask/RemoteAgentTask.tsx` | [`references/typescript/src/tasks/RemoteAgentTask/RemoteAgentTask.tsx`](references/typescript/src/tasks/RemoteAgentTask/RemoteAgentTask.tsx) |
| `references/typescript/src/tasks/InProcessTeammateTask/types.ts` | [`references/typescript/src/tasks/InProcessTeammateTask/types.ts`](references/typescript/src/tasks/InProcessTeammateTask/types.ts) |
| `references/typescript/src/tasks/InProcessTeammateTask/InProcessTeammateTask.tsx` | [`references/typescript/src/tasks/InProcessTeammateTask/InProcessTeammateTask.tsx`](references/typescript/src/tasks/InProcessTeammateTask/InProcessTeammateTask.tsx) |
| `references/typescript/src/tasks/types.ts` | [`references/typescript/src/tasks/types.ts`](references/typescript/src/tasks/types.ts) |
| `references/typescript/src/tasks/pillLabel.ts` | [`references/typescript/src/tasks/pillLabel.ts`](references/typescript/src/tasks/pillLabel.ts) |
| `references/typescript/src/tasks/LocalAgentTask/LocalAgentTask.tsx` | [`references/typescript/src/tasks/LocalAgentTask/LocalAgentTask.tsx`](references/typescript/src/tasks/LocalAgentTask/LocalAgentTask.tsx) |
| `references/typescript/src/tasks/LocalShellTask/LocalShellTask.tsx` | [`references/typescript/src/tasks/LocalShellTask/LocalShellTask.tsx`](references/typescript/src/tasks/LocalShellTask/LocalShellTask.tsx) |
| `references/typescript/src/tasks/LocalShellTask/killShellTasks.ts` | [`references/typescript/src/tasks/LocalShellTask/killShellTasks.ts`](references/typescript/src/tasks/LocalShellTask/killShellTasks.ts) |
| `references/typescript/src/tasks/LocalShellTask/guards.ts` | [`references/typescript/src/tasks/LocalShellTask/guards.ts`](references/typescript/src/tasks/LocalShellTask/guards.ts) |
| `references/typescript/src/memdir/teamMemPaths.ts` | [`references/typescript/src/memdir/teamMemPaths.ts`](references/typescript/src/memdir/teamMemPaths.ts) |
| `references/typescript/src/memdir/paths.ts` | [`references/typescript/src/memdir/paths.ts`](references/typescript/src/memdir/paths.ts) |
| `references/typescript/src/memdir/teamMemPrompts.ts` | [`references/typescript/src/memdir/teamMemPrompts.ts`](references/typescript/src/memdir/teamMemPrompts.ts) |
| `references/typescript/src/memdir/memoryAge.ts` | [`references/typescript/src/memdir/memoryAge.ts`](references/typescript/src/memdir/memoryAge.ts) |
| `references/typescript/src/memdir/memoryScan.ts` | [`references/typescript/src/memdir/memoryScan.ts`](references/typescript/src/memdir/memoryScan.ts) |
| `references/typescript/src/memdir/memoryTypes.ts` | [`references/typescript/src/memdir/memoryTypes.ts`](references/typescript/src/memdir/memoryTypes.ts) |
| `references/typescript/src/memdir/memdir.ts` | [`references/typescript/src/memdir/memdir.ts`](references/typescript/src/memdir/memdir.ts) |
| `references/typescript/src/memdir/findRelevantMemories.ts` | [`references/typescript/src/memdir/findRelevantMemories.ts`](references/typescript/src/memdir/findRelevantMemories.ts) |
| `references/typescript/src/ink.ts` | [`references/typescript/src/ink.ts`](references/typescript/src/ink.ts) |
| `references/typescript/src/buddy/CompanionSprite.tsx` | [`references/typescript/src/buddy/CompanionSprite.tsx`](references/typescript/src/buddy/CompanionSprite.tsx) |
| `references/typescript/src/buddy/prompt.ts` | [`references/typescript/src/buddy/prompt.ts`](references/typescript/src/buddy/prompt.ts) |
| `references/typescript/src/buddy/types.ts` | [`references/typescript/src/buddy/types.ts`](references/typescript/src/buddy/types.ts) |
| `references/typescript/src/buddy/useBuddyNotification.tsx` | [`references/typescript/src/buddy/useBuddyNotification.tsx`](references/typescript/src/buddy/useBuddyNotification.tsx) |
| `references/typescript/src/buddy/sprites.ts` | [`references/typescript/src/buddy/sprites.ts`](references/typescript/src/buddy/sprites.ts) |
| `references/typescript/src/buddy/companion.ts` | [`references/typescript/src/buddy/companion.ts`](references/typescript/src/buddy/companion.ts) |
| `references/typescript/src/outputStyles/loadOutputStylesDir.ts` | [`references/typescript/src/outputStyles/loadOutputStylesDir.ts`](references/typescript/src/outputStyles/loadOutputStylesDir.ts) |
| `references/typescript/src/coordinator/coordinatorMode.ts` | [`references/typescript/src/coordinator/coordinatorMode.ts`](references/typescript/src/coordinator/coordinatorMode.ts) |
| `references/typescript/src/commands/mock-limits/index.js` | [`references/typescript/src/commands/mock-limits/index.js`](references/typescript/src/commands/mock-limits/index.js) |
| `references/typescript/src/commands/chrome/index.ts` | [`references/typescript/src/commands/chrome/index.ts`](references/typescript/src/commands/chrome/index.ts) |
| `references/typescript/src/commands/chrome/chrome.tsx` | [`references/typescript/src/commands/chrome/chrome.tsx`](references/typescript/src/commands/chrome/chrome.tsx) |
| `references/typescript/src/commands/add-dir/validation.ts` | [`references/typescript/src/commands/add-dir/validation.ts`](references/typescript/src/commands/add-dir/validation.ts) |
| `references/typescript/src/commands/add-dir/add-dir.tsx` | [`references/typescript/src/commands/add-dir/add-dir.tsx`](references/typescript/src/commands/add-dir/add-dir.tsx) |
| `references/typescript/src/commands/add-dir/index.ts` | [`references/typescript/src/commands/add-dir/index.ts`](references/typescript/src/commands/add-dir/index.ts) |
| `references/typescript/src/commands/login/index.ts` | [`references/typescript/src/commands/login/index.ts`](references/typescript/src/commands/login/index.ts) |
| `references/typescript/src/commands/login/login.tsx` | [`references/typescript/src/commands/login/login.tsx`](references/typescript/src/commands/login/login.tsx) |
| `references/typescript/src/commands/issue/index.js` | [`references/typescript/src/commands/issue/index.js`](references/typescript/src/commands/issue/index.js) |
| `references/typescript/src/commands/resume/resume.tsx` | [`references/typescript/src/commands/resume/resume.tsx`](references/typescript/src/commands/resume/resume.tsx) |
| `references/typescript/src/commands/resume/index.ts` | [`references/typescript/src/commands/resume/index.ts`](references/typescript/src/commands/resume/index.ts) |
| `references/typescript/src/commands/privacy-settings/privacy-settings.tsx` | [`references/typescript/src/commands/privacy-settings/privacy-settings.tsx`](references/typescript/src/commands/privacy-settings/privacy-settings.tsx) |
| `references/typescript/src/commands/privacy-settings/index.ts` | [`references/typescript/src/commands/privacy-settings/index.ts`](references/typescript/src/commands/privacy-settings/index.ts) |
| `references/typescript/src/commands/help/index.ts` | [`references/typescript/src/commands/help/index.ts`](references/typescript/src/commands/help/index.ts) |
| `references/typescript/src/commands/help/help.tsx` | [`references/typescript/src/commands/help/help.tsx`](references/typescript/src/commands/help/help.tsx) |
| `references/typescript/src/commands/version.ts` | [`references/typescript/src/commands/version.ts`](references/typescript/src/commands/version.ts) |
| `references/typescript/src/commands/memory/index.ts` | [`references/typescript/src/commands/memory/index.ts`](references/typescript/src/commands/memory/index.ts) |
| `references/typescript/src/commands/memory/memory.tsx` | [`references/typescript/src/commands/memory/memory.tsx`](references/typescript/src/commands/memory/memory.tsx) |
| `references/typescript/src/commands/agents/agents.tsx` | [`references/typescript/src/commands/agents/agents.tsx`](references/typescript/src/commands/agents/agents.tsx) |
| `references/typescript/src/commands/agents/index.ts` | [`references/typescript/src/commands/agents/index.ts`](references/typescript/src/commands/agents/index.ts) |
| `references/typescript/src/commands/remote-env/index.ts` | [`references/typescript/src/commands/remote-env/index.ts`](references/typescript/src/commands/remote-env/index.ts) |
| `references/typescript/src/commands/remote-env/remote-env.tsx` | [`references/typescript/src/commands/remote-env/remote-env.tsx`](references/typescript/src/commands/remote-env/remote-env.tsx) |
| `references/typescript/src/commands/theme/theme.tsx` | [`references/typescript/src/commands/theme/theme.tsx`](references/typescript/src/commands/theme/theme.tsx) |
| `references/typescript/src/commands/theme/index.ts` | [`references/typescript/src/commands/theme/index.ts`](references/typescript/src/commands/theme/index.ts) |
| `references/typescript/src/commands/output-style/output-style.tsx` | [`references/typescript/src/commands/output-style/output-style.tsx`](references/typescript/src/commands/output-style/output-style.tsx) |
| `references/typescript/src/commands/output-style/index.ts` | [`references/typescript/src/commands/output-style/index.ts`](references/typescript/src/commands/output-style/index.ts) |
| `references/typescript/src/commands/env/index.js` | [`references/typescript/src/commands/env/index.js`](references/typescript/src/commands/env/index.js) |
| `references/typescript/src/commands/advisor.ts` | [`references/typescript/src/commands/advisor.ts`](references/typescript/src/commands/advisor.ts) |
| `references/typescript/src/commands/sandbox-toggle/sandbox-toggle.tsx` | [`references/typescript/src/commands/sandbox-toggle/sandbox-toggle.tsx`](references/typescript/src/commands/sandbox-toggle/sandbox-toggle.tsx) |
| `references/typescript/src/commands/sandbox-toggle/index.ts` | [`references/typescript/src/commands/sandbox-toggle/index.ts`](references/typescript/src/commands/sandbox-toggle/index.ts) |
| `references/typescript/src/commands/files/files.ts` | [`references/typescript/src/commands/files/files.ts`](references/typescript/src/commands/files/files.ts) |
| `references/typescript/src/commands/files/index.ts` | [`references/typescript/src/commands/files/index.ts`](references/typescript/src/commands/files/index.ts) |
| `references/typescript/src/commands/voice/index.ts` | [`references/typescript/src/commands/voice/index.ts`](references/typescript/src/commands/voice/index.ts) |
| `references/typescript/src/commands/voice/voice.ts` | [`references/typescript/src/commands/voice/voice.ts`](references/typescript/src/commands/voice/voice.ts) |
| `references/typescript/src/commands/stats/stats.tsx` | [`references/typescript/src/commands/stats/stats.tsx`](references/typescript/src/commands/stats/stats.tsx) |
| `references/typescript/src/commands/stats/index.ts` | [`references/typescript/src/commands/stats/index.ts`](references/typescript/src/commands/stats/index.ts) |
| `references/typescript/src/commands/feedback/feedback.tsx` | [`references/typescript/src/commands/feedback/feedback.tsx`](references/typescript/src/commands/feedback/feedback.tsx) |
| `references/typescript/src/commands/feedback/index.ts` | [`references/typescript/src/commands/feedback/index.ts`](references/typescript/src/commands/feedback/index.ts) |
| `references/typescript/src/commands/passes/passes.tsx` | [`references/typescript/src/commands/passes/passes.tsx`](references/typescript/src/commands/passes/passes.tsx) |
| `references/typescript/src/commands/passes/index.ts` | [`references/typescript/src/commands/passes/index.ts`](references/typescript/src/commands/passes/index.ts) |
| `references/typescript/src/commands/reset-limits/index.js` | [`references/typescript/src/commands/reset-limits/index.js`](references/typescript/src/commands/reset-limits/index.js) |
| `references/typescript/src/commands/status/index.ts` | [`references/typescript/src/commands/status/index.ts`](references/typescript/src/commands/status/index.ts) |
| `references/typescript/src/commands/status/status.tsx` | [`references/typescript/src/commands/status/status.tsx`](references/typescript/src/commands/status/status.tsx) |
| `references/typescript/src/commands/break-cache/index.js` | [`references/typescript/src/commands/break-cache/index.js`](references/typescript/src/commands/break-cache/index.js) |
| `references/typescript/src/commands/init.ts` | [`references/typescript/src/commands/init.ts`](references/typescript/src/commands/init.ts) |
| `references/typescript/src/commands/desktop/desktop.tsx` | [`references/typescript/src/commands/desktop/desktop.tsx`](references/typescript/src/commands/desktop/desktop.tsx) |
| `references/typescript/src/commands/desktop/index.ts` | [`references/typescript/src/commands/desktop/index.ts`](references/typescript/src/commands/desktop/index.ts) |
| `references/typescript/src/commands/tag/tag.tsx` | [`references/typescript/src/commands/tag/tag.tsx`](references/typescript/src/commands/tag/tag.tsx) |
| `references/typescript/src/commands/tag/index.ts` | [`references/typescript/src/commands/tag/index.ts`](references/typescript/src/commands/tag/index.ts) |
| `references/typescript/src/commands/context/context.tsx` | [`references/typescript/src/commands/context/context.tsx`](references/typescript/src/commands/context/context.tsx) |
| `references/typescript/src/commands/context/context-noninteractive.ts` | [`references/typescript/src/commands/context/context-noninteractive.ts`](references/typescript/src/commands/context/context-noninteractive.ts) |
| `references/typescript/src/commands/context/index.ts` | [`references/typescript/src/commands/context/index.ts`](references/typescript/src/commands/context/index.ts) |
| `references/typescript/src/commands/ultraplan.tsx` | [`references/typescript/src/commands/ultraplan.tsx`](references/typescript/src/commands/ultraplan.tsx) |
| `references/typescript/src/commands/doctor/index.ts` | [`references/typescript/src/commands/doctor/index.ts`](references/typescript/src/commands/doctor/index.ts) |
| `references/typescript/src/commands/doctor/doctor.tsx` | [`references/typescript/src/commands/doctor/doctor.tsx`](references/typescript/src/commands/doctor/doctor.tsx) |
| `references/typescript/src/commands/bridge-kick.ts` | [`references/typescript/src/commands/bridge-kick.ts`](references/typescript/src/commands/bridge-kick.ts) |
| `references/typescript/src/commands/extra-usage/extra-usage.tsx` | [`references/typescript/src/commands/extra-usage/extra-usage.tsx`](references/typescript/src/commands/extra-usage/extra-usage.tsx) |
| `references/typescript/src/commands/extra-usage/extra-usage-noninteractive.ts` | [`references/typescript/src/commands/extra-usage/extra-usage-noninteractive.ts`](references/typescript/src/commands/extra-usage/extra-usage-noninteractive.ts) |
| `references/typescript/src/commands/extra-usage/index.ts` | [`references/typescript/src/commands/extra-usage/index.ts`](references/typescript/src/commands/extra-usage/index.ts) |
| `references/typescript/src/commands/extra-usage/extra-usage-core.ts` | [`references/typescript/src/commands/extra-usage/extra-usage-core.ts`](references/typescript/src/commands/extra-usage/extra-usage-core.ts) |
| `references/typescript/src/commands/reload-plugins/reload-plugins.ts` | [`references/typescript/src/commands/reload-plugins/reload-plugins.ts`](references/typescript/src/commands/reload-plugins/reload-plugins.ts) |
| `references/typescript/src/commands/reload-plugins/index.ts` | [`references/typescript/src/commands/reload-plugins/index.ts`](references/typescript/src/commands/reload-plugins/index.ts) |
| `references/typescript/src/commands/compact/compact.ts` | [`references/typescript/src/commands/compact/compact.ts`](references/typescript/src/commands/compact/compact.ts) |
| `references/typescript/src/commands/compact/index.ts` | [`references/typescript/src/commands/compact/index.ts`](references/typescript/src/commands/compact/index.ts) |
| `references/typescript/src/commands/bridge/bridge.tsx` | [`references/typescript/src/commands/bridge/bridge.tsx`](references/typescript/src/commands/bridge/bridge.tsx) |
| `references/typescript/src/commands/bridge/index.ts` | [`references/typescript/src/commands/bridge/index.ts`](references/typescript/src/commands/bridge/index.ts) |
| `references/typescript/src/commands/mcp/xaaIdpCommand.ts` | [`references/typescript/src/commands/mcp/xaaIdpCommand.ts`](references/typescript/src/commands/mcp/xaaIdpCommand.ts) |
| `references/typescript/src/commands/mcp/mcp.tsx` | [`references/typescript/src/commands/mcp/mcp.tsx`](references/typescript/src/commands/mcp/mcp.tsx) |
| `references/typescript/src/commands/mcp/index.ts` | [`references/typescript/src/commands/mcp/index.ts`](references/typescript/src/commands/mcp/index.ts) |
| `references/typescript/src/commands/mcp/addCommand.ts` | [`references/typescript/src/commands/mcp/addCommand.ts`](references/typescript/src/commands/mcp/addCommand.ts) |
| `references/typescript/src/commands/ctx_viz/index.js` | [`references/typescript/src/commands/ctx_viz/index.js`](references/typescript/src/commands/ctx_viz/index.js) |
| `references/typescript/src/commands/review.ts` | [`references/typescript/src/commands/review.ts`](references/typescript/src/commands/review.ts) |
| `references/typescript/src/commands/oauth-refresh/index.js` | [`references/typescript/src/commands/oauth-refresh/index.js`](references/typescript/src/commands/oauth-refresh/index.js) |
| `references/typescript/src/commands/btw/btw.tsx` | [`references/typescript/src/commands/btw/btw.tsx`](references/typescript/src/commands/btw/btw.tsx) |
| `references/typescript/src/commands/btw/index.ts` | [`references/typescript/src/commands/btw/index.ts`](references/typescript/src/commands/btw/index.ts) |
| `references/typescript/src/commands/vim/vim.ts` | [`references/typescript/src/commands/vim/vim.ts`](references/typescript/src/commands/vim/vim.ts) |
| `references/typescript/src/commands/vim/index.ts` | [`references/typescript/src/commands/vim/index.ts`](references/typescript/src/commands/vim/index.ts) |
| `references/typescript/src/commands/perf-issue/index.js` | [`references/typescript/src/commands/perf-issue/index.js`](references/typescript/src/commands/perf-issue/index.js) |
| `references/typescript/src/commands/security-review.ts` | [`references/typescript/src/commands/security-review.ts`](references/typescript/src/commands/security-review.ts) |
| `references/typescript/src/commands/rewind/index.ts` | [`references/typescript/src/commands/rewind/index.ts`](references/typescript/src/commands/rewind/index.ts) |
| `references/typescript/src/commands/rewind/rewind.ts` | [`references/typescript/src/commands/rewind/rewind.ts`](references/typescript/src/commands/rewind/rewind.ts) |
| `references/typescript/src/commands/good-claude/index.js` | [`references/typescript/src/commands/good-claude/index.js`](references/typescript/src/commands/good-claude/index.js) |
| `references/typescript/src/commands/autofix-pr/index.js` | [`references/typescript/src/commands/autofix-pr/index.js`](references/typescript/src/commands/autofix-pr/index.js) |
| `references/typescript/src/commands/heapdump/heapdump.ts` | [`references/typescript/src/commands/heapdump/heapdump.ts`](references/typescript/src/commands/heapdump/heapdump.ts) |
| `references/typescript/src/commands/heapdump/index.ts` | [`references/typescript/src/commands/heapdump/index.ts`](references/typescript/src/commands/heapdump/index.ts) |
| `references/typescript/src/commands/fast/fast.tsx` | [`references/typescript/src/commands/fast/fast.tsx`](references/typescript/src/commands/fast/fast.tsx) |
| `references/typescript/src/commands/fast/index.ts` | [`references/typescript/src/commands/fast/index.ts`](references/typescript/src/commands/fast/index.ts) |
| `references/typescript/src/commands/tasks/index.ts` | [`references/typescript/src/commands/tasks/index.ts`](references/typescript/src/commands/tasks/index.ts) |
| `references/typescript/src/commands/tasks/tasks.tsx` | [`references/typescript/src/commands/tasks/tasks.tsx`](references/typescript/src/commands/tasks/tasks.tsx) |
| `references/typescript/src/commands/backfill-sessions/index.js` | [`references/typescript/src/commands/backfill-sessions/index.js`](references/typescript/src/commands/backfill-sessions/index.js) |
| `references/typescript/src/commands/effort/effort.tsx` | [`references/typescript/src/commands/effort/effort.tsx`](references/typescript/src/commands/effort/effort.tsx) |
| `references/typescript/src/commands/effort/index.ts` | [`references/typescript/src/commands/effort/index.ts`](references/typescript/src/commands/effort/index.ts) |
| `references/typescript/src/commands/share/index.js` | [`references/typescript/src/commands/share/index.js`](references/typescript/src/commands/share/index.js) |
| `references/typescript/src/commands/bughunter/index.js` | [`references/typescript/src/commands/bughunter/index.js`](references/typescript/src/commands/bughunter/index.js) |
| `references/typescript/src/commands/stickers/index.ts` | [`references/typescript/src/commands/stickers/index.ts`](references/typescript/src/commands/stickers/index.ts) |
| `references/typescript/src/commands/stickers/stickers.ts` | [`references/typescript/src/commands/stickers/stickers.ts`](references/typescript/src/commands/stickers/stickers.ts) |
| `references/typescript/src/commands/color/color.ts` | [`references/typescript/src/commands/color/color.ts`](references/typescript/src/commands/color/color.ts) |
| `references/typescript/src/commands/color/index.ts` | [`references/typescript/src/commands/color/index.ts`](references/typescript/src/commands/color/index.ts) |
| `references/typescript/src/commands/keybindings/keybindings.ts` | [`references/typescript/src/commands/keybindings/keybindings.ts`](references/typescript/src/commands/keybindings/keybindings.ts) |
| `references/typescript/src/commands/keybindings/index.ts` | [`references/typescript/src/commands/keybindings/index.ts`](references/typescript/src/commands/keybindings/index.ts) |
| `references/typescript/src/commands/copy/index.ts` | [`references/typescript/src/commands/copy/index.ts`](references/typescript/src/commands/copy/index.ts) |
| `references/typescript/src/commands/copy/copy.tsx` | [`references/typescript/src/commands/copy/copy.tsx`](references/typescript/src/commands/copy/copy.tsx) |
| `references/typescript/src/commands/model/model.tsx` | [`references/typescript/src/commands/model/model.tsx`](references/typescript/src/commands/model/model.tsx) |
| `references/typescript/src/commands/model/index.ts` | [`references/typescript/src/commands/model/index.ts`](references/typescript/src/commands/model/index.ts) |
| `references/typescript/src/commands/onboarding/index.js` | [`references/typescript/src/commands/onboarding/index.js`](references/typescript/src/commands/onboarding/index.js) |
| `references/typescript/src/commands/commit.ts` | [`references/typescript/src/commands/commit.ts`](references/typescript/src/commands/commit.ts) |
| `references/typescript/src/commands/release-notes/release-notes.ts` | [`references/typescript/src/commands/release-notes/release-notes.ts`](references/typescript/src/commands/release-notes/release-notes.ts) |
| `references/typescript/src/commands/release-notes/index.ts` | [`references/typescript/src/commands/release-notes/index.ts`](references/typescript/src/commands/release-notes/index.ts) |
| `references/typescript/src/commands/diff/diff.tsx` | [`references/typescript/src/commands/diff/diff.tsx`](references/typescript/src/commands/diff/diff.tsx) |
| `references/typescript/src/commands/diff/index.ts` | [`references/typescript/src/commands/diff/index.ts`](references/typescript/src/commands/diff/index.ts) |
| `references/typescript/src/commands/terminalSetup/index.ts` | [`references/typescript/src/commands/terminalSetup/index.ts`](references/typescript/src/commands/terminalSetup/index.ts) |
| `references/typescript/src/commands/terminalSetup/terminalSetup.tsx` | [`references/typescript/src/commands/terminalSetup/terminalSetup.tsx`](references/typescript/src/commands/terminalSetup/terminalSetup.tsx) |
| `references/typescript/src/commands/clear/conversation.ts` | [`references/typescript/src/commands/clear/conversation.ts`](references/typescript/src/commands/clear/conversation.ts) |
| `references/typescript/src/commands/clear/caches.ts` | [`references/typescript/src/commands/clear/caches.ts`](references/typescript/src/commands/clear/caches.ts) |
| `references/typescript/src/commands/clear/clear.ts` | [`references/typescript/src/commands/clear/clear.ts`](references/typescript/src/commands/clear/clear.ts) |
| `references/typescript/src/commands/clear/index.ts` | [`references/typescript/src/commands/clear/index.ts`](references/typescript/src/commands/clear/index.ts) |
| `references/typescript/src/commands/statusline.tsx` | [`references/typescript/src/commands/statusline.tsx`](references/typescript/src/commands/statusline.tsx) |
| `references/typescript/src/commands/brief.ts` | [`references/typescript/src/commands/brief.ts`](references/typescript/src/commands/brief.ts) |
| `references/typescript/src/commands/plan/plan.tsx` | [`references/typescript/src/commands/plan/plan.tsx`](references/typescript/src/commands/plan/plan.tsx) |
| `references/typescript/src/commands/plan/index.ts` | [`references/typescript/src/commands/plan/index.ts`](references/typescript/src/commands/plan/index.ts) |
| `references/typescript/src/commands/logout/logout.tsx` | [`references/typescript/src/commands/logout/logout.tsx`](references/typescript/src/commands/logout/logout.tsx) |
| `references/typescript/src/commands/logout/index.ts` | [`references/typescript/src/commands/logout/index.ts`](references/typescript/src/commands/logout/index.ts) |
| `references/typescript/src/commands/install-github-app/ErrorStep.tsx` | [`references/typescript/src/commands/install-github-app/ErrorStep.tsx`](references/typescript/src/commands/install-github-app/ErrorStep.tsx) |
| `references/typescript/src/commands/install-github-app/SuccessStep.tsx` | [`references/typescript/src/commands/install-github-app/SuccessStep.tsx`](references/typescript/src/commands/install-github-app/SuccessStep.tsx) |
| `references/typescript/src/commands/install-github-app/CreatingStep.tsx` | [`references/typescript/src/commands/install-github-app/CreatingStep.tsx`](references/typescript/src/commands/install-github-app/CreatingStep.tsx) |
| `references/typescript/src/commands/install-github-app/WarningsStep.tsx` | [`references/typescript/src/commands/install-github-app/WarningsStep.tsx`](references/typescript/src/commands/install-github-app/WarningsStep.tsx) |
| `references/typescript/src/commands/install-github-app/OAuthFlowStep.tsx` | [`references/typescript/src/commands/install-github-app/OAuthFlowStep.tsx`](references/typescript/src/commands/install-github-app/OAuthFlowStep.tsx) |
| `references/typescript/src/commands/install-github-app/InstallAppStep.tsx` | [`references/typescript/src/commands/install-github-app/InstallAppStep.tsx`](references/typescript/src/commands/install-github-app/InstallAppStep.tsx) |
| `references/typescript/src/commands/install-github-app/ExistingWorkflowStep.tsx` | [`references/typescript/src/commands/install-github-app/ExistingWorkflowStep.tsx`](references/typescript/src/commands/install-github-app/ExistingWorkflowStep.tsx) |
| `references/typescript/src/commands/install-github-app/install-github-app.tsx` | [`references/typescript/src/commands/install-github-app/install-github-app.tsx`](references/typescript/src/commands/install-github-app/install-github-app.tsx) |
| `references/typescript/src/commands/install-github-app/setupGitHubActions.ts` | [`references/typescript/src/commands/install-github-app/setupGitHubActions.ts`](references/typescript/src/commands/install-github-app/setupGitHubActions.ts) |
| `references/typescript/src/commands/install-github-app/index.ts` | [`references/typescript/src/commands/install-github-app/index.ts`](references/typescript/src/commands/install-github-app/index.ts) |
| `references/typescript/src/commands/install-github-app/ApiKeyStep.tsx` | [`references/typescript/src/commands/install-github-app/ApiKeyStep.tsx`](references/typescript/src/commands/install-github-app/ApiKeyStep.tsx) |
| `references/typescript/src/commands/install-github-app/CheckGitHubStep.tsx` | [`references/typescript/src/commands/install-github-app/CheckGitHubStep.tsx`](references/typescript/src/commands/install-github-app/CheckGitHubStep.tsx) |
| `references/typescript/src/commands/install-github-app/ChooseRepoStep.tsx` | [`references/typescript/src/commands/install-github-app/ChooseRepoStep.tsx`](references/typescript/src/commands/install-github-app/ChooseRepoStep.tsx) |
| `references/typescript/src/commands/install-github-app/CheckExistingSecretStep.tsx` | [`references/typescript/src/commands/install-github-app/CheckExistingSecretStep.tsx`](references/typescript/src/commands/install-github-app/CheckExistingSecretStep.tsx) |
| `references/typescript/src/commands/commit-push-pr.ts` | [`references/typescript/src/commands/commit-push-pr.ts`](references/typescript/src/commands/commit-push-pr.ts) |
| `references/typescript/src/commands/exit/exit.tsx` | [`references/typescript/src/commands/exit/exit.tsx`](references/typescript/src/commands/exit/exit.tsx) |
| `references/typescript/src/commands/exit/index.ts` | [`references/typescript/src/commands/exit/index.ts`](references/typescript/src/commands/exit/index.ts) |
| `references/typescript/src/commands/hooks/hooks.tsx` | [`references/typescript/src/commands/hooks/hooks.tsx`](references/typescript/src/commands/hooks/hooks.tsx) |
| `references/typescript/src/commands/hooks/index.ts` | [`references/typescript/src/commands/hooks/index.ts`](references/typescript/src/commands/hooks/index.ts) |
| `references/typescript/src/commands/ant-trace/index.js` | [`references/typescript/src/commands/ant-trace/index.js`](references/typescript/src/commands/ant-trace/index.js) |
| `references/typescript/src/commands/export/index.ts` | [`references/typescript/src/commands/export/index.ts`](references/typescript/src/commands/export/index.ts) |
| `references/typescript/src/commands/export/export.tsx` | [`references/typescript/src/commands/export/export.tsx`](references/typescript/src/commands/export/export.tsx) |
| `references/typescript/src/commands/install.tsx` | [`references/typescript/src/commands/install.tsx`](references/typescript/src/commands/install.tsx) |
| `references/typescript/src/commands/config/config.tsx` | [`references/typescript/src/commands/config/config.tsx`](references/typescript/src/commands/config/config.tsx) |
| `references/typescript/src/commands/config/index.ts` | [`references/typescript/src/commands/config/index.ts`](references/typescript/src/commands/config/index.ts) |
| `references/typescript/src/commands/cost/cost.ts` | [`references/typescript/src/commands/cost/cost.ts`](references/typescript/src/commands/cost/cost.ts) |
| `references/typescript/src/commands/cost/index.ts` | [`references/typescript/src/commands/cost/index.ts`](references/typescript/src/commands/cost/index.ts) |
| `references/typescript/src/commands/thinkback/thinkback.tsx` | [`references/typescript/src/commands/thinkback/thinkback.tsx`](references/typescript/src/commands/thinkback/thinkback.tsx) |
| `references/typescript/src/commands/thinkback/index.ts` | [`references/typescript/src/commands/thinkback/index.ts`](references/typescript/src/commands/thinkback/index.ts) |
| `references/typescript/src/commands/plugin/PluginOptionsFlow.tsx` | [`references/typescript/src/commands/plugin/PluginOptionsFlow.tsx`](references/typescript/src/commands/plugin/PluginOptionsFlow.tsx) |
| `references/typescript/src/commands/plugin/PluginErrors.tsx` | [`references/typescript/src/commands/plugin/PluginErrors.tsx`](references/typescript/src/commands/plugin/PluginErrors.tsx) |
| `references/typescript/src/commands/plugin/BrowseMarketplace.tsx` | [`references/typescript/src/commands/plugin/BrowseMarketplace.tsx`](references/typescript/src/commands/plugin/BrowseMarketplace.tsx) |
| `references/typescript/src/commands/plugin/PluginTrustWarning.tsx` | [`references/typescript/src/commands/plugin/PluginTrustWarning.tsx`](references/typescript/src/commands/plugin/PluginTrustWarning.tsx) |
| `references/typescript/src/commands/plugin/pluginDetailsHelpers.tsx` | [`references/typescript/src/commands/plugin/pluginDetailsHelpers.tsx`](references/typescript/src/commands/plugin/pluginDetailsHelpers.tsx) |
| `references/typescript/src/commands/plugin/ManagePlugins.tsx` | [`references/typescript/src/commands/plugin/ManagePlugins.tsx`](references/typescript/src/commands/plugin/ManagePlugins.tsx) |
| `references/typescript/src/commands/plugin/AddMarketplace.tsx` | [`references/typescript/src/commands/plugin/AddMarketplace.tsx`](references/typescript/src/commands/plugin/AddMarketplace.tsx) |
| `references/typescript/src/commands/plugin/UnifiedInstalledCell.tsx` | [`references/typescript/src/commands/plugin/UnifiedInstalledCell.tsx`](references/typescript/src/commands/plugin/UnifiedInstalledCell.tsx) |
| `references/typescript/src/commands/plugin/PluginOptionsDialog.tsx` | [`references/typescript/src/commands/plugin/PluginOptionsDialog.tsx`](references/typescript/src/commands/plugin/PluginOptionsDialog.tsx) |
| `references/typescript/src/commands/plugin/usePagination.ts` | [`references/typescript/src/commands/plugin/usePagination.ts`](references/typescript/src/commands/plugin/usePagination.ts) |
| `references/typescript/src/commands/plugin/ManageMarketplaces.tsx` | [`references/typescript/src/commands/plugin/ManageMarketplaces.tsx`](references/typescript/src/commands/plugin/ManageMarketplaces.tsx) |
| `references/typescript/src/commands/plugin/PluginSettings.tsx` | [`references/typescript/src/commands/plugin/PluginSettings.tsx`](references/typescript/src/commands/plugin/PluginSettings.tsx) |
| `references/typescript/src/commands/plugin/index.tsx` | [`references/typescript/src/commands/plugin/index.tsx`](references/typescript/src/commands/plugin/index.tsx) |
| `references/typescript/src/commands/plugin/parseArgs.ts` | [`references/typescript/src/commands/plugin/parseArgs.ts`](references/typescript/src/commands/plugin/parseArgs.ts) |
| `references/typescript/src/commands/plugin/DiscoverPlugins.tsx` | [`references/typescript/src/commands/plugin/DiscoverPlugins.tsx`](references/typescript/src/commands/plugin/DiscoverPlugins.tsx) |
| `references/typescript/src/commands/plugin/plugin.tsx` | [`references/typescript/src/commands/plugin/plugin.tsx`](references/typescript/src/commands/plugin/plugin.tsx) |
| `references/typescript/src/commands/plugin/ValidatePlugin.tsx` | [`references/typescript/src/commands/plugin/ValidatePlugin.tsx`](references/typescript/src/commands/plugin/ValidatePlugin.tsx) |
| `references/typescript/src/commands/thinkback-play/thinkback-play.ts` | [`references/typescript/src/commands/thinkback-play/thinkback-play.ts`](references/typescript/src/commands/thinkback-play/thinkback-play.ts) |
| `references/typescript/src/commands/thinkback-play/index.ts` | [`references/typescript/src/commands/thinkback-play/index.ts`](references/typescript/src/commands/thinkback-play/index.ts) |
| `references/typescript/src/commands/assistant/assistant.tsx` | [`references/typescript/src/commands/assistant/assistant.tsx`](references/typescript/src/commands/assistant/assistant.tsx) |
| `references/typescript/src/commands/rename/rename.ts` | [`references/typescript/src/commands/rename/rename.ts`](references/typescript/src/commands/rename/rename.ts) |
| `references/typescript/src/commands/rename/generateSessionName.ts` | [`references/typescript/src/commands/rename/generateSessionName.ts`](references/typescript/src/commands/rename/generateSessionName.ts) |
| `references/typescript/src/commands/rename/index.ts` | [`references/typescript/src/commands/rename/index.ts`](references/typescript/src/commands/rename/index.ts) |
| `references/typescript/src/commands/usage/usage.tsx` | [`references/typescript/src/commands/usage/usage.tsx`](references/typescript/src/commands/usage/usage.tsx) |
| `references/typescript/src/commands/usage/index.ts` | [`references/typescript/src/commands/usage/index.ts`](references/typescript/src/commands/usage/index.ts) |
| `references/typescript/src/commands/pr_comments/index.ts` | [`references/typescript/src/commands/pr_comments/index.ts`](references/typescript/src/commands/pr_comments/index.ts) |
| `references/typescript/src/commands/insights.ts` | [`references/typescript/src/commands/insights.ts`](references/typescript/src/commands/insights.ts) |
| `references/typescript/src/commands/branch/branch.ts` | [`references/typescript/src/commands/branch/branch.ts`](references/typescript/src/commands/branch/branch.ts) |
| `references/typescript/src/commands/branch/index.ts` | [`references/typescript/src/commands/branch/index.ts`](references/typescript/src/commands/branch/index.ts) |
| `references/typescript/src/commands/review/ultrareviewEnabled.ts` | [`references/typescript/src/commands/review/ultrareviewEnabled.ts`](references/typescript/src/commands/review/ultrareviewEnabled.ts) |
| `references/typescript/src/commands/review/ultrareviewCommand.tsx` | [`references/typescript/src/commands/review/ultrareviewCommand.tsx`](references/typescript/src/commands/review/ultrareviewCommand.tsx) |
| `references/typescript/src/commands/review/UltrareviewOverageDialog.tsx` | [`references/typescript/src/commands/review/UltrareviewOverageDialog.tsx`](references/typescript/src/commands/review/UltrareviewOverageDialog.tsx) |
| `references/typescript/src/commands/review/reviewRemote.ts` | [`references/typescript/src/commands/review/reviewRemote.ts`](references/typescript/src/commands/review/reviewRemote.ts) |
| `references/typescript/src/commands/ide/ide.tsx` | [`references/typescript/src/commands/ide/ide.tsx`](references/typescript/src/commands/ide/ide.tsx) |
| `references/typescript/src/commands/ide/index.ts` | [`references/typescript/src/commands/ide/index.ts`](references/typescript/src/commands/ide/index.ts) |
| `references/typescript/src/commands/teleport/index.js` | [`references/typescript/src/commands/teleport/index.js`](references/typescript/src/commands/teleport/index.js) |
| `references/typescript/src/commands/skills/skills.tsx` | [`references/typescript/src/commands/skills/skills.tsx`](references/typescript/src/commands/skills/skills.tsx) |
| `references/typescript/src/commands/skills/index.ts` | [`references/typescript/src/commands/skills/index.ts`](references/typescript/src/commands/skills/index.ts) |
| `references/typescript/src/commands/summary/index.js` | [`references/typescript/src/commands/summary/index.js`](references/typescript/src/commands/summary/index.js) |
| `references/typescript/src/commands/debug-tool-call/index.js` | [`references/typescript/src/commands/debug-tool-call/index.js`](references/typescript/src/commands/debug-tool-call/index.js) |
| `references/typescript/src/commands/remote-setup/remote-setup.tsx` | [`references/typescript/src/commands/remote-setup/remote-setup.tsx`](references/typescript/src/commands/remote-setup/remote-setup.tsx) |
| `references/typescript/src/commands/remote-setup/index.ts` | [`references/typescript/src/commands/remote-setup/index.ts`](references/typescript/src/commands/remote-setup/index.ts) |
| `references/typescript/src/commands/remote-setup/api.ts` | [`references/typescript/src/commands/remote-setup/api.ts`](references/typescript/src/commands/remote-setup/api.ts) |
| `references/typescript/src/commands/rate-limit-options/rate-limit-options.tsx` | [`references/typescript/src/commands/rate-limit-options/rate-limit-options.tsx`](references/typescript/src/commands/rate-limit-options/rate-limit-options.tsx) |
| `references/typescript/src/commands/rate-limit-options/index.ts` | [`references/typescript/src/commands/rate-limit-options/index.ts`](references/typescript/src/commands/rate-limit-options/index.ts) |
| `references/typescript/src/commands/install-slack-app/install-slack-app.ts` | [`references/typescript/src/commands/install-slack-app/install-slack-app.ts`](references/typescript/src/commands/install-slack-app/install-slack-app.ts) |
| `references/typescript/src/commands/install-slack-app/index.ts` | [`references/typescript/src/commands/install-slack-app/index.ts`](references/typescript/src/commands/install-slack-app/index.ts) |
| `references/typescript/src/commands/init-verifiers.ts` | [`references/typescript/src/commands/init-verifiers.ts`](references/typescript/src/commands/init-verifiers.ts) |
| `references/typescript/src/commands/createMovedToPluginCommand.ts` | [`references/typescript/src/commands/createMovedToPluginCommand.ts`](references/typescript/src/commands/createMovedToPluginCommand.ts) |
| `references/typescript/src/commands/upgrade/upgrade.tsx` | [`references/typescript/src/commands/upgrade/upgrade.tsx`](references/typescript/src/commands/upgrade/upgrade.tsx) |
| `references/typescript/src/commands/upgrade/index.ts` | [`references/typescript/src/commands/upgrade/index.ts`](references/typescript/src/commands/upgrade/index.ts) |
| `references/typescript/src/commands/mobile/mobile.tsx` | [`references/typescript/src/commands/mobile/mobile.tsx`](references/typescript/src/commands/mobile/mobile.tsx) |
| `references/typescript/src/commands/mobile/index.ts` | [`references/typescript/src/commands/mobile/index.ts`](references/typescript/src/commands/mobile/index.ts) |
| `references/typescript/src/commands/session/session.tsx` | [`references/typescript/src/commands/session/session.tsx`](references/typescript/src/commands/session/session.tsx) |
| `references/typescript/src/commands/session/index.ts` | [`references/typescript/src/commands/session/index.ts`](references/typescript/src/commands/session/index.ts) |
| `references/typescript/src/commands/permissions/index.ts` | [`references/typescript/src/commands/permissions/index.ts`](references/typescript/src/commands/permissions/index.ts) |
| `references/typescript/src/commands/permissions/permissions.tsx` | [`references/typescript/src/commands/permissions/permissions.tsx`](references/typescript/src/commands/permissions/permissions.tsx) |
| `references/typescript/src/keybindings/match.ts` | [`references/typescript/src/keybindings/match.ts`](references/typescript/src/keybindings/match.ts) |
| `references/typescript/src/keybindings/loadUserBindings.ts` | [`references/typescript/src/keybindings/loadUserBindings.ts`](references/typescript/src/keybindings/loadUserBindings.ts) |
| `references/typescript/src/keybindings/shortcutFormat.ts` | [`references/typescript/src/keybindings/shortcutFormat.ts`](references/typescript/src/keybindings/shortcutFormat.ts) |
| `references/typescript/src/keybindings/defaultBindings.ts` | [`references/typescript/src/keybindings/defaultBindings.ts`](references/typescript/src/keybindings/defaultBindings.ts) |
| `references/typescript/src/keybindings/reservedShortcuts.ts` | [`references/typescript/src/keybindings/reservedShortcuts.ts`](references/typescript/src/keybindings/reservedShortcuts.ts) |
| `references/typescript/src/keybindings/useKeybinding.ts` | [`references/typescript/src/keybindings/useKeybinding.ts`](references/typescript/src/keybindings/useKeybinding.ts) |
| `references/typescript/src/keybindings/KeybindingContext.tsx` | [`references/typescript/src/keybindings/KeybindingContext.tsx`](references/typescript/src/keybindings/KeybindingContext.tsx) |
| `references/typescript/src/keybindings/validate.ts` | [`references/typescript/src/keybindings/validate.ts`](references/typescript/src/keybindings/validate.ts) |
| `references/typescript/src/keybindings/KeybindingProviderSetup.tsx` | [`references/typescript/src/keybindings/KeybindingProviderSetup.tsx`](references/typescript/src/keybindings/KeybindingProviderSetup.tsx) |
| `references/typescript/src/keybindings/useShortcutDisplay.ts` | [`references/typescript/src/keybindings/useShortcutDisplay.ts`](references/typescript/src/keybindings/useShortcutDisplay.ts) |
| `references/typescript/src/keybindings/schema.ts` | [`references/typescript/src/keybindings/schema.ts`](references/typescript/src/keybindings/schema.ts) |
| `references/typescript/src/keybindings/template.ts` | [`references/typescript/src/keybindings/template.ts`](references/typescript/src/keybindings/template.ts) |
| `references/typescript/src/keybindings/parser.ts` | [`references/typescript/src/keybindings/parser.ts`](references/typescript/src/keybindings/parser.ts) |
| `references/typescript/src/keybindings/resolver.ts` | [`references/typescript/src/keybindings/resolver.ts`](references/typescript/src/keybindings/resolver.ts) |
| `references/typescript/src/upstreamproxy/upstreamproxy.ts` | [`references/typescript/src/upstreamproxy/upstreamproxy.ts`](references/typescript/src/upstreamproxy/upstreamproxy.ts) |
| `references/typescript/src/upstreamproxy/relay.ts` | [`references/typescript/src/upstreamproxy/relay.ts`](references/typescript/src/upstreamproxy/relay.ts) |
| `references/typescript/src/remote/remotePermissionBridge.ts` | [`references/typescript/src/remote/remotePermissionBridge.ts`](references/typescript/src/remote/remotePermissionBridge.ts) |
| `references/typescript/src/remote/RemoteSessionManager.ts` | [`references/typescript/src/remote/RemoteSessionManager.ts`](references/typescript/src/remote/RemoteSessionManager.ts) |
| `references/typescript/src/remote/sdkMessageAdapter.ts` | [`references/typescript/src/remote/sdkMessageAdapter.ts`](references/typescript/src/remote/sdkMessageAdapter.ts) |
| `references/typescript/src/remote/SessionsWebSocket.ts` | [`references/typescript/src/remote/SessionsWebSocket.ts`](references/typescript/src/remote/SessionsWebSocket.ts) |
| `references/typescript/src/main.tsx` | [`references/typescript/src/main.tsx`](references/typescript/src/main.tsx) |
| `references/typescript/src/native-ts/file-index/index.ts` | [`references/typescript/src/native-ts/file-index/index.ts`](references/typescript/src/native-ts/file-index/index.ts) |
| `references/typescript/src/native-ts/yoga-layout/enums.ts` | [`references/typescript/src/native-ts/yoga-layout/enums.ts`](references/typescript/src/native-ts/yoga-layout/enums.ts) |
| `references/typescript/src/native-ts/yoga-layout/index.ts` | [`references/typescript/src/native-ts/yoga-layout/index.ts`](references/typescript/src/native-ts/yoga-layout/index.ts) |
| `references/typescript/src/native-ts/color-diff/index.ts` | [`references/typescript/src/native-ts/color-diff/index.ts`](references/typescript/src/native-ts/color-diff/index.ts) |
| `references/typescript/src/QueryEngine.ts` | [`references/typescript/src/QueryEngine.ts`](references/typescript/src/QueryEngine.ts) |
| `references/typescript/src/types/permissions.ts` | [`references/typescript/src/types/permissions.ts`](references/typescript/src/types/permissions.ts) |
| `references/typescript/src/types/generated/events_mono/growthbook/v1/growthbook_experiment_event.ts` | [`references/typescript/src/types/generated/events_mono/growthbook/v1/growthbook_experiment_event.ts`](references/typescript/src/types/generated/events_mono/growthbook/v1/growthbook_experiment_event.ts) |
| `references/typescript/src/types/generated/events_mono/common/v1/auth.ts` | [`references/typescript/src/types/generated/events_mono/common/v1/auth.ts`](references/typescript/src/types/generated/events_mono/common/v1/auth.ts) |
| `references/typescript/src/types/generated/events_mono/claude_code/v1/claude_code_internal_event.ts` | [`references/typescript/src/types/generated/events_mono/claude_code/v1/claude_code_internal_event.ts`](references/typescript/src/types/generated/events_mono/claude_code/v1/claude_code_internal_event.ts) |
| `references/typescript/src/types/generated/google/protobuf/timestamp.ts` | [`references/typescript/src/types/generated/google/protobuf/timestamp.ts`](references/typescript/src/types/generated/google/protobuf/timestamp.ts) |
| `references/typescript/src/types/textInputTypes.ts` | [`references/typescript/src/types/textInputTypes.ts`](references/typescript/src/types/textInputTypes.ts) |
| `references/typescript/src/types/connectorText.ts` | [`references/typescript/src/types/connectorText.ts`](references/typescript/src/types/connectorText.ts) |
| `references/typescript/src/types/hooks.ts` | [`references/typescript/src/types/hooks.ts`](references/typescript/src/types/hooks.ts) |
| `references/typescript/src/types/logs.ts` | [`references/typescript/src/types/logs.ts`](references/typescript/src/types/logs.ts) |
| `references/typescript/src/types/command.ts` | [`references/typescript/src/types/command.ts`](references/typescript/src/types/command.ts) |
| `references/typescript/src/types/plugin.ts` | [`references/typescript/src/types/plugin.ts`](references/typescript/src/types/plugin.ts) |
| `references/typescript/src/types/ids.ts` | [`references/typescript/src/types/ids.ts`](references/typescript/src/types/ids.ts) |
| `references/typescript/src/hooks/useReplBridge.tsx` | [`references/typescript/src/hooks/useReplBridge.tsx`](references/typescript/src/hooks/useReplBridge.tsx) |
| `references/typescript/src/hooks/useIdeLogging.ts` | [`references/typescript/src/hooks/useIdeLogging.ts`](references/typescript/src/hooks/useIdeLogging.ts) |
| `references/typescript/src/hooks/useExitOnCtrlCDWithKeybindings.ts` | [`references/typescript/src/hooks/useExitOnCtrlCDWithKeybindings.ts`](references/typescript/src/hooks/useExitOnCtrlCDWithKeybindings.ts) |
| `references/typescript/src/hooks/useSettingsChange.ts` | [`references/typescript/src/hooks/useSettingsChange.ts`](references/typescript/src/hooks/useSettingsChange.ts) |
| `references/typescript/src/hooks/useSkillImprovementSurvey.ts` | [`references/typescript/src/hooks/useSkillImprovementSurvey.ts`](references/typescript/src/hooks/useSkillImprovementSurvey.ts) |
| `references/typescript/src/hooks/fileSuggestions.ts` | [`references/typescript/src/hooks/fileSuggestions.ts`](references/typescript/src/hooks/fileSuggestions.ts) |
| `references/typescript/src/hooks/useArrowKeyHistory.tsx` | [`references/typescript/src/hooks/useArrowKeyHistory.tsx`](references/typescript/src/hooks/useArrowKeyHistory.tsx) |
| `references/typescript/src/hooks/useIDEIntegration.tsx` | [`references/typescript/src/hooks/useIDEIntegration.tsx`](references/typescript/src/hooks/useIDEIntegration.tsx) |
| `references/typescript/src/hooks/useCommandQueue.ts` | [`references/typescript/src/hooks/useCommandQueue.ts`](references/typescript/src/hooks/useCommandQueue.ts) |
| `references/typescript/src/hooks/usePasteHandler.ts` | [`references/typescript/src/hooks/usePasteHandler.ts`](references/typescript/src/hooks/usePasteHandler.ts) |
| `references/typescript/src/hooks/useSwarmInitialization.ts` | [`references/typescript/src/hooks/useSwarmInitialization.ts`](references/typescript/src/hooks/useSwarmInitialization.ts) |
| `references/typescript/src/hooks/useScheduledTasks.ts` | [`references/typescript/src/hooks/useScheduledTasks.ts`](references/typescript/src/hooks/useScheduledTasks.ts) |
| `references/typescript/src/hooks/usePromptsFromClaudeInChrome.tsx` | [`references/typescript/src/hooks/usePromptsFromClaudeInChrome.tsx`](references/typescript/src/hooks/usePromptsFromClaudeInChrome.tsx) |
| `references/typescript/src/hooks/useCopyOnSelect.ts` | [`references/typescript/src/hooks/useCopyOnSelect.ts`](references/typescript/src/hooks/useCopyOnSelect.ts) |
| `references/typescript/src/hooks/useInputBuffer.ts` | [`references/typescript/src/hooks/useInputBuffer.ts`](references/typescript/src/hooks/useInputBuffer.ts) |
| `references/typescript/src/hooks/useTurnDiffs.ts` | [`references/typescript/src/hooks/useTurnDiffs.ts`](references/typescript/src/hooks/useTurnDiffs.ts) |
| `references/typescript/src/hooks/usePrStatus.ts` | [`references/typescript/src/hooks/usePrStatus.ts`](references/typescript/src/hooks/usePrStatus.ts) |
| `references/typescript/src/hooks/useIdeSelection.ts` | [`references/typescript/src/hooks/useIdeSelection.ts`](references/typescript/src/hooks/useIdeSelection.ts) |
| `references/typescript/src/hooks/useManagePlugins.ts` | [`references/typescript/src/hooks/useManagePlugins.ts`](references/typescript/src/hooks/useManagePlugins.ts) |
| `references/typescript/src/hooks/renderPlaceholder.ts` | [`references/typescript/src/hooks/renderPlaceholder.ts`](references/typescript/src/hooks/renderPlaceholder.ts) |
| `references/typescript/src/hooks/useCancelRequest.ts` | [`references/typescript/src/hooks/useCancelRequest.ts`](references/typescript/src/hooks/useCancelRequest.ts) |
| `references/typescript/src/hooks/useVoiceEnabled.ts` | [`references/typescript/src/hooks/useVoiceEnabled.ts`](references/typescript/src/hooks/useVoiceEnabled.ts) |
| `references/typescript/src/hooks/useMailboxBridge.ts` | [`references/typescript/src/hooks/useMailboxBridge.ts`](references/typescript/src/hooks/useMailboxBridge.ts) |
| `references/typescript/src/hooks/useRemoteSession.ts` | [`references/typescript/src/hooks/useRemoteSession.ts`](references/typescript/src/hooks/useRemoteSession.ts) |
| `references/typescript/src/hooks/useLogMessages.ts` | [`references/typescript/src/hooks/useLogMessages.ts`](references/typescript/src/hooks/useLogMessages.ts) |
| `references/typescript/src/hooks/useDiffInIDE.ts` | [`references/typescript/src/hooks/useDiffInIDE.ts`](references/typescript/src/hooks/useDiffInIDE.ts) |
| `references/typescript/src/hooks/useMainLoopModel.ts` | [`references/typescript/src/hooks/useMainLoopModel.ts`](references/typescript/src/hooks/useMainLoopModel.ts) |
| `references/typescript/src/hooks/useExitOnCtrlCD.ts` | [`references/typescript/src/hooks/useExitOnCtrlCD.ts`](references/typescript/src/hooks/useExitOnCtrlCD.ts) |
| `references/typescript/src/hooks/useTaskListWatcher.ts` | [`references/typescript/src/hooks/useTaskListWatcher.ts`](references/typescript/src/hooks/useTaskListWatcher.ts) |
| `references/typescript/src/hooks/useGlobalKeybindings.tsx` | [`references/typescript/src/hooks/useGlobalKeybindings.tsx`](references/typescript/src/hooks/useGlobalKeybindings.tsx) |
| `references/typescript/src/hooks/useSearchInput.ts` | [`references/typescript/src/hooks/useSearchInput.ts`](references/typescript/src/hooks/useSearchInput.ts) |
| `references/typescript/src/hooks/toolPermission/handlers/coordinatorHandler.ts` | [`references/typescript/src/hooks/toolPermission/handlers/coordinatorHandler.ts`](references/typescript/src/hooks/toolPermission/handlers/coordinatorHandler.ts) |
| `references/typescript/src/hooks/toolPermission/handlers/swarmWorkerHandler.ts` | [`references/typescript/src/hooks/toolPermission/handlers/swarmWorkerHandler.ts`](references/typescript/src/hooks/toolPermission/handlers/swarmWorkerHandler.ts) |
| `references/typescript/src/hooks/toolPermission/handlers/interactiveHandler.ts` | [`references/typescript/src/hooks/toolPermission/handlers/interactiveHandler.ts`](references/typescript/src/hooks/toolPermission/handlers/interactiveHandler.ts) |
| `references/typescript/src/hooks/toolPermission/PermissionContext.ts` | [`references/typescript/src/hooks/toolPermission/PermissionContext.ts`](references/typescript/src/hooks/toolPermission/PermissionContext.ts) |
| `references/typescript/src/hooks/toolPermission/permissionLogging.ts` | [`references/typescript/src/hooks/toolPermission/permissionLogging.ts`](references/typescript/src/hooks/toolPermission/permissionLogging.ts) |
| `references/typescript/src/hooks/useDeferredHookMessages.ts` | [`references/typescript/src/hooks/useDeferredHookMessages.ts`](references/typescript/src/hooks/useDeferredHookMessages.ts) |
| `references/typescript/src/hooks/useChromeExtensionNotification.tsx` | [`references/typescript/src/hooks/useChromeExtensionNotification.tsx`](references/typescript/src/hooks/useChromeExtensionNotification.tsx) |
| `references/typescript/src/hooks/useHistorySearch.ts` | [`references/typescript/src/hooks/useHistorySearch.ts`](references/typescript/src/hooks/useHistorySearch.ts) |
| `references/typescript/src/hooks/useBlink.ts` | [`references/typescript/src/hooks/useBlink.ts`](references/typescript/src/hooks/useBlink.ts) |
| `references/typescript/src/hooks/useVoiceIntegration.tsx` | [`references/typescript/src/hooks/useVoiceIntegration.tsx`](references/typescript/src/hooks/useVoiceIntegration.tsx) |
| `references/typescript/src/hooks/useDirectConnect.ts` | [`references/typescript/src/hooks/useDirectConnect.ts`](references/typescript/src/hooks/useDirectConnect.ts) |
| `references/typescript/src/hooks/useClaudeCodeHintRecommendation.tsx` | [`references/typescript/src/hooks/useClaudeCodeHintRecommendation.tsx`](references/typescript/src/hooks/useClaudeCodeHintRecommendation.tsx) |
| `references/typescript/src/hooks/unifiedSuggestions.ts` | [`references/typescript/src/hooks/unifiedSuggestions.ts`](references/typescript/src/hooks/unifiedSuggestions.ts) |
| `references/typescript/src/hooks/useDiffData.ts` | [`references/typescript/src/hooks/useDiffData.ts`](references/typescript/src/hooks/useDiffData.ts) |
| `references/typescript/src/hooks/useTimeout.ts` | [`references/typescript/src/hooks/useTimeout.ts`](references/typescript/src/hooks/useTimeout.ts) |
| `references/typescript/src/hooks/useVoice.ts` | [`references/typescript/src/hooks/useVoice.ts`](references/typescript/src/hooks/useVoice.ts) |
| `references/typescript/src/hooks/useMemoryUsage.ts` | [`references/typescript/src/hooks/useMemoryUsage.ts`](references/typescript/src/hooks/useMemoryUsage.ts) |
| `references/typescript/src/hooks/useTextInput.ts` | [`references/typescript/src/hooks/useTextInput.ts`](references/typescript/src/hooks/useTextInput.ts) |
| `references/typescript/src/hooks/useBackgroundTaskNavigation.ts` | [`references/typescript/src/hooks/useBackgroundTaskNavigation.ts`](references/typescript/src/hooks/useBackgroundTaskNavigation.ts) |
| `references/typescript/src/hooks/useUpdateNotification.ts` | [`references/typescript/src/hooks/useUpdateNotification.ts`](references/typescript/src/hooks/useUpdateNotification.ts) |
| `references/typescript/src/hooks/useTypeahead.tsx` | [`references/typescript/src/hooks/useTypeahead.tsx`](references/typescript/src/hooks/useTypeahead.tsx) |
| `references/typescript/src/hooks/notifs/useDeprecationWarningNotification.tsx` | [`references/typescript/src/hooks/notifs/useDeprecationWarningNotification.tsx`](references/typescript/src/hooks/notifs/useDeprecationWarningNotification.tsx) |
| `references/typescript/src/hooks/notifs/useAutoModeUnavailableNotification.ts` | [`references/typescript/src/hooks/notifs/useAutoModeUnavailableNotification.ts`](references/typescript/src/hooks/notifs/useAutoModeUnavailableNotification.ts) |
| `references/typescript/src/hooks/notifs/useLspInitializationNotification.tsx` | [`references/typescript/src/hooks/notifs/useLspInitializationNotification.tsx`](references/typescript/src/hooks/notifs/useLspInitializationNotification.tsx) |
| `references/typescript/src/hooks/notifs/useFastModeNotification.tsx` | [`references/typescript/src/hooks/notifs/useFastModeNotification.tsx`](references/typescript/src/hooks/notifs/useFastModeNotification.tsx) |
| `references/typescript/src/hooks/notifs/usePluginAutoupdateNotification.tsx` | [`references/typescript/src/hooks/notifs/usePluginAutoupdateNotification.tsx`](references/typescript/src/hooks/notifs/usePluginAutoupdateNotification.tsx) |
| `references/typescript/src/hooks/notifs/useCanSwitchToExistingSubscription.tsx` | [`references/typescript/src/hooks/notifs/useCanSwitchToExistingSubscription.tsx`](references/typescript/src/hooks/notifs/useCanSwitchToExistingSubscription.tsx) |
| `references/typescript/src/hooks/notifs/useNpmDeprecationNotification.tsx` | [`references/typescript/src/hooks/notifs/useNpmDeprecationNotification.tsx`](references/typescript/src/hooks/notifs/useNpmDeprecationNotification.tsx) |
| `references/typescript/src/hooks/notifs/useMcpConnectivityStatus.tsx` | [`references/typescript/src/hooks/notifs/useMcpConnectivityStatus.tsx`](references/typescript/src/hooks/notifs/useMcpConnectivityStatus.tsx) |
| `references/typescript/src/hooks/notifs/useRateLimitWarningNotification.tsx` | [`references/typescript/src/hooks/notifs/useRateLimitWarningNotification.tsx`](references/typescript/src/hooks/notifs/useRateLimitWarningNotification.tsx) |
| `references/typescript/src/hooks/notifs/useTeammateShutdownNotification.ts` | [`references/typescript/src/hooks/notifs/useTeammateShutdownNotification.ts`](references/typescript/src/hooks/notifs/useTeammateShutdownNotification.ts) |
| `references/typescript/src/hooks/notifs/useSettingsErrors.tsx` | [`references/typescript/src/hooks/notifs/useSettingsErrors.tsx`](references/typescript/src/hooks/notifs/useSettingsErrors.tsx) |
| `references/typescript/src/hooks/notifs/useIDEStatusIndicator.tsx` | [`references/typescript/src/hooks/notifs/useIDEStatusIndicator.tsx`](references/typescript/src/hooks/notifs/useIDEStatusIndicator.tsx) |
| `references/typescript/src/hooks/notifs/useStartupNotification.ts` | [`references/typescript/src/hooks/notifs/useStartupNotification.ts`](references/typescript/src/hooks/notifs/useStartupNotification.ts) |
| `references/typescript/src/hooks/notifs/usePluginInstallationStatus.tsx` | [`references/typescript/src/hooks/notifs/usePluginInstallationStatus.tsx`](references/typescript/src/hooks/notifs/usePluginInstallationStatus.tsx) |
| `references/typescript/src/hooks/notifs/useModelMigrationNotifications.tsx` | [`references/typescript/src/hooks/notifs/useModelMigrationNotifications.tsx`](references/typescript/src/hooks/notifs/useModelMigrationNotifications.tsx) |
| `references/typescript/src/hooks/notifs/useInstallMessages.tsx` | [`references/typescript/src/hooks/notifs/useInstallMessages.tsx`](references/typescript/src/hooks/notifs/useInstallMessages.tsx) |
| `references/typescript/src/hooks/useCommandKeybindings.tsx` | [`references/typescript/src/hooks/useCommandKeybindings.tsx`](references/typescript/src/hooks/useCommandKeybindings.tsx) |
| `references/typescript/src/hooks/useQueueProcessor.ts` | [`references/typescript/src/hooks/useQueueProcessor.ts`](references/typescript/src/hooks/useQueueProcessor.ts) |
| `references/typescript/src/hooks/useVimInput.ts` | [`references/typescript/src/hooks/useVimInput.ts`](references/typescript/src/hooks/useVimInput.ts) |
| `references/typescript/src/hooks/useSettings.ts` | [`references/typescript/src/hooks/useSettings.ts`](references/typescript/src/hooks/useSettings.ts) |
| `references/typescript/src/hooks/useTeammateViewAutoExit.ts` | [`references/typescript/src/hooks/useTeammateViewAutoExit.ts`](references/typescript/src/hooks/useTeammateViewAutoExit.ts) |
| `references/typescript/src/hooks/useAwaySummary.ts` | [`references/typescript/src/hooks/useAwaySummary.ts`](references/typescript/src/hooks/useAwaySummary.ts) |
| `references/typescript/src/hooks/usePluginRecommendationBase.tsx` | [`references/typescript/src/hooks/usePluginRecommendationBase.tsx`](references/typescript/src/hooks/usePluginRecommendationBase.tsx) |
| `references/typescript/src/hooks/useSessionBackgrounding.ts` | [`references/typescript/src/hooks/useSessionBackgrounding.ts`](references/typescript/src/hooks/useSessionBackgrounding.ts) |
| `references/typescript/src/hooks/useDynamicConfig.ts` | [`references/typescript/src/hooks/useDynamicConfig.ts`](references/typescript/src/hooks/useDynamicConfig.ts) |
| `references/typescript/src/hooks/useSkillsChange.ts` | [`references/typescript/src/hooks/useSkillsChange.ts`](references/typescript/src/hooks/useSkillsChange.ts) |
| `references/typescript/src/hooks/useAssistantHistory.ts` | [`references/typescript/src/hooks/useAssistantHistory.ts`](references/typescript/src/hooks/useAssistantHistory.ts) |
| `references/typescript/src/hooks/usePromptSuggestion.ts` | [`references/typescript/src/hooks/usePromptSuggestion.ts`](references/typescript/src/hooks/usePromptSuggestion.ts) |
| `references/typescript/src/hooks/useDoublePress.ts` | [`references/typescript/src/hooks/useDoublePress.ts`](references/typescript/src/hooks/useDoublePress.ts) |
| `references/typescript/src/hooks/useIssueFlagBanner.ts` | [`references/typescript/src/hooks/useIssueFlagBanner.ts`](references/typescript/src/hooks/useIssueFlagBanner.ts) |
| `references/typescript/src/hooks/useMergedClients.ts` | [`references/typescript/src/hooks/useMergedClients.ts`](references/typescript/src/hooks/useMergedClients.ts) |
| `references/typescript/src/hooks/useInboxPoller.ts` | [`references/typescript/src/hooks/useInboxPoller.ts`](references/typescript/src/hooks/useInboxPoller.ts) |
| `references/typescript/src/hooks/useSSHSession.ts` | [`references/typescript/src/hooks/useSSHSession.ts`](references/typescript/src/hooks/useSSHSession.ts) |
| `references/typescript/src/hooks/useCanUseTool.tsx` | [`references/typescript/src/hooks/useCanUseTool.tsx`](references/typescript/src/hooks/useCanUseTool.tsx) |
| `references/typescript/src/hooks/useLspPluginRecommendation.tsx` | [`references/typescript/src/hooks/useLspPluginRecommendation.tsx`](references/typescript/src/hooks/useLspPluginRecommendation.tsx) |
| `references/typescript/src/hooks/useIdeAtMentioned.ts` | [`references/typescript/src/hooks/useIdeAtMentioned.ts`](references/typescript/src/hooks/useIdeAtMentioned.ts) |
| `references/typescript/src/hooks/useTerminalSize.ts` | [`references/typescript/src/hooks/useTerminalSize.ts`](references/typescript/src/hooks/useTerminalSize.ts) |
| `references/typescript/src/hooks/useNotifyAfterTimeout.ts` | [`references/typescript/src/hooks/useNotifyAfterTimeout.ts`](references/typescript/src/hooks/useNotifyAfterTimeout.ts) |
| `references/typescript/src/hooks/useFileHistorySnapshotInit.ts` | [`references/typescript/src/hooks/useFileHistorySnapshotInit.ts`](references/typescript/src/hooks/useFileHistorySnapshotInit.ts) |
| `references/typescript/src/hooks/useApiKeyVerification.ts` | [`references/typescript/src/hooks/useApiKeyVerification.ts`](references/typescript/src/hooks/useApiKeyVerification.ts) |
| `references/typescript/src/hooks/useVirtualScroll.ts` | [`references/typescript/src/hooks/useVirtualScroll.ts`](references/typescript/src/hooks/useVirtualScroll.ts) |
| `references/typescript/src/hooks/useTeleportResume.tsx` | [`references/typescript/src/hooks/useTeleportResume.tsx`](references/typescript/src/hooks/useTeleportResume.tsx) |
| `references/typescript/src/hooks/useIdeConnectionStatus.ts` | [`references/typescript/src/hooks/useIdeConnectionStatus.ts`](references/typescript/src/hooks/useIdeConnectionStatus.ts) |
| `references/typescript/src/hooks/useAfterFirstRender.ts` | [`references/typescript/src/hooks/useAfterFirstRender.ts`](references/typescript/src/hooks/useAfterFirstRender.ts) |
| `references/typescript/src/hooks/useMinDisplayTime.ts` | [`references/typescript/src/hooks/useMinDisplayTime.ts`](references/typescript/src/hooks/useMinDisplayTime.ts) |
| `references/typescript/src/hooks/useClipboardImageHint.ts` | [`references/typescript/src/hooks/useClipboardImageHint.ts`](references/typescript/src/hooks/useClipboardImageHint.ts) |
| `references/typescript/src/hooks/useMergedTools.ts` | [`references/typescript/src/hooks/useMergedTools.ts`](references/typescript/src/hooks/useMergedTools.ts) |
| `references/typescript/src/hooks/useSwarmPermissionPoller.ts` | [`references/typescript/src/hooks/useSwarmPermissionPoller.ts`](references/typescript/src/hooks/useSwarmPermissionPoller.ts) |
| `references/typescript/src/hooks/useOfficialMarketplaceNotification.tsx` | [`references/typescript/src/hooks/useOfficialMarketplaceNotification.tsx`](references/typescript/src/hooks/useOfficialMarketplaceNotification.tsx) |
| `references/typescript/src/hooks/useElapsedTime.ts` | [`references/typescript/src/hooks/useElapsedTime.ts`](references/typescript/src/hooks/useElapsedTime.ts) |
| `references/typescript/src/hooks/useTasksV2.ts` | [`references/typescript/src/hooks/useTasksV2.ts`](references/typescript/src/hooks/useTasksV2.ts) |
| `references/typescript/src/hooks/useMergedCommands.ts` | [`references/typescript/src/hooks/useMergedCommands.ts`](references/typescript/src/hooks/useMergedCommands.ts) |
| `references/typescript/src/tasks.ts` | [`references/typescript/src/tasks.ts`](references/typescript/src/tasks.ts) |
| `references/typescript/src/migrations/migrateBypassPermissionsAcceptedToSettings.ts` | [`references/typescript/src/migrations/migrateBypassPermissionsAcceptedToSettings.ts`](references/typescript/src/migrations/migrateBypassPermissionsAcceptedToSettings.ts) |
| `references/typescript/src/migrations/migrateOpusToOpus1m.ts` | [`references/typescript/src/migrations/migrateOpusToOpus1m.ts`](references/typescript/src/migrations/migrateOpusToOpus1m.ts) |
| `references/typescript/src/migrations/migrateEnableAllProjectMcpServersToSettings.ts` | [`references/typescript/src/migrations/migrateEnableAllProjectMcpServersToSettings.ts`](references/typescript/src/migrations/migrateEnableAllProjectMcpServersToSettings.ts) |
| `references/typescript/src/migrations/resetAutoModeOptInForDefaultOffer.ts` | [`references/typescript/src/migrations/resetAutoModeOptInForDefaultOffer.ts`](references/typescript/src/migrations/resetAutoModeOptInForDefaultOffer.ts) |
| `references/typescript/src/migrations/migrateReplBridgeEnabledToRemoteControlAtStartup.ts` | [`references/typescript/src/migrations/migrateReplBridgeEnabledToRemoteControlAtStartup.ts`](references/typescript/src/migrations/migrateReplBridgeEnabledToRemoteControlAtStartup.ts) |
| `references/typescript/src/migrations/migrateSonnet45ToSonnet46.ts` | [`references/typescript/src/migrations/migrateSonnet45ToSonnet46.ts`](references/typescript/src/migrations/migrateSonnet45ToSonnet46.ts) |
| `references/typescript/src/migrations/migrateLegacyOpusToCurrent.ts` | [`references/typescript/src/migrations/migrateLegacyOpusToCurrent.ts`](references/typescript/src/migrations/migrateLegacyOpusToCurrent.ts) |
| `references/typescript/src/migrations/resetProToOpusDefault.ts` | [`references/typescript/src/migrations/resetProToOpusDefault.ts`](references/typescript/src/migrations/resetProToOpusDefault.ts) |
| `references/typescript/src/migrations/migrateFennecToOpus.ts` | [`references/typescript/src/migrations/migrateFennecToOpus.ts`](references/typescript/src/migrations/migrateFennecToOpus.ts) |
| `references/typescript/src/migrations/migrateSonnet1mToSonnet45.ts` | [`references/typescript/src/migrations/migrateSonnet1mToSonnet45.ts`](references/typescript/src/migrations/migrateSonnet1mToSonnet45.ts) |
| `references/typescript/src/migrations/migrateAutoUpdatesToSettings.ts` | [`references/typescript/src/migrations/migrateAutoUpdatesToSettings.ts`](references/typescript/src/migrations/migrateAutoUpdatesToSettings.ts) |
| `references/typescript/src/costHook.ts` | [`references/typescript/src/costHook.ts`](references/typescript/src/costHook.ts) |
| `references/typescript/src/vendor/ripgrep/x64-linux/rg` | [`references/typescript/src/vendor/ripgrep/x64-linux/rg`](references/typescript/src/vendor/ripgrep/x64-linux/rg) |
| `references/typescript/src/tools.ts` | [`references/typescript/src/tools.ts`](references/typescript/src/tools.ts) |
| `references/typescript/src/dialogLaunchers.tsx` | [`references/typescript/src/dialogLaunchers.tsx`](references/typescript/src/dialogLaunchers.tsx) |
| `references/typescript/src/query/config.ts` | [`references/typescript/src/query/config.ts`](references/typescript/src/query/config.ts) |
| `references/typescript/src/query/tokenBudget.ts` | [`references/typescript/src/query/tokenBudget.ts`](references/typescript/src/query/tokenBudget.ts) |
| `references/typescript/src/query/stopHooks.ts` | [`references/typescript/src/query/stopHooks.ts`](references/typescript/src/query/stopHooks.ts) |
| `references/typescript/src/query/deps.ts` | [`references/typescript/src/query/deps.ts`](references/typescript/src/query/deps.ts) |
| `references/typescript/src/cli/ndjsonSafeStringify.ts` | [`references/typescript/src/cli/ndjsonSafeStringify.ts`](references/typescript/src/cli/ndjsonSafeStringify.ts) |
| `references/typescript/src/cli/transports/transportUtils.ts` | [`references/typescript/src/cli/transports/transportUtils.ts`](references/typescript/src/cli/transports/transportUtils.ts) |
| `references/typescript/src/cli/transports/WebSocketTransport.ts` | [`references/typescript/src/cli/transports/WebSocketTransport.ts`](references/typescript/src/cli/transports/WebSocketTransport.ts) |
| `references/typescript/src/cli/transports/ccrClient.ts` | [`references/typescript/src/cli/transports/ccrClient.ts`](references/typescript/src/cli/transports/ccrClient.ts) |
| `references/typescript/src/cli/transports/SSETransport.ts` | [`references/typescript/src/cli/transports/SSETransport.ts`](references/typescript/src/cli/transports/SSETransport.ts) |
| `references/typescript/src/cli/transports/HybridTransport.ts` | [`references/typescript/src/cli/transports/HybridTransport.ts`](references/typescript/src/cli/transports/HybridTransport.ts) |
| `references/typescript/src/cli/transports/SerialBatchEventUploader.ts` | [`references/typescript/src/cli/transports/SerialBatchEventUploader.ts`](references/typescript/src/cli/transports/SerialBatchEventUploader.ts) |
| `references/typescript/src/cli/transports/WorkerStateUploader.ts` | [`references/typescript/src/cli/transports/WorkerStateUploader.ts`](references/typescript/src/cli/transports/WorkerStateUploader.ts) |
| `references/typescript/src/cli/structuredIO.ts` | [`references/typescript/src/cli/structuredIO.ts`](references/typescript/src/cli/structuredIO.ts) |
| `references/typescript/src/cli/handlers/agents.ts` | [`references/typescript/src/cli/handlers/agents.ts`](references/typescript/src/cli/handlers/agents.ts) |
| `references/typescript/src/cli/handlers/autoMode.ts` | [`references/typescript/src/cli/handlers/autoMode.ts`](references/typescript/src/cli/handlers/autoMode.ts) |
| `references/typescript/src/cli/handlers/mcp.tsx` | [`references/typescript/src/cli/handlers/mcp.tsx`](references/typescript/src/cli/handlers/mcp.tsx) |
| `references/typescript/src/cli/handlers/plugins.ts` | [`references/typescript/src/cli/handlers/plugins.ts`](references/typescript/src/cli/handlers/plugins.ts) |
| `references/typescript/src/cli/handlers/util.tsx` | [`references/typescript/src/cli/handlers/util.tsx`](references/typescript/src/cli/handlers/util.tsx) |
| `references/typescript/src/cli/handlers/auth.ts` | [`references/typescript/src/cli/handlers/auth.ts`](references/typescript/src/cli/handlers/auth.ts) |
| `references/typescript/src/cli/print.ts` | [`references/typescript/src/cli/print.ts`](references/typescript/src/cli/print.ts) |
| `references/typescript/src/cli/exit.ts` | [`references/typescript/src/cli/exit.ts`](references/typescript/src/cli/exit.ts) |
| `references/typescript/src/cli/update.ts` | [`references/typescript/src/cli/update.ts`](references/typescript/src/cli/update.ts) |
| `references/typescript/src/cli/remoteIO.ts` | [`references/typescript/src/cli/remoteIO.ts`](references/typescript/src/cli/remoteIO.ts) |
| `references/typescript/src/assistant/AssistantSessionChooser.tsx` | [`references/typescript/src/assistant/AssistantSessionChooser.tsx`](references/typescript/src/assistant/AssistantSessionChooser.tsx) |
| `references/typescript/src/assistant/sessionHistory.ts` | [`references/typescript/src/assistant/sessionHistory.ts`](references/typescript/src/assistant/sessionHistory.ts) |
| `references/typescript/src/bootstrap/state.ts` | [`references/typescript/src/bootstrap/state.ts`](references/typescript/src/bootstrap/state.ts) |
| `references/typescript/src/server/createDirectConnectSession.ts` | [`references/typescript/src/server/createDirectConnectSession.ts`](references/typescript/src/server/createDirectConnectSession.ts) |
| `references/typescript/src/server/types.ts` | [`references/typescript/src/server/types.ts`](references/typescript/src/server/types.ts) |
| `references/typescript/src/server/directConnectManager.ts` | [`references/typescript/src/server/directConnectManager.ts`](references/typescript/src/server/directConnectManager.ts) |
| `references/typescript/src/ink/stringWidth.ts` | [`references/typescript/src/ink/stringWidth.ts`](references/typescript/src/ink/stringWidth.ts) |
| `references/typescript/src/ink/renderer.ts` | [`references/typescript/src/ink/renderer.ts`](references/typescript/src/ink/renderer.ts) |
| `references/typescript/src/ink/screen.ts` | [`references/typescript/src/ink/screen.ts`](references/typescript/src/ink/screen.ts) |
| `references/typescript/src/ink/render-border.ts` | [`references/typescript/src/ink/render-border.ts`](references/typescript/src/ink/render-border.ts) |
| `references/typescript/src/ink/node-cache.ts` | [`references/typescript/src/ink/node-cache.ts`](references/typescript/src/ink/node-cache.ts) |
| `references/typescript/src/ink/get-max-width.ts` | [`references/typescript/src/ink/get-max-width.ts`](references/typescript/src/ink/get-max-width.ts) |
| `references/typescript/src/ink/frame.ts` | [`references/typescript/src/ink/frame.ts`](references/typescript/src/ink/frame.ts) |
| `references/typescript/src/ink/measure-text.ts` | [`references/typescript/src/ink/measure-text.ts`](references/typescript/src/ink/measure-text.ts) |
| `references/typescript/src/ink/termio/esc.ts` | [`references/typescript/src/ink/termio/esc.ts`](references/typescript/src/ink/termio/esc.ts) |
| `references/typescript/src/ink/termio/types.ts` | [`references/typescript/src/ink/termio/types.ts`](references/typescript/src/ink/termio/types.ts) |
| `references/typescript/src/ink/termio/csi.ts` | [`references/typescript/src/ink/termio/csi.ts`](references/typescript/src/ink/termio/csi.ts) |
| `references/typescript/src/ink/termio/dec.ts` | [`references/typescript/src/ink/termio/dec.ts`](references/typescript/src/ink/termio/dec.ts) |
| `references/typescript/src/ink/termio/osc.ts` | [`references/typescript/src/ink/termio/osc.ts`](references/typescript/src/ink/termio/osc.ts) |
| `references/typescript/src/ink/termio/sgr.ts` | [`references/typescript/src/ink/termio/sgr.ts`](references/typescript/src/ink/termio/sgr.ts) |
| `references/typescript/src/ink/termio/tokenize.ts` | [`references/typescript/src/ink/termio/tokenize.ts`](references/typescript/src/ink/termio/tokenize.ts) |
| `references/typescript/src/ink/termio/parser.ts` | [`references/typescript/src/ink/termio/parser.ts`](references/typescript/src/ink/termio/parser.ts) |
| `references/typescript/src/ink/termio/ansi.ts` | [`references/typescript/src/ink/termio/ansi.ts`](references/typescript/src/ink/termio/ansi.ts) |
| `references/typescript/src/ink/render-to-screen.ts` | [`references/typescript/src/ink/render-to-screen.ts`](references/typescript/src/ink/render-to-screen.ts) |
| `references/typescript/src/ink/termio.ts` | [`references/typescript/src/ink/termio.ts`](references/typescript/src/ink/termio.ts) |
| `references/typescript/src/ink/layout/engine.ts` | [`references/typescript/src/ink/layout/engine.ts`](references/typescript/src/ink/layout/engine.ts) |
| `references/typescript/src/ink/layout/node.ts` | [`references/typescript/src/ink/layout/node.ts`](references/typescript/src/ink/layout/node.ts) |
| `references/typescript/src/ink/layout/geometry.ts` | [`references/typescript/src/ink/layout/geometry.ts`](references/typescript/src/ink/layout/geometry.ts) |
| `references/typescript/src/ink/layout/yoga.ts` | [`references/typescript/src/ink/layout/yoga.ts`](references/typescript/src/ink/layout/yoga.ts) |
| `references/typescript/src/ink/global.d.ts` | [`references/typescript/src/ink/global.d.ts`](references/typescript/src/ink/global.d.ts) |
| `references/typescript/src/ink/wrapAnsi.ts` | [`references/typescript/src/ink/wrapAnsi.ts`](references/typescript/src/ink/wrapAnsi.ts) |
| `references/typescript/src/ink/searchHighlight.ts` | [`references/typescript/src/ink/searchHighlight.ts`](references/typescript/src/ink/searchHighlight.ts) |
| `references/typescript/src/ink/widest-line.ts` | [`references/typescript/src/ink/widest-line.ts`](references/typescript/src/ink/widest-line.ts) |
| `references/typescript/src/ink/hit-test.ts` | [`references/typescript/src/ink/hit-test.ts`](references/typescript/src/ink/hit-test.ts) |
| `references/typescript/src/ink/reconciler.ts` | [`references/typescript/src/ink/reconciler.ts`](references/typescript/src/ink/reconciler.ts) |
| `references/typescript/src/ink/useTerminalNotification.ts` | [`references/typescript/src/ink/useTerminalNotification.ts`](references/typescript/src/ink/useTerminalNotification.ts) |
| `references/typescript/src/ink/tabstops.ts` | [`references/typescript/src/ink/tabstops.ts`](references/typescript/src/ink/tabstops.ts) |
| `references/typescript/src/ink/output.ts` | [`references/typescript/src/ink/output.ts`](references/typescript/src/ink/output.ts) |
| `references/typescript/src/ink/ink.tsx` | [`references/typescript/src/ink/ink.tsx`](references/typescript/src/ink/ink.tsx) |
| `references/typescript/src/ink/colorize.ts` | [`references/typescript/src/ink/colorize.ts`](references/typescript/src/ink/colorize.ts) |
| `references/typescript/src/ink/focus.ts` | [`references/typescript/src/ink/focus.ts`](references/typescript/src/ink/focus.ts) |
| `references/typescript/src/ink/dom.ts` | [`references/typescript/src/ink/dom.ts`](references/typescript/src/ink/dom.ts) |
| `references/typescript/src/ink/Ansi.tsx` | [`references/typescript/src/ink/Ansi.tsx`](references/typescript/src/ink/Ansi.tsx) |
| `references/typescript/src/ink/terminal.ts` | [`references/typescript/src/ink/terminal.ts`](references/typescript/src/ink/terminal.ts) |
| `references/typescript/src/ink/bidi.ts` | [`references/typescript/src/ink/bidi.ts`](references/typescript/src/ink/bidi.ts) |
| `references/typescript/src/ink/selection.ts` | [`references/typescript/src/ink/selection.ts`](references/typescript/src/ink/selection.ts) |
| `references/typescript/src/ink/events/input-event.ts` | [`references/typescript/src/ink/events/input-event.ts`](references/typescript/src/ink/events/input-event.ts) |
| `references/typescript/src/ink/events/terminal-event.ts` | [`references/typescript/src/ink/events/terminal-event.ts`](references/typescript/src/ink/events/terminal-event.ts) |
| `references/typescript/src/ink/events/dispatcher.ts` | [`references/typescript/src/ink/events/dispatcher.ts`](references/typescript/src/ink/events/dispatcher.ts) |
| `references/typescript/src/ink/events/focus-event.ts` | [`references/typescript/src/ink/events/focus-event.ts`](references/typescript/src/ink/events/focus-event.ts) |
| `references/typescript/src/ink/events/keyboard-event.ts` | [`references/typescript/src/ink/events/keyboard-event.ts`](references/typescript/src/ink/events/keyboard-event.ts) |
| `references/typescript/src/ink/events/event.ts` | [`references/typescript/src/ink/events/event.ts`](references/typescript/src/ink/events/event.ts) |
| `references/typescript/src/ink/events/click-event.ts` | [`references/typescript/src/ink/events/click-event.ts`](references/typescript/src/ink/events/click-event.ts) |
| `references/typescript/src/ink/events/event-handlers.ts` | [`references/typescript/src/ink/events/event-handlers.ts`](references/typescript/src/ink/events/event-handlers.ts) |
| `references/typescript/src/ink/events/emitter.ts` | [`references/typescript/src/ink/events/emitter.ts`](references/typescript/src/ink/events/emitter.ts) |
| `references/typescript/src/ink/events/terminal-focus-event.ts` | [`references/typescript/src/ink/events/terminal-focus-event.ts`](references/typescript/src/ink/events/terminal-focus-event.ts) |
| `references/typescript/src/ink/render-node-to-output.ts` | [`references/typescript/src/ink/render-node-to-output.ts`](references/typescript/src/ink/render-node-to-output.ts) |
| `references/typescript/src/ink/hooks/use-animation-frame.ts` | [`references/typescript/src/ink/hooks/use-animation-frame.ts`](references/typescript/src/ink/hooks/use-animation-frame.ts) |
| `references/typescript/src/ink/hooks/use-search-highlight.ts` | [`references/typescript/src/ink/hooks/use-search-highlight.ts`](references/typescript/src/ink/hooks/use-search-highlight.ts) |
| `references/typescript/src/ink/hooks/use-terminal-title.ts` | [`references/typescript/src/ink/hooks/use-terminal-title.ts`](references/typescript/src/ink/hooks/use-terminal-title.ts) |
| `references/typescript/src/ink/hooks/use-declared-cursor.ts` | [`references/typescript/src/ink/hooks/use-declared-cursor.ts`](references/typescript/src/ink/hooks/use-declared-cursor.ts) |
| `references/typescript/src/ink/hooks/use-interval.ts` | [`references/typescript/src/ink/hooks/use-interval.ts`](references/typescript/src/ink/hooks/use-interval.ts) |
| `references/typescript/src/ink/hooks/use-tab-status.ts` | [`references/typescript/src/ink/hooks/use-tab-status.ts`](references/typescript/src/ink/hooks/use-tab-status.ts) |
| `references/typescript/src/ink/hooks/use-terminal-viewport.ts` | [`references/typescript/src/ink/hooks/use-terminal-viewport.ts`](references/typescript/src/ink/hooks/use-terminal-viewport.ts) |
| `references/typescript/src/ink/hooks/use-selection.ts` | [`references/typescript/src/ink/hooks/use-selection.ts`](references/typescript/src/ink/hooks/use-selection.ts) |
| `references/typescript/src/ink/hooks/use-input.ts` | [`references/typescript/src/ink/hooks/use-input.ts`](references/typescript/src/ink/hooks/use-input.ts) |
| `references/typescript/src/ink/hooks/use-terminal-focus.ts` | [`references/typescript/src/ink/hooks/use-terminal-focus.ts`](references/typescript/src/ink/hooks/use-terminal-focus.ts) |
| `references/typescript/src/ink/hooks/use-app.ts` | [`references/typescript/src/ink/hooks/use-app.ts`](references/typescript/src/ink/hooks/use-app.ts) |
| `references/typescript/src/ink/hooks/use-stdin.ts` | [`references/typescript/src/ink/hooks/use-stdin.ts`](references/typescript/src/ink/hooks/use-stdin.ts) |
| `references/typescript/src/ink/log-update.ts` | [`references/typescript/src/ink/log-update.ts`](references/typescript/src/ink/log-update.ts) |
| `references/typescript/src/ink/terminal-focus-state.ts` | [`references/typescript/src/ink/terminal-focus-state.ts`](references/typescript/src/ink/terminal-focus-state.ts) |
| `references/typescript/src/ink/instances.ts` | [`references/typescript/src/ink/instances.ts`](references/typescript/src/ink/instances.ts) |
| `references/typescript/src/ink/root.ts` | [`references/typescript/src/ink/root.ts`](references/typescript/src/ink/root.ts) |
| `references/typescript/src/ink/wrap-text.ts` | [`references/typescript/src/ink/wrap-text.ts`](references/typescript/src/ink/wrap-text.ts) |
| `references/typescript/src/ink/warn.ts` | [`references/typescript/src/ink/warn.ts`](references/typescript/src/ink/warn.ts) |
| `references/typescript/src/ink/line-width-cache.ts` | [`references/typescript/src/ink/line-width-cache.ts`](references/typescript/src/ink/line-width-cache.ts) |
| `references/typescript/src/ink/constants.ts` | [`references/typescript/src/ink/constants.ts`](references/typescript/src/ink/constants.ts) |
| `references/typescript/src/ink/optimizer.ts` | [`references/typescript/src/ink/optimizer.ts`](references/typescript/src/ink/optimizer.ts) |
| `references/typescript/src/ink/clearTerminal.ts` | [`references/typescript/src/ink/clearTerminal.ts`](references/typescript/src/ink/clearTerminal.ts) |
| `references/typescript/src/ink/measure-element.ts` | [`references/typescript/src/ink/measure-element.ts`](references/typescript/src/ink/measure-element.ts) |
| `references/typescript/src/ink/components/StdinContext.ts` | [`references/typescript/src/ink/components/StdinContext.ts`](references/typescript/src/ink/components/StdinContext.ts) |
| `references/typescript/src/ink/components/Newline.tsx` | [`references/typescript/src/ink/components/Newline.tsx`](references/typescript/src/ink/components/Newline.tsx) |
| `references/typescript/src/ink/components/TerminalFocusContext.tsx` | [`references/typescript/src/ink/components/TerminalFocusContext.tsx`](references/typescript/src/ink/components/TerminalFocusContext.tsx) |
| `references/typescript/src/ink/components/Spacer.tsx` | [`references/typescript/src/ink/components/Spacer.tsx`](references/typescript/src/ink/components/Spacer.tsx) |
| `references/typescript/src/ink/components/NoSelect.tsx` | [`references/typescript/src/ink/components/NoSelect.tsx`](references/typescript/src/ink/components/NoSelect.tsx) |
| `references/typescript/src/ink/components/Link.tsx` | [`references/typescript/src/ink/components/Link.tsx`](references/typescript/src/ink/components/Link.tsx) |
| `references/typescript/src/ink/components/ClockContext.tsx` | [`references/typescript/src/ink/components/ClockContext.tsx`](references/typescript/src/ink/components/ClockContext.tsx) |
| `references/typescript/src/ink/components/AppContext.ts` | [`references/typescript/src/ink/components/AppContext.ts`](references/typescript/src/ink/components/AppContext.ts) |
| `references/typescript/src/ink/components/CursorDeclarationContext.ts` | [`references/typescript/src/ink/components/CursorDeclarationContext.ts`](references/typescript/src/ink/components/CursorDeclarationContext.ts) |
| `references/typescript/src/ink/components/App.tsx` | [`references/typescript/src/ink/components/App.tsx`](references/typescript/src/ink/components/App.tsx) |
| `references/typescript/src/ink/components/Text.tsx` | [`references/typescript/src/ink/components/Text.tsx`](references/typescript/src/ink/components/Text.tsx) |
| `references/typescript/src/ink/components/Box.tsx` | [`references/typescript/src/ink/components/Box.tsx`](references/typescript/src/ink/components/Box.tsx) |
| `references/typescript/src/ink/components/Button.tsx` | [`references/typescript/src/ink/components/Button.tsx`](references/typescript/src/ink/components/Button.tsx) |
| `references/typescript/src/ink/components/ScrollBox.tsx` | [`references/typescript/src/ink/components/ScrollBox.tsx`](references/typescript/src/ink/components/ScrollBox.tsx) |
| `references/typescript/src/ink/components/RawAnsi.tsx` | [`references/typescript/src/ink/components/RawAnsi.tsx`](references/typescript/src/ink/components/RawAnsi.tsx) |
| `references/typescript/src/ink/components/TerminalSizeContext.tsx` | [`references/typescript/src/ink/components/TerminalSizeContext.tsx`](references/typescript/src/ink/components/TerminalSizeContext.tsx) |
| `references/typescript/src/ink/components/ErrorOverview.tsx` | [`references/typescript/src/ink/components/ErrorOverview.tsx`](references/typescript/src/ink/components/ErrorOverview.tsx) |
| `references/typescript/src/ink/components/AlternateScreen.tsx` | [`references/typescript/src/ink/components/AlternateScreen.tsx`](references/typescript/src/ink/components/AlternateScreen.tsx) |
| `references/typescript/src/ink/terminal-querier.ts` | [`references/typescript/src/ink/terminal-querier.ts`](references/typescript/src/ink/terminal-querier.ts) |
| `references/typescript/src/ink/devtools.ts` | [`references/typescript/src/ink/devtools.ts`](references/typescript/src/ink/devtools.ts) |
| `references/typescript/src/ink/parse-keypress.ts` | [`references/typescript/src/ink/parse-keypress.ts`](references/typescript/src/ink/parse-keypress.ts) |
| `references/typescript/src/ink/squash-text-nodes.ts` | [`references/typescript/src/ink/squash-text-nodes.ts`](references/typescript/src/ink/squash-text-nodes.ts) |
| `references/typescript/src/ink/supports-hyperlinks.ts` | [`references/typescript/src/ink/supports-hyperlinks.ts`](references/typescript/src/ink/supports-hyperlinks.ts) |
| `references/typescript/src/ink/styles.ts` | [`references/typescript/src/ink/styles.ts`](references/typescript/src/ink/styles.ts) |
| `references/typescript/src/commands.ts` | [`references/typescript/src/commands.ts`](references/typescript/src/commands.ts) |
| `references/typescript/src/skills/bundled/loremIpsum.ts` | [`references/typescript/src/skills/bundled/loremIpsum.ts`](references/typescript/src/skills/bundled/loremIpsum.ts) |
| `references/typescript/src/skills/bundled/verify.ts` | [`references/typescript/src/skills/bundled/verify.ts`](references/typescript/src/skills/bundled/verify.ts) |
| `references/typescript/src/skills/bundled/loop.ts` | [`references/typescript/src/skills/bundled/loop.ts`](references/typescript/src/skills/bundled/loop.ts) |
| `references/typescript/src/skills/bundled/keybindings.ts` | [`references/typescript/src/skills/bundled/keybindings.ts`](references/typescript/src/skills/bundled/keybindings.ts) |
| `references/typescript/src/skills/bundled/batch.ts` | [`references/typescript/src/skills/bundled/batch.ts`](references/typescript/src/skills/bundled/batch.ts) |
| `references/typescript/src/skills/bundled/scheduleRemoteAgents.ts` | [`references/typescript/src/skills/bundled/scheduleRemoteAgents.ts`](references/typescript/src/skills/bundled/scheduleRemoteAgents.ts) |
| `references/typescript/src/skills/bundled/simplify.ts` | [`references/typescript/src/skills/bundled/simplify.ts`](references/typescript/src/skills/bundled/simplify.ts) |
| `references/typescript/src/skills/bundled/index.ts` | [`references/typescript/src/skills/bundled/index.ts`](references/typescript/src/skills/bundled/index.ts) |
| `references/typescript/src/skills/bundled/claudeInChrome.ts` | [`references/typescript/src/skills/bundled/claudeInChrome.ts`](references/typescript/src/skills/bundled/claudeInChrome.ts) |
| `references/typescript/src/skills/bundled/stuck.ts` | [`references/typescript/src/skills/bundled/stuck.ts`](references/typescript/src/skills/bundled/stuck.ts) |
| `references/typescript/src/skills/bundled/debug.ts` | [`references/typescript/src/skills/bundled/debug.ts`](references/typescript/src/skills/bundled/debug.ts) |
| `references/typescript/src/skills/bundled/skillify.ts` | [`references/typescript/src/skills/bundled/skillify.ts`](references/typescript/src/skills/bundled/skillify.ts) |
| `references/typescript/src/skills/bundled/verifyContent.ts` | [`references/typescript/src/skills/bundled/verifyContent.ts`](references/typescript/src/skills/bundled/verifyContent.ts) |
| `references/typescript/src/skills/bundled/claudeApiContent.ts` | [`references/typescript/src/skills/bundled/claudeApiContent.ts`](references/typescript/src/skills/bundled/claudeApiContent.ts) |
| `references/typescript/src/skills/bundled/remember.ts` | [`references/typescript/src/skills/bundled/remember.ts`](references/typescript/src/skills/bundled/remember.ts) |
| `references/typescript/src/skills/bundled/updateConfig.ts` | [`references/typescript/src/skills/bundled/updateConfig.ts`](references/typescript/src/skills/bundled/updateConfig.ts) |
| `references/typescript/src/skills/bundled/claudeApi.ts` | [`references/typescript/src/skills/bundled/claudeApi.ts`](references/typescript/src/skills/bundled/claudeApi.ts) |
| `references/typescript/src/skills/bundledSkills.ts` | [`references/typescript/src/skills/bundledSkills.ts`](references/typescript/src/skills/bundledSkills.ts) |
| `references/typescript/src/skills/mcpSkillBuilders.ts` | [`references/typescript/src/skills/mcpSkillBuilders.ts`](references/typescript/src/skills/mcpSkillBuilders.ts) |
| `references/typescript/src/skills/loadSkillsDir.ts` | [`references/typescript/src/skills/loadSkillsDir.ts`](references/typescript/src/skills/loadSkillsDir.ts) |
| `references/typescript/src/components/Message.tsx` | [`references/typescript/src/components/Message.tsx`](references/typescript/src/components/Message.tsx) |
| `references/typescript/src/components/TeleportResumeWrapper.tsx` | [`references/typescript/src/components/TeleportResumeWrapper.tsx`](references/typescript/src/components/TeleportResumeWrapper.tsx) |
| `references/typescript/src/components/ContextSuggestions.tsx` | [`references/typescript/src/components/ContextSuggestions.tsx`](references/typescript/src/components/ContextSuggestions.tsx) |
| `references/typescript/src/components/CustomSelect/SelectMulti.tsx` | [`references/typescript/src/components/CustomSelect/SelectMulti.tsx`](references/typescript/src/components/CustomSelect/SelectMulti.tsx) |
| `references/typescript/src/components/CustomSelect/select.tsx` | [`references/typescript/src/components/CustomSelect/select.tsx`](references/typescript/src/components/CustomSelect/select.tsx) |
| `references/typescript/src/components/CustomSelect/use-multi-select-state.ts` | [`references/typescript/src/components/CustomSelect/use-multi-select-state.ts`](references/typescript/src/components/CustomSelect/use-multi-select-state.ts) |
| `references/typescript/src/components/CustomSelect/select-option.tsx` | [`references/typescript/src/components/CustomSelect/select-option.tsx`](references/typescript/src/components/CustomSelect/select-option.tsx) |
| `references/typescript/src/components/CustomSelect/use-select-input.ts` | [`references/typescript/src/components/CustomSelect/use-select-input.ts`](references/typescript/src/components/CustomSelect/use-select-input.ts) |
| `references/typescript/src/components/CustomSelect/select-input-option.tsx` | [`references/typescript/src/components/CustomSelect/select-input-option.tsx`](references/typescript/src/components/CustomSelect/select-input-option.tsx) |
| `references/typescript/src/components/CustomSelect/option-map.ts` | [`references/typescript/src/components/CustomSelect/option-map.ts`](references/typescript/src/components/CustomSelect/option-map.ts) |
| `references/typescript/src/components/CustomSelect/index.ts` | [`references/typescript/src/components/CustomSelect/index.ts`](references/typescript/src/components/CustomSelect/index.ts) |
| `references/typescript/src/components/CustomSelect/use-select-state.ts` | [`references/typescript/src/components/CustomSelect/use-select-state.ts`](references/typescript/src/components/CustomSelect/use-select-state.ts) |
| `references/typescript/src/components/CustomSelect/use-select-navigation.ts` | [`references/typescript/src/components/CustomSelect/use-select-navigation.ts`](references/typescript/src/components/CustomSelect/use-select-navigation.ts) |
| `references/typescript/src/components/messageActions.tsx` | [`references/typescript/src/components/messageActions.tsx`](references/typescript/src/components/messageActions.tsx) |
| `references/typescript/src/components/CoordinatorAgentStatus.tsx` | [`references/typescript/src/components/CoordinatorAgentStatus.tsx`](references/typescript/src/components/CoordinatorAgentStatus.tsx) |
| `references/typescript/src/components/ThinkingToggle.tsx` | [`references/typescript/src/components/ThinkingToggle.tsx`](references/typescript/src/components/ThinkingToggle.tsx) |
| `references/typescript/src/components/sandbox/SandboxConfigTab.tsx` | [`references/typescript/src/components/sandbox/SandboxConfigTab.tsx`](references/typescript/src/components/sandbox/SandboxConfigTab.tsx) |
| `references/typescript/src/components/sandbox/SandboxDependenciesTab.tsx` | [`references/typescript/src/components/sandbox/SandboxDependenciesTab.tsx`](references/typescript/src/components/sandbox/SandboxDependenciesTab.tsx) |
| `references/typescript/src/components/sandbox/SandboxOverridesTab.tsx` | [`references/typescript/src/components/sandbox/SandboxOverridesTab.tsx`](references/typescript/src/components/sandbox/SandboxOverridesTab.tsx) |
| `references/typescript/src/components/sandbox/SandboxDoctorSection.tsx` | [`references/typescript/src/components/sandbox/SandboxDoctorSection.tsx`](references/typescript/src/components/sandbox/SandboxDoctorSection.tsx) |
| `references/typescript/src/components/sandbox/SandboxSettings.tsx` | [`references/typescript/src/components/sandbox/SandboxSettings.tsx`](references/typescript/src/components/sandbox/SandboxSettings.tsx) |
| `references/typescript/src/components/messages/CollapsedReadSearchContent.tsx` | [`references/typescript/src/components/messages/CollapsedReadSearchContent.tsx`](references/typescript/src/components/messages/CollapsedReadSearchContent.tsx) |
| `references/typescript/src/components/messages/ShutdownMessage.tsx` | [`references/typescript/src/components/messages/ShutdownMessage.tsx`](references/typescript/src/components/messages/ShutdownMessage.tsx) |
| `references/typescript/src/components/messages/UserResourceUpdateMessage.tsx` | [`references/typescript/src/components/messages/UserResourceUpdateMessage.tsx`](references/typescript/src/components/messages/UserResourceUpdateMessage.tsx) |
| `references/typescript/src/components/messages/AssistantTextMessage.tsx` | [`references/typescript/src/components/messages/AssistantTextMessage.tsx`](references/typescript/src/components/messages/AssistantTextMessage.tsx) |
| `references/typescript/src/components/messages/UserToolResultMessage/UserToolSuccessMessage.tsx` | [`references/typescript/src/components/messages/UserToolResultMessage/UserToolSuccessMessage.tsx`](references/typescript/src/components/messages/UserToolResultMessage/UserToolSuccessMessage.tsx) |
| `references/typescript/src/components/messages/UserToolResultMessage/RejectedToolUseMessage.tsx` | [`references/typescript/src/components/messages/UserToolResultMessage/RejectedToolUseMessage.tsx`](references/typescript/src/components/messages/UserToolResultMessage/RejectedToolUseMessage.tsx) |
| `references/typescript/src/components/messages/UserToolResultMessage/UserToolResultMessage.tsx` | [`references/typescript/src/components/messages/UserToolResultMessage/UserToolResultMessage.tsx`](references/typescript/src/components/messages/UserToolResultMessage/UserToolResultMessage.tsx) |
| `references/typescript/src/components/messages/UserToolResultMessage/UserToolRejectMessage.tsx` | [`references/typescript/src/components/messages/UserToolResultMessage/UserToolRejectMessage.tsx`](references/typescript/src/components/messages/UserToolResultMessage/UserToolRejectMessage.tsx) |
| `references/typescript/src/components/messages/UserToolResultMessage/RejectedPlanMessage.tsx` | [`references/typescript/src/components/messages/UserToolResultMessage/RejectedPlanMessage.tsx`](references/typescript/src/components/messages/UserToolResultMessage/RejectedPlanMessage.tsx) |
| `references/typescript/src/components/messages/UserToolResultMessage/UserToolCanceledMessage.tsx` | [`references/typescript/src/components/messages/UserToolResultMessage/UserToolCanceledMessage.tsx`](references/typescript/src/components/messages/UserToolResultMessage/UserToolCanceledMessage.tsx) |
| `references/typescript/src/components/messages/UserToolResultMessage/utils.tsx` | [`references/typescript/src/components/messages/UserToolResultMessage/utils.tsx`](references/typescript/src/components/messages/UserToolResultMessage/utils.tsx) |
| `references/typescript/src/components/messages/UserToolResultMessage/UserToolErrorMessage.tsx` | [`references/typescript/src/components/messages/UserToolResultMessage/UserToolErrorMessage.tsx`](references/typescript/src/components/messages/UserToolResultMessage/UserToolErrorMessage.tsx) |
| `references/typescript/src/components/messages/HighlightedThinkingText.tsx` | [`references/typescript/src/components/messages/HighlightedThinkingText.tsx`](references/typescript/src/components/messages/HighlightedThinkingText.tsx) |
| `references/typescript/src/components/messages/UserBashOutputMessage.tsx` | [`references/typescript/src/components/messages/UserBashOutputMessage.tsx`](references/typescript/src/components/messages/UserBashOutputMessage.tsx) |
| `references/typescript/src/components/messages/AttachmentMessage.tsx` | [`references/typescript/src/components/messages/AttachmentMessage.tsx`](references/typescript/src/components/messages/AttachmentMessage.tsx) |
| `references/typescript/src/components/messages/UserAgentNotificationMessage.tsx` | [`references/typescript/src/components/messages/UserAgentNotificationMessage.tsx`](references/typescript/src/components/messages/UserAgentNotificationMessage.tsx) |
| `references/typescript/src/components/messages/UserPlanMessage.tsx` | [`references/typescript/src/components/messages/UserPlanMessage.tsx`](references/typescript/src/components/messages/UserPlanMessage.tsx) |
| `references/typescript/src/components/messages/teamMemCollapsed.tsx` | [`references/typescript/src/components/messages/teamMemCollapsed.tsx`](references/typescript/src/components/messages/teamMemCollapsed.tsx) |
| `references/typescript/src/components/messages/UserTeammateMessage.tsx` | [`references/typescript/src/components/messages/UserTeammateMessage.tsx`](references/typescript/src/components/messages/UserTeammateMessage.tsx) |
| `references/typescript/src/components/messages/GroupedToolUseContent.tsx` | [`references/typescript/src/components/messages/GroupedToolUseContent.tsx`](references/typescript/src/components/messages/GroupedToolUseContent.tsx) |
| `references/typescript/src/components/messages/UserImageMessage.tsx` | [`references/typescript/src/components/messages/UserImageMessage.tsx`](references/typescript/src/components/messages/UserImageMessage.tsx) |
| `references/typescript/src/components/messages/AssistantRedactedThinkingMessage.tsx` | [`references/typescript/src/components/messages/AssistantRedactedThinkingMessage.tsx`](references/typescript/src/components/messages/AssistantRedactedThinkingMessage.tsx) |
| `references/typescript/src/components/messages/CompactBoundaryMessage.tsx` | [`references/typescript/src/components/messages/CompactBoundaryMessage.tsx`](references/typescript/src/components/messages/CompactBoundaryMessage.tsx) |
| `references/typescript/src/components/messages/HookProgressMessage.tsx` | [`references/typescript/src/components/messages/HookProgressMessage.tsx`](references/typescript/src/components/messages/HookProgressMessage.tsx) |
| `references/typescript/src/components/messages/SystemTextMessage.tsx` | [`references/typescript/src/components/messages/SystemTextMessage.tsx`](references/typescript/src/components/messages/SystemTextMessage.tsx) |
| `references/typescript/src/components/messages/UserMemoryInputMessage.tsx` | [`references/typescript/src/components/messages/UserMemoryInputMessage.tsx`](references/typescript/src/components/messages/UserMemoryInputMessage.tsx) |
| `references/typescript/src/components/messages/UserCommandMessage.tsx` | [`references/typescript/src/components/messages/UserCommandMessage.tsx`](references/typescript/src/components/messages/UserCommandMessage.tsx) |
| `references/typescript/src/components/messages/AssistantThinkingMessage.tsx` | [`references/typescript/src/components/messages/AssistantThinkingMessage.tsx`](references/typescript/src/components/messages/AssistantThinkingMessage.tsx) |
| `references/typescript/src/components/messages/UserBashInputMessage.tsx` | [`references/typescript/src/components/messages/UserBashInputMessage.tsx`](references/typescript/src/components/messages/UserBashInputMessage.tsx) |
| `references/typescript/src/components/messages/SystemAPIErrorMessage.tsx` | [`references/typescript/src/components/messages/SystemAPIErrorMessage.tsx`](references/typescript/src/components/messages/SystemAPIErrorMessage.tsx) |
| `references/typescript/src/components/messages/UserPromptMessage.tsx` | [`references/typescript/src/components/messages/UserPromptMessage.tsx`](references/typescript/src/components/messages/UserPromptMessage.tsx) |
| `references/typescript/src/components/messages/AssistantToolUseMessage.tsx` | [`references/typescript/src/components/messages/AssistantToolUseMessage.tsx`](references/typescript/src/components/messages/AssistantToolUseMessage.tsx) |
| `references/typescript/src/components/messages/TaskAssignmentMessage.tsx` | [`references/typescript/src/components/messages/TaskAssignmentMessage.tsx`](references/typescript/src/components/messages/TaskAssignmentMessage.tsx) |
| `references/typescript/src/components/messages/UserChannelMessage.tsx` | [`references/typescript/src/components/messages/UserChannelMessage.tsx`](references/typescript/src/components/messages/UserChannelMessage.tsx) |
| `references/typescript/src/components/messages/teamMemSaved.ts` | [`references/typescript/src/components/messages/teamMemSaved.ts`](references/typescript/src/components/messages/teamMemSaved.ts) |
| `references/typescript/src/components/messages/RateLimitMessage.tsx` | [`references/typescript/src/components/messages/RateLimitMessage.tsx`](references/typescript/src/components/messages/RateLimitMessage.tsx) |
| `references/typescript/src/components/messages/nullRenderingAttachments.ts` | [`references/typescript/src/components/messages/nullRenderingAttachments.ts`](references/typescript/src/components/messages/nullRenderingAttachments.ts) |
| `references/typescript/src/components/messages/PlanApprovalMessage.tsx` | [`references/typescript/src/components/messages/PlanApprovalMessage.tsx`](references/typescript/src/components/messages/PlanApprovalMessage.tsx) |
| `references/typescript/src/components/messages/AdvisorMessage.tsx` | [`references/typescript/src/components/messages/AdvisorMessage.tsx`](references/typescript/src/components/messages/AdvisorMessage.tsx) |
| `references/typescript/src/components/messages/UserLocalCommandOutputMessage.tsx` | [`references/typescript/src/components/messages/UserLocalCommandOutputMessage.tsx`](references/typescript/src/components/messages/UserLocalCommandOutputMessage.tsx) |
| `references/typescript/src/components/messages/UserTextMessage.tsx` | [`references/typescript/src/components/messages/UserTextMessage.tsx`](references/typescript/src/components/messages/UserTextMessage.tsx) |
| `references/typescript/src/components/FilePathLink.tsx` | [`references/typescript/src/components/FilePathLink.tsx`](references/typescript/src/components/FilePathLink.tsx) |
| `references/typescript/src/components/DesktopUpsell/DesktopUpsellStartup.tsx` | [`references/typescript/src/components/DesktopUpsell/DesktopUpsellStartup.tsx`](references/typescript/src/components/DesktopUpsell/DesktopUpsellStartup.tsx) |
| `references/typescript/src/components/memory/MemoryFileSelector.tsx` | [`references/typescript/src/components/memory/MemoryFileSelector.tsx`](references/typescript/src/components/memory/MemoryFileSelector.tsx) |
| `references/typescript/src/components/memory/MemoryUpdateNotification.tsx` | [`references/typescript/src/components/memory/MemoryUpdateNotification.tsx`](references/typescript/src/components/memory/MemoryUpdateNotification.tsx) |
| `references/typescript/src/components/UltraplanChoiceDialog.tsx` | [`references/typescript/src/components/UltraplanChoiceDialog.tsx`](references/typescript/src/components/UltraplanChoiceDialog.tsx) |
| `references/typescript/src/components/KeybindingWarnings.tsx` | [`references/typescript/src/components/KeybindingWarnings.tsx`](references/typescript/src/components/KeybindingWarnings.tsx) |
| `references/typescript/src/components/agents/ModelSelector.tsx` | [`references/typescript/src/components/agents/ModelSelector.tsx`](references/typescript/src/components/agents/ModelSelector.tsx) |
| `references/typescript/src/components/agents/AgentNavigationFooter.tsx` | [`references/typescript/src/components/agents/AgentNavigationFooter.tsx`](references/typescript/src/components/agents/AgentNavigationFooter.tsx) |
| `references/typescript/src/components/agents/agentFileUtils.ts` | [`references/typescript/src/components/agents/agentFileUtils.ts`](references/typescript/src/components/agents/agentFileUtils.ts) |
| `references/typescript/src/components/agents/ColorPicker.tsx` | [`references/typescript/src/components/agents/ColorPicker.tsx`](references/typescript/src/components/agents/ColorPicker.tsx) |
| `references/typescript/src/components/agents/generateAgent.ts` | [`references/typescript/src/components/agents/generateAgent.ts`](references/typescript/src/components/agents/generateAgent.ts) |
| `references/typescript/src/components/agents/types.ts` | [`references/typescript/src/components/agents/types.ts`](references/typescript/src/components/agents/types.ts) |
| `references/typescript/src/components/agents/SnapshotUpdateDialog.tsx` | [`references/typescript/src/components/agents/SnapshotUpdateDialog.tsx`](references/typescript/src/components/agents/SnapshotUpdateDialog.tsx) |
| `references/typescript/src/components/agents/AgentDetail.tsx` | [`references/typescript/src/components/agents/AgentDetail.tsx`](references/typescript/src/components/agents/AgentDetail.tsx) |
| `references/typescript/src/components/agents/utils.ts` | [`references/typescript/src/components/agents/utils.ts`](references/typescript/src/components/agents/utils.ts) |
| `references/typescript/src/components/agents/AgentsList.tsx` | [`references/typescript/src/components/agents/AgentsList.tsx`](references/typescript/src/components/agents/AgentsList.tsx) |
| `references/typescript/src/components/agents/new-agent-creation/wizard-steps/ToolsStep.tsx` | [`references/typescript/src/components/agents/new-agent-creation/wizard-steps/ToolsStep.tsx`](references/typescript/src/components/agents/new-agent-creation/wizard-steps/ToolsStep.tsx) |
| `references/typescript/src/components/agents/new-agent-creation/wizard-steps/ConfirmStep.tsx` | [`references/typescript/src/components/agents/new-agent-creation/wizard-steps/ConfirmStep.tsx`](references/typescript/src/components/agents/new-agent-creation/wizard-steps/ConfirmStep.tsx) |
| `references/typescript/src/components/agents/new-agent-creation/wizard-steps/LocationStep.tsx` | [`references/typescript/src/components/agents/new-agent-creation/wizard-steps/LocationStep.tsx`](references/typescript/src/components/agents/new-agent-creation/wizard-steps/LocationStep.tsx) |
| `references/typescript/src/components/agents/new-agent-creation/wizard-steps/MethodStep.tsx` | [`references/typescript/src/components/agents/new-agent-creation/wizard-steps/MethodStep.tsx`](references/typescript/src/components/agents/new-agent-creation/wizard-steps/MethodStep.tsx) |
| `references/typescript/src/components/agents/new-agent-creation/wizard-steps/ColorStep.tsx` | [`references/typescript/src/components/agents/new-agent-creation/wizard-steps/ColorStep.tsx`](references/typescript/src/components/agents/new-agent-creation/wizard-steps/ColorStep.tsx) |
| `references/typescript/src/components/agents/new-agent-creation/wizard-steps/PromptStep.tsx` | [`references/typescript/src/components/agents/new-agent-creation/wizard-steps/PromptStep.tsx`](references/typescript/src/components/agents/new-agent-creation/wizard-steps/PromptStep.tsx) |
| `references/typescript/src/components/agents/new-agent-creation/wizard-steps/TypeStep.tsx` | [`references/typescript/src/components/agents/new-agent-creation/wizard-steps/TypeStep.tsx`](references/typescript/src/components/agents/new-agent-creation/wizard-steps/TypeStep.tsx) |
| `references/typescript/src/components/agents/new-agent-creation/wizard-steps/ConfirmStepWrapper.tsx` | [`references/typescript/src/components/agents/new-agent-creation/wizard-steps/ConfirmStepWrapper.tsx`](references/typescript/src/components/agents/new-agent-creation/wizard-steps/ConfirmStepWrapper.tsx) |
| `references/typescript/src/components/agents/new-agent-creation/wizard-steps/ModelStep.tsx` | [`references/typescript/src/components/agents/new-agent-creation/wizard-steps/ModelStep.tsx`](references/typescript/src/components/agents/new-agent-creation/wizard-steps/ModelStep.tsx) |
| `references/typescript/src/components/agents/new-agent-creation/wizard-steps/DescriptionStep.tsx` | [`references/typescript/src/components/agents/new-agent-creation/wizard-steps/DescriptionStep.tsx`](references/typescript/src/components/agents/new-agent-creation/wizard-steps/DescriptionStep.tsx) |
| `references/typescript/src/components/agents/new-agent-creation/wizard-steps/GenerateStep.tsx` | [`references/typescript/src/components/agents/new-agent-creation/wizard-steps/GenerateStep.tsx`](references/typescript/src/components/agents/new-agent-creation/wizard-steps/GenerateStep.tsx) |
| `references/typescript/src/components/agents/new-agent-creation/wizard-steps/MemoryStep.tsx` | [`references/typescript/src/components/agents/new-agent-creation/wizard-steps/MemoryStep.tsx`](references/typescript/src/components/agents/new-agent-creation/wizard-steps/MemoryStep.tsx) |
| `references/typescript/src/components/agents/new-agent-creation/CreateAgentWizard.tsx` | [`references/typescript/src/components/agents/new-agent-creation/CreateAgentWizard.tsx`](references/typescript/src/components/agents/new-agent-creation/CreateAgentWizard.tsx) |
| `references/typescript/src/components/agents/validateAgent.ts` | [`references/typescript/src/components/agents/validateAgent.ts`](references/typescript/src/components/agents/validateAgent.ts) |
| `references/typescript/src/components/agents/ToolSelector.tsx` | [`references/typescript/src/components/agents/ToolSelector.tsx`](references/typescript/src/components/agents/ToolSelector.tsx) |
| `references/typescript/src/components/agents/AgentsMenu.tsx` | [`references/typescript/src/components/agents/AgentsMenu.tsx`](references/typescript/src/components/agents/AgentsMenu.tsx) |
| `references/typescript/src/components/agents/AgentEditor.tsx` | [`references/typescript/src/components/agents/AgentEditor.tsx`](references/typescript/src/components/agents/AgentEditor.tsx) |
| `references/typescript/src/components/ExitFlow.tsx` | [`references/typescript/src/components/ExitFlow.tsx`](references/typescript/src/components/ExitFlow.tsx) |
| `references/typescript/src/components/ToolUseLoader.tsx` | [`references/typescript/src/components/ToolUseLoader.tsx`](references/typescript/src/components/ToolUseLoader.tsx) |
| `references/typescript/src/components/MCPServerDialogCopy.tsx` | [`references/typescript/src/components/MCPServerDialogCopy.tsx`](references/typescript/src/components/MCPServerDialogCopy.tsx) |
| `references/typescript/src/components/FeedbackSurvey/useSurveyState.tsx` | [`references/typescript/src/components/FeedbackSurvey/useSurveyState.tsx`](references/typescript/src/components/FeedbackSurvey/useSurveyState.tsx) |
| `references/typescript/src/components/FeedbackSurvey/FeedbackSurveyView.tsx` | [`references/typescript/src/components/FeedbackSurvey/FeedbackSurveyView.tsx`](references/typescript/src/components/FeedbackSurvey/FeedbackSurveyView.tsx) |
| `references/typescript/src/components/FeedbackSurvey/TranscriptSharePrompt.tsx` | [`references/typescript/src/components/FeedbackSurvey/TranscriptSharePrompt.tsx`](references/typescript/src/components/FeedbackSurvey/TranscriptSharePrompt.tsx) |
| `references/typescript/src/components/FeedbackSurvey/useMemorySurvey.tsx` | [`references/typescript/src/components/FeedbackSurvey/useMemorySurvey.tsx`](references/typescript/src/components/FeedbackSurvey/useMemorySurvey.tsx) |
| `references/typescript/src/components/FeedbackSurvey/submitTranscriptShare.ts` | [`references/typescript/src/components/FeedbackSurvey/submitTranscriptShare.ts`](references/typescript/src/components/FeedbackSurvey/submitTranscriptShare.ts) |
| `references/typescript/src/components/FeedbackSurvey/FeedbackSurvey.tsx` | [`references/typescript/src/components/FeedbackSurvey/FeedbackSurvey.tsx`](references/typescript/src/components/FeedbackSurvey/FeedbackSurvey.tsx) |
| `references/typescript/src/components/FeedbackSurvey/useDebouncedDigitInput.ts` | [`references/typescript/src/components/FeedbackSurvey/useDebouncedDigitInput.ts`](references/typescript/src/components/FeedbackSurvey/useDebouncedDigitInput.ts) |
| `references/typescript/src/components/FeedbackSurvey/usePostCompactSurvey.tsx` | [`references/typescript/src/components/FeedbackSurvey/usePostCompactSurvey.tsx`](references/typescript/src/components/FeedbackSurvey/usePostCompactSurvey.tsx) |
| `references/typescript/src/components/FeedbackSurvey/useFeedbackSurvey.tsx` | [`references/typescript/src/components/FeedbackSurvey/useFeedbackSurvey.tsx`](references/typescript/src/components/FeedbackSurvey/useFeedbackSurvey.tsx) |
| `references/typescript/src/components/AgentProgressLine.tsx` | [`references/typescript/src/components/AgentProgressLine.tsx`](references/typescript/src/components/AgentProgressLine.tsx) |
| `references/typescript/src/components/ClaudeCodeHint/PluginHintMenu.tsx` | [`references/typescript/src/components/ClaudeCodeHint/PluginHintMenu.tsx`](references/typescript/src/components/ClaudeCodeHint/PluginHintMenu.tsx) |
| `references/typescript/src/components/LanguagePicker.tsx` | [`references/typescript/src/components/LanguagePicker.tsx`](references/typescript/src/components/LanguagePicker.tsx) |
| `references/typescript/src/components/ModelPicker.tsx` | [`references/typescript/src/components/ModelPicker.tsx`](references/typescript/src/components/ModelPicker.tsx) |
| `references/typescript/src/components/EffortIndicator.ts` | [`references/typescript/src/components/EffortIndicator.ts`](references/typescript/src/components/EffortIndicator.ts) |
| `references/typescript/src/components/StatusLine.tsx` | [`references/typescript/src/components/StatusLine.tsx`](references/typescript/src/components/StatusLine.tsx) |
| `references/typescript/src/components/SearchBox.tsx` | [`references/typescript/src/components/SearchBox.tsx`](references/typescript/src/components/SearchBox.tsx) |
| `references/typescript/src/components/IdeAutoConnectDialog.tsx` | [`references/typescript/src/components/IdeAutoConnectDialog.tsx`](references/typescript/src/components/IdeAutoConnectDialog.tsx) |
| `references/typescript/src/components/SentryErrorBoundary.ts` | [`references/typescript/src/components/SentryErrorBoundary.ts`](references/typescript/src/components/SentryErrorBoundary.ts) |
| `references/typescript/src/components/SandboxViolationExpandedView.tsx` | [`references/typescript/src/components/SandboxViolationExpandedView.tsx`](references/typescript/src/components/SandboxViolationExpandedView.tsx) |
| `references/typescript/src/components/MCPServerApprovalDialog.tsx` | [`references/typescript/src/components/MCPServerApprovalDialog.tsx`](references/typescript/src/components/MCPServerApprovalDialog.tsx) |
| `references/typescript/src/components/ui/OrderedList.tsx` | [`references/typescript/src/components/ui/OrderedList.tsx`](references/typescript/src/components/ui/OrderedList.tsx) |
| `references/typescript/src/components/ui/TreeSelect.tsx` | [`references/typescript/src/components/ui/TreeSelect.tsx`](references/typescript/src/components/ui/TreeSelect.tsx) |
| `references/typescript/src/components/ui/OrderedListItem.tsx` | [`references/typescript/src/components/ui/OrderedListItem.tsx`](references/typescript/src/components/ui/OrderedListItem.tsx) |
| `references/typescript/src/components/HistorySearchDialog.tsx` | [`references/typescript/src/components/HistorySearchDialog.tsx`](references/typescript/src/components/HistorySearchDialog.tsx) |
| `references/typescript/src/components/MemoryUsageIndicator.tsx` | [`references/typescript/src/components/MemoryUsageIndicator.tsx`](references/typescript/src/components/MemoryUsageIndicator.tsx) |
| `references/typescript/src/components/InterruptedByUser.tsx` | [`references/typescript/src/components/InterruptedByUser.tsx`](references/typescript/src/components/InterruptedByUser.tsx) |
| `references/typescript/src/components/grove/Grove.tsx` | [`references/typescript/src/components/grove/Grove.tsx`](references/typescript/src/components/grove/Grove.tsx) |
| `references/typescript/src/components/MessageResponse.tsx` | [`references/typescript/src/components/MessageResponse.tsx`](references/typescript/src/components/MessageResponse.tsx) |
| `references/typescript/src/components/SessionBackgroundHint.tsx` | [`references/typescript/src/components/SessionBackgroundHint.tsx`](references/typescript/src/components/SessionBackgroundHint.tsx) |
| `references/typescript/src/components/PackageManagerAutoUpdater.tsx` | [`references/typescript/src/components/PackageManagerAutoUpdater.tsx`](references/typescript/src/components/PackageManagerAutoUpdater.tsx) |
| `references/typescript/src/components/InvalidConfigDialog.tsx` | [`references/typescript/src/components/InvalidConfigDialog.tsx`](references/typescript/src/components/InvalidConfigDialog.tsx) |
| `references/typescript/src/components/InvalidSettingsDialog.tsx` | [`references/typescript/src/components/InvalidSettingsDialog.tsx`](references/typescript/src/components/InvalidSettingsDialog.tsx) |
| `references/typescript/src/components/ThemePicker.tsx` | [`references/typescript/src/components/ThemePicker.tsx`](references/typescript/src/components/ThemePicker.tsx) |
| `references/typescript/src/components/GlobalSearchDialog.tsx` | [`references/typescript/src/components/GlobalSearchDialog.tsx`](references/typescript/src/components/GlobalSearchDialog.tsx) |
| `references/typescript/src/components/teams/TeamsDialog.tsx` | [`references/typescript/src/components/teams/TeamsDialog.tsx`](references/typescript/src/components/teams/TeamsDialog.tsx) |
| `references/typescript/src/components/teams/TeamStatus.tsx` | [`references/typescript/src/components/teams/TeamStatus.tsx`](references/typescript/src/components/teams/TeamStatus.tsx) |
| `references/typescript/src/components/MCPServerMultiselectDialog.tsx` | [`references/typescript/src/components/MCPServerMultiselectDialog.tsx`](references/typescript/src/components/MCPServerMultiselectDialog.tsx) |
| `references/typescript/src/components/VirtualMessageList.tsx` | [`references/typescript/src/components/VirtualMessageList.tsx`](references/typescript/src/components/VirtualMessageList.tsx) |
| `references/typescript/src/components/Spinner/FlashingChar.tsx` | [`references/typescript/src/components/Spinner/FlashingChar.tsx`](references/typescript/src/components/Spinner/FlashingChar.tsx) |
| `references/typescript/src/components/Spinner/TeammateSpinnerLine.tsx` | [`references/typescript/src/components/Spinner/TeammateSpinnerLine.tsx`](references/typescript/src/components/Spinner/TeammateSpinnerLine.tsx) |
| `references/typescript/src/components/Spinner/useStalledAnimation.ts` | [`references/typescript/src/components/Spinner/useStalledAnimation.ts`](references/typescript/src/components/Spinner/useStalledAnimation.ts) |
| `references/typescript/src/components/Spinner/teammateSelectHint.ts` | [`references/typescript/src/components/Spinner/teammateSelectHint.ts`](references/typescript/src/components/Spinner/teammateSelectHint.ts) |
| `references/typescript/src/components/Spinner/SpinnerAnimationRow.tsx` | [`references/typescript/src/components/Spinner/SpinnerAnimationRow.tsx`](references/typescript/src/components/Spinner/SpinnerAnimationRow.tsx) |
| `references/typescript/src/components/Spinner/utils.ts` | [`references/typescript/src/components/Spinner/utils.ts`](references/typescript/src/components/Spinner/utils.ts) |
| `references/typescript/src/components/Spinner/useShimmerAnimation.ts` | [`references/typescript/src/components/Spinner/useShimmerAnimation.ts`](references/typescript/src/components/Spinner/useShimmerAnimation.ts) |
| `references/typescript/src/components/Spinner/index.ts` | [`references/typescript/src/components/Spinner/index.ts`](references/typescript/src/components/Spinner/index.ts) |
| `references/typescript/src/components/Spinner/TeammateSpinnerTree.tsx` | [`references/typescript/src/components/Spinner/TeammateSpinnerTree.tsx`](references/typescript/src/components/Spinner/TeammateSpinnerTree.tsx) |
| `references/typescript/src/components/Spinner/GlimmerMessage.tsx` | [`references/typescript/src/components/Spinner/GlimmerMessage.tsx`](references/typescript/src/components/Spinner/GlimmerMessage.tsx) |
| `references/typescript/src/components/Spinner/SpinnerGlyph.tsx` | [`references/typescript/src/components/Spinner/SpinnerGlyph.tsx`](references/typescript/src/components/Spinner/SpinnerGlyph.tsx) |
| `references/typescript/src/components/Spinner/ShimmerChar.tsx` | [`references/typescript/src/components/Spinner/ShimmerChar.tsx`](references/typescript/src/components/Spinner/ShimmerChar.tsx) |
| `references/typescript/src/components/CostThresholdDialog.tsx` | [`references/typescript/src/components/CostThresholdDialog.tsx`](references/typescript/src/components/CostThresholdDialog.tsx) |
| `references/typescript/src/components/Passes/Passes.tsx` | [`references/typescript/src/components/Passes/Passes.tsx`](references/typescript/src/components/Passes/Passes.tsx) |
| `references/typescript/src/components/FileEditToolUseRejectedMessage.tsx` | [`references/typescript/src/components/FileEditToolUseRejectedMessage.tsx`](references/typescript/src/components/FileEditToolUseRejectedMessage.tsx) |
| `references/typescript/src/components/PromptInput/PromptInputFooter.tsx` | [`references/typescript/src/components/PromptInput/PromptInputFooter.tsx`](references/typescript/src/components/PromptInput/PromptInputFooter.tsx) |
| `references/typescript/src/components/PromptInput/PromptInputModeIndicator.tsx` | [`references/typescript/src/components/PromptInput/PromptInputModeIndicator.tsx`](references/typescript/src/components/PromptInput/PromptInputModeIndicator.tsx) |
| `references/typescript/src/components/PromptInput/SandboxPromptFooterHint.tsx` | [`references/typescript/src/components/PromptInput/SandboxPromptFooterHint.tsx`](references/typescript/src/components/PromptInput/SandboxPromptFooterHint.tsx) |
| `references/typescript/src/components/PromptInput/useMaybeTruncateInput.ts` | [`references/typescript/src/components/PromptInput/useMaybeTruncateInput.ts`](references/typescript/src/components/PromptInput/useMaybeTruncateInput.ts) |
| `references/typescript/src/components/PromptInput/Notifications.tsx` | [`references/typescript/src/components/PromptInput/Notifications.tsx`](references/typescript/src/components/PromptInput/Notifications.tsx) |
| `references/typescript/src/components/PromptInput/HistorySearchInput.tsx` | [`references/typescript/src/components/PromptInput/HistorySearchInput.tsx`](references/typescript/src/components/PromptInput/HistorySearchInput.tsx) |
| `references/typescript/src/components/PromptInput/PromptInputHelpMenu.tsx` | [`references/typescript/src/components/PromptInput/PromptInputHelpMenu.tsx`](references/typescript/src/components/PromptInput/PromptInputHelpMenu.tsx) |
| `references/typescript/src/components/PromptInput/IssueFlagBanner.tsx` | [`references/typescript/src/components/PromptInput/IssueFlagBanner.tsx`](references/typescript/src/components/PromptInput/IssueFlagBanner.tsx) |
| `references/typescript/src/components/PromptInput/useShowFastIconHint.ts` | [`references/typescript/src/components/PromptInput/useShowFastIconHint.ts`](references/typescript/src/components/PromptInput/useShowFastIconHint.ts) |
| `references/typescript/src/components/PromptInput/useSwarmBanner.ts` | [`references/typescript/src/components/PromptInput/useSwarmBanner.ts`](references/typescript/src/components/PromptInput/useSwarmBanner.ts) |
| `references/typescript/src/components/PromptInput/VoiceIndicator.tsx` | [`references/typescript/src/components/PromptInput/VoiceIndicator.tsx`](references/typescript/src/components/PromptInput/VoiceIndicator.tsx) |
| `references/typescript/src/components/PromptInput/inputPaste.ts` | [`references/typescript/src/components/PromptInput/inputPaste.ts`](references/typescript/src/components/PromptInput/inputPaste.ts) |
| `references/typescript/src/components/PromptInput/PromptInputStashNotice.tsx` | [`references/typescript/src/components/PromptInput/PromptInputStashNotice.tsx`](references/typescript/src/components/PromptInput/PromptInputStashNotice.tsx) |
| `references/typescript/src/components/PromptInput/utils.ts` | [`references/typescript/src/components/PromptInput/utils.ts`](references/typescript/src/components/PromptInput/utils.ts) |
| `references/typescript/src/components/PromptInput/usePromptInputPlaceholder.ts` | [`references/typescript/src/components/PromptInput/usePromptInputPlaceholder.ts`](references/typescript/src/components/PromptInput/usePromptInputPlaceholder.ts) |
| `references/typescript/src/components/PromptInput/PromptInput.tsx` | [`references/typescript/src/components/PromptInput/PromptInput.tsx`](references/typescript/src/components/PromptInput/PromptInput.tsx) |
| `references/typescript/src/components/PromptInput/inputModes.ts` | [`references/typescript/src/components/PromptInput/inputModes.ts`](references/typescript/src/components/PromptInput/inputModes.ts) |
| `references/typescript/src/components/PromptInput/ShimmeredInput.tsx` | [`references/typescript/src/components/PromptInput/ShimmeredInput.tsx`](references/typescript/src/components/PromptInput/ShimmeredInput.tsx) |
| `references/typescript/src/components/PromptInput/PromptInputQueuedCommands.tsx` | [`references/typescript/src/components/PromptInput/PromptInputQueuedCommands.tsx`](references/typescript/src/components/PromptInput/PromptInputQueuedCommands.tsx) |
| `references/typescript/src/components/PromptInput/PromptInputFooterSuggestions.tsx` | [`references/typescript/src/components/PromptInput/PromptInputFooterSuggestions.tsx`](references/typescript/src/components/PromptInput/PromptInputFooterSuggestions.tsx) |
| `references/typescript/src/components/PromptInput/PromptInputFooterLeftSide.tsx` | [`references/typescript/src/components/PromptInput/PromptInputFooterLeftSide.tsx`](references/typescript/src/components/PromptInput/PromptInputFooterLeftSide.tsx) |
| `references/typescript/src/components/MCPServerDesktopImportDialog.tsx` | [`references/typescript/src/components/MCPServerDesktopImportDialog.tsx`](references/typescript/src/components/MCPServerDesktopImportDialog.tsx) |
| `references/typescript/src/components/mcp/utils/reconnectHelpers.tsx` | [`references/typescript/src/components/mcp/utils/reconnectHelpers.tsx`](references/typescript/src/components/mcp/utils/reconnectHelpers.tsx) |
| `references/typescript/src/components/mcp/MCPToolListView.tsx` | [`references/typescript/src/components/mcp/MCPToolListView.tsx`](references/typescript/src/components/mcp/MCPToolListView.tsx) |
| `references/typescript/src/components/mcp/MCPSettings.tsx` | [`references/typescript/src/components/mcp/MCPSettings.tsx`](references/typescript/src/components/mcp/MCPSettings.tsx) |
| `references/typescript/src/components/mcp/MCPListPanel.tsx` | [`references/typescript/src/components/mcp/MCPListPanel.tsx`](references/typescript/src/components/mcp/MCPListPanel.tsx) |
| `references/typescript/src/components/mcp/MCPRemoteServerMenu.tsx` | [`references/typescript/src/components/mcp/MCPRemoteServerMenu.tsx`](references/typescript/src/components/mcp/MCPRemoteServerMenu.tsx) |
| `references/typescript/src/components/mcp/MCPAgentServerMenu.tsx` | [`references/typescript/src/components/mcp/MCPAgentServerMenu.tsx`](references/typescript/src/components/mcp/MCPAgentServerMenu.tsx) |
| `references/typescript/src/components/mcp/McpParsingWarnings.tsx` | [`references/typescript/src/components/mcp/McpParsingWarnings.tsx`](references/typescript/src/components/mcp/McpParsingWarnings.tsx) |
| `references/typescript/src/components/mcp/CapabilitiesSection.tsx` | [`references/typescript/src/components/mcp/CapabilitiesSection.tsx`](references/typescript/src/components/mcp/CapabilitiesSection.tsx) |
| `references/typescript/src/components/mcp/ElicitationDialog.tsx` | [`references/typescript/src/components/mcp/ElicitationDialog.tsx`](references/typescript/src/components/mcp/ElicitationDialog.tsx) |
| `references/typescript/src/components/mcp/MCPReconnect.tsx` | [`references/typescript/src/components/mcp/MCPReconnect.tsx`](references/typescript/src/components/mcp/MCPReconnect.tsx) |
| `references/typescript/src/components/mcp/index.ts` | [`references/typescript/src/components/mcp/index.ts`](references/typescript/src/components/mcp/index.ts) |
| `references/typescript/src/components/mcp/MCPToolDetailView.tsx` | [`references/typescript/src/components/mcp/MCPToolDetailView.tsx`](references/typescript/src/components/mcp/MCPToolDetailView.tsx) |
| `references/typescript/src/components/mcp/MCPStdioServerMenu.tsx` | [`references/typescript/src/components/mcp/MCPStdioServerMenu.tsx`](references/typescript/src/components/mcp/MCPStdioServerMenu.tsx) |
| `references/typescript/src/components/PrBadge.tsx` | [`references/typescript/src/components/PrBadge.tsx`](references/typescript/src/components/PrBadge.tsx) |
| `references/typescript/src/components/PressEnterToContinue.tsx` | [`references/typescript/src/components/PressEnterToContinue.tsx`](references/typescript/src/components/PressEnterToContinue.tsx) |
| `references/typescript/src/components/FastIcon.tsx` | [`references/typescript/src/components/FastIcon.tsx`](references/typescript/src/components/FastIcon.tsx) |
| `references/typescript/src/components/CtrlOToExpand.tsx` | [`references/typescript/src/components/CtrlOToExpand.tsx`](references/typescript/src/components/CtrlOToExpand.tsx) |
| `references/typescript/src/components/ClaudeMdExternalIncludesDialog.tsx` | [`references/typescript/src/components/ClaudeMdExternalIncludesDialog.tsx`](references/typescript/src/components/ClaudeMdExternalIncludesDialog.tsx) |
| `references/typescript/src/components/DiagnosticsDisplay.tsx` | [`references/typescript/src/components/DiagnosticsDisplay.tsx`](references/typescript/src/components/DiagnosticsDisplay.tsx) |
| `references/typescript/src/components/FileEditToolUpdatedMessage.tsx` | [`references/typescript/src/components/FileEditToolUpdatedMessage.tsx`](references/typescript/src/components/FileEditToolUpdatedMessage.tsx) |
| `references/typescript/src/components/HighlightedCode/Fallback.tsx` | [`references/typescript/src/components/HighlightedCode/Fallback.tsx`](references/typescript/src/components/HighlightedCode/Fallback.tsx) |
| `references/typescript/src/components/SessionPreview.tsx` | [`references/typescript/src/components/SessionPreview.tsx`](references/typescript/src/components/SessionPreview.tsx) |
| `references/typescript/src/components/App.tsx` | [`references/typescript/src/components/App.tsx`](references/typescript/src/components/App.tsx) |
| `references/typescript/src/components/WorkflowMultiselectDialog.tsx` | [`references/typescript/src/components/WorkflowMultiselectDialog.tsx`](references/typescript/src/components/WorkflowMultiselectDialog.tsx) |
| `references/typescript/src/components/VimTextInput.tsx` | [`references/typescript/src/components/VimTextInput.tsx`](references/typescript/src/components/VimTextInput.tsx) |
| `references/typescript/src/components/BaseTextInput.tsx` | [`references/typescript/src/components/BaseTextInput.tsx`](references/typescript/src/components/BaseTextInput.tsx) |
| `references/typescript/src/components/TrustDialog/utils.ts` | [`references/typescript/src/components/TrustDialog/utils.ts`](references/typescript/src/components/TrustDialog/utils.ts) |
| `references/typescript/src/components/TrustDialog/TrustDialog.tsx` | [`references/typescript/src/components/TrustDialog/TrustDialog.tsx`](references/typescript/src/components/TrustDialog/TrustDialog.tsx) |
| `references/typescript/src/components/Onboarding.tsx` | [`references/typescript/src/components/Onboarding.tsx`](references/typescript/src/components/Onboarding.tsx) |
| `references/typescript/src/components/ClickableImageRef.tsx` | [`references/typescript/src/components/ClickableImageRef.tsx`](references/typescript/src/components/ClickableImageRef.tsx) |
| `references/typescript/src/components/IdeStatusIndicator.tsx` | [`references/typescript/src/components/IdeStatusIndicator.tsx`](references/typescript/src/components/IdeStatusIndicator.tsx) |
| `references/typescript/src/components/TeleportProgress.tsx` | [`references/typescript/src/components/TeleportProgress.tsx`](references/typescript/src/components/TeleportProgress.tsx) |
| `references/typescript/src/components/MessageModel.tsx` | [`references/typescript/src/components/MessageModel.tsx`](references/typescript/src/components/MessageModel.tsx) |
| `references/typescript/src/components/TeleportRepoMismatchDialog.tsx` | [`references/typescript/src/components/TeleportRepoMismatchDialog.tsx`](references/typescript/src/components/TeleportRepoMismatchDialog.tsx) |
| `references/typescript/src/components/StructuredDiff.tsx` | [`references/typescript/src/components/StructuredDiff.tsx`](references/typescript/src/components/StructuredDiff.tsx) |
| `references/typescript/src/components/TaskListV2.tsx` | [`references/typescript/src/components/TaskListV2.tsx`](references/typescript/src/components/TaskListV2.tsx) |
| `references/typescript/src/components/BashModeProgress.tsx` | [`references/typescript/src/components/BashModeProgress.tsx`](references/typescript/src/components/BashModeProgress.tsx) |
| `references/typescript/src/components/tasks/RemoteSessionDetailDialog.tsx` | [`references/typescript/src/components/tasks/RemoteSessionDetailDialog.tsx`](references/typescript/src/components/tasks/RemoteSessionDetailDialog.tsx) |
| `references/typescript/src/components/tasks/InProcessTeammateDetailDialog.tsx` | [`references/typescript/src/components/tasks/InProcessTeammateDetailDialog.tsx`](references/typescript/src/components/tasks/InProcessTeammateDetailDialog.tsx) |
| `references/typescript/src/components/tasks/BackgroundTask.tsx` | [`references/typescript/src/components/tasks/BackgroundTask.tsx`](references/typescript/src/components/tasks/BackgroundTask.tsx) |
| `references/typescript/src/components/tasks/RemoteSessionProgress.tsx` | [`references/typescript/src/components/tasks/RemoteSessionProgress.tsx`](references/typescript/src/components/tasks/RemoteSessionProgress.tsx) |
| `references/typescript/src/components/tasks/AsyncAgentDetailDialog.tsx` | [`references/typescript/src/components/tasks/AsyncAgentDetailDialog.tsx`](references/typescript/src/components/tasks/AsyncAgentDetailDialog.tsx) |
| `references/typescript/src/components/tasks/ShellProgress.tsx` | [`references/typescript/src/components/tasks/ShellProgress.tsx`](references/typescript/src/components/tasks/ShellProgress.tsx) |
| `references/typescript/src/components/tasks/DreamDetailDialog.tsx` | [`references/typescript/src/components/tasks/DreamDetailDialog.tsx`](references/typescript/src/components/tasks/DreamDetailDialog.tsx) |
| `references/typescript/src/components/tasks/taskStatusUtils.tsx` | [`references/typescript/src/components/tasks/taskStatusUtils.tsx`](references/typescript/src/components/tasks/taskStatusUtils.tsx) |
| `references/typescript/src/components/tasks/renderToolActivity.tsx` | [`references/typescript/src/components/tasks/renderToolActivity.tsx`](references/typescript/src/components/tasks/renderToolActivity.tsx) |
| `references/typescript/src/components/tasks/BackgroundTasksDialog.tsx` | [`references/typescript/src/components/tasks/BackgroundTasksDialog.tsx`](references/typescript/src/components/tasks/BackgroundTasksDialog.tsx) |
| `references/typescript/src/components/tasks/BackgroundTaskStatus.tsx` | [`references/typescript/src/components/tasks/BackgroundTaskStatus.tsx`](references/typescript/src/components/tasks/BackgroundTaskStatus.tsx) |
| `references/typescript/src/components/tasks/ShellDetailDialog.tsx` | [`references/typescript/src/components/tasks/ShellDetailDialog.tsx`](references/typescript/src/components/tasks/ShellDetailDialog.tsx) |
| `references/typescript/src/components/ClaudeInChromeOnboarding.tsx` | [`references/typescript/src/components/ClaudeInChromeOnboarding.tsx`](references/typescript/src/components/ClaudeInChromeOnboarding.tsx) |
| `references/typescript/src/components/StructuredDiffList.tsx` | [`references/typescript/src/components/StructuredDiffList.tsx`](references/typescript/src/components/StructuredDiffList.tsx) |
| `references/typescript/src/components/RemoteCallout.tsx` | [`references/typescript/src/components/RemoteCallout.tsx`](references/typescript/src/components/RemoteCallout.tsx) |
| `references/typescript/src/components/MessageSelector.tsx` | [`references/typescript/src/components/MessageSelector.tsx`](references/typescript/src/components/MessageSelector.tsx) |
| `references/typescript/src/components/TextInput.tsx` | [`references/typescript/src/components/TextInput.tsx`](references/typescript/src/components/TextInput.tsx) |
| `references/typescript/src/components/Messages.tsx` | [`references/typescript/src/components/Messages.tsx`](references/typescript/src/components/Messages.tsx) |
| `references/typescript/src/components/TeammateViewHeader.tsx` | [`references/typescript/src/components/TeammateViewHeader.tsx`](references/typescript/src/components/TeammateViewHeader.tsx) |
| `references/typescript/src/components/AwsAuthStatusBox.tsx` | [`references/typescript/src/components/AwsAuthStatusBox.tsx`](references/typescript/src/components/AwsAuthStatusBox.tsx) |
| `references/typescript/src/components/ChannelDowngradeDialog.tsx` | [`references/typescript/src/components/ChannelDowngradeDialog.tsx`](references/typescript/src/components/ChannelDowngradeDialog.tsx) |
| `references/typescript/src/components/diff/DiffFileList.tsx` | [`references/typescript/src/components/diff/DiffFileList.tsx`](references/typescript/src/components/diff/DiffFileList.tsx) |
| `references/typescript/src/components/diff/DiffDialog.tsx` | [`references/typescript/src/components/diff/DiffDialog.tsx`](references/typescript/src/components/diff/DiffDialog.tsx) |
| `references/typescript/src/components/diff/DiffDetailView.tsx` | [`references/typescript/src/components/diff/DiffDetailView.tsx`](references/typescript/src/components/diff/DiffDetailView.tsx) |
| `references/typescript/src/components/ApproveApiKey.tsx` | [`references/typescript/src/components/ApproveApiKey.tsx`](references/typescript/src/components/ApproveApiKey.tsx) |
| `references/typescript/src/components/IdleReturnDialog.tsx` | [`references/typescript/src/components/IdleReturnDialog.tsx`](references/typescript/src/components/IdleReturnDialog.tsx) |
| `references/typescript/src/components/UltraplanLaunchDialog.tsx` | [`references/typescript/src/components/UltraplanLaunchDialog.tsx`](references/typescript/src/components/UltraplanLaunchDialog.tsx) |
| `references/typescript/src/components/OutputStylePicker.tsx` | [`references/typescript/src/components/OutputStylePicker.tsx`](references/typescript/src/components/OutputStylePicker.tsx) |
| `references/typescript/src/components/QuickOpenDialog.tsx` | [`references/typescript/src/components/QuickOpenDialog.tsx`](references/typescript/src/components/QuickOpenDialog.tsx) |
| `references/typescript/src/components/BridgeDialog.tsx` | [`references/typescript/src/components/BridgeDialog.tsx`](references/typescript/src/components/BridgeDialog.tsx) |
| `references/typescript/src/components/LogSelector.tsx` | [`references/typescript/src/components/LogSelector.tsx`](references/typescript/src/components/LogSelector.tsx) |
| `references/typescript/src/components/MessageTimestamp.tsx` | [`references/typescript/src/components/MessageTimestamp.tsx`](references/typescript/src/components/MessageTimestamp.tsx) |
| `references/typescript/src/components/ExportDialog.tsx` | [`references/typescript/src/components/ExportDialog.tsx`](references/typescript/src/components/ExportDialog.tsx) |
| `references/typescript/src/components/ConfigurableShortcutHint.tsx` | [`references/typescript/src/components/ConfigurableShortcutHint.tsx`](references/typescript/src/components/ConfigurableShortcutHint.tsx) |
| `references/typescript/src/components/hooks/SelectMatcherMode.tsx` | [`references/typescript/src/components/hooks/SelectMatcherMode.tsx`](references/typescript/src/components/hooks/SelectMatcherMode.tsx) |
| `references/typescript/src/components/hooks/PromptDialog.tsx` | [`references/typescript/src/components/hooks/PromptDialog.tsx`](references/typescript/src/components/hooks/PromptDialog.tsx) |
| `references/typescript/src/components/hooks/ViewHookMode.tsx` | [`references/typescript/src/components/hooks/ViewHookMode.tsx`](references/typescript/src/components/hooks/ViewHookMode.tsx) |
| `references/typescript/src/components/hooks/SelectHookMode.tsx` | [`references/typescript/src/components/hooks/SelectHookMode.tsx`](references/typescript/src/components/hooks/SelectHookMode.tsx) |
| `references/typescript/src/components/hooks/SelectEventMode.tsx` | [`references/typescript/src/components/hooks/SelectEventMode.tsx`](references/typescript/src/components/hooks/SelectEventMode.tsx) |
| `references/typescript/src/components/hooks/HooksConfigMenu.tsx` | [`references/typescript/src/components/hooks/HooksConfigMenu.tsx`](references/typescript/src/components/hooks/HooksConfigMenu.tsx) |
| `references/typescript/src/components/FallbackToolUseRejectedMessage.tsx` | [`references/typescript/src/components/FallbackToolUseRejectedMessage.tsx`](references/typescript/src/components/FallbackToolUseRejectedMessage.tsx) |
| `references/typescript/src/components/SkillImprovementSurvey.tsx` | [`references/typescript/src/components/SkillImprovementSurvey.tsx`](references/typescript/src/components/SkillImprovementSurvey.tsx) |
| `references/typescript/src/components/FileEditToolDiff.tsx` | [`references/typescript/src/components/FileEditToolDiff.tsx`](references/typescript/src/components/FileEditToolDiff.tsx) |
| `references/typescript/src/components/wizard/WizardDialogLayout.tsx` | [`references/typescript/src/components/wizard/WizardDialogLayout.tsx`](references/typescript/src/components/wizard/WizardDialogLayout.tsx) |
| `references/typescript/src/components/wizard/WizardProvider.tsx` | [`references/typescript/src/components/wizard/WizardProvider.tsx`](references/typescript/src/components/wizard/WizardProvider.tsx) |
| `references/typescript/src/components/wizard/WizardNavigationFooter.tsx` | [`references/typescript/src/components/wizard/WizardNavigationFooter.tsx`](references/typescript/src/components/wizard/WizardNavigationFooter.tsx) |
| `references/typescript/src/components/wizard/index.ts` | [`references/typescript/src/components/wizard/index.ts`](references/typescript/src/components/wizard/index.ts) |
| `references/typescript/src/components/wizard/useWizard.ts` | [`references/typescript/src/components/wizard/useWizard.ts`](references/typescript/src/components/wizard/useWizard.ts) |
| `references/typescript/src/components/ConsoleOAuthFlow.tsx` | [`references/typescript/src/components/ConsoleOAuthFlow.tsx`](references/typescript/src/components/ConsoleOAuthFlow.tsx) |
| `references/typescript/src/components/NativeAutoUpdater.tsx` | [`references/typescript/src/components/NativeAutoUpdater.tsx`](references/typescript/src/components/NativeAutoUpdater.tsx) |
| `references/typescript/src/components/FullscreenLayout.tsx` | [`references/typescript/src/components/FullscreenLayout.tsx`](references/typescript/src/components/FullscreenLayout.tsx) |
| `references/typescript/src/components/TokenWarning.tsx` | [`references/typescript/src/components/TokenWarning.tsx`](references/typescript/src/components/TokenWarning.tsx) |
| `references/typescript/src/components/RemoteEnvironmentDialog.tsx` | [`references/typescript/src/components/RemoteEnvironmentDialog.tsx`](references/typescript/src/components/RemoteEnvironmentDialog.tsx) |
| `references/typescript/src/components/design-system/color.ts` | [`references/typescript/src/components/design-system/color.ts`](references/typescript/src/components/design-system/color.ts) |
| `references/typescript/src/components/design-system/ThemedText.tsx` | [`references/typescript/src/components/design-system/ThemedText.tsx`](references/typescript/src/components/design-system/ThemedText.tsx) |
| `references/typescript/src/components/design-system/FuzzyPicker.tsx` | [`references/typescript/src/components/design-system/FuzzyPicker.tsx`](references/typescript/src/components/design-system/FuzzyPicker.tsx) |
| `references/typescript/src/components/design-system/Pane.tsx` | [`references/typescript/src/components/design-system/Pane.tsx`](references/typescript/src/components/design-system/Pane.tsx) |
| `references/typescript/src/components/design-system/ThemedBox.tsx` | [`references/typescript/src/components/design-system/ThemedBox.tsx`](references/typescript/src/components/design-system/ThemedBox.tsx) |
| `references/typescript/src/components/design-system/StatusIcon.tsx` | [`references/typescript/src/components/design-system/StatusIcon.tsx`](references/typescript/src/components/design-system/StatusIcon.tsx) |
| `references/typescript/src/components/design-system/ThemeProvider.tsx` | [`references/typescript/src/components/design-system/ThemeProvider.tsx`](references/typescript/src/components/design-system/ThemeProvider.tsx) |
| `references/typescript/src/components/design-system/ListItem.tsx` | [`references/typescript/src/components/design-system/ListItem.tsx`](references/typescript/src/components/design-system/ListItem.tsx) |
| `references/typescript/src/components/design-system/Divider.tsx` | [`references/typescript/src/components/design-system/Divider.tsx`](references/typescript/src/components/design-system/Divider.tsx) |
| `references/typescript/src/components/design-system/Tabs.tsx` | [`references/typescript/src/components/design-system/Tabs.tsx`](references/typescript/src/components/design-system/Tabs.tsx) |
| `references/typescript/src/components/design-system/ProgressBar.tsx` | [`references/typescript/src/components/design-system/ProgressBar.tsx`](references/typescript/src/components/design-system/ProgressBar.tsx) |
| `references/typescript/src/components/design-system/Ratchet.tsx` | [`references/typescript/src/components/design-system/Ratchet.tsx`](references/typescript/src/components/design-system/Ratchet.tsx) |
| `references/typescript/src/components/design-system/KeyboardShortcutHint.tsx` | [`references/typescript/src/components/design-system/KeyboardShortcutHint.tsx`](references/typescript/src/components/design-system/KeyboardShortcutHint.tsx) |
| `references/typescript/src/components/design-system/Byline.tsx` | [`references/typescript/src/components/design-system/Byline.tsx`](references/typescript/src/components/design-system/Byline.tsx) |
| `references/typescript/src/components/design-system/LoadingState.tsx` | [`references/typescript/src/components/design-system/LoadingState.tsx`](references/typescript/src/components/design-system/LoadingState.tsx) |
| `references/typescript/src/components/design-system/Dialog.tsx` | [`references/typescript/src/components/design-system/Dialog.tsx`](references/typescript/src/components/design-system/Dialog.tsx) |
| `references/typescript/src/components/Settings/Settings.tsx` | [`references/typescript/src/components/Settings/Settings.tsx`](references/typescript/src/components/Settings/Settings.tsx) |
| `references/typescript/src/components/Settings/Config.tsx` | [`references/typescript/src/components/Settings/Config.tsx`](references/typescript/src/components/Settings/Config.tsx) |
| `references/typescript/src/components/Settings/Usage.tsx` | [`references/typescript/src/components/Settings/Usage.tsx`](references/typescript/src/components/Settings/Usage.tsx) |
| `references/typescript/src/components/Settings/Status.tsx` | [`references/typescript/src/components/Settings/Status.tsx`](references/typescript/src/components/Settings/Status.tsx) |
| `references/typescript/src/components/WorktreeExitDialog.tsx` | [`references/typescript/src/components/WorktreeExitDialog.tsx`](references/typescript/src/components/WorktreeExitDialog.tsx) |
| `references/typescript/src/components/DesktopHandoff.tsx` | [`references/typescript/src/components/DesktopHandoff.tsx`](references/typescript/src/components/DesktopHandoff.tsx) |
| `references/typescript/src/components/Feedback.tsx` | [`references/typescript/src/components/Feedback.tsx`](references/typescript/src/components/Feedback.tsx) |
| `references/typescript/src/components/LspRecommendation/LspRecommendationMenu.tsx` | [`references/typescript/src/components/LspRecommendation/LspRecommendationMenu.tsx`](references/typescript/src/components/LspRecommendation/LspRecommendationMenu.tsx) |
| `references/typescript/src/components/AutoModeOptInDialog.tsx` | [`references/typescript/src/components/AutoModeOptInDialog.tsx`](references/typescript/src/components/AutoModeOptInDialog.tsx) |
| `references/typescript/src/components/ContextVisualization.tsx` | [`references/typescript/src/components/ContextVisualization.tsx`](references/typescript/src/components/ContextVisualization.tsx) |
| `references/typescript/src/components/BypassPermissionsModeDialog.tsx` | [`references/typescript/src/components/BypassPermissionsModeDialog.tsx`](references/typescript/src/components/BypassPermissionsModeDialog.tsx) |
| `references/typescript/src/components/DevBar.tsx` | [`references/typescript/src/components/DevBar.tsx`](references/typescript/src/components/DevBar.tsx) |
| `references/typescript/src/components/AutoUpdater.tsx` | [`references/typescript/src/components/AutoUpdater.tsx`](references/typescript/src/components/AutoUpdater.tsx) |
| `references/typescript/src/components/NotebookEditToolUseRejectedMessage.tsx` | [`references/typescript/src/components/NotebookEditToolUseRejectedMessage.tsx`](references/typescript/src/components/NotebookEditToolUseRejectedMessage.tsx) |
| `references/typescript/src/components/TagTabs.tsx` | [`references/typescript/src/components/TagTabs.tsx`](references/typescript/src/components/TagTabs.tsx) |
| `references/typescript/src/components/DevChannelsDialog.tsx` | [`references/typescript/src/components/DevChannelsDialog.tsx`](references/typescript/src/components/DevChannelsDialog.tsx) |
| `references/typescript/src/components/AutoUpdaterWrapper.tsx` | [`references/typescript/src/components/AutoUpdaterWrapper.tsx`](references/typescript/src/components/AutoUpdaterWrapper.tsx) |
| `references/typescript/src/components/TeleportError.tsx` | [`references/typescript/src/components/TeleportError.tsx`](references/typescript/src/components/TeleportError.tsx) |
| `references/typescript/src/components/ManagedSettingsSecurityDialog/ManagedSettingsSecurityDialog.tsx` | [`references/typescript/src/components/ManagedSettingsSecurityDialog/ManagedSettingsSecurityDialog.tsx`](references/typescript/src/components/ManagedSettingsSecurityDialog/ManagedSettingsSecurityDialog.tsx) |
| `references/typescript/src/components/ManagedSettingsSecurityDialog/utils.ts` | [`references/typescript/src/components/ManagedSettingsSecurityDialog/utils.ts`](references/typescript/src/components/ManagedSettingsSecurityDialog/utils.ts) |
| `references/typescript/src/components/StatusNotices.tsx` | [`references/typescript/src/components/StatusNotices.tsx`](references/typescript/src/components/StatusNotices.tsx) |
| `references/typescript/src/components/IdeOnboardingDialog.tsx` | [`references/typescript/src/components/IdeOnboardingDialog.tsx`](references/typescript/src/components/IdeOnboardingDialog.tsx) |
| `references/typescript/src/components/ScrollKeybindingHandler.tsx` | [`references/typescript/src/components/ScrollKeybindingHandler.tsx`](references/typescript/src/components/ScrollKeybindingHandler.tsx) |
| `references/typescript/src/components/OffscreenFreeze.tsx` | [`references/typescript/src/components/OffscreenFreeze.tsx`](references/typescript/src/components/OffscreenFreeze.tsx) |
| `references/typescript/src/components/skills/SkillsMenu.tsx` | [`references/typescript/src/components/skills/SkillsMenu.tsx`](references/typescript/src/components/skills/SkillsMenu.tsx) |
| `references/typescript/src/components/LogoV2/Clawd.tsx` | [`references/typescript/src/components/LogoV2/Clawd.tsx`](references/typescript/src/components/LogoV2/Clawd.tsx) |
| `references/typescript/src/components/LogoV2/LogoV2.tsx` | [`references/typescript/src/components/LogoV2/LogoV2.tsx`](references/typescript/src/components/LogoV2/LogoV2.tsx) |
| `references/typescript/src/components/LogoV2/Opus1mMergeNotice.tsx` | [`references/typescript/src/components/LogoV2/Opus1mMergeNotice.tsx`](references/typescript/src/components/LogoV2/Opus1mMergeNotice.tsx) |
| `references/typescript/src/components/LogoV2/AnimatedAsterisk.tsx` | [`references/typescript/src/components/LogoV2/AnimatedAsterisk.tsx`](references/typescript/src/components/LogoV2/AnimatedAsterisk.tsx) |
| `references/typescript/src/components/LogoV2/GuestPassesUpsell.tsx` | [`references/typescript/src/components/LogoV2/GuestPassesUpsell.tsx`](references/typescript/src/components/LogoV2/GuestPassesUpsell.tsx) |
| `references/typescript/src/components/LogoV2/CondensedLogo.tsx` | [`references/typescript/src/components/LogoV2/CondensedLogo.tsx`](references/typescript/src/components/LogoV2/CondensedLogo.tsx) |
| `references/typescript/src/components/LogoV2/OverageCreditUpsell.tsx` | [`references/typescript/src/components/LogoV2/OverageCreditUpsell.tsx`](references/typescript/src/components/LogoV2/OverageCreditUpsell.tsx) |
| `references/typescript/src/components/LogoV2/AnimatedClawd.tsx` | [`references/typescript/src/components/LogoV2/AnimatedClawd.tsx`](references/typescript/src/components/LogoV2/AnimatedClawd.tsx) |
| `references/typescript/src/components/LogoV2/feedConfigs.tsx` | [`references/typescript/src/components/LogoV2/feedConfigs.tsx`](references/typescript/src/components/LogoV2/feedConfigs.tsx) |
| `references/typescript/src/components/LogoV2/WelcomeV2.tsx` | [`references/typescript/src/components/LogoV2/WelcomeV2.tsx`](references/typescript/src/components/LogoV2/WelcomeV2.tsx) |
| `references/typescript/src/components/LogoV2/FeedColumn.tsx` | [`references/typescript/src/components/LogoV2/FeedColumn.tsx`](references/typescript/src/components/LogoV2/FeedColumn.tsx) |
| `references/typescript/src/components/LogoV2/Feed.tsx` | [`references/typescript/src/components/LogoV2/Feed.tsx`](references/typescript/src/components/LogoV2/Feed.tsx) |
| `references/typescript/src/components/LogoV2/EmergencyTip.tsx` | [`references/typescript/src/components/LogoV2/EmergencyTip.tsx`](references/typescript/src/components/LogoV2/EmergencyTip.tsx) |
| `references/typescript/src/components/LogoV2/ChannelsNotice.tsx` | [`references/typescript/src/components/LogoV2/ChannelsNotice.tsx`](references/typescript/src/components/LogoV2/ChannelsNotice.tsx) |
| `references/typescript/src/components/LogoV2/VoiceModeNotice.tsx` | [`references/typescript/src/components/LogoV2/VoiceModeNotice.tsx`](references/typescript/src/components/LogoV2/VoiceModeNotice.tsx) |
| `references/typescript/src/components/EffortCallout.tsx` | [`references/typescript/src/components/EffortCallout.tsx`](references/typescript/src/components/EffortCallout.tsx) |
| `references/typescript/src/components/ValidationErrorsList.tsx` | [`references/typescript/src/components/ValidationErrorsList.tsx`](references/typescript/src/components/ValidationErrorsList.tsx) |
| `references/typescript/src/components/FallbackToolUseErrorMessage.tsx` | [`references/typescript/src/components/FallbackToolUseErrorMessage.tsx`](references/typescript/src/components/FallbackToolUseErrorMessage.tsx) |
| `references/typescript/src/components/MessageRow.tsx` | [`references/typescript/src/components/MessageRow.tsx`](references/typescript/src/components/MessageRow.tsx) |
| `references/typescript/src/components/HighlightedCode.tsx` | [`references/typescript/src/components/HighlightedCode.tsx`](references/typescript/src/components/HighlightedCode.tsx) |
| `references/typescript/src/components/Spinner.tsx` | [`references/typescript/src/components/Spinner.tsx`](references/typescript/src/components/Spinner.tsx) |
| `references/typescript/src/components/HelpV2/General.tsx` | [`references/typescript/src/components/HelpV2/General.tsx`](references/typescript/src/components/HelpV2/General.tsx) |
| `references/typescript/src/components/HelpV2/Commands.tsx` | [`references/typescript/src/components/HelpV2/Commands.tsx`](references/typescript/src/components/HelpV2/Commands.tsx) |
| `references/typescript/src/components/HelpV2/HelpV2.tsx` | [`references/typescript/src/components/HelpV2/HelpV2.tsx`](references/typescript/src/components/HelpV2/HelpV2.tsx) |
| `references/typescript/src/components/MarkdownTable.tsx` | [`references/typescript/src/components/MarkdownTable.tsx`](references/typescript/src/components/MarkdownTable.tsx) |
| `references/typescript/src/components/ResumeTask.tsx` | [`references/typescript/src/components/ResumeTask.tsx`](references/typescript/src/components/ResumeTask.tsx) |
| `references/typescript/src/components/TeleportStash.tsx` | [`references/typescript/src/components/TeleportStash.tsx`](references/typescript/src/components/TeleportStash.tsx) |
| `references/typescript/src/components/StructuredDiff/colorDiff.ts` | [`references/typescript/src/components/StructuredDiff/colorDiff.ts`](references/typescript/src/components/StructuredDiff/colorDiff.ts) |
| `references/typescript/src/components/StructuredDiff/Fallback.tsx` | [`references/typescript/src/components/StructuredDiff/Fallback.tsx`](references/typescript/src/components/StructuredDiff/Fallback.tsx) |
| `references/typescript/src/components/ShowInIDEPrompt.tsx` | [`references/typescript/src/components/ShowInIDEPrompt.tsx`](references/typescript/src/components/ShowInIDEPrompt.tsx) |
| `references/typescript/src/components/shell/ShellTimeDisplay.tsx` | [`references/typescript/src/components/shell/ShellTimeDisplay.tsx`](references/typescript/src/components/shell/ShellTimeDisplay.tsx) |
| `references/typescript/src/components/shell/ShellProgressMessage.tsx` | [`references/typescript/src/components/shell/ShellProgressMessage.tsx`](references/typescript/src/components/shell/ShellProgressMessage.tsx) |
| `references/typescript/src/components/shell/OutputLine.tsx` | [`references/typescript/src/components/shell/OutputLine.tsx`](references/typescript/src/components/shell/OutputLine.tsx) |
| `references/typescript/src/components/shell/ExpandShellOutputContext.tsx` | [`references/typescript/src/components/shell/ExpandShellOutputContext.tsx`](references/typescript/src/components/shell/ExpandShellOutputContext.tsx) |
| `references/typescript/src/components/CompactSummary.tsx` | [`references/typescript/src/components/CompactSummary.tsx`](references/typescript/src/components/CompactSummary.tsx) |
| `references/typescript/src/components/Markdown.tsx` | [`references/typescript/src/components/Markdown.tsx`](references/typescript/src/components/Markdown.tsx) |
| `references/typescript/src/components/Stats.tsx` | [`references/typescript/src/components/Stats.tsx`](references/typescript/src/components/Stats.tsx) |
| `references/typescript/src/components/permissions/EnterPlanModePermissionRequest/EnterPlanModePermissionRequest.tsx` | [`references/typescript/src/components/permissions/EnterPlanModePermissionRequest/EnterPlanModePermissionRequest.tsx`](references/typescript/src/components/permissions/EnterPlanModePermissionRequest/EnterPlanModePermissionRequest.tsx) |
| `references/typescript/src/components/permissions/FileEditPermissionRequest/FileEditPermissionRequest.tsx` | [`references/typescript/src/components/permissions/FileEditPermissionRequest/FileEditPermissionRequest.tsx`](references/typescript/src/components/permissions/FileEditPermissionRequest/FileEditPermissionRequest.tsx) |
| `references/typescript/src/components/permissions/useShellPermissionFeedback.ts` | [`references/typescript/src/components/permissions/useShellPermissionFeedback.ts`](references/typescript/src/components/permissions/useShellPermissionFeedback.ts) |
| `references/typescript/src/components/permissions/ExitPlanModePermissionRequest/ExitPlanModePermissionRequest.tsx` | [`references/typescript/src/components/permissions/ExitPlanModePermissionRequest/ExitPlanModePermissionRequest.tsx`](references/typescript/src/components/permissions/ExitPlanModePermissionRequest/ExitPlanModePermissionRequest.tsx) |
| `references/typescript/src/components/permissions/NotebookEditPermissionRequest/NotebookEditToolDiff.tsx` | [`references/typescript/src/components/permissions/NotebookEditPermissionRequest/NotebookEditToolDiff.tsx`](references/typescript/src/components/permissions/NotebookEditPermissionRequest/NotebookEditToolDiff.tsx) |
| `references/typescript/src/components/permissions/NotebookEditPermissionRequest/NotebookEditPermissionRequest.tsx` | [`references/typescript/src/components/permissions/NotebookEditPermissionRequest/NotebookEditPermissionRequest.tsx`](references/typescript/src/components/permissions/NotebookEditPermissionRequest/NotebookEditPermissionRequest.tsx) |
| `references/typescript/src/components/permissions/shellPermissionHelpers.tsx` | [`references/typescript/src/components/permissions/shellPermissionHelpers.tsx`](references/typescript/src/components/permissions/shellPermissionHelpers.tsx) |
| `references/typescript/src/components/permissions/SandboxPermissionRequest.tsx` | [`references/typescript/src/components/permissions/SandboxPermissionRequest.tsx`](references/typescript/src/components/permissions/SandboxPermissionRequest.tsx) |
| `references/typescript/src/components/permissions/FallbackPermissionRequest.tsx` | [`references/typescript/src/components/permissions/FallbackPermissionRequest.tsx`](references/typescript/src/components/permissions/FallbackPermissionRequest.tsx) |
| `references/typescript/src/components/permissions/PermissionRequestTitle.tsx` | [`references/typescript/src/components/permissions/PermissionRequestTitle.tsx`](references/typescript/src/components/permissions/PermissionRequestTitle.tsx) |
| `references/typescript/src/components/permissions/FilePermissionDialog/FilePermissionDialog.tsx` | [`references/typescript/src/components/permissions/FilePermissionDialog/FilePermissionDialog.tsx`](references/typescript/src/components/permissions/FilePermissionDialog/FilePermissionDialog.tsx) |
| `references/typescript/src/components/permissions/FilePermissionDialog/permissionOptions.tsx` | [`references/typescript/src/components/permissions/FilePermissionDialog/permissionOptions.tsx`](references/typescript/src/components/permissions/FilePermissionDialog/permissionOptions.tsx) |
| `references/typescript/src/components/permissions/FilePermissionDialog/useFilePermissionDialog.ts` | [`references/typescript/src/components/permissions/FilePermissionDialog/useFilePermissionDialog.ts`](references/typescript/src/components/permissions/FilePermissionDialog/useFilePermissionDialog.ts) |
| `references/typescript/src/components/permissions/FilePermissionDialog/ideDiffConfig.ts` | [`references/typescript/src/components/permissions/FilePermissionDialog/ideDiffConfig.ts`](references/typescript/src/components/permissions/FilePermissionDialog/ideDiffConfig.ts) |
| `references/typescript/src/components/permissions/FilePermissionDialog/usePermissionHandler.ts` | [`references/typescript/src/components/permissions/FilePermissionDialog/usePermissionHandler.ts`](references/typescript/src/components/permissions/FilePermissionDialog/usePermissionHandler.ts) |
| `references/typescript/src/components/permissions/SedEditPermissionRequest/SedEditPermissionRequest.tsx` | [`references/typescript/src/components/permissions/SedEditPermissionRequest/SedEditPermissionRequest.tsx`](references/typescript/src/components/permissions/SedEditPermissionRequest/SedEditPermissionRequest.tsx) |
| `references/typescript/src/components/permissions/WorkerBadge.tsx` | [`references/typescript/src/components/permissions/WorkerBadge.tsx`](references/typescript/src/components/permissions/WorkerBadge.tsx) |
| `references/typescript/src/components/permissions/PermissionDecisionDebugInfo.tsx` | [`references/typescript/src/components/permissions/PermissionDecisionDebugInfo.tsx`](references/typescript/src/components/permissions/PermissionDecisionDebugInfo.tsx) |
| `references/typescript/src/components/permissions/WebFetchPermissionRequest/WebFetchPermissionRequest.tsx` | [`references/typescript/src/components/permissions/WebFetchPermissionRequest/WebFetchPermissionRequest.tsx`](references/typescript/src/components/permissions/WebFetchPermissionRequest/WebFetchPermissionRequest.tsx) |
| `references/typescript/src/components/permissions/FilesystemPermissionRequest/FilesystemPermissionRequest.tsx` | [`references/typescript/src/components/permissions/FilesystemPermissionRequest/FilesystemPermissionRequest.tsx`](references/typescript/src/components/permissions/FilesystemPermissionRequest/FilesystemPermissionRequest.tsx) |
| `references/typescript/src/components/permissions/PowerShellPermissionRequest/PowerShellPermissionRequest.tsx` | [`references/typescript/src/components/permissions/PowerShellPermissionRequest/PowerShellPermissionRequest.tsx`](references/typescript/src/components/permissions/PowerShellPermissionRequest/PowerShellPermissionRequest.tsx) |
| `references/typescript/src/components/permissions/PowerShellPermissionRequest/powershellToolUseOptions.tsx` | [`references/typescript/src/components/permissions/PowerShellPermissionRequest/powershellToolUseOptions.tsx`](references/typescript/src/components/permissions/PowerShellPermissionRequest/powershellToolUseOptions.tsx) |
| `references/typescript/src/components/permissions/ComputerUseApproval/ComputerUseApproval.tsx` | [`references/typescript/src/components/permissions/ComputerUseApproval/ComputerUseApproval.tsx`](references/typescript/src/components/permissions/ComputerUseApproval/ComputerUseApproval.tsx) |
| `references/typescript/src/components/permissions/hooks.ts` | [`references/typescript/src/components/permissions/hooks.ts`](references/typescript/src/components/permissions/hooks.ts) |
| `references/typescript/src/components/permissions/PermissionRuleExplanation.tsx` | [`references/typescript/src/components/permissions/PermissionRuleExplanation.tsx`](references/typescript/src/components/permissions/PermissionRuleExplanation.tsx) |
| `references/typescript/src/components/permissions/utils.ts` | [`references/typescript/src/components/permissions/utils.ts`](references/typescript/src/components/permissions/utils.ts) |
| `references/typescript/src/components/permissions/FileWritePermissionRequest/FileWritePermissionRequest.tsx` | [`references/typescript/src/components/permissions/FileWritePermissionRequest/FileWritePermissionRequest.tsx`](references/typescript/src/components/permissions/FileWritePermissionRequest/FileWritePermissionRequest.tsx) |
| `references/typescript/src/components/permissions/FileWritePermissionRequest/FileWriteToolDiff.tsx` | [`references/typescript/src/components/permissions/FileWritePermissionRequest/FileWriteToolDiff.tsx`](references/typescript/src/components/permissions/FileWritePermissionRequest/FileWriteToolDiff.tsx) |
| `references/typescript/src/components/permissions/rules/AddPermissionRules.tsx` | [`references/typescript/src/components/permissions/rules/AddPermissionRules.tsx`](references/typescript/src/components/permissions/rules/AddPermissionRules.tsx) |
| `references/typescript/src/components/permissions/rules/PermissionRuleInput.tsx` | [`references/typescript/src/components/permissions/rules/PermissionRuleInput.tsx`](references/typescript/src/components/permissions/rules/PermissionRuleInput.tsx) |
| `references/typescript/src/components/permissions/rules/PermissionRuleDescription.tsx` | [`references/typescript/src/components/permissions/rules/PermissionRuleDescription.tsx`](references/typescript/src/components/permissions/rules/PermissionRuleDescription.tsx) |
| `references/typescript/src/components/permissions/rules/RecentDenialsTab.tsx` | [`references/typescript/src/components/permissions/rules/RecentDenialsTab.tsx`](references/typescript/src/components/permissions/rules/RecentDenialsTab.tsx) |
| `references/typescript/src/components/permissions/rules/AddWorkspaceDirectory.tsx` | [`references/typescript/src/components/permissions/rules/AddWorkspaceDirectory.tsx`](references/typescript/src/components/permissions/rules/AddWorkspaceDirectory.tsx) |
| `references/typescript/src/components/permissions/rules/RemoveWorkspaceDirectory.tsx` | [`references/typescript/src/components/permissions/rules/RemoveWorkspaceDirectory.tsx`](references/typescript/src/components/permissions/rules/RemoveWorkspaceDirectory.tsx) |
| `references/typescript/src/components/permissions/rules/PermissionRuleList.tsx` | [`references/typescript/src/components/permissions/rules/PermissionRuleList.tsx`](references/typescript/src/components/permissions/rules/PermissionRuleList.tsx) |
| `references/typescript/src/components/permissions/rules/WorkspaceTab.tsx` | [`references/typescript/src/components/permissions/rules/WorkspaceTab.tsx`](references/typescript/src/components/permissions/rules/WorkspaceTab.tsx) |
| `references/typescript/src/components/permissions/BashPermissionRequest/bashToolUseOptions.tsx` | [`references/typescript/src/components/permissions/BashPermissionRequest/bashToolUseOptions.tsx`](references/typescript/src/components/permissions/BashPermissionRequest/bashToolUseOptions.tsx) |
| `references/typescript/src/components/permissions/BashPermissionRequest/BashPermissionRequest.tsx` | [`references/typescript/src/components/permissions/BashPermissionRequest/BashPermissionRequest.tsx`](references/typescript/src/components/permissions/BashPermissionRequest/BashPermissionRequest.tsx) |
| `references/typescript/src/components/permissions/WorkerPendingPermission.tsx` | [`references/typescript/src/components/permissions/WorkerPendingPermission.tsx`](references/typescript/src/components/permissions/WorkerPendingPermission.tsx) |
| `references/typescript/src/components/permissions/PermissionDialog.tsx` | [`references/typescript/src/components/permissions/PermissionDialog.tsx`](references/typescript/src/components/permissions/PermissionDialog.tsx) |
| `references/typescript/src/components/permissions/PermissionPrompt.tsx` | [`references/typescript/src/components/permissions/PermissionPrompt.tsx`](references/typescript/src/components/permissions/PermissionPrompt.tsx) |
| `references/typescript/src/components/permissions/PermissionRequest.tsx` | [`references/typescript/src/components/permissions/PermissionRequest.tsx`](references/typescript/src/components/permissions/PermissionRequest.tsx) |
| `references/typescript/src/components/permissions/PermissionExplanation.tsx` | [`references/typescript/src/components/permissions/PermissionExplanation.tsx`](references/typescript/src/components/permissions/PermissionExplanation.tsx) |
| `references/typescript/src/components/permissions/AskUserQuestionPermissionRequest/AskUserQuestionPermissionRequest.tsx` | [`references/typescript/src/components/permissions/AskUserQuestionPermissionRequest/AskUserQuestionPermissionRequest.tsx`](references/typescript/src/components/permissions/AskUserQuestionPermissionRequest/AskUserQuestionPermissionRequest.tsx) |
| `references/typescript/src/components/permissions/AskUserQuestionPermissionRequest/PreviewBox.tsx` | [`references/typescript/src/components/permissions/AskUserQuestionPermissionRequest/PreviewBox.tsx`](references/typescript/src/components/permissions/AskUserQuestionPermissionRequest/PreviewBox.tsx) |
| `references/typescript/src/components/permissions/AskUserQuestionPermissionRequest/use-multiple-choice-state.ts` | [`references/typescript/src/components/permissions/AskUserQuestionPermissionRequest/use-multiple-choice-state.ts`](references/typescript/src/components/permissions/AskUserQuestionPermissionRequest/use-multiple-choice-state.ts) |
| `references/typescript/src/components/permissions/AskUserQuestionPermissionRequest/QuestionNavigationBar.tsx` | [`references/typescript/src/components/permissions/AskUserQuestionPermissionRequest/QuestionNavigationBar.tsx`](references/typescript/src/components/permissions/AskUserQuestionPermissionRequest/QuestionNavigationBar.tsx) |
| `references/typescript/src/components/permissions/AskUserQuestionPermissionRequest/SubmitQuestionsView.tsx` | [`references/typescript/src/components/permissions/AskUserQuestionPermissionRequest/SubmitQuestionsView.tsx`](references/typescript/src/components/permissions/AskUserQuestionPermissionRequest/SubmitQuestionsView.tsx) |
| `references/typescript/src/components/permissions/AskUserQuestionPermissionRequest/PreviewQuestionView.tsx` | [`references/typescript/src/components/permissions/AskUserQuestionPermissionRequest/PreviewQuestionView.tsx`](references/typescript/src/components/permissions/AskUserQuestionPermissionRequest/PreviewQuestionView.tsx) |
| `references/typescript/src/components/permissions/AskUserQuestionPermissionRequest/QuestionView.tsx` | [`references/typescript/src/components/permissions/AskUserQuestionPermissionRequest/QuestionView.tsx`](references/typescript/src/components/permissions/AskUserQuestionPermissionRequest/QuestionView.tsx) |
| `references/typescript/src/components/permissions/SkillPermissionRequest/SkillPermissionRequest.tsx` | [`references/typescript/src/components/permissions/SkillPermissionRequest/SkillPermissionRequest.tsx`](references/typescript/src/components/permissions/SkillPermissionRequest/SkillPermissionRequest.tsx) |
| `references/typescript/src/screens/Doctor.tsx` | [`references/typescript/src/screens/Doctor.tsx`](references/typescript/src/screens/Doctor.tsx) |
| `references/typescript/src/screens/ResumeConversation.tsx` | [`references/typescript/src/screens/ResumeConversation.tsx`](references/typescript/src/screens/ResumeConversation.tsx) |
| `references/typescript/src/screens/REPL.tsx` | [`references/typescript/src/screens/REPL.tsx`](references/typescript/src/screens/REPL.tsx) |
| `references/typescript/src/moreright/useMoreRight.tsx` | [`references/typescript/src/moreright/useMoreRight.tsx`](references/typescript/src/moreright/useMoreRight.tsx) |
| `references/typescript/bun.lock` | [`references/typescript/bun.lock`](references/typescript/bun.lock) |
| `references/typescript/changes.md` | [`references/typescript/changes.md`](references/typescript/changes.md) |
| `references/typescript/assets/screenshot.png` | [`references/typescript/assets/screenshot.png`](references/typescript/assets/screenshot.png) |

### 20.3 Rust Inventory

- Count: `123`

| Canonical archived source path | Link |
| --- | --- |
| `references/rust/TUI-ENHANCEMENT-PLAN.md` | [`references/rust/TUI-ENHANCEMENT-PLAN.md`](references/rust/TUI-ENHANCEMENT-PLAN.md) |
| `references/rust/crates/runtime/Cargo.toml` | [`references/rust/crates/runtime/Cargo.toml`](references/rust/crates/runtime/Cargo.toml) |
| `references/rust/crates/runtime/src/file_ops.rs` | [`references/rust/crates/runtime/src/file_ops.rs`](references/rust/crates/runtime/src/file_ops.rs) |
| `references/rust/crates/runtime/src/session.rs` | [`references/rust/crates/runtime/src/session.rs`](references/rust/crates/runtime/src/session.rs) |
| `references/rust/crates/runtime/src/green_contract.rs` | [`references/rust/crates/runtime/src/green_contract.rs`](references/rust/crates/runtime/src/green_contract.rs) |
| `references/rust/crates/runtime/src/json.rs` | [`references/rust/crates/runtime/src/json.rs`](references/rust/crates/runtime/src/json.rs) |
| `references/rust/crates/runtime/src/lib.rs` | [`references/rust/crates/runtime/src/lib.rs`](references/rust/crates/runtime/src/lib.rs) |
| `references/rust/crates/runtime/src/team_cron_registry.rs` | [`references/rust/crates/runtime/src/team_cron_registry.rs`](references/rust/crates/runtime/src/team_cron_registry.rs) |
| `references/rust/crates/runtime/src/mcp_tool_bridge.rs` | [`references/rust/crates/runtime/src/mcp_tool_bridge.rs`](references/rust/crates/runtime/src/mcp_tool_bridge.rs) |
| `references/rust/crates/runtime/src/permission_enforcer.rs` | [`references/rust/crates/runtime/src/permission_enforcer.rs`](references/rust/crates/runtime/src/permission_enforcer.rs) |
| `references/rust/crates/runtime/src/sse.rs` | [`references/rust/crates/runtime/src/sse.rs`](references/rust/crates/runtime/src/sse.rs) |
| `references/rust/crates/runtime/src/trust_resolver.rs` | [`references/rust/crates/runtime/src/trust_resolver.rs`](references/rust/crates/runtime/src/trust_resolver.rs) |
| `references/rust/crates/runtime/src/compact.rs` | [`references/rust/crates/runtime/src/compact.rs`](references/rust/crates/runtime/src/compact.rs) |
| `references/rust/crates/runtime/src/bash_validation.rs` | [`references/rust/crates/runtime/src/bash_validation.rs`](references/rust/crates/runtime/src/bash_validation.rs) |
| `references/rust/crates/runtime/src/summary_compression.rs` | [`references/rust/crates/runtime/src/summary_compression.rs`](references/rust/crates/runtime/src/summary_compression.rs) |
| `references/rust/crates/runtime/src/plugin_lifecycle.rs` | [`references/rust/crates/runtime/src/plugin_lifecycle.rs`](references/rust/crates/runtime/src/plugin_lifecycle.rs) |
| `references/rust/crates/runtime/src/oauth.rs` | [`references/rust/crates/runtime/src/oauth.rs`](references/rust/crates/runtime/src/oauth.rs) |
| `references/rust/crates/runtime/src/bash.rs` | [`references/rust/crates/runtime/src/bash.rs`](references/rust/crates/runtime/src/bash.rs) |
| `references/rust/crates/runtime/src/mcp.rs` | [`references/rust/crates/runtime/src/mcp.rs`](references/rust/crates/runtime/src/mcp.rs) |
| `references/rust/crates/runtime/src/permissions.rs` | [`references/rust/crates/runtime/src/permissions.rs`](references/rust/crates/runtime/src/permissions.rs) |
| `references/rust/crates/runtime/src/recovery_recipes.rs` | [`references/rust/crates/runtime/src/recovery_recipes.rs`](references/rust/crates/runtime/src/recovery_recipes.rs) |
| `references/rust/crates/runtime/src/worker_boot.rs` | [`references/rust/crates/runtime/src/worker_boot.rs`](references/rust/crates/runtime/src/worker_boot.rs) |
| `references/rust/crates/runtime/src/mcp_client.rs` | [`references/rust/crates/runtime/src/mcp_client.rs`](references/rust/crates/runtime/src/mcp_client.rs) |
| `references/rust/crates/runtime/src/task_registry.rs` | [`references/rust/crates/runtime/src/task_registry.rs`](references/rust/crates/runtime/src/task_registry.rs) |
| `references/rust/crates/runtime/src/mcp_stdio.rs` | [`references/rust/crates/runtime/src/mcp_stdio.rs`](references/rust/crates/runtime/src/mcp_stdio.rs) |
| `references/rust/crates/runtime/src/sandbox.rs` | [`references/rust/crates/runtime/src/sandbox.rs`](references/rust/crates/runtime/src/sandbox.rs) |
| `references/rust/crates/runtime/src/usage.rs` | [`references/rust/crates/runtime/src/usage.rs`](references/rust/crates/runtime/src/usage.rs) |
| `references/rust/crates/runtime/src/lsp_client.rs` | [`references/rust/crates/runtime/src/lsp_client.rs`](references/rust/crates/runtime/src/lsp_client.rs) |
| `references/rust/crates/runtime/src/config.rs` | [`references/rust/crates/runtime/src/config.rs`](references/rust/crates/runtime/src/config.rs) |
| `references/rust/crates/runtime/src/task_packet.rs` | [`references/rust/crates/runtime/src/task_packet.rs`](references/rust/crates/runtime/src/task_packet.rs) |
| `references/rust/crates/runtime/src/stale_branch.rs` | [`references/rust/crates/runtime/src/stale_branch.rs`](references/rust/crates/runtime/src/stale_branch.rs) |
| `references/rust/crates/runtime/src/policy_engine.rs` | [`references/rust/crates/runtime/src/policy_engine.rs`](references/rust/crates/runtime/src/policy_engine.rs) |
| `references/rust/crates/runtime/src/bootstrap.rs` | [`references/rust/crates/runtime/src/bootstrap.rs`](references/rust/crates/runtime/src/bootstrap.rs) |
| `references/rust/crates/runtime/src/remote.rs` | [`references/rust/crates/runtime/src/remote.rs`](references/rust/crates/runtime/src/remote.rs) |
| `references/rust/crates/runtime/src/prompt.rs` | [`references/rust/crates/runtime/src/prompt.rs`](references/rust/crates/runtime/src/prompt.rs) |
| `references/rust/crates/runtime/src/mcp_lifecycle_hardened.rs` | [`references/rust/crates/runtime/src/mcp_lifecycle_hardened.rs`](references/rust/crates/runtime/src/mcp_lifecycle_hardened.rs) |
| `references/rust/crates/runtime/src/session_control.rs` | [`references/rust/crates/runtime/src/session_control.rs`](references/rust/crates/runtime/src/session_control.rs) |
| `references/rust/crates/runtime/src/conversation.rs` | [`references/rust/crates/runtime/src/conversation.rs`](references/rust/crates/runtime/src/conversation.rs) |
| `references/rust/crates/runtime/src/hooks.rs` | [`references/rust/crates/runtime/src/hooks.rs`](references/rust/crates/runtime/src/hooks.rs) |
| `references/rust/crates/api/Cargo.toml` | [`references/rust/crates/api/Cargo.toml`](references/rust/crates/api/Cargo.toml) |
| `references/rust/crates/api/tests/client_integration.rs` | [`references/rust/crates/api/tests/client_integration.rs`](references/rust/crates/api/tests/client_integration.rs) |
| `references/rust/crates/api/tests/provider_client_integration.rs` | [`references/rust/crates/api/tests/provider_client_integration.rs`](references/rust/crates/api/tests/provider_client_integration.rs) |
| `references/rust/crates/api/tests/openai_compat_integration.rs` | [`references/rust/crates/api/tests/openai_compat_integration.rs`](references/rust/crates/api/tests/openai_compat_integration.rs) |
| `references/rust/crates/api/src/client.rs` | [`references/rust/crates/api/src/client.rs`](references/rust/crates/api/src/client.rs) |
| `references/rust/crates/api/src/error.rs` | [`references/rust/crates/api/src/error.rs`](references/rust/crates/api/src/error.rs) |
| `references/rust/crates/api/src/lib.rs` | [`references/rust/crates/api/src/lib.rs`](references/rust/crates/api/src/lib.rs) |
| `references/rust/crates/api/src/types.rs` | [`references/rust/crates/api/src/types.rs`](references/rust/crates/api/src/types.rs) |
| `references/rust/crates/api/src/sse.rs` | [`references/rust/crates/api/src/sse.rs`](references/rust/crates/api/src/sse.rs) |
| `references/rust/crates/api/src/providers/mod.rs` | [`references/rust/crates/api/src/providers/mod.rs`](references/rust/crates/api/src/providers/mod.rs) |
| `references/rust/crates/api/src/providers/anthropic.rs` | [`references/rust/crates/api/src/providers/anthropic.rs`](references/rust/crates/api/src/providers/anthropic.rs) |
| `references/rust/crates/api/src/providers/openai_compat.rs` | [`references/rust/crates/api/src/providers/openai_compat.rs`](references/rust/crates/api/src/providers/openai_compat.rs) |
| `references/rust/crates/api/src/prompt_cache.rs` | [`references/rust/crates/api/src/prompt_cache.rs`](references/rust/crates/api/src/prompt_cache.rs) |
| `references/rust/crates/tools/Cargo.toml` | [`references/rust/crates/tools/Cargo.toml`](references/rust/crates/tools/Cargo.toml) |
| `references/rust/crates/tools/.gitignore` | [`references/rust/crates/tools/.gitignore`](references/rust/crates/tools/.gitignore) |
| `references/rust/crates/tools/src/lib.rs` | [`references/rust/crates/tools/src/lib.rs`](references/rust/crates/tools/src/lib.rs) |
| `references/rust/crates/mock-anthropic-service/Cargo.toml` | [`references/rust/crates/mock-anthropic-service/Cargo.toml`](references/rust/crates/mock-anthropic-service/Cargo.toml) |
| `references/rust/crates/mock-anthropic-service/src/main.rs` | [`references/rust/crates/mock-anthropic-service/src/main.rs`](references/rust/crates/mock-anthropic-service/src/main.rs) |
| `references/rust/crates/mock-anthropic-service/src/lib.rs` | [`references/rust/crates/mock-anthropic-service/src/lib.rs`](references/rust/crates/mock-anthropic-service/src/lib.rs) |
| `references/rust/crates/compat-harness/Cargo.toml` | [`references/rust/crates/compat-harness/Cargo.toml`](references/rust/crates/compat-harness/Cargo.toml) |
| `references/rust/crates/compat-harness/src/lib.rs` | [`references/rust/crates/compat-harness/src/lib.rs`](references/rust/crates/compat-harness/src/lib.rs) |
| `references/rust/crates/plugins/bundled/example-bundled/.claude-plugin/plugin.json` | [`references/rust/crates/plugins/bundled/example-bundled/.claude-plugin/plugin.json`](references/rust/crates/plugins/bundled/example-bundled/.claude-plugin/plugin.json) |
| `references/rust/crates/plugins/bundled/example-bundled/hooks/post.sh` | [`references/rust/crates/plugins/bundled/example-bundled/hooks/post.sh`](references/rust/crates/plugins/bundled/example-bundled/hooks/post.sh) |
| `references/rust/crates/plugins/bundled/example-bundled/hooks/pre.sh` | [`references/rust/crates/plugins/bundled/example-bundled/hooks/pre.sh`](references/rust/crates/plugins/bundled/example-bundled/hooks/pre.sh) |
| `references/rust/crates/plugins/bundled/sample-hooks/.claude-plugin/plugin.json` | [`references/rust/crates/plugins/bundled/sample-hooks/.claude-plugin/plugin.json`](references/rust/crates/plugins/bundled/sample-hooks/.claude-plugin/plugin.json) |
| `references/rust/crates/plugins/bundled/sample-hooks/hooks/post.sh` | [`references/rust/crates/plugins/bundled/sample-hooks/hooks/post.sh`](references/rust/crates/plugins/bundled/sample-hooks/hooks/post.sh) |
| `references/rust/crates/plugins/bundled/sample-hooks/hooks/pre.sh` | [`references/rust/crates/plugins/bundled/sample-hooks/hooks/pre.sh`](references/rust/crates/plugins/bundled/sample-hooks/hooks/pre.sh) |
| `references/rust/crates/plugins/Cargo.toml` | [`references/rust/crates/plugins/Cargo.toml`](references/rust/crates/plugins/Cargo.toml) |
| `references/rust/crates/plugins/src/lib.rs` | [`references/rust/crates/plugins/src/lib.rs`](references/rust/crates/plugins/src/lib.rs) |
| `references/rust/crates/plugins/src/hooks.rs` | [`references/rust/crates/plugins/src/hooks.rs`](references/rust/crates/plugins/src/hooks.rs) |
| `references/rust/crates/commands/Cargo.toml` | [`references/rust/crates/commands/Cargo.toml`](references/rust/crates/commands/Cargo.toml) |
| `references/rust/crates/commands/src/lib.rs` | [`references/rust/crates/commands/src/lib.rs`](references/rust/crates/commands/src/lib.rs) |
| `references/rust/crates/rusty-claude-cli/Cargo.toml` | [`references/rust/crates/rusty-claude-cli/Cargo.toml`](references/rust/crates/rusty-claude-cli/Cargo.toml) |
| `references/rust/crates/rusty-claude-cli/tests/resume_slash_commands.rs` | [`references/rust/crates/rusty-claude-cli/tests/resume_slash_commands.rs`](references/rust/crates/rusty-claude-cli/tests/resume_slash_commands.rs) |
| `references/rust/crates/rusty-claude-cli/tests/cli_flags_and_config_defaults.rs` | [`references/rust/crates/rusty-claude-cli/tests/cli_flags_and_config_defaults.rs`](references/rust/crates/rusty-claude-cli/tests/cli_flags_and_config_defaults.rs) |
| `references/rust/crates/rusty-claude-cli/tests/mock_parity_harness.rs` | [`references/rust/crates/rusty-claude-cli/tests/mock_parity_harness.rs`](references/rust/crates/rusty-claude-cli/tests/mock_parity_harness.rs) |
| `references/rust/crates/rusty-claude-cli/src/app.rs` | [`references/rust/crates/rusty-claude-cli/src/app.rs`](references/rust/crates/rusty-claude-cli/src/app.rs) |
| `references/rust/crates/rusty-claude-cli/src/init.rs` | [`references/rust/crates/rusty-claude-cli/src/init.rs`](references/rust/crates/rusty-claude-cli/src/init.rs) |
| `references/rust/crates/rusty-claude-cli/src/main.rs` | [`references/rust/crates/rusty-claude-cli/src/main.rs`](references/rust/crates/rusty-claude-cli/src/main.rs) |
| `references/rust/crates/rusty-claude-cli/src/args.rs` | [`references/rust/crates/rusty-claude-cli/src/args.rs`](references/rust/crates/rusty-claude-cli/src/args.rs) |
| `references/rust/crates/rusty-claude-cli/src/input.rs` | [`references/rust/crates/rusty-claude-cli/src/input.rs`](references/rust/crates/rusty-claude-cli/src/input.rs) |
| `references/rust/crates/rusty-claude-cli/src/render.rs` | [`references/rust/crates/rusty-claude-cli/src/render.rs`](references/rust/crates/rusty-claude-cli/src/render.rs) |
| `references/rust/crates/rusty-claude-cli/.claw/sessions/session-newer.jsonl` | [`references/rust/crates/rusty-claude-cli/.claw/sessions/session-newer.jsonl`](references/rust/crates/rusty-claude-cli/.claw/sessions/session-newer.jsonl) |
| `references/rust/crates/telemetry/Cargo.toml` | [`references/rust/crates/telemetry/Cargo.toml`](references/rust/crates/telemetry/Cargo.toml) |
| `references/rust/crates/telemetry/src/lib.rs` | [`references/rust/crates/telemetry/src/lib.rs`](references/rust/crates/telemetry/src/lib.rs) |
| `references/rust/.omc/plans/tui-enhancement-plan.md` | [`references/rust/.omc/plans/tui-enhancement-plan.md`](references/rust/.omc/plans/tui-enhancement-plan.md) |
| `references/rust/.claude/sessions/session-1775009431231.json` | [`references/rust/.claude/sessions/session-1775009431231.json`](references/rust/.claude/sessions/session-1775009431231.json) |
| `references/rust/.claude/sessions/session-1775009769569.json` | [`references/rust/.claude/sessions/session-1775009769569.json`](references/rust/.claude/sessions/session-1775009769569.json) |
| `references/rust/.claude/sessions/session-1775008071886.json` | [`references/rust/.claude/sessions/session-1775008071886.json`](references/rust/.claude/sessions/session-1775008071886.json) |
| `references/rust/.claude/sessions/session-1775008464519.json` | [`references/rust/.claude/sessions/session-1775008464519.json`](references/rust/.claude/sessions/session-1775008464519.json) |
| `references/rust/.claude/sessions/session-1775008308936.json` | [`references/rust/.claude/sessions/session-1775008308936.json`](references/rust/.claude/sessions/session-1775008308936.json) |
| `references/rust/.claude/sessions/session-1775011146355.json` | [`references/rust/.claude/sessions/session-1775011146355.json`](references/rust/.claude/sessions/session-1775011146355.json) |
| `references/rust/.claude/sessions/session-1775012687059.json` | [`references/rust/.claude/sessions/session-1775012687059.json`](references/rust/.claude/sessions/session-1775012687059.json) |
| `references/rust/.claude/sessions/session-1775010047738.json` | [`references/rust/.claude/sessions/session-1775010047738.json`](references/rust/.claude/sessions/session-1775010047738.json) |
| `references/rust/.claude/sessions/session-1775008137143.json` | [`references/rust/.claude/sessions/session-1775008137143.json`](references/rust/.claude/sessions/session-1775008137143.json) |
| `references/rust/.claude/sessions/session-1775008007069.json` | [`references/rust/.claude/sessions/session-1775008007069.json`](references/rust/.claude/sessions/session-1775008007069.json) |
| `references/rust/.claude/sessions/session-1775012674485.json` | [`references/rust/.claude/sessions/session-1775012674485.json`](references/rust/.claude/sessions/session-1775012674485.json) |
| `references/rust/.claude/sessions/session-1775010384918.json` | [`references/rust/.claude/sessions/session-1775010384918.json`](references/rust/.claude/sessions/session-1775010384918.json) |
| `references/rust/.claude/sessions/session-1775009841982.json` | [`references/rust/.claude/sessions/session-1775009841982.json`](references/rust/.claude/sessions/session-1775009841982.json) |
| `references/rust/.claude/sessions/session-1775011562247.json` | [`references/rust/.claude/sessions/session-1775011562247.json`](references/rust/.claude/sessions/session-1775011562247.json) |
| `references/rust/.claude/sessions/session-1775010333630.json` | [`references/rust/.claude/sessions/session-1775010333630.json`](references/rust/.claude/sessions/session-1775010333630.json) |
| `references/rust/.claude/sessions/session-1775007981374.json` | [`references/rust/.claude/sessions/session-1775007981374.json`](references/rust/.claude/sessions/session-1775007981374.json) |
| `references/rust/.claude/sessions/session-1775010909274.json` | [`references/rust/.claude/sessions/session-1775010909274.json`](references/rust/.claude/sessions/session-1775010909274.json) |
| `references/rust/.claude/sessions/session-1775008161929.json` | [`references/rust/.claude/sessions/session-1775008161929.json`](references/rust/.claude/sessions/session-1775008161929.json) |
| `references/rust/.claude/sessions/session-1775013221875.json` | [`references/rust/.claude/sessions/session-1775013221875.json`](references/rust/.claude/sessions/session-1775013221875.json) |
| `references/rust/.claude/sessions/session-1775007484031.json` | [`references/rust/.claude/sessions/session-1775007484031.json`](references/rust/.claude/sessions/session-1775007484031.json) |
| `references/rust/.claude/sessions/session-1775009126336.json` | [`references/rust/.claude/sessions/session-1775009126336.json`](references/rust/.claude/sessions/session-1775009126336.json) |
| `references/rust/.claude/sessions/session-1775008997307.json` | [`references/rust/.claude/sessions/session-1775008997307.json`](references/rust/.claude/sessions/session-1775008997307.json) |
| `references/rust/.claude/sessions/session-1775007453382.json` | [`references/rust/.claude/sessions/session-1775007453382.json`](references/rust/.claude/sessions/session-1775007453382.json) |
| `references/rust/.claude/sessions/session-1775008427969.json` | [`references/rust/.claude/sessions/session-1775008427969.json`](references/rust/.claude/sessions/session-1775008427969.json) |
| `references/rust/.claude/sessions/session-1775009119214.json` | [`references/rust/.claude/sessions/session-1775009119214.json`](references/rust/.claude/sessions/session-1775009119214.json) |
| `references/rust/.claude/sessions/session-1775007490104.json` | [`references/rust/.claude/sessions/session-1775007490104.json`](references/rust/.claude/sessions/session-1775007490104.json) |
| `references/rust/.claude/sessions/session-1775009145469.json` | [`references/rust/.claude/sessions/session-1775009145469.json`](references/rust/.claude/sessions/session-1775009145469.json) |
| `references/rust/.claude/sessions/session-1775009869734.json` | [`references/rust/.claude/sessions/session-1775009869734.json`](references/rust/.claude/sessions/session-1775009869734.json) |
| `references/rust/scripts/run_mock_parity_diff.py` | [`references/rust/scripts/run_mock_parity_diff.py`](references/rust/scripts/run_mock_parity_diff.py) |
| `references/rust/scripts/run_mock_parity_harness.sh` | [`references/rust/scripts/run_mock_parity_harness.sh`](references/rust/scripts/run_mock_parity_harness.sh) |
| `references/rust/.clawd-todos.json` | [`references/rust/.clawd-todos.json`](references/rust/.clawd-todos.json) |
| `references/rust/PARITY.md` | [`references/rust/PARITY.md`](references/rust/PARITY.md) |
| `references/rust/mock_parity_scenarios.json` | [`references/rust/mock_parity_scenarios.json`](references/rust/mock_parity_scenarios.json) |
| `references/rust/.sandbox-home/.rustup/settings.toml` | [`references/rust/.sandbox-home/.rustup/settings.toml`](references/rust/.sandbox-home/.rustup/settings.toml) |
| `references/rust/Cargo.toml` | [`references/rust/Cargo.toml`](references/rust/Cargo.toml) |
| `references/rust/.gitignore` | [`references/rust/.gitignore`](references/rust/.gitignore) |
| `references/rust/MOCK_PARITY_HARNESS.md` | [`references/rust/MOCK_PARITY_HARNESS.md`](references/rust/MOCK_PARITY_HARNESS.md) |
| `references/rust/Cargo.lock` | [`references/rust/Cargo.lock`](references/rust/Cargo.lock) |
