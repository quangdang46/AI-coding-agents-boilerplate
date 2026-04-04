# BRAND_CONFIG

This document defines the branded configuration and instruction surfaces that AICD should support.

It is based on:

- official Claude Code documentation on settings, subagents, and plugins
- direct source evidence from `references/`
- explicit AICD interoperability decisions where we intentionally standardize beyond what the docs state

It is not based on invented boilerplate structure.

The key rule is:

- do not hardcode upstream brands like `.claude` or `.claw` into the generated boilerplate
- do not invent a boilerplate-native brand like `.aicd`
- replace branded source surfaces with a dynamic user/tool brand token: `.<brand>` and `<BRAND>.md`

Examples:

- brand `join` → `.<brand>` becomes `.join`, `<BRAND>.md` becomes `JOIN.md`
- brand `acme` → `.<brand>` becomes `.acme`, `<BRAND>.md` becomes `ACME.md`

---

## 0. Official Claude Code evidence from the web

Primary official docs:

- Claude Code settings: <https://code.claude.com/docs/en/settings>
- Claude Code subagents: <https://code.claude.com/docs/en/subagents>
- Claude Code plugins: <https://code.claude.com/docs/en/plugins>
- Claude Code plugins reference: <https://code.claude.com/docs/en/plugins-reference>

Important facts confirmed by the official docs:

### 0.1 `settings.json` is a real Claude settings surface, not an invented schema bucket

Official settings docs say Claude Code uses hierarchical JSON settings files:

- `~/.claude/settings.json`
- `./.claude/settings.json`
- `./.claude/settings.local.json`

Those files are for merged runtime settings such as:

- `permissions.allow`
- `permissions.ask`
- `permissions.deny`
- `permissions.defaultMode`
- `additionalDirectories`

This means:

- `settings.json` and `settings.local.json` must be treated as Claude-style merged settings surfaces
- they are not the place to invent an AICD-specific metadata schema like custom instruction registries, runtime-root declarations, or agent catalogs unless upstream-compatible keys actually exist

#### 0.1.1 The real upstream schema is broad, optional, and backward-compatible

Source evidence:

- [settings/types.ts](/home/quangdang/projects/aicd/references/typescript/src/utils/settings/types.ts)
- [settings/constants.ts](/home/quangdang/projects/aicd/references/typescript/src/utils/settings/constants.ts)

The upstream TypeScript source defines `SettingsSchema` as:

- a top-level JSON object
- with many optional fields
- with `.passthrough()` enabled
- with explicit backward-compatibility comments saying new optional fields are expected over time

This means:

- AICD must not model `settings.json` as a narrow, closed schema
- unknown keys may be preserved intentionally by upstream
- the absence of a key from a single parser or runtime slice is not proof that the key is globally invalid

#### 0.1.2 Publicly evidenced settings categories

From the upstream TypeScript schema and official docs, `settings.json` can carry categories such as:

- authentication helpers:
  - `apiKeyHelper`
  - `awsCredentialExport`
  - `awsAuthRefresh`
  - `gcpAuthRefresh`
- environment:
  - `env`
- attribution and git behavior:
  - `attribution`
  - `includeCoAuthoredBy`
  - `includeGitInstructions`
- model and response behavior:
  - `model`
  - `availableModels`
  - `modelOverrides`
  - `outputStyle`
  - `language`
  - `alwaysThinkingEnabled`
  - `effortLevel`
- permissions:
  - `permissions.allow`
  - `permissions.deny`
  - `permissions.ask`
  - `permissions.defaultMode`
  - `permissions.additionalDirectories`
  - `permissions.disableBypassPermissionsMode`
- hooks and status line:
  - `hooks`
  - `statusLine`
  - `disableAllHooks`
- MCP and plugin policy:
  - `enabledMcpjsonServers`
  - `disabledMcpjsonServers`
  - `allowedMcpServers`
  - `deniedMcpServers`
  - `enabledPlugins`
  - `pluginConfigs`
  - `extraKnownMarketplaces`
  - `strictKnownMarketplaces`
  - `blockedMarketplaces`
- worktree and sandbox:
  - `worktree`
  - `sandbox`
- session / UX / local behavior:
  - `cleanupPeriodDays`
  - `defaultShell`
  - `spinnerTipsEnabled`
  - `spinnerVerbs`
  - `prefersReducedMotion`
- agent and remote behavior:
  - `agent`
  - `remote`
  - `sshConfigs`

This list is evidence of the type of schema `settings.json` really has.

It is not a license for AICD to redefine those keys semantically.

### 0.2 `CLAUDE.md` is an instruction surface, separate from `settings.json`

Official settings docs explicitly say custom instructions should be added with:

- `CLAUDE.md`
- `--append-system-prompt`

This confirms:

- `CLAUDE.md` is an instruction-memory surface
- `settings.json` is not the canonical home for free-form instruction text

### 0.3 Claude subagents are Markdown files with YAML frontmatter

Official subagents docs say:

- subagents are defined in Markdown files with YAML frontmatter
- project scope root is `.claude/agents/`
- user scope root is `~/.claude/agents/`

This confirms:

- subagent definitions are not JSON config objects
- a branded equivalent must preserve the concept “agent definitions are Markdown files under `.<brand>/agents/`”

### 0.4 Custom slash commands are Markdown files under `commands/`

Official slash-command docs say:

- project commands live in `.claude/commands/`
- personal commands live in `~/.claude/commands/`
- custom slash commands are single Markdown files
- command names are derived from Markdown filenames
- directory structures can be used for namespacing

This confirms:

- a branded equivalent must preserve the concept “commands are Markdown files under `.<brand>/commands/`”
- unlike Skills, commands are not directory bundles centered on `SKILL.md`

### 0.5 Skills are directory bundles centered on `SKILL.md`

Official skills docs say:

- project Skills live in `.claude/skills/`
- personal Skills live in `~/.claude/skills/`
- each Skill is a directory containing `SKILL.md`
- optional supporting files may sit beside `SKILL.md`

This confirms:

- a branded equivalent should preserve the concept `.<brand>/skills/<skill-name>/SKILL.md`
- `.agents/skills/...` can only be justified as an interoperability mirror, not as the primary branded root

### 0.6 Plugin agents live in `agents/` and plugin settings live in `settings.json`

Official plugin docs and plugin reference say:

- plugin manifest lives at `.claude-plugin/plugin.json`
- plugin agents live in plugin-root `agents/`
- plugin skills live in `skills/<name>/SKILL.md`
- plugin root may also include `settings.json`
- plugin-root `settings.json` currently supports only the `agent` setting

This confirms:

- `.<brand>-plugin/plugin.json` is the correct branded plugin manifest target
- `settings.json` exists as a real upstream plugin surface too
- agent settings in plugin `settings.json` are upstream-defined, but broad custom settings schemas are not justified

### 0.7 The docs prove the roots, but not a single mandatory flat file shape for agents

The official subagents docs prove the discovery root `.claude/agents/`, but they do not by themselves prove that:

- only flat `agents/*.md` is valid, or
- nested agent subdirectories are forbidden, or
- nested agent subdirectories are canonical

For nested behavior, we need source evidence.

---

## 1. Source-evidenced branded surfaces

The original/source-derived code supports branded surfaces in these categories.

### 1.1 Config discovery

Evidence:

- [config.rs](/home/quangdang/projects/aicd/references/rust/crates/runtime/src/config.rs)

The source runtime loads config from:

- user legacy file: `.<brand>.json`
- user config home: `~/.<brand>/settings.json`
- project legacy file: `./.<brand>.json`
- project config file: `./.<brand>/settings.json`
- project local override: `./.<brand>/settings.local.json`

The source code currently shows this with `.claw` and `.claw.json`:

- [config.rs:207](/home/quangdang/projects/aicd/references/rust/crates/runtime/src/config.rs#L207)
- [config.rs:223](/home/quangdang/projects/aicd/references/rust/crates/runtime/src/config.rs#L223)
- [config.rs:227](/home/quangdang/projects/aicd/references/rust/crates/runtime/src/config.rs#L227)
- [config.rs:231](/home/quangdang/projects/aicd/references/rust/crates/runtime/src/config.rs#L231)
- [config.rs:443](/home/quangdang/projects/aicd/references/rust/crates/runtime/src/config.rs#L443)

#### 1.1.1 Subset actively consumed by the Rust reference runtime

Source evidence:

- [config.rs](/home/quangdang/projects/aicd/references/rust/crates/runtime/src/config.rs)

The Rust reference runtime currently parses and actively consumes a subset including:

- `model`
- `mcpServers`
- `hooks`
- `permissions.allow`
- `permissions.deny`
- `permissions.ask`
- `permissions.defaultMode`
- top-level `permissionMode` as a compatibility alias
- `sandbox`
- `oauth`
- `enabledPlugins`
- `plugins.enabled`
- `plugins.externalDirectories`
- `plugins.installRoot`
- `plugins.registryPath`
- `plugins.bundledRoot`

This is important because:

- AICD should distinguish “full upstream settings schema” from “subset currently consumed by one runtime slice”
- branded scaffolding may safely emit empty `{}` settings files
- branded scaffolding must not invent unrelated top-level metadata just because `settings.json` exists

### 1.2 Instruction / prompt discovery

Evidence:

- [prompt.rs](/home/quangdang/projects/aicd/references/rust/crates/runtime/src/prompt.rs)

The source runtime discovers instruction files from:

- `<BRAND>.md`
- `<BRAND>.local.md`
- `./.<brand>/<BRAND>.md`
- `./.<brand>/instructions.md`

The source code currently shows this with `CLAUDE.md` and `.claw/`:

- [prompt.rs:203](/home/quangdang/projects/aicd/references/rust/crates/runtime/src/prompt.rs#L203)
- [prompt.rs:204](/home/quangdang/projects/aicd/references/rust/crates/runtime/src/prompt.rs#L204)
- [prompt.rs:206](/home/quangdang/projects/aicd/references/rust/crates/runtime/src/prompt.rs#L206)
- [prompt.rs:207](/home/quangdang/projects/aicd/references/rust/crates/runtime/src/prompt.rs#L207)

### 1.3 Agent definition discovery

Evidence:

- [commands lib](/home/quangdang/projects/aicd/references/rust/crates/commands/src/lib.rs)
- [loadAgentsDir.ts](/home/quangdang/projects/aicd/references/typescript/src/tools/AgentTool/loadAgentsDir.ts)
- [markdownConfigLoader.ts](/home/quangdang/projects/aicd/references/typescript/src/utils/markdownConfigLoader.ts)
- [plugins reference web doc](https://code.claude.com/docs/en/plugins-reference)

The source command layer discovers agent definitions from:

- project `.codex/agents`
- project `.claude/agents`
- user `$CODEX_HOME/agents`
- user `~/.codex/agents`
- user `~/.claude/agents`

Relevant lines:

- [lib.rs:2272](/home/quangdang/projects/aicd/references/rust/crates/commands/src/lib.rs#L2272)
- [lib.rs:2284](/home/quangdang/projects/aicd/references/rust/crates/commands/src/lib.rs#L2284)
- [lib.rs:2306](/home/quangdang/projects/aicd/references/rust/crates/commands/src/lib.rs#L2306)

For AICD brand-generalization, the branded compatibility equivalent is:

- project `./.<brand>/agents/`
- user `~/.<brand>/agents/`

Important refinement from the TypeScript loader:

- Claude’s Markdown agent loader recursively collects all `.md` files under the `agents/` root
- plugin validation code also treats `commands/agents` as recursively scanned Markdown trees

This means the safe statement is:

- root discovery is `./.<brand>/agents/`
- loader-supported file shape is any Markdown file anywhere under that tree

Examples that are source-compatible:

- `./.<brand>/agents/reviewer.md`
- `./.<brand>/agents/reviewer/main.md`
- `./.<brand>/agents/reviewer/prompt.md`
- `./.<brand>/agents/security/reviewer.md`

So the previous shorthand `./.<brand>/agents/` was incomplete as a shape description, but it remains the correct discovery root.

Important:

- AICD should not require an extra mandatory agent-name directory level
- flat `agents/*.md` and nested `agents/**.md` are both consistent with source-supported recursive discovery
- generated layouts may choose flat or nested organization, but the runtime should not depend on a mandatory extra folder layer

### 1.4 Skill and command discovery

Evidence:

- [commands lib](/home/quangdang/projects/aicd/references/rust/crates/commands/src/lib.rs)
- [skills web doc](https://docs.claude.com/en/docs/claude-code/skills)
- [slash commands web doc](https://docs.claude.com/en/docs/claude-code/slash-commands)

The source command layer discovers skills and legacy command-backed skills from:

- project `.codex/skills`
- project `.claude/skills`
- project `.codex/commands`
- project `.claude/commands`
- user `$CODEX_HOME/skills`
- user `$CODEX_HOME/commands`
- user `~/.codex/skills`
- user `~/.codex/commands`
- user `~/.claude/skills`
- user `~/.claude/commands`

Relevant lines:

- [lib.rs:2313](/home/quangdang/projects/aicd/references/rust/crates/commands/src/lib.rs#L2313)
- [lib.rs:2326](/home/quangdang/projects/aicd/references/rust/crates/commands/src/lib.rs#L2326)
- [lib.rs:2338](/home/quangdang/projects/aicd/references/rust/crates/commands/src/lib.rs#L2338)
- [lib.rs:2376](/home/quangdang/projects/aicd/references/rust/crates/commands/src/lib.rs#L2376)
- [lib.rs:2382](/home/quangdang/projects/aicd/references/rust/crates/commands/src/lib.rs#L2382)

For AICD brand-generalization, the branded compatibility equivalent is:

- project `./.<brand>/skills/`
- project `./.<brand>/commands/`
- user `~/.<brand>/skills/`
- user `~/.<brand>/commands/`

Shape clarification from official docs:

- Skills should use `./.<brand>/skills/<skill-name>/SKILL.md`
- Commands should use Markdown files under `./.<brand>/commands/`, with nested directories allowed for namespacing

### 1.5 Managed sessions

Evidence:

- [session_control.rs](/home/quangdang/projects/aicd/references/rust/crates/runtime/src/session_control.rs)

Managed sessions live under:

- `./.<brand>/sessions/`

The source code currently shows this with `.claw/sessions`:

- [session_control.rs:83](/home/quangdang/projects/aicd/references/rust/crates/runtime/src/session_control.rs#L83)

### 1.6 Plugin manifests

Evidence:

- [plugins lib](/home/quangdang/projects/aicd/references/rust/crates/plugins/src/lib.rs)

Plugin manifests live under:

- `./.<brand>-plugin/plugin.json`

The source code currently shows this with `.claude-plugin/plugin.json`:

- [plugins lib:21](/home/quangdang/projects/aicd/references/rust/crates/plugins/src/lib.rs#L21)

### 1.7 Init scaffolding

Evidence:

- [rusty-claude-cli init](/home/quangdang/projects/aicd/references/rust/crates/rusty-claude-cli/src/init.rs)

The upstream init flow scaffolds:

- `./.claude/`
- `./.claude.json`
- `./CLAUDE.md`

Relevant lines:

- [init.rs:83](/home/quangdang/projects/aicd/references/rust/crates/rusty-claude-cli/src/init.rs#L83)
- [init.rs:89](/home/quangdang/projects/aicd/references/rust/crates/rusty-claude-cli/src/init.rs#L89)
- [init.rs:101](/home/quangdang/projects/aicd/references/rust/crates/rusty-claude-cli/src/init.rs#L101)

For AICD brand-generalization, this should become:

- `./.<brand>/`
- `./.<brand>.json`
- `./<BRAND>.md`

---

## 2. Canonical AICD brand-generalized model

Based on the source evidence above, AICD should support the following branded surfaces.

### 2.1 Primary branded surfaces

The generated project SHOULD use:

- `./.<brand>/settings.json`
- `./.<brand>/settings.local.json`
- `./.<brand>.json`
- `./<BRAND>.md`
- `./.<brand>/instructions.md`
- `./.<brand>/agents/`
- `./.<brand>/skills/<skill-name>/SKILL.md`
- `./.<brand>/commands/`
- `./.<brand>/sessions/`
- `./.<brand>-plugin/plugin.json`

This is the primary brand-aware model.

Clarification:

- `./.<brand>/agents/` is the discovery root
- agent files are Markdown with YAML frontmatter somewhere under that root
- because the loader is recursive, the runtime should treat nested Markdown files under `agents/` as valid
- but it should not require one extra directory level
- `./.<brand>/commands/` holds Markdown command files, potentially namespaced by subdirectory
- `./.<brand>/skills/<skill-name>/SKILL.md` is the source-aligned Skill shape
- `./.<brand>/settings.json` and `./.<brand>/settings.local.json` should default to empty objects or only upstream-compatible settings keys

### 2.2 Compatibility instruction surfaces

The runtime SHOULD still support reading:

- `AGENTS.md`
- `CLAUDE.md`
- `<BRAND>.md`

Reason:

- `AGENTS.md` is the current repo-governance and multi-agent contract surface in this workspace
- `CLAUDE.md` is a strong compatibility surface from the upstream ecosystem
- `<BRAND>.md` is the branded canonical instruction file for the generated tool

Important:

- `AGENTS.md` support is a current AICD interoperability decision
- it is not directly proven by the upstream Claude source shown above
- it should therefore be documented as an intentional compatibility surface, not as source-derived Claude behavior

### 2.3 Compatibility discovery roots

The runtime/tooling SHOULD support these discovery families:

- branded roots: `./.<brand>/agents`, `./.<brand>/skills`, `./.<brand>/commands`
- Claude compatibility: `./.claude/agents`, `./.claude/skills`, `./.claude/commands`
- Codex compatibility when needed: `./.codex/agents`, `./.codex/skills`, `./.codex/commands`

Equivalent user/home roots MAY also be supported:

- `~/.<brand>/agents`, `~/.<brand>/skills`, `~/.<brand>/commands`
- `~/.claude/agents`, `~/.claude/skills`, `~/.claude/commands`
- `~/.codex/agents`, `~/.codex/skills`, `~/.codex/commands`

For agents specifically, discovery should be recursive within the discovered root.

---

## 3. What AICD should not invent

The following are not justified by the source evidence and should not be introduced as if they were canonical:

- a boilerplate-native hidden root such as `.aicd`
- treating `.agents` as a full branded runtime root
- inventing arbitrary branded subtrees like `.<brand>/context`, `.<brand>/usage`, or `.<brand>/prompts` unless and until actual runtime code is designed to load them
- inventing a custom top-level schema for `settings.json` / `settings.local.json` that is not Claude-compatible

In particular:

- `.agents` may still exist as a generic skill/interoperability surface if AICD chooses
- but `.agents` must not be confused with the primary branded runtime/config root from the source model

---

## 4. Minimum supported structure for `.<brand>`

This is the minimum structure that is directly supported by source evidence once brand-generalized:

```text
<project>/
├─ <BRAND>.md
├─ .<brand>.json
├─ .<brand>/
│  ├─ settings.json
│  ├─ settings.local.json
│  ├─ instructions.md
│  ├─ <BRAND>.md
│  ├─ agents/
│  │  └─ **/*.md
│  ├─ skills/
│  │  └─ <skill-name>/
│  │     └─ SKILL.md
│  ├─ commands/
│  │  └─ **/*.md
│  └─ sessions/
└─ .<brand>-plugin/
   └─ plugin.json
```

This is the strongest safe structure to standardize today from the evidence that exists.

---

## 5. Decision summary

The correct direction is:

- use `.<brand>` and `<BRAND>.md` as dynamic branded surfaces
- treat `.claw`, `.claude`, and future branded variants as the same category of branded root
- keep `AGENTS.md`, `CLAUDE.md`, and `<BRAND>.md` readable as compatibility instruction surfaces
- keep agent definitions as Markdown with YAML frontmatter under the branded `agents/` root
- support recursive agent discovery under `.<brand>/agents/`
- allow both flat and nested agent Markdown layouts under `.<brand>/agents/`
- keep `.agents/skills/...` only if AICD intentionally wants a generic skill surface
- do not hardcode `.aicd`
- do not pretend `.agents` is the same thing as the source-derived branded runtime root
- do not invent a non-Claude schema for `settings.json`
