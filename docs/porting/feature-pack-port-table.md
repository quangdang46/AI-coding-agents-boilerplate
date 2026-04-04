# Feature-Pack Port Table

Generated from `shared/docs/capability-matrix.json` and the bead graph. This table is the exhaustive optional-capability control surface for the migration.

It is a family-coverage table, not proof of shipped parity.

Under the current repository contract:

- feature-pack delivery is add-only
- a feature may ship one MVP skill authored under `features/<feature-id>/skill/SKILL.md`
- that skill materializes into `.agents/skills/<skill-name>/SKILL.md`

| Feature ID | Capability IDs | User-Facing Scope | Owning Bead | Current State | Notes |
| --- | --- | --- | --- | --- | --- |
| `advanced-planning` | `advanced-planning-modes`<br>`planning-and-todo-tools` | Planning modes and deep-plan workflows<br>Planning, todo, and workflow control tools | `Design and ship the advanced planning feature-pack family` | `declared` | Explicit ultraplan/plan-mode behavior should remain optional. |
| `assistant-session-history` | `assistant-session-history` | Session history around assistant operation | `Design and ship memory, team-memory, compaction, hook-runtime, and prompt-suggestion families` | `declared` | Separate from core resume/session persistence semantics. |
| `bridge-remote-control` | `bridge-remote-session-control` | Bridge / remote session / relay functionality | `Design and ship the remote transport, bridge, proxy, and direct-connect feature-pack family` | `declared` | Explicit optional bridge subsystem. |
| `bundled-skills` | `bundled-skill-pipelines` | Bundled and registry-style skill pipelines | `Design and ship memory, team-memory, compaction, hook-runtime, and prompt-suggestion families` | `declared` | Broader than local skill discovery and can remain optional. |
| `direct-connect-server` | `server-direct-connect` | Server/direct-connect capabilities | `Design and ship the remote transport, bridge, proxy, and direct-connect feature-pack family` | `declared` | Supporting server layer, not minimal core. |
| `doctor-ui` | `doctor-and-resume-screens` | Doctor, resume, and workflow-specific screens | `Design and ship the onboarding and auth UX feature-pack family` | `declared` | Candidate rich diagnostics/status pack rather than base core UI. |
| `external-app-integrations` | `external-app-integrations` | External app and channel integrations | `Design and ship the onboarding and auth UX feature-pack family` | `declared` | Covers Slack/GitHub app/install channel style integrations. |
| `git-workflows` | `git-and-github-workflow-assistance` | Git and GitHub workflow assistance | `Design and ship the Git and GitHub workflow feature-pack family` | `declared` | Optional Git/GitHub workflow automation family. |
| `hooks-runtime` | `hook-runtime` | Hook-driven runtime behavior | `Design and ship memory, team-memory, compaction, hook-runtime, and prompt-suggestion families` | `declared` | Optional extension/runtime behavior. |
| `interactive-clarification-tools` | `ask-user-and-brief-tools` | Ask-user / brief / message tools | `Design and ship the advanced planning feature-pack family` | `declared` | Useful but not minimal core for the first migration wave. |
| `lsp-tooling` | `lsp-code-intelligence` | Code intelligence and language-server support | `Design and ship the LSP tooling feature-pack family` | `declared` | Could be optional depending on language/runtime support. |
| `mcp-integration` | `mcp-resource-tooling`<br>`mcp-integration-subsystem` | MCP tool invocation and MCP resource access<br>MCP integration as a major subsystem | `Design and ship the MCP integration feature-pack family` | `declared` | Explicit optional integration axis from sections 7.1 and 11.2. |
| `multi-agent-workflows` | `task-team-orchestration` | Task and team orchestration | `Design and ship the multi-agent workflow feature-pack family` | `declared` | Maps cleanly to optional agent-team workflows. |
| `oauth-onboarding` | `onboarding-auth-dialogs`<br>`oauth-login-flows` | Onboarding, auth, and guided dialogs<br>OAuth and credential-backed login flows | `Design and ship the onboarding and auth UX feature-pack family` | `declared` | Optional guided setup and auth flows. |
| `plugin-marketplace-ui` | `plugin-marketplace-lifecycle` | Plugin installation, discovery, enable/disable, update, uninstall | `Design and ship the plugin runtime and marketplace feature-pack family` | `declared` | Lifecycle/UI pack distinct from plugin runtime mechanics. |
| `plugin-runtime` | `plugin-runtime-manifests`<br>`plugin-source-kinds` | Plugin manifests, permissions, lifecycle, hooks, tools, and commands<br>Bundled, builtin, and external plugin sources | `Design and ship the plugin runtime and marketplace feature-pack family` | `declared` | Core plugin runtime semantics, separate from marketplace UI. |
| `prompt-suggestion-services` | `magic-docs-and-prompt-suggestions` | Prompt suggestion, magic docs, and summarization helpers | `Design and ship memory, team-memory, compaction, hook-runtime, and prompt-suggestion families` | `declared` | Decomposed service slice rather than full services ecosystem. |
| `session-compaction` | `session-compaction` | Session compaction and token-boundary management | `Design and ship memory, team-memory, compaction, hook-runtime, and prompt-suggestion families` | `declared` | Useful but can remain optional beyond base resume semantics. |
| `session-memory` | `session-memory` | Session memory and relevant-memory retrieval | `Design and ship memory, team-memory, compaction, hook-runtime, and prompt-suggestion families` | `declared` | Explicit optional memory subsystem. |
| `specialist-agents` | `built-in-specialist-agents` | Built-in specialist agents | `Design and ship the multi-agent workflow feature-pack family` | `declared` | Can layer beyond base agent discovery. |
| `streaming-responses` | `streaming-provider-responses` | Streaming model responses | `Design and ship the onboarding and auth UX feature-pack family` | `declared` | Can be optional if first migration slice keeps simpler response handling. |
| `structured-remote-transport` | `structured-remote-cli-transport`<br>`remote-cli-transport-stack` | Structured and remote CLI transports<br>Remote/structured assistant and CLI transport | `Design and ship the remote transport, bridge, proxy, and direct-connect feature-pack family` | `declared` | Separate transport stack rather than base CLI behavior. |
| `swarm-permissions` | `coordinator-permission-modes` | Coordinator and swarm-aware permission handling | `Design and ship the multi-agent workflow feature-pack family` | `declared` | Worker/coordinator permission policy is optional orchestration behavior. |
| `team-memory` | `team-memory` | Team/shared memory concepts | `Design and ship memory, team-memory, compaction, hook-runtime, and prompt-suggestion families` | `declared` | Distinct from local session memory by semantics. |
| `upstream-proxy` | `upstream-proxy-support` | Upstream proxy and remote runtime support | `Design and ship the remote transport, bridge, proxy, and direct-connect feature-pack family` | `declared` | Optional remote runtime/proxy capability. |
