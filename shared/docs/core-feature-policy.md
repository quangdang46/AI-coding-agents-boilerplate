# Core vs Feature-Pack Policy

This document defines what may remain in shipped AICD core and what must be extracted into optional feature packs.

It is the canonical policy for Task 5 of `.sisyphus/plans/canonical-migration-plan.md`.

---

## 1. Minimal core policy

Shipped core must stay minimal.

Core is limited to semantics that are required for every language pack to satisfy `RULES.md` and produce a valid generated project.

### 1.1 Allowed core semantics

The following capability families are allowed in core when they are implemented:

- `interactive-repl`
- `local-agent-discovery`
- `session-management-and-export`
- `workspace-bootstrap-init`
- `workspace-file-shell-web-tools`
- `web-fetch-and-search`
- `local-skill-discovery`
- `workspace-instruction-memory`
- `system-prompt-layering`
- `provider-and-model-selection`
- `usage-and-cost-tracking`
- `saved-sessions-and-resume`
- `workspace-context-building`
- `permissions-and-sandbox-safety`
- `workspace-editable-extension-seams`
- `schema-backed-validation`

These map to the minimal cross-language semantics already locked in `RULES.md`:

- app
- prompts
- tools
- providers
- agents
- skills
- features

### 1.2 What core must not absorb

The following are not allowed to enter core without an explicit product decision and a policy update:

- plugin marketplace and plugin lifecycle UIs
- MCP integration breadth
- hook runtime breadth
- Git/GitHub automation flows
- bridge and remote-control stacks
- onboarding/auth app-install flows
- multi-agent task/team orchestration
- rich diagnostics/status dashboards
- memory systems beyond the minimal session/resume baseline
- broad archived clusters such as `services`, `components`, `hooks`, `commands`, `tools`, or `utils`

---

## 2. Feature-pack policy

Anything not required for minimal core must default to `feature-pack`, `deferred`, or `reference-only`.

### 2.1 Required properties of a shipped feature pack

A shipped feature pack must:

- have a stable feature id
- declare dependencies explicitly
- declare conflicts explicitly when relevant
- be reversible through CLI add/remove behavior
- identify the concrete files/assets it adds
- identify the concrete config/runtime patches it applies
- avoid storing derived verification or parity state inside the pack manifest

### 2.2 Initial proving-slice feature catalog

The first proving-slice catalog for AICD should treat the following as feature-pack families:

- `doctor-ui`
- `oauth-onboarding`
- `specialist-agents`
- `advanced-planning`
- `multi-agent-workflows`
- `swarm-permissions`
- `assistant-session-history`
- `structured-remote-transport`
- `git-workflows`
- `mcp-integration`
- `lsp-tooling`
- `session-memory`
- `team-memory`
- `hooks-runtime`
- `plugin-runtime`
- `plugin-marketplace-ui`
- `bridge-remote-control`
- `upstream-proxy`
- `direct-connect-server`
- `prompt-suggestion-services`

These are initial catalog candidates, not promises that every pack is already implemented.

---

## 3. Reference-only and deferred policy

### 3.1 Reference-only

The following remain `reference-only` until intentionally decomposed and re-extracted:

- `slash-command-surface`
- `tool-platform-breadth`
- `services-cluster-decomposition`
- `components-cluster-decomposition`
- `hooks-cluster-decomposition`
- `command-inventory-decomposition`
- `tool-inventory-decomposition`
- `utils-cluster-decomposition`

These rows exist specifically to prevent wholesale migration of broad source trees.

### 3.2 Deferred

Deferred capability rows are acknowledged but intentionally postponed, for example:

- `rich-terminal-ui`
- `companion-affordances`
- `model-migration-rules`
- `analytics-and-telemetry`
- `native-performance-modules`

---

## 4. Reversibility rule

If a capability is classified as `feature-pack`, its manifest and implementation must be compatible with reversible add/remove flows.

At minimum, a feature-pack manifest must support:

- dependency declarations
- conflict declarations when needed
- added assets under `adds`
- patch instructions that can be reversed safely

Feature packs that cannot be removed without leaving broken config or orphaned files are not complete.

---

## 5. Decision rule

When deciding between `core` and `feature-pack`, use this rule:

- choose `core` only if the capability is required for every valid generated project and maps directly to `RULES.md` base semantics
- otherwise choose `feature-pack`, `deferred`, or `reference-only`

If a broad source area is being proposed as one pack, stop and decompose it first.
