# RULES

This document defines the canonical repository rules for AICD as a multi-language boilerplate system.

The goal is to make the repository safe to extend in four directions at the same time:

1. add a new language
2. add new optional features
3. let users initialize projects through the CLI
4. let users add feature packs through the CLI after initialization

These rules are normative. New work MUST follow this contract unless the contract is updated first.

---

## 1. Design goals

The repository MUST support all of the following:

- multiple language packs living in one repo
- one user-facing CLI for `init`, `feature add`, and `doctor`
- language-specific runtime helpers and templates
- language-specific feature packs
- consistent user experience across languages
- clear separation between contributor authoring trees and generated-project runtime trees
- clear separation between shipped code and archived/reference material

The repository MUST NOT depend on ad hoc path knowledge embedded in the CLI.

The CLI MUST discover supported languages and their capabilities from manifests, not from hardcoded per-language logic spread throughout the codebase.

---

## 2. Top-level repository contract

The canonical long-term top-level structure is:

```text
aicd/
├─ install/                   # user-facing CLI / installer / orchestration layer
├─ languages/                 # all supported language packs
│  ├─ python/
│  ├─ typescript/
│  ├─ rust/
│  └─ <future-language>/
├─ shared/                    # shared schemas, shared docs, shared contracts
│  ├─ schemas/
│  └─ docs/
├─ references/                # archived sources, parity notes, imported snapshots
├─ docs/                      # repo-level docs
├─ tests/                     # cross-language and CLI integration tests
└─ scripts/                   # repo maintenance only
```

### Top-level rules

- `install/` owns the user-facing CLI contract.
- `languages/` is the only valid home for new language packs.
- `shared/` contains cross-language contracts, schemas, and shared architectural docs.
- `references/` contains material that is not part of the shipped boilerplate surface.
- generated artifacts, caches, session files, databases, and local dependency directories MUST NOT be treated as part of the architecture.

### Migration compatibility rule

Earlier migration phases may have used top-level language roots such as `python/` and `typescript/`.

The current compatibility rule is:

- no new language may be introduced outside `languages/`
- no new long-term architecture document may assume top-level language roots are permanent
- legacy language workspaces should be retired from repo root and, if still needed for history, preserved under `references/`

---

## 3. Repository roles

The repo is organized by role, not by historical source origin.

### 3.1 `install/`

`install/` owns:

- CLI argument parsing
- language discovery
- project initialization
- feature add orchestration
- validation entrypoints
- user-facing output and workflow

`install/` MUST NOT own language-specific template internals beyond manifest-driven orchestration.

### 3.2 `languages/<id>/`

Each language directory owns:

- its runtime helpers
- its generated project template
- its feature catalog
- its docs
- its language-pack tests

### 3.3 `shared/`

`shared/` owns:

- manifest schemas
- shared terminology
- shared config semantics
- shared feature-pack semantics

### 3.4 `references/`

`references/` owns:

- imported source references
- parity notes
- archived snapshots
- research-only materials

Nothing under `references/` may be required by a shipped language pack at runtime.

---

## 4. Per-language contract

Every language pack MUST have the same minimum contributor-facing shape.

```text
languages/<language-id>/
├─ language.manifest.json
├─ README.md
├─ docs/
│  ├─ config-reference.md
│  ├─ extension-model.md
│  └─ feature-packs.md
├─ runtime/
│  ├─ config/
│  ├─ registry/
│  ├─ prompts/
│  ├─ entrypoints/
│  └─ <other-language-specific-runtime-pieces>/
├─ template/
│  └─ base/
│     ├─ <language-config-file>
│     ├─ src/
│     ├─ tests/
│     └─ README.md
├─ features/
│  ├─ registry.json
│  └─ <feature-id>/
│     ├─ feature.json
│     ├─ files/
│     ├─ skill/
│     │  └─ SKILL.md
│     ├─ patches/                # optional
│     └─ docs.md                 # optional
└─ tests/
```

### Per-language requirements

Every language pack MUST provide:

- one language manifest
- one base template
- one config contract document
- one feature-pack document
- one extension-model document
- one runtime/validation path used by the CLI or installer
- one language-pack test suite
- one feature authoring registry under `features/registry.json`

### Per-language naming rules

- `language-id` MUST be stable and lowercase.
- language directories MUST use the same names across docs, manifests, and CLI values.
- every language pack MUST expose the same semantic concepts even if file formats differ.

Those semantic concepts are:

- app
- prompts
- tools
- providers
- agents
- skills
- features

### Authoring rules

- contributor feature authoring MUST happen under `languages/<id>/features/`
- feature source MUST NOT be authored canonically under a generated-project hidden root
- `template/base/` owns base scaffold files only
- `features/<feature-id>/skill/SKILL.md` is the canonical authoring location for the MVP feature skill

---

## 5. Language manifest contract

Each language pack MUST define `language.manifest.json`.

This file is the CLI discovery contract for that language.

Minimum required fields:

```json
{
  "id": "python",
  "displayName": "Python",
  "status": "stable",
  "templateRoot": "template/base",
  "featureRegistry": "features/registry.json",
  "runtime": {
    "configFile": "agentkit.toml",
    "genericWorkspaceRoot": ".agents",
    "nativeWorkspaceRoot": ".join"
  },
  "supports": {
    "init": true,
    "featureAdd": true,
    "doctor": true
  }
}
```

### Manifest rules

- the CLI MUST discover languages from `language.manifest.json`
- adding a language MUST NOT require editing hardcoded lists in multiple places
- every manifest MUST declare whether the language supports `init`, `featureAdd`, and `doctor`
- every manifest MUST identify the config file name used by generated projects
- every manifest MUST identify the template root and language-pack feature registry path
- every manifest MUST identify the generic hidden root used for agent-facing assets
- every manifest MAY identify a native hidden root used by the runtime/tool being developed

Optional fields may include placeholder conventions, runtime entrypoint hints, status, and capability flags.

---

## 6. Generated project contract

Every generated project MUST be customizable through normal project code plus hidden local asset roots rather than by editing the boilerplate kernel blindly.

Minimum generated project shape:

```text
<generated-project>/
├─ <language-config-file>
├─ src/
├─ tests/
├─ README.md
├─ .agents/
│  ├─ skills/
│  │  └─ <skill-name>/
│  │     └─ SKILL.md
│  ├─ prompts/
│  ├─ agents/
│  ├─ features/
│  ├─ context/
│  ├─ sessions/
│  └─ usage/
└─ .<runtime-name>/           # optional native self-host root
```

### Generated project rules

- every generated project MUST include a base config file
- every generated project MUST include `src/`
- every generated project MUST include `tests/`
- every generated project MUST include `.agents/`
- every generated project MAY include one or more native hidden roots such as `.<runtime-name>/`
- every generated project MUST have a base runtime entrypoint or application entrypoint
- every generated project MUST include at least one smoke test
- every generated project MUST be valid immediately after `init`

### Hidden-root rules

- `.agents/` is the generic agent-facing bootstrap root
- `.agents/` is the preferred home for reusable AI development assets
- `.<runtime-name>/` is the native self-host/runtime root for the tool being developed
- `.agents/` and `.<runtime-name>/` MAY coexist in the same project
- hidden roots are named by surface identity, not by the user project name
- hidden roots are generated-project runtime surfaces, not contributor authoring roots

---

## 7. Extension model contract

Across all languages, the extension model is based on five layers:

1. runtime defaults
2. generated project code under normal source directories
3. project-local `.agents/*` assets
4. project-local native hidden roots such as `.<runtime-name>/*`
5. explicit CLI overrides

The exact file format may vary by language, but the precedence rules MUST stay consistent.

The following extension seams are first-class and MUST remain user-editable:

- prompts
- agents
- skills
- features

Tool and provider configuration MAY be file-backed or typed config-backed depending on language, but the semantics MUST remain aligned.

### Skill rule

For the MVP:

- each feature MAY ship exactly one skill
- that skill MUST be authored at `features/<feature-id>/skill/SKILL.md`
- when materialized into a generated project, it MUST land at `.agents/skills/<skill-name>/SKILL.md`
- the feature skill is the preferred AI development interface for continuing work on that feature

---

## 8. Feature-pack contract

Feature packs are first-class assets. They are not ad hoc code changes.

Canonical location inside a language pack:

```text
languages/<language-id>/features/
├─ registry.json
└─ <feature-id>/
   ├─ feature.json
   ├─ files/
   ├─ skill/
   │  └─ SKILL.md
   ├─ patches/                # optional
   └─ docs.md                 # optional
```

### `registry.json`

`registry.json` MUST be the source of truth for discoverable features in that language pack.

### `feature.json`

Every feature pack MUST declare:

- `id`
- `name`
- `version`
- `description`
- `dependsOn`
- `conflictsWith` if applicable
- `adds`
- patch behavior needed to wire the feature into config/runtime
- `skill` metadata when the feature ships an MVP skill

Example:

```json
{
  "id": "github-pr-review",
  "name": "GitHub PR Review",
  "version": "0.1.0",
  "description": "Adds review-oriented prompts, skill guidance, and runtime wiring.",
  "dependsOn": [],
  "conflictsWith": [],
  "adds": {
    "agents": ["review-agent.md"],
    "skill": "review-pr",
    "prompts": [],
    "tools": ["mcp"]
  },
  "patches": [
    {
      "target": "config",
      "op": "merge",
      "path": "features.enabled",
      "value": ["github-pr-review"]
    }
  ]
}
```

### Feature-pack rules

- a feature pack MAY be language-specific
- the same feature id MAY exist in multiple languages with different implementations
- feature packs MUST be self-contained
- feature packs MUST declare dependencies explicitly
- feature packs MUST declare conflicts explicitly when relevant
- feature packs MUST support `feature add`
- feature packs MUST NOT require manual edits immediately after `feature add`
- feature packs are add-only in the long-term contract
- feature packs SHOULD generate isolated files where practical
- when a feature ships a skill, that skill SHOULD capture feature-specific best practices so the user does not need to re-explain the feature from scratch

### Cross-language feature naming rule

If two language packs implement the same user-facing capability, they SHOULD reuse the same feature id.

Example:

- `python`: `github-pr-review`
- `typescript`: `github-pr-review`

The implementation may differ, but the semantic meaning SHOULD stay aligned.

---

## 9. CLI contract

The user-facing CLI MUST support a stable cross-language workflow.

### Required commands

#### 9.1 Initialize a project

```bash
aicd init my-app --language python
```

Behavior:

1. discover the language manifest
2. load the language template
3. copy template files into the destination
4. apply placeholder replacements
5. validate generated structure
6. print next steps

#### 9.2 List supported languages

```bash
aicd languages list
```

Behavior:

- discover all `languages/*/language.manifest.json`
- print stable ids, display names, and status

#### 9.3 List features

```bash
aicd features list --language python
aicd features list --project ./my-app
```

Behavior:

- by language: list features from that language pack’s authoring registry
- by project: list features recorded or detectable in that generated project

#### 9.4 Add a feature

```bash
aicd feature add github-pr-review --project ./my-app
```

Behavior:

1. detect project language
2. load the language-pack feature registry
3. resolve feature manifest
4. verify dependencies and conflicts
5. copy feature files into the generated project
6. materialize the feature skill into `.agents/skills/<skill-name>/SKILL.md` when present
7. apply config patches
8. validate result

#### 9.5 Validate a project

```bash
aicd doctor --project ./my-app
```

Behavior:

- verify config file presence
- verify generated project bootstrap roots are present and coherent
- verify enabled prompts, agents, skills, and feature assets exist where required
- verify dependency wiring is valid
- return machine- and human-readable failure information

### CLI rules

- the CLI MUST be manifest-driven
- the CLI MUST NOT hardcode per-language internal paths in multiple places
- the CLI MUST fail clearly when a language or feature is unsupported
- the CLI MUST treat validation as part of `init` and `feature add`, not as an optional afterthought
- the long-term CLI contract is add-only; `feature remove` is not part of the target architecture

---

## 10. Testing and validation rules

Every language pack MUST prove the following through tests:

1. template scaffold works
2. generated project passes smoke validation
3. feature add works for at least one sample feature
4. doctor catches missing or broken feature wiring
5. feature skill materialization works when a feature ships a skill

The repo root SHOULD also contain cross-language integration tests for:

- language discovery
- `aicd init --language <id>`
- `aicd feature add`
- invalid manifest handling
- unsupported feature or language errors
- repeated `feature add` behavior after user customization where practical

### Definition of Done for a new language

A language is not considered ready unless all of the following are true:

- it has a valid `language.manifest.json`
- `aicd init --language <id>` works
- the generated project passes smoke tests
- `aicd doctor` works
- at least one feature can be added
- docs exist for config, extension model, and feature packs

### Definition of Done for a new feature

A feature is not considered ready unless all of the following are true:

- it is registered in `registry.json`
- it has a valid `feature.json`
- it can be added by CLI
- the project remains valid after add
- when it ships a skill, that skill materializes to `.agents/skills/<skill-name>/SKILL.md`
- tests prove the feature is actually wired, not just copied

---

## 11. Guardrails

These rules exist to prevent long-term structural drift.

### 11.1 No language-specific snowflakes

New languages MUST follow the same minimum directory contract.

### 11.2 No hidden feature authoring locations

Feature packs MUST live canonically under the language pack’s `features/` tree.

### 11.3 No product-source / boilerplate-source mixing

A language pack MUST NOT mix full standalone product source and long-term boilerplate authoring concerns in a way that obscures ownership.

If a language tree currently mixes both, migration work SHOULD separate them before large feature expansion.

### 11.4 No runtime dependency on archived references

Anything under `references/` is for research or migration only.
Shipped templates and runtime helpers MUST NOT depend on it.

### 11.5 No root-level operational clutter in the architecture contract

Files such as local databases, caches, session logs, virtual environments, and dependency install directories are not part of the architectural model and SHOULD be ignored or moved out of the long-term contract.

### 11.6 Shared semantics, language-specific implementation

Different languages MAY use different formats such as TOML, JSON, or TypeScript config files.
They MUST still represent the same semantic model.

### 11.7 Skills are first-class feature outputs

For the MVP, feature-owned `SKILL.md` files are first-class outputs rather than optional nice-to-have docs.

The reason is architectural:

- users should be able to invoke a shipped feature skill instead of re-describing the feature ad hoc
- a feature should carry both executable code and AI-usable best-practice guidance
- `.agents/skills/<skill-name>/SKILL.md` is the generic interoperability surface for that guidance

---

## 12. Recommended migration direction from the current repo

The current repository already contains useful building blocks, especially in Python.

Migration SHOULD move toward:

- legacy `python/` workspace → `languages/python/` plus `references/legacy-python-workspace/`
- legacy `typescript/` workspace → `languages/typescript/` plus `references/legacy-typescript-workspace/`
- legacy source-archive roots → `references/`
- shared schemas and cross-language docs → `shared/`
- generated-project `.agent/*` assumptions → explicit `.agents/*` plus optional native hidden roots
- reversible feature-pack assumptions → add-only feature-pack lifecycle

During migration:

- preserve existing behavior through compatibility shims where practical
- do not add new language packs outside the canonical structure
- prefer manifest-first discovery over path-specific branching

---

## 13. Decision summary

The repository standard is:

- one CLI/orchestrator layer in `install/`
- one standardized contributor-facing pack per language in `languages/<id>/`
- one base template per language
- one language-pack feature registry per language
- one generic generated-project agent-facing root at `.agents/`
- zero or more native generated-project hidden roots such as `.<runtime-name>/`
- one manifest-driven CLI workflow across all languages
- one feature skill per feature in the MVP, authored at `features/<feature-id>/skill/SKILL.md` and materialized at `.agents/skills/<skill-name>/SKILL.md`

This is the required shape for future expansion.
