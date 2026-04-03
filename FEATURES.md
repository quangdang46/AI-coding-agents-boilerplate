# Original-Source Feature Inventory

This document inventories the **original-source capability surface** preserved under `references/`. It is intended as a research/backlog artifact for future extraction work.

It is **not** a claim that the current AICD Python or TypeScript boilerplates already implement all of these features. That distinction matters because `RULES.md` defines AICD as a manifest-driven, multi-language boilerplate system with clear separation between shipped code and archived/reference material. In that model, archived material under `references/` is research input, not runtime product surface.

## What this inventory is for

- Preserve visibility into the broader original-source capability surface while the repo migrates toward language packs and feature packs.
- Help future work decide which capabilities belong in base templates versus optional feature packs.
- Provide a grounded starting point for manifest design, per-language feature catalogs, and CLI workflows such as `init`, `feature add`, `feature remove`, and `doctor`.

## Evidence basis and scope

This inventory is grounded in checked-in original-source reference material now canonically preserved under `references/`. `RULES.md` informs how this inventory should be used by the migration, but it is context for interpretation rather than part of the original-source evidence base.

Primary evidence includes:

- `references/source-typescript/README.md`
- `references/parity/source-typescript-parity.md`
- `references/source-python/SNAPSHOT.md`
- `references/source-python/SNAPSHOT.md`
- `references/source-python/SNAPSHOT.md`
- `references/source-python/SNAPSHOT.md`
- `references/source-rust/README.md`
- `references/source-rust/SNAPSHOT.md`
- `references/source-rust/SNAPSHOT.md` preserves the archived Rust implementation tree, including the original `crates/` inventory used for migration evidence

The strongest quantitative evidence from the archived TypeScript inventory is:

- `1902` TypeScript-like files
- `207` command entries
- `184` tool entries
- broad top-level subsystem coverage including `assistant`, `bridge`, `buddy`, `cli`, `commands`, `components`, `hooks`, `memdir`, `plugins`, `screens`, `server`, `services`, `skills`, `tasks`, `tools`, `upstreamproxy`, `voice`, and more

Source: `references/source-python/SNAPSHOT.md`

---

## Current-boilerplate parity note

The current repo direction is intentionally narrower than this inventory.

- `RULES.md` says future shipped architecture should be organized as manifest-driven language packs plus feature packs, with `references/`/reference material kept separate from shipped runtime concerns.
- `references/source-typescript/README.md` explicitly says the Python workspace is **not yet** a full runtime-equivalent replacement for the original TypeScript system.
- `references/parity/source-typescript-parity.md` explicitly says the Rust port has a strong foundation but is **not feature-parity** with the TypeScript CLI and calls out major gaps in plugins, hooks, CLI breadth, skills pipelines, assistant orchestration, and service ecosystem breadth.

This means the lists below should be read as **original-source feature inventory**, not as “already extracted feature packs” or “current boilerplate parity.”

---

## 1. Interaction and UI surface

### 1.1 Interactive REPL and prompt-driven coding sessions

The original/source-derived system supports an interactive local coding-agent experience, not just one-shot automation. That includes REPL-style workflows, prompt entry, resume flows, and session-oriented interaction.

Evidence:

- `references/source-rust/README.md` — “Interactive REPL and one-shot prompt execution”; “Saved-session inspection and resume flows”
- `references/source-rust/SNAPSHOT.md` — “interactive sessions” and “non-interactive prompts”
- `references/source-python/SNAPSHOT.md` — `screens/REPL.tsx`, `screens/ResumeConversation.tsx`
- `references/source-python/SNAPSHOT.md` — root files include `replLauncher.tsx`, `interactiveHelpers.tsx`, `dialogLaunchers.tsx`, `main.tsx`

### 1.2 Rich terminal/UI component system

The original surface includes a substantial terminal UI layer with many dedicated components for progress, dialogs, onboarding, permissions, status, context display, and update flows.

Evidence:

- `references/source-python/SNAPSHOT.md` — `module_count: 389`
- sample component files include `components/App.tsx`, `components/AgentProgressLine.tsx`, `components/AutoModeOptInDialog.tsx`, `components/BridgeDialog.tsx`, `components/ConsoleOAuthFlow.tsx`, `components/ContextVisualization.tsx`, `components/CoordinatorAgentStatus.tsx`, `components/CostThresholdDialog.tsx`

### 1.3 Doctor, resume, and workflow-specific screens

The UI is not generic only; it contains dedicated screens for system health, session resume, and REPL usage.

Evidence:

- `references/source-python/SNAPSHOT.md` — `screens/Doctor.tsx`, `screens/REPL.tsx`, `screens/ResumeConversation.tsx`
- `references/source-python/SNAPSHOT.md` — command families for `doctor`, `resume`, `status`, `config`, `permissions`, `theme`, and onboarding-related flows

### 1.4 Onboarding, auth, and guided dialogs

The original surface includes guided user flows for setup and credentials, not only raw config files.

Evidence:

- `references/source-python/SNAPSHOT.md` — `ApproveApiKey.tsx`, `ConsoleOAuthFlow.tsx`, `ClawInChromeOnboarding.tsx`
- `references/source-python/SNAPSHOT.md` — `login`, `logout`, `install-github-app`, `install-slack-app`, `onboarding`, `remote-setup`

### 1.5 Companion and user-facing helper affordances

The original source also includes lighter user-facing affordances such as companion/buddy behavior and notification helpers.

Evidence:

- `references/source-python/SNAPSHOT.md` — `CompanionSprite.tsx`, `companion.ts`, `useBuddyNotification.tsx`

---

## 2. Agents, planning, and multi-agent workflows

### 2.1 Local agent workflows and agent discovery

The source surface includes explicit agent concepts rather than a single monolithic assistant. Users can discover and work with configured agents.

Evidence:

- `references/source-rust/README.md` — “local agent workflows”; “Local agent and skill discovery with `claw agents` and `claw skills`”
- `references/source-rust/SNAPSHOT.md` — `/agents` and agent-definition discovery across project/user directories
- `references/source-python/SNAPSHOT.md` — `AgentTool`, `loadAgentsDir`, `runAgent`, `resumeAgent`, `forkSubagent`, `builtInAgents`

### 2.2 Built-in specialist agents

The original tool surface includes built-in agent roles aimed at exploration, planning, guidance, and verification.

Evidence:

- `references/source-python/SNAPSHOT.md` — built-in agent entries `clawCodeGuideAgent`, `exploreAgent`, `generalPurposeAgent`, `planAgent`, `verificationAgent`

### 2.3 Planning modes and deep-plan workflows

Planning is a first-class behavior, not only something done ad hoc in prompts. The inventory clearly includes explicit plan mode entry/exit and dedicated planning commands.

Evidence:

- `references/source-python/SNAPSHOT.md` — `EnterPlanModeTool`, `ExitPlanModeV2Tool`
- `references/source-rust/SNAPSHOT.md` — `/ultraplan` command with summary “Run a deep planning prompt with multi-step reasoning”
- `references/source-python/SNAPSHOT.md` — `plan`, `ultraplan`

### 2.4 Task and team orchestration

The original source surface supports explicit task and team management concepts, including creation, listing, output retrieval, updates, stopping, and team lifecycle operations.

Evidence:

- `references/source-python/SNAPSHOT.md` — `TaskCreateTool`, `TaskGetTool`, `TaskListTool`, `TaskOutputTool`, `TaskStopTool`, `TaskUpdateTool`, `TeamCreateTool`, `TeamDeleteTool`
- `references/source-python/SNAPSHOT.md` — `tasks` command family

### 2.5 Coordinator and swarm-aware permission handling

The source material points to orchestration modes where coordination and worker behavior affect permissions and status.

Evidence:

- `references/source-python/SNAPSHOT.md` — `coordinator/coordinatorMode.ts`
- `references/source-python/SNAPSHOT.md` — permission handlers include `coordinatorHandler.ts`, `swarmWorkerHandler.ts`, `interactiveHandler.ts`
- `references/source-python/SNAPSHOT.md` — `CoordinatorAgentStatus.tsx`

### 2.6 Session history around assistant operation

The assistant side includes explicit session-history handling rather than purely ephemeral turn-by-turn execution.

Evidence:

- `references/source-python/SNAPSHOT.md` — `assistant/sessionHistory.ts`
- `references/parity/source-typescript-parity.md` — describes TS assistant/session-history/background-task integration as richer than the Rust port

---

## 3. CLI, command surface, and operator workflows

### 3.1 Broad slash-command and command-module surface

The original source exposes a very broad command surface spanning core flow, workspace management, sessions, Git/GitHub operations, automation, integrations, review, planning, plugins, and environment setup.

Evidence:

- `references/source-python/SNAPSHOT.md` — `command_entry_count: 207`
- `references/source-python/SNAPSHOT.md`
- `references/source-rust/SNAPSHOT.md` — slash command registry with categories `Core`, `Workspace`, `Session`, `Git`, and `Automation`

Representative command families evidenced in the snapshots:

- workspace/config: `config`, `context`, `files`, `doctor`, `diff`, `memory`, `permissions`, `status`, `theme`, `version`
- agent/workflow: `agents`, `skills`, `tasks`, `plan`, `review`, `resume`, `session`
- Git/GitHub: `branch`, `commit`, `commit-push-pr`, `pr_comments`, `install-github-app`
- plugins: `plugin`, `reload-plugins`
- integrations/setup: `mcp`, `install-slack-app`, `remote-env`, `remote-setup`, `terminalSetup`, `ide`, `chrome`, `mobile`, `desktop`

### 3.2 Structured and remote CLI transports

The source surface supports structured and remote transport layers for CLI operation, not just local stdin/stdout prompt handling.

Evidence:

- `references/source-python/SNAPSHOT.md` — `cli/remoteIO.ts`, `cli/structuredIO.ts`
- same file lists transports such as `HybridTransport.ts`, `SSETransport.ts`, `WebSocketTransport.ts`, `SerialBatchEventUploader.ts`, `WorkerStateUploader.ts`, `ccrClient.ts`
- `references/parity/source-typescript-parity.md` — explicitly describes TS structured IO / remote transport stack as broader than Rust parity

### 3.3 Session management, export, compaction, and status

The operator workflow includes managing sessions over time, not just running stateless prompts.

Evidence:

- `references/source-rust/README.md` — “Saved-session inspection and resume flows”; slash commands for “status, compaction, config inspection, diff, export, session management”
- `references/source-rust/SNAPSHOT.md` — `/status`, `/compact`, `/resume`, `/export`, `/session`, `/cost`
- `references/source-python/SNAPSHOT.md` — `compact`, `resume`, `session`, `export`, `status`, `cost`

### 3.4 Git and GitHub workflow assistance

The original source supports repository operations as first-class CLI actions, including branching, commits, pull requests, issues, and worktree flows.

Evidence:

- `references/source-rust/SNAPSHOT.md` — `/branch`, `/worktree`, `/commit`, `/commit-push-pr`, `/pr`, `/issue`, `/diff`
- `references/source-python/SNAPSHOT.md` — `branch`, `commit`, `commit-push-pr`, `review`, `security-review`, `pr_comments`

### 3.5 Initialization and workspace bootstrap

The original surface includes workspace bootstrap/init flows and starter instruction generation.

Evidence:

- `references/source-rust/README.md` — workspace-aware instruction/config loading
- `references/source-rust/SNAPSHOT.md` — `/init` “Create a starter CLAW.md for this repo”
- `references/source-python/SNAPSHOT.md` — `entrypoints/init.ts`

---

## 4. Tooling surface available to the agent runtime

### 4.1 Workspace file and shell tools

The original source includes a broad built-in tool layer for reading, writing, editing, searching, and executing against the workspace.

Evidence:

- `references/source-rust/README.md` — built-in tools for shell, file read/write/edit, search, web fetch/search, todos, notebook updates
- `references/source-python/SNAPSHOT.md` — `BashTool`, `FileReadTool`, `FileWriteTool`, `FileEditTool`, `GlobTool`, `GrepTool`, `NotebookEditTool`, `PowerShellTool`
- `references/source-rust/SNAPSHOT.md` — exports `execute_bash`, `read_file`, `write_file`, `edit_file`, `glob_search`, `grep_search`

### 4.2 Code intelligence and language-server support

The source surface includes code intelligence through LSP-aware tooling.

Evidence:

- `references/source-python/SNAPSHOT.md` — `LSPTool`
- `references/source-rust/README.md` — `lsp` crate for language-server support types and process helpers
- `references/source-rust/SNAPSHOT.md` — exports `LspManager`, `WorkspaceDiagnostics`, `FileDiagnostics`, `SymbolLocation`

### 4.3 MCP tool invocation and MCP resource access

The original surface includes first-class MCP interaction: listing tools/resources, calling tools, and reading MCP-backed resources.

Evidence:

- `references/source-python/SNAPSHOT.md` — `MCPTool`, `ListMcpResourcesTool`, `ReadMcpResourceTool`, `McpAuthTool`
- `references/source-rust/SNAPSHOT.md` — exports MCP config, client, stdio process, list-resources/list-tools/read-resource/tool-call types and managers
- `references/parity/source-typescript-parity.md` — identifies MCP services and MCP resource/tool flows as part of the TypeScript system

### 4.4 Web and remote fetch/search

The source-derived tooling surface includes web content access as part of the agent loop.

Evidence:

- `references/source-python/SNAPSHOT.md` — `WebFetchTool`, `WebSearchTool`, `RemoteTriggerTool`
- `references/source-rust/README.md` — built-in tools include web fetch/search

### 4.5 Planning, todo, and workflow control tools

The tool surface includes runtime workflow controls for plan mode, task execution, todo management, and synthetic/system outputs.

Evidence:

- `references/source-python/SNAPSHOT.md` — `TodoWriteTool`, `EnterPlanModeTool`, `ExitPlanModeV2Tool`, `SyntheticOutputTool`, task/team tools

### 4.6 Ask-user / brief / message tools

The original system includes explicit tools for user questioning, briefing, and message sending, which suggests user-facing clarification and communication behaviors are part of the source surface.

Evidence:

- `references/source-python/SNAPSHOT.md` — `AskUserQuestionTool`, `BriefTool`, `SendMessageTool`

### 4.7 Breadth of tool system overall

This is a large tool platform, not a handful of utility calls.

Evidence:

- `references/source-python/SNAPSHOT.md` — `tool_entry_count: 184`
- `references/parity/source-typescript-parity.md` — tool families span `AgentTool`, `AskUserQuestionTool`, `BashTool`, `ConfigTool`, `FileReadTool`, `FileWriteTool`, `GlobTool`, `GrepTool`, `LSPTool`, `MCPTool`, task/team tools, `TodoWriteTool`, `ToolSearchTool`, `WebFetchTool`, `WebSearchTool`, and more

---

## 5. Skills, prompts, and instruction loading

### 5.1 Local skill discovery and invocation

The source surface includes discoverable skills as a first-class extension seam.

Evidence:

- `references/source-rust/README.md` — local agent and skill discovery
- `references/source-rust/SNAPSHOT.md` — `/skills` command and skill root discovery from project/user directories
- `references/source-python/SNAPSHOT.md` — `SkillTool`

### 5.2 Bundled and registry-style skill pipelines

The TypeScript-side evidence shows that skills were not only local markdown files; there was also a broader bundled/registry pipeline.

Evidence:

- `references/parity/source-typescript-parity.md` — cites `src/skills/loadSkillsDir.ts`, `src/skills/bundledSkills.ts`, `src/skills/mcpSkillBuilders.ts`, bundled skills under `src/skills/bundled/`

### 5.3 Workspace-aware instruction memory files

The source-derived runtimes load repo-local instruction/config context, including `CLAW.md` and other memory/instruction files.

Evidence:

- `references/source-rust/README.md` — “Workspace-aware instruction/config loading (`CLAW.md`, config files, permissions, plugin settings)”
- `references/parity/source-typescript-parity.md` — CLAW.md discovery implemented and skill handling discussed
- `references/source-rust/SNAPSHOT.md` — exports `load_system_prompt`, `SystemPromptBuilder`, `ProjectContext`, `ContextFile`

### 5.4 Prompt construction and system prompt layering

Prompt construction is a dedicated runtime concern rather than hardcoded string assembly.

Evidence:

- `references/source-rust/README.md` — runtime crate includes prompts
- `references/source-rust/SNAPSHOT.md` — prompt/system-prompt exports
- `references/parity/source-typescript-parity.md` — Rust runtime includes prompt construction; TypeScript had richer surrounding orchestration

---

## 6. Providers, models, auth, and usage accounting

### 6.1 Multiple provider families and model/provider selection

The original/source-derived surface includes provider abstraction and model switching, rather than a single hardcoded model backend.

Evidence:

- `references/source-rust/README.md` — model/provider selection from command line
- `references/source-rust/SNAPSHOT.md` — provider trait plus `ProviderKind::{ClawApi, Xai, OpenAi}`
- same file maps aliases and metadata for Claude-family and Grok-family models

### 6.2 OAuth and credential-backed login flows

The source surface includes OAuth login/logout and credential lifecycle behaviors.

Evidence:

- `references/source-rust/README.md` — “OAuth login is also available”; “OAuth login/logout plus model/provider selection”
- `references/source-rust/SNAPSHOT.md` — exports OAuth request/refresh/token types and credential helpers
- `references/source-python/SNAPSHOT.md` — `login`, `logout`, `oauth-refresh`

### 6.3 Streaming model responses

The source-derived runtimes support streaming responses from providers instead of only fully buffered outputs.

Evidence:

- `references/source-typescript/README.md` — Rust workspace includes API client with “streaming support”
- `references/source-rust/README.md` — `api` crate covers provider clients and streaming
- `references/source-rust/SNAPSHOT.md` — `stream_message` in provider trait

### 6.4 Usage and cost tracking

The source surface includes explicit token/cost tracking and reporting.

Evidence:

- `references/source-python/SNAPSHOT.md` — root file `cost-tracker.ts`
- `references/source-python/SNAPSHOT.md` — `cost`, `usage`, `extra-usage`
- `references/source-rust/SNAPSHOT.md` — exports `TokenUsage`, `UsageCostEstimate`, `UsageTracker`, `pricing_for_model`, `format_usd`

### 6.5 Model migration and settings evolution

The original/source-derived surface includes explicit handling for model/version migrations over time.

Evidence:

- `references/source-python/SNAPSHOT.md` — includes `migrateFennecToOpus`, `migrateLegacyOpusToCurrent`, `migrateOpusToOpus1m`, `migrateSonnet1mToSonnet45`, `migrateSonnet45ToSonnet46`

---

## 7. Integrations, remote control, and transport layers

### 7.1 MCP integration as a major subsystem

MCP is a core integration axis across config, bootstrap, client transport, stdio process management, resource reading, and tool execution.

Evidence:

- `references/parity/source-typescript-parity.md` — TS had MCP services under `src/services/mcp/*`
- `references/source-rust/SNAPSHOT.md` — MCP config/bootstrap/client/stdio exports and manager types
- `references/source-python/SNAPSHOT.md` — `mcp` command family

### 7.2 Remote/structured assistant and CLI transport

The original source includes explicit support for remote and structured transport rather than only local interactive terminal mode.

Evidence:

- `references/source-python/SNAPSHOT.md` — `remoteIO.ts`, `structuredIO.ts`, `WebSocketTransport.ts`, `SSETransport.ts`
- `references/parity/source-typescript-parity.md` — TS had remote/structured transport layers not fully matched in Rust

### 7.3 Bridge / remote session / relay functionality

The source surface includes a distinct bridge subsystem for remote or bridged session control, messaging, permissions, and status.

Evidence:

- `references/source-python/SNAPSHOT.md` — includes `bridgeApi.ts`, `bridgeMessaging.ts`, `bridgePermissionCallbacks.ts`, `codeSessionApi.ts`, `createSession.ts`, `remoteBridgeCore.ts`, `replBridge.ts`

### 7.4 Upstream proxy and remote runtime support

The source-derived surface includes proxy/relay support for upstream and remote execution contexts.

Evidence:

- `references/source-python/SNAPSHOT.md` — `relay.ts`, `upstreamproxy.ts`
- `references/source-rust/SNAPSHOT.md` — exports remote/upstream proxy bootstrap and state helpers
- `references/parity/source-typescript-parity.md` — notes remote upstream-proxy support in Rust runtime and richer TS remote stack

### 7.5 Server/direct-connect capabilities

The source surface includes a supporting server layer for direct connection/session management.

Evidence:

- `references/source-python/SNAPSHOT.md` — `createDirectConnectSession.ts`, `directConnectManager.ts`, `types.ts`
- `references/source-rust/README.md` — `server` and `compat-harness` listed as supporting services and compatibility tooling

### 7.6 External app and channel integrations

The command inventory shows explicit integration/setup flows for external systems beyond core model providers.

Evidence:

- `references/source-python/SNAPSHOT.md` — `install-github-app`, `install-slack-app`, `remote-setup`, `chrome`, `desktop`, `mobile`, `ide`

---

## 8. Memory, sessions, context, and runtime behavior

### 8.1 Saved sessions, resume, and local conversation state

The source-derived system maintains local session state and supports later inspection/resume.

Evidence:

- `references/parity/source-typescript-parity.md` — Rust foundation includes “local conversation/session state”
- `references/source-rust/README.md` — “Saved-session inspection and resume flows”
- `references/source-rust/SNAPSHOT.md` — exports `Session`, `ConversationMessage`, `session` module, runtime conversation types

### 8.2 Session memory and relevant-memory retrieval

The original surface includes dedicated session-memory and memory lookup services.

Evidence:

- `references/source-python/SNAPSHOT.md` — `services/SessionMemory/sessionMemory.ts`, `sessionMemoryUtils.ts`, `prompts.ts`
- `references/source-python/SNAPSHOT.md` — `findRelevantMemories.ts`, `memoryScan.ts`, `teamMemPaths.ts`, `teamMemPrompts.ts`

### 8.3 Team/shared memory concepts

The memory model extends beyond a single-user scratchpad to include team/shared memory paths and prompts.

Evidence:

- `references/source-python/SNAPSHOT.md` — `teamMemPaths.ts`, `teamMemPrompts.ts`
- `references/parity/source-typescript-parity.md` — mentions missing TS-equivalent team-memory subsystems on the Rust side

### 8.4 Prompt/context building from workspace state

Runtime behavior includes converting workspace files and metadata into system/user prompt context.

Evidence:

- `references/source-rust/SNAPSHOT.md` — `ProjectContext`, `ContextFile`, `SystemPromptBuilder`
- `references/source-python/SNAPSHOT.md` — root files include `context.ts`, `query.ts`, `QueryEngine.ts`, `history.ts`

### 8.5 Session compaction and token-boundary management

The source-derived runtimes explicitly manage long-running context windows through compaction and token estimation.

Evidence:

- `references/source-rust/SNAPSHOT.md` — exports `compact_session`, `estimate_session_tokens`, `should_compact`, `CompactionConfig`
- `references/source-rust/SNAPSHOT.md` — `/compact`

### 8.6 Permission modes and sandbox/runtime safety

The runtime includes explicit permission models and sandbox-aware execution decisions.

Evidence:

- `references/source-rust/SNAPSHOT.md` — `PermissionMode`, `PermissionPolicy`, `PermissionRequest`, sandbox module
- `references/source-python/SNAPSHOT.md` — `toolPermission/*` handlers and permission logging
- `references/source-python/SNAPSHOT.md` — `permissions`, `sandbox-toggle`

---

## 9. Hooks, plugins, and extension mechanisms

### 9.1 Hook-driven runtime behavior

The original feature surface includes hook-aware behavior around tool execution and runtime decisions.

Evidence:

- `references/parity/source-typescript-parity.md` — preserves the archived TypeScript parity note that TS supports `PreToolUse`, `PostToolUse`, hook-driven behaviors, and `/hooks`
- `references/source-python/SNAPSHOT.md` — preserves the archived Python reference inventory for the hook-heavy subsystem (`module_count: 104`) and the `hooks` command family

### 9.2 Plugin installation, discovery, enable/disable, update, uninstall

The original/source-derived system includes a true plugin lifecycle, not just static extensions checked into the repo.

Evidence:

- `references/source-rust/README.md` — “Plugin discovery and management through the CLI and slash-command surfaces”
- `references/source-rust/SNAPSHOT.md` — preserves the archived Rust commands tree for `/plugin` aliases and actions `list|install|enable|disable|uninstall|update`
- `references/source-python/SNAPSHOT.md` — preserves the archived Python reference inventory for plugin UI/flows such as `BrowseMarketplace`, `ManagePlugins`, `PluginSettings`, `ValidatePlugin`, and `reload-plugins`

### 9.3 Plugin manifests, permissions, lifecycle, hooks, tools, and commands

Plugins are structured packages with metadata, permissions, lifecycle hooks, tool definitions, and command definitions.

Evidence:

- `references/source-rust/SNAPSHOT.md` — preserves the archived Rust plugins tree for `PluginManifest`, `PluginPermission`, `PluginHooks`, `PluginLifecycle`, `PluginToolManifest`, `PluginCommandManifest`, and installed-registry/validation logic

### 9.4 Bundled, builtin, and external plugin sources

The extension model spans multiple plugin origins, which matters for future feature-pack modeling.

Evidence:

- `references/source-rust/SNAPSHOT.md` — preserves the archived Rust plugins tree for `PluginKind::{Builtin, Bundled, External}` and sync/discovery logic for bundled and installed plugins
- `references/parity/source-typescript-parity.md` — preserves the archived TypeScript parity note that TS had builtin/bundled plugin scaffolding and plugin installation/operations services

### 9.5 Skills/agents/plugins as user-editable workspace extension seams

The combined evidence shows a broader extension philosophy: the runtime discovers customizable assets from workspace and user directories instead of hardcoding everything into the binary.

Evidence:

- `references/source-rust/README.md` — workspace-aware loading of instructions/config/plugins
- `references/source-rust/SNAPSHOT.md` — preserves the archived Rust commands tree for agent/skill discovery across project and user directories
- `references/parity/source-typescript-parity.md` — preserves the archived TypeScript parity note for bundled skills/plugins pipelines on the TS side

---

## 10. Services and infrastructure subsystems

### 10.1 Large service ecosystem beyond the core loop

The original source is supported by a broad internal services layer, not just CLI code and tools.

Evidence:

- `references/source-python/SNAPSHOT.md` — preserves the archived Python reference inventory for the services-heavy subsystem (`module_count: 130`)
- sample files include `services/AgentSummary/agentSummary.ts`, `services/MagicDocs/*`, `services/PromptSuggestion/*`, `services/SessionMemory/*`, `services/analytics/*`, `services/api/*`

### 10.2 Analytics, event logging, and telemetry plumbing

The service inventory includes analytics and logging infrastructure.

Evidence:

- `references/source-python/SNAPSHOT.md` — preserves the archived Python reference inventory for `services/analytics/datadog.ts`, `firstPartyEventLogger.ts`, `growthbook.ts`, `sink.ts`, and `sinkKillswitch.ts`

### 10.3 Prompt suggestion, magic docs, and summarization helpers

The service layer includes user-assistance systems for documentation and prompt suggestion.

Evidence:

- `references/source-python/SNAPSHOT.md` — preserves the archived Python reference inventory for `services/MagicDocs/magicDocs.ts`, `services/MagicDocs/prompts.ts`, `services/PromptSuggestion/promptSuggestion.ts`, `services/PromptSuggestion/speculation.ts`, and `services/AgentSummary/agentSummary.ts`

### 10.4 Native/high-performance support modules

The original source also includes native/high-performance helper modules for specific tasks.

Evidence:

- `references/source-python/SNAPSHOT.md` — preserves the archived Python reference inventory for `native-ts/color-diff/index.ts`, `native-ts/file-index/index.ts`, and `native-ts/yoga-layout/*`

### 10.5 Schema-backed configuration and validation

There is explicit schema material supporting runtime behaviors such as hooks and SDK/control types.

Evidence:

- `references/source-python/SNAPSHOT.md` — preserves the archived Python reference inventory for `schemas/hooks.ts`, `entrypoints/sdk/controlSchemas.ts`, `sdk/coreSchemas.ts`, and `sdk/coreTypes.ts`

---

## 11. Feature clusters that look especially promising for future extraction

The following clusters appear well-defined enough to become future AICD feature packs or pack families, subject to product decisions and per-language feasibility:

### 11.1 Base candidate clusters

These are strong candidates for a cross-language base template or minimal first-class semantics because they show up repeatedly across sources and align with `RULES.md` semantics:

- interactive CLI / REPL shell
- session persistence and resume
- prompt/system-instruction loading (`CLAW.md`-style context)
- file/search/shell/web/todo tool baseline
- permissions and config inspection
- model/provider selection and usage tracking

### 11.2 Optional feature-pack candidates

These look more like optional packs than required base behavior:

- plugin marketplace and plugin lifecycle management
- hook execution and hook configuration UX
- MCP server/resource/tool integration
- agent registry and multi-agent task/team workflows
- Git/GitHub workflow automation (`commit`, PR, issue, worktree helpers)
- bridge/remote-control/transport features
- memory/team-memory systems
- onboarding/auth/app-install integrations (GitHub app, Slack app, OAuth-first flows)
- advanced planning/review/ultraplan flows
- diagnostics/doctor/status dashboards and rich UI panels

### 11.3 Extraction caution areas

These clusters are broad enough that they likely need decomposition before being turned into packs:

- `services/` ecosystem (`130` modules in the snapshot)
- `components/` UI layer (`389` modules)
- `utils/` support layer (`564` modules)
- full command inventory (`207` entries)
- full tool inventory (`184` entries)

Those should be split into smaller user-facing capabilities rather than migrated as giant undifferentiated packs.

---

## 12. How to use this inventory for future feature-pack extraction

1. Treat each section above as an **input backlog**, not an implementation promise.
2. Before extracting any pack, define its user-facing semantic contract in the style required by `RULES.md`:
   - feature id
   - description
   - dependencies
   - conflicts
   - files/assets added
   - config/runtime patches needed
3. Prefer extracting **coherent capability slices** rather than source-tree mirrors. For example:
   - “mcp-integration” instead of copying all MCP-adjacent files wholesale
   - “plugin-runtime” and “plugin-marketplace-ui” as separate packs if needed
   - “session-memory” separate from “team-memory” if they have distinct semantics
4. Preserve the distinction between:
   - **original-source capability inventory**
   - **current parity status in each language pack**
   - **shipped feature-pack support in AICD manifests**
5. When a feature is implemented in multiple languages, prefer one shared semantic feature id, even if the implementations differ, consistent with `RULES.md`.

In short: this file should help future work avoid losing the broader original-source feature map while still building AICD as a clean, manifest-driven, multi-language boilerplate system rather than as a direct mirror of the archived source trees.
