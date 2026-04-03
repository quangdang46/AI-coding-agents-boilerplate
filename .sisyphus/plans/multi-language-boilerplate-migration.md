# Multi-Language Boilerplate Migration Plan

## Purpose

This document defines the migration plan for bringing the current Python, TypeScript, Rust/reference, and installer/orchestration surfaces onto the repository architecture required by `RULES.md`.

It is a planning artifact for later execution work. It does **not** claim current parity across languages. It is explicitly designed to prevent capability loss while moving the repository toward a clean, manifest-driven, multi-language boilerplate with:

- a small shipped core,
- optional/dynamic feature packs,
- reference-only archival material kept separate,
- CLI-driven `init`, `feature add`, `feature remove`, and `doctor`,
- and a future-safe structure where adding new languages does not require restructuring the repository.

---

## 1. Canonical target to migrate toward

Per `RULES.md`, the long-term shape is:

```text
aicd/
├─ install/
├─ languages/
│  ├─ python/
│  ├─ typescript/
│  ├─ rust/
│  └─ <future-language>/
├─ shared/
│  ├─ schemas/
│  └─ docs/
├─ references/
├─ docs/
├─ tests/
└─ scripts/
```

The migration must optimize for these architectural properties:

1. `install/` is the only user-facing orchestration layer.
2. Each language pack has a discoverable manifest and a stable pack contract.
3. Core shipped semantics are aligned across languages even when implementation formats differ.
4. Feature packs are first-class, self-contained, reversible units.
5. Archived/reference material never becomes an implicit runtime dependency.
6. Users can initialize a project and then add or remove features without hand-editing boilerplate internals.

---

## 2. Evidence base used for this plan

This plan is grounded in the following checked-in evidence:

- `RULES.md`
- `FEATURES.md`
- `python/README.md`
- `python/docs/migration.md`
- `python/templates/base/agentkit.toml`
- `python/templates/base/.agent/features/registry.json`
- `python/templates/base/.agent/features/github-pr-review/feature.json`
- `python/tests/test_agent_boilerplate.py`
- `typescript/README.md`
- `typescript/docs/migration.md`
- `typescript/docs/architecture/core-runtime-boundaries.md`
- `typescript/src/core/config/schema.ts`
- `typescript/src/core/config/defaults.ts`
- `typescript/src/core/registry/builtInCommands.ts`
- `install/src/commands/init.rs`
- `install/src/commands/feature.rs`
- `install/src/commands/doctor.rs`
- `install/src/commands/list.rs`
- `source-references/source-typescript/PARITY.md`
- `source-references/source-rust/README.md`
- `source-references/source-python/src/reference_data/subsystems/*.json`

Important evidence constraints:

- Python currently demonstrates a real template + feature-registry + add/remove + doctor flow.
- TypeScript currently demonstrates a broad product/runtime surface plus partial extraction boundaries, not a finished language-pack layout.
- `install/` currently exposes the future command families but is still materially Python-specific.
- Rust/source reference material demonstrates capability breadth and alternative architecture, but not shipped AICD parity.

---

## 3. Migration problem statement by surface

### 3.1 Python surface

Current state:

- `python/README.md` describes the directory as a reusable boilerplate authoring workspace.
- `python/templates/base/` already contains a generated-project template.
- `python/templates/base/agentkit.toml` already models app, prompts, tools, providers, agents, skills, and features.
- `.agent/features/registry.json` exists and currently registers `github-pr-review`.
- `python/src/agent_boilerplate/` contains config loading, registry, feature add/remove logic, and a project doctor path.
- `python/tests/test_agent_boilerplate.py` proves scaffold, feature add/remove, dependency checks, and doctor behavior.

Interpretation:

- Python is the clearest current candidate for the first fully compliant language pack.
- Its main remaining work is structural normalization into `languages/python/`, manifest formalization, and removal of remaining migration/reporting clutter from the shipped surface.

### 3.2 TypeScript surface

Current state:

- `typescript/README.md` still describes a standalone product/fork (`free-code`) rather than a language pack.
- The tree contains broad product source: commands, tools, services, plugins, tasks, screens, providers, bridge, voice, and UI.
- `typescript/docs/migration.md` and `typescript/docs/architecture/core-runtime-boundaries.md` already recognize the need to extract a reusable runtime kernel under `src/core/`.
- `typescript/src/core/config/schema.ts` and `defaults.ts` show an emerging boilerplate config model aligned to the Python semantics.
- `typescript/src/core/registry/builtInCommands.ts` still centralizes a very large command surface tied to compile-time feature flags and product runtime assumptions.

Interpretation:

- TypeScript is promising but still mixed between product source, extracted reusable kernel, compile-time flagging, and future-boilerplate intent.
- It cannot yet be treated as a clean language pack because ownership boundaries are still blurred.
- The largest migration risk is collapsing too much product-specific breadth into a vague “TypeScript pack” without deciding what is truly base, what becomes optional packs, and what stays reference-only.

### 3.3 Installer/orchestrator surface (`install/`)

Current state:

- `install/` already exposes `init`, `feature`, `doctor`, and `list` command families.
- `install/src/commands/init.rs` hardcodes Python rendering and rejects non-Python languages.
- `install/src/commands/list.rs` uses static catalog constants rather than manifest discovery.
- `install/src/commands/feature.rs` and `doctor.rs` assume Python project structure and fixed file paths.

Interpretation:

- `install/` already has the correct role but the wrong implementation strategy.
- It is a language-aware CLI by naming, but not yet a manifest-driven CLI by architecture.

### 3.4 Rust/source reference surfaces

Current state:

- `source-references/source-rust/README.md` shows a mature Rust product surface with crates for runtime, tools, commands, plugins, lsp, server, and compat harness.
- `source-references/source-typescript/PARITY.md` explicitly says Rust is not feature-parity with the original TypeScript CLI and calls out gaps in plugins, hooks, skills pipeline, CLI breadth, remote/structured transport, and services.
- `FEATURES.md` also distinguishes original-source capability inventory from shipped pack support.

Interpretation:

- Rust must be treated as one of two things depending on timeframe:
  1. near-term: reference evidence for capability taxonomy and future pack design,
  2. later: input for a real `languages/rust/` pack once its contract is intentionally designed.
- It must **not** be implicitly upgraded to “already ready as an AICD language pack” just because the reference source is substantial.

### 3.5 Original-source inventory surface (`source-references/`)

Current state:

- `FEATURES.md` documents a very large original-source capability map, including 207 command entries, 184 tool entries, and broad subsystem coverage.
- Subsystem snapshots show very large clusters: `components` 389 modules, `utils` 564, `services` 130, `hooks` 104, plus bridge, memdir, plugins, screens, CLI, assistant, and server slices.

Interpretation:

- This is the evidence base for parity tracking and feature extraction backlog.
- It is not a candidate to be copied wholesale into shipped code.
- The migration must preserve discoverability of this capability map without making shipped packs depend on the raw archive.

---

## 4. Target classification model: core vs feature packs vs reference-only

The repository must preserve three distinct categories at all times.

### 4.1 Shipped boilerplate core

Definition:

- Minimal language-pack content required so `aicd init --language <id>` yields a valid, editable, smoke-tested project.
- Shared semantics across languages.
- No product-specific bloat.

Core must include, per language:

1. language manifest,
2. base template,
3. config contract,
4. prompt layering contract,
5. agent discovery/editability contract,
6. skill discovery/editability contract,
7. project-local feature registry contract,
8. runtime validation path used by CLI,
9. one smoke-testable runtime entrypoint.

Core semantic categories that must exist in every language pack:

- app
- prompts
- tools
- providers
- agents
- skills
- features

### 4.2 Optional / dynamic feature packs

Definition:

- Self-contained, discoverable, reversible capability bundles layered onto a valid base project.
- Added and removed through CLI using language-pack metadata, not ad hoc path rules.

Feature packs should own capabilities that are meaningful, user-facing slices but not required for the base scaffold.

Examples strongly suggested by evidence:

- `github-pr-review`
- `mcp-integration`
- `plugin-runtime`
- `hooks-runtime`
- `multi-agent-workflows`
- `git-workflows`
- `session-memory`
- `team-memory`
- `bridge-remote-control`
- `oauth-onboarding`
- `voice-mode`
- `advanced-planning`
- `doctor-dashboard` or richer diagnostics UI

### 4.3 Research/reference-only material

Definition:

- Capability evidence, parity notes, archived sources, subsystem inventories, imported snapshots, and migration analysis not required by generated projects at runtime.

This bucket must include:

- original source trees,
- parity audits,
- giant subsystem maps,
- reconstruction notes,
- release/reference docs from imported projects,
- compatibility experiments not shipped to end users.

Non-negotiable rule:

> Nothing in a shipped language pack may require files under `references/` at runtime, during `init`, or during `feature add/remove`.

---

## 5. Proposed future repository layout

### 5.1 Top-level target

```text
aicd/
├─ install/
├─ languages/
│  ├─ python/
│  │  ├─ language.manifest.json
│  │  ├─ README.md
│  │  ├─ docs/
│  │  ├─ runtime/
│  │  ├─ template/base/
│  │  └─ tests/
│  ├─ typescript/
│  │  ├─ language.manifest.json
│  │  ├─ README.md
│  │  ├─ docs/
│  │  ├─ runtime/
│  │  ├─ template/base/
│  │  └─ tests/
│  └─ rust/
│     ├─ language.manifest.json
│     ├─ README.md
│     ├─ docs/
│     ├─ runtime/
│     ├─ template/base/
│     └─ tests/
├─ shared/
│  ├─ schemas/
│  ├─ manifests/
│  └─ docs/
├─ references/
│  ├─ source-python/
│  ├─ source-typescript/
│  ├─ source-rust/
│  └─ parity/
├─ docs/
├─ tests/
└─ scripts/
```

### 5.2 Concrete folder moves

#### Python

Planned move:

- `python/` → `languages/python/`

Sub-mapping:

- `python/src/agent_boilerplate/` → `languages/python/runtime/`
- `python/templates/base/` → `languages/python/template/base/`
- `python/docs/` → `languages/python/docs/`
- `python/tests/test_agent_boilerplate.py` and language-specific tests → `languages/python/tests/`
- Python migration-only/reporting helpers that describe the old porting workspace but are not part of shipped boilerplate should either:
  - move to `references/source-python/` if archival/research in nature, or
  - move to `docs/` or `scripts/` if they remain useful for migration maintenance.

#### TypeScript

Planned move:

- `typescript/` → staged split, not a single blind move.

Required first split before final move:

1. extract reusable language-pack runtime and template assets,
2. isolate remaining product/source-only material,
3. only then move the pack-ready part to `languages/typescript/`.

Proposed landing zones:

- reusable config, prompt, registry, provider, agent, skill, and feature-pack support → `languages/typescript/runtime/`
- base generated-project assets → `languages/typescript/template/base/`
- language-pack docs → `languages/typescript/docs/`
- language-pack tests → `languages/typescript/tests/`
- upstream-like product source and broad archival product behavior not selected for the shipped pack → `references/source-typescript/` or a clearly named reference subtree

#### Rust / references

Planned move:

- `source-references/source-rust/` → `references/source-rust/`
- `source-references/source-typescript/` → `references/source-typescript/`
- `source-references/source-python/` → `references/source-python/`

Only after an intentional pack design:

- selected Rust language-pack material → `languages/rust/`

#### Shared contracts

Move or create under `shared/`:

- language manifest schema
- feature pack manifest schema
- registry schema
- shared feature taxonomy docs
- parity tracking format
- cross-language semantic contract docs

---

## 6. Language-by-language migration tracks

## 6.1 Python migration track

### Current maturity summary

Python already demonstrates:

- typed config loading,
- a project-local `.agent/features/registry.json`,
- sample feature manifesting,
- reversible feature add/remove,
- doctor validation,
- and a generated template.

This makes Python the first anchor language for the normalized pack contract.

### Python target shape

```text
languages/python/
├─ language.manifest.json
├─ README.md
├─ docs/
├─ runtime/
│  ├─ config/
│  ├─ registry/
│  ├─ prompts/
│  ├─ feature_ops/
│  ├─ doctor/
│  └─ entrypoints/
├─ template/
│  └─ base/
│     ├─ agentkit.toml
│     ├─ .agent/
│     ├─ src/
│     ├─ tests/
│     └─ README.md
└─ tests/
```

### Python migration work

1. Create `languages/python/language.manifest.json`.
2. Normalize Python runtime package layout from `src/agent_boilerplate/` into `runtime/` ownership language consistent with `RULES.md`.
3. Keep `agentkit.toml` semantic categories aligned with `RULES.md`.
4. Preserve `.agent/features/registry.json` as source of truth for project-local feature discovery.
5. Preserve current feature patch semantics but normalize them against a shared feature-manifest contract.
6. Move migration-only/reporting surfaces out of the shipped language-pack path.
7. Ensure `install/` consumes Python only through its manifest, not direct knowledge of Python path structure.

### Python shipped core recommendation

Python core should include:

- basic runtime entrypoint,
- config loader,
- prompt loading and section layering,
- manifest-backed agents and skills,
- baseline tool enablement,
- provider config semantics,
- doctor,
- sample smoke test,
- one sample feature pack for CLI integration testing.

### Python optional feature-pack candidates

Near-term:

- `github-pr-review` (already present)
- `mcp-integration`
- `advanced-verification`

Possible later packs informed by original-source inventory:

- `session-memory`
- `multi-agent-workflows`
- `team-memory`
- `git-workflows`

### Python anti-drift requirements

- Do not let snapshot-backed or parity-audit helper code remain coupled to runtime code paths.
- Do not keep migration-reporting commands in the shipped base if they are not part of the generated-project contract.
- Do not allow Python’s current stronger pack readiness to become an excuse for Python-specific snowflake semantics in the CLI.

---

## 6.2 TypeScript migration track

### Current maturity summary

TypeScript currently contains both:

- the richest currently available product/runtime surface,
- and the highest architectural mixing risk.

It has broad capability coverage, but its current shape is still centered around a standalone product fork with:

- product branding,
- compile-time feature flags,
- large command and tool registries,
- plugins-first extensibility,
- and broad service/UI surface.

### TypeScript migration principle

Do **not** migrate TypeScript by moving the entire existing tree under `languages/typescript/` and calling it done.

Instead, perform a deliberate decomposition into three buckets:

1. **boilerplate core kernel**,
2. **optional feature packs**,
3. **reference/product-only material**.

### TypeScript target shape

```text
languages/typescript/
├─ language.manifest.json
├─ README.md
├─ docs/
├─ runtime/
│  ├─ config/
│  ├─ registry/
│  ├─ prompts/
│  ├─ providers/
│  ├─ doctor/
│  ├─ feature_ops/
│  └─ entrypoints/
├─ template/
│  └─ base/
│     ├─ <ts config file>
│     ├─ .agent/
│     ├─ src/
│     ├─ tests/
│     └─ README.md
└─ tests/
```

### TypeScript core extraction recommendations

The following should become the TypeScript language-pack core:

1. typed config contract from `src/core/config/*`,
2. prompt composition and file-backed prompt loading,
3. manifest-backed agent loading,
4. file-backed skill loading,
5. provider-neutral base provider interfaces,
6. minimal command/tool registry composition needed for generated projects,
7. doctor/validation path,
8. one smoke-testable runtime entrypoint.

### TypeScript features that should **not** remain hardwired in core

These should be extracted into feature packs or deferred as non-core optional layers:

- plugin management lifecycle,
- hook execution system,
- MCP service breadth,
- multi-agent task/team orchestration,
- bridge/remote-control stack,
- voice mode,
- GitHub app and Slack app onboarding,
- advanced planning/review flows,
- remote/structured transports,
- team memory and session memory enhancements,
- rich UI panels beyond the base interaction shell.

### TypeScript features that should remain reference-only until intentionally extracted

- product branding/docs/install surfaces tied to `free-code`,
- giant compile-time feature-flag inventory as the user-facing packaging model,
- broad product service ecosystem with unclear pack boundaries,
- broad command families that have not yet been mapped to stable AICD semantics,
- source-only conveniences with no stable cross-language equivalent.

### TypeScript decomposition method

The TypeScript tree should be decomposed by **capability contract**, not by raw source folder mirroring.

Recommended phases:

#### Phase TS-1: semantic inventory normalization

Map existing TypeScript runtime categories to AICD semantics:

- prompts
- tools
- providers
- agents
- skills
- features

Also classify each command/tool/service as one of:

- base core,
- optional pack candidate,
- reference-only.

#### Phase TS-2: remove packaging model mismatch

Current TypeScript product logic uses compile-time flags heavily. AICD needs runtime-discoverable feature packs. Therefore:

- compile-time flags may remain an internal implementation detail during migration,
- but user-facing capability discovery must shift to manifest/registry-driven feature metadata,
- and no long-term design may equate “Bun compile-time feature flag” with “AICD feature pack”.

#### Phase TS-3: extract template/base

Construct a minimal generated-project template containing:

- config file,
- `.agent/prompts/`,
- `.agent/agents/`,
- `.agent/skills/`,
- `.agent/features/registry.json`,
- minimal `src/`,
- tests,
- README.

#### Phase TS-4: extract pack runtime

Move reusable runtime logic into `languages/typescript/runtime/` behind stable pack-owned boundaries.

#### Phase TS-5: feature-pack slicing

Convert selected capabilities into reversible feature packs with:

- feature manifest,
- files/ assets,
- patch semantics,
- dependency/conflict metadata,
- tests proving add/remove behavior.

### TypeScript initial feature-pack candidates

Most plausible first packs, based on evidence and likely user value:

1. `github-pr-review`
2. `mcp-integration`
3. `multi-agent-workflows`
4. `git-workflows`
5. `session-memory`
6. `hooks-runtime`
7. `plugin-runtime`
8. `bridge-remote-control`

### TypeScript anti-drift requirements

- Do not let product-source directories remain hidden inside the language-pack core path.
- Do not leave compile-time flags as the only discoverability mechanism for end-user features.
- Do not treat enormous registries like `builtInCommands.ts` as a final architecture; they are evidence of capability breadth, not a pack boundary.
- Do not promise TypeScript pack parity with original-source breadth until each feature is mapped and tested.

---

## 6.3 Rust/reference migration track

### Current maturity summary

The Rust reference surface is architecturally rich and useful, but `source-references/source-typescript/PARITY.md` explicitly states it is not feature-parity with the original TypeScript CLI. It already includes useful crate boundaries:

- `api`
- `runtime`
- `tools`
- `commands`
- `plugins`
- `lsp`
- `server`
- `compat-harness`
- `claw-cli`

### Rust migration principle

Treat Rust as a **future language-pack candidate informed by strong reference material**, not as an already-compliant language pack.

### Near-term Rust plan

1. Move current Rust reference material into `references/source-rust/`.
2. Capture parity notes under `references/parity/` or equivalent.
3. Use Rust reference crates to define the future Rust pack capability backlog and contract shape.
4. Only create `languages/rust/` when there is an intentional base-template and feature-pack strategy aligned with AICD semantics.

### Future Rust language-pack design principles

When Rust is productized as a language pack, it should reuse the same semantic contract as Python and TypeScript:

- manifest-driven discovery,
- generated project template,
- project-local `.agent/` assets,
- feature registry,
- add/remove support,
- doctor,
- docs and tests.

### Rust anti-drift requirements

- Do not treat Rust’s current source-build CLI as automatically equivalent to AICD language-pack readiness.
- Do not copy large reference crates directly into `languages/rust/` without deciding base vs pack vs reference-only boundaries.
- Do not use parity gaps as a reason to erase useful Rust crate boundaries; instead use them as inputs to a clean pack design.

---

## 7. Feature taxonomy for future pack extraction

To avoid collapsing giant source trees into vague buckets, the migration must use a stable taxonomy.

### 7.1 Base core taxonomy

These belong in every language-pack base, though implementations may differ:

1. config contract
2. prompt loading and layering
3. agent discovery
4. skill discovery
5. minimal tool baseline
6. provider configuration
7. runtime entrypoint
8. session persistence minimum
9. project-local feature registry
10. doctor/validation

### 7.2 Optional feature-pack families

#### A. Integrations

- `mcp-integration`
- `github-app-integration`
- `slack-integration`
- `oauth-onboarding`

#### B. Workflow automation

- `git-workflows`
- `github-pr-review`
- `advanced-planning`
- `advanced-review`
- `multi-agent-workflows`

#### C. Runtime extension systems

- `hooks-runtime`
- `plugin-runtime`
- `bundled-skills`

#### D. Memory and context

- `session-memory`
- `team-memory`
- `context-compaction-plus`

#### E. Remote/transport/UI enhancements

- `bridge-remote-control`
- `structured-io`
- `voice-mode`
- `rich-doctor-ui`

### 7.3 Reference-only families

These stay non-shipped until broken into coherent packs:

- monolithic `components/` UI surface,
- monolithic `services/` ecosystem,
- monolithic `utils/` ecosystem,
- full 207-command surface as one giant bucket,
- full 184-tool surface as one giant bucket,
- reconstruction notes for broken compile-time flags,
- parity work products and imported source artifacts.

---

## 8. Contract normalization requirements

## 8.1 Language manifest normalization

Every language pack must expose `language.manifest.json` with at least:

```json
{
  "id": "python",
  "displayName": "Python",
  "status": "stable",
  "templateRoot": "template/base",
  "configFile": "agentkit.toml",
  "featureRegistry": ".agent/features/registry.json",
  "supports": {
    "init": true,
    "featureAdd": true,
    "featureRemove": true,
    "doctor": true
  }
}
```

Normalization rules:

- `install/` must read manifests, not code constants.
- manifests must become the single source of truth for template roots and project detection.
- status values must reflect evidence, not aspiration.

### Required status discipline

- Python may become `stable` once it satisfies the full `RULES.md` DoD after relocation and manifest-driven CLI integration.
- TypeScript should remain `experimental` or equivalent until template/feature-pack/doctor workflows are actually working through the CLI.
- Rust should remain `reference` or `experimental` until a real language pack exists.

## 8.2 Feature registry normalization

Every generated project must own:

- `.agent/features/registry.json`

The registry must list discoverable features for that project and remain the source of truth for `feature add/remove` at project scope.

### Required registry guarantees

- feature ids are stable,
- paths are relative and language-pack owned,
- every registered feature has a valid `feature.json`,
- registry state can be validated by `doctor`.

## 8.3 Feature manifest normalization

Every feature pack must declare:

- id
- name
- version
- description
- dependsOn
- conflictsWith when relevant
- adds
- patch behavior

Python already has a strong starting shape here via `github-pr-review/feature.json`.

## 8.4 Config semantic normalization

Config formats may differ by language, but these meanings may not drift:

- app metadata
- prompt system and section layering
- enabled tools and policy
- provider definitions
- enabled agents and skill directories
- enabled features and feature registry path

The migration must produce a shared semantic spec under `shared/` so languages can differ syntactically while remaining equivalent semantically.

---

## 9. Parity tracking model and anti-loss safeguards

This migration is at high risk of accidental capability loss unless parity is tracked explicitly.

## 9.1 What parity means here

Parity does **not** mean every language implements every historical source feature today.

Parity means:

1. the original-source capability is inventoried,
2. its current status per language is known,
3. its intended target bucket is known:
   - core,
   - optional pack,
   - reference-only,
   - explicitly deferred,
4. no capability disappears silently during restructuring.

## 9.2 Required parity matrix

Create and maintain a shared parity matrix keyed by semantic capability id, not raw file path.

Suggested columns:

- `capabilityId`
- `userFacingName`
- `sourceEvidence`
- `sourceSubsystems`
- `targetBucket` (`core`, `feature-pack`, `reference-only`, `deferred`)
- `pythonStatus`
- `typescriptStatus`
- `rustStatus`
- `featureId` if applicable
- `validationProof`
- `notes`

Example capability ids:

- `interactive-repl`
- `session-resume`
- `project-local-skill-discovery`
- `mcp-resource-tooling`
- `plugin-lifecycle`
- `hooks-runtime`
- `git-pr-review`
- `team-memory`
- `structured-remote-transport`

## 9.3 Anti-loss safeguards

The following safeguards are mandatory during migration.

### Safeguard A: no silent source-tree deletion

No large source subtree should be deleted or rehomed unless its semantic capabilities have been classified into the parity matrix.

### Safeguard B: no giant undifferentiated “misc pack”

Broad subsystems like `services`, `components`, and `utils` must be decomposed by user-facing capability. They must not be hidden inside vague catch-all packs.

### Safeguard C: no unsupported parity claims

No document may claim that TypeScript, Python, or Rust has “full parity” unless proven by evidence and tests. Existing evidence already warns against this for Rust and for the Python workspace relative to original TypeScript.

### Safeguard D: reference/runtime separation must be enforced

Any runtime path that reaches into `references/` is a migration failure.

### Safeguard E: feature ids must be stable across languages when semantics match

If two languages implement the same user-facing capability, they should reuse the same feature id.

### Safeguard F: CLI cannot accumulate hidden path rules

All language-specific paths must come from manifests and shared contract logic, not from branching logic scattered in `install/`.

### Safeguard G: reversibility is part of feature completeness

If a feature cannot be removed safely through the CLI, it is not done.

---

## 10. Validation and testing expectations

Validation must exist at four levels.

## 10.1 Language-pack validation

Every language pack must prove:

1. manifest is valid,
2. template scaffolds correctly,
3. generated project is immediately valid,
4. doctor succeeds on fresh scaffold,
5. at least one feature adds correctly,
6. that same feature removes correctly,
7. doctor catches broken wiring.

## 10.2 CLI/orchestrator validation

Cross-language integration tests under root `tests/` should verify:

- language discovery from manifests,
- `aicd init --language <id>`,
- `aicd features list --language <id>`,
- `aicd feature add/remove`,
- `aicd doctor`,
- invalid manifest handling,
- unsupported feature/language errors.

## 10.3 Parity/process validation

Migration work should fail review if:

- a folder moved but no parity mapping exists,
- a feature is declared shipped without manifest/registry/tests,
- a language status was upgraded without DoD evidence,
- a large capability cluster was collapsed without decomposition rationale.

## 10.4 Documentation validation

Each language pack must include:

- `config-reference.md`
- `extension-model.md`
- `feature-packs.md`

And the repo must maintain:

- shared manifest/feature schema docs,
- parity tracking docs,
- migration notes that distinguish shipped support from reference inventory.

---

## 11. Sequenced migration plan with gates

## Phase 0 — Freeze the vocabulary and tracking model

Objective:

- establish the shared semantic model before more code moves happen.

Actions:

1. define shared manifest schema,
2. define shared feature-manifest schema,
3. define parity matrix format,
4. define canonical capability taxonomy,
5. define language status vocabulary.

Exit gate:

- shared contract documents exist and are agreed as the sole vocabulary for later migration work.

Failure mode to avoid:

- moving trees first and inventing the taxonomy later.

## Phase 1 — Normalize Python as the first canonical language pack

Objective:

- turn the strongest existing implementation into the first full `RULES.md`-compliant pack.

Actions:

1. relocate Python into `languages/python/`,
2. add `language.manifest.json`,
3. make `install/` consume Python via manifest-driven paths,
4. keep current feature/doctor tests green after relocation,
5. separate migration/reporting-only material from the shipped runtime path.

Exit gate:

- Python satisfies the Definition of Done for a new language from `RULES.md`.

Failure mode to avoid:

- preserving Python functionality by sneaking Python-specific path assumptions into `install/`.

## Phase 2 — Convert `install/` from Python-aware to manifest-driven

Objective:

- make the CLI architecture correct before broadening language support.

Actions:

1. replace static catalog constants with manifest discovery,
2. replace Python-specific init rendering with language manifest resolution,
3. replace Python-specific doctor checks with language-pack doctor dispatch,
4. replace feature add/remove assumptions with registry/manifest-driven behavior.

Exit gate:

- `install/` no longer has hidden hardcoded Python ownership beyond the existence of the first supported language pack.

Failure mode to avoid:

- adding TypeScript support by copying another set of special cases into `install/`.

## Phase 3 — Build the TypeScript semantic decomposition map

Objective:

- prevent capability loss while extracting TypeScript.

Actions:

1. inventory TypeScript commands/tools/services against the shared taxonomy,
2. classify each into core / pack / reference-only / deferred,
3. identify the smallest valid TypeScript base template,
4. define the first TypeScript feature-pack set.

Exit gate:

- every major TypeScript capability family is classified before the tree is restructured.

Failure mode to avoid:

- moving the TypeScript tree wholesale and losing the distinction between product runtime and boilerplate authoring.

## Phase 4 — Create `languages/typescript/` from extracted pack-ready material

Objective:

- produce the first actual TypeScript language pack.

Actions:

1. create TypeScript manifest,
2. create template/base,
3. wire doctor,
4. wire at least one feature add/remove flow,
5. move residual non-pack product material to references.

Exit gate:

- `aicd init --language typescript` works with a valid generated project and at least one reversible feature pack.

Failure mode to avoid:

- declaring TypeScript “done” because the source tree is large, even if the CLI pack workflow is incomplete.

## Phase 5 — Rehome source references into `references/`

Objective:

- make the architecture honest and prevent accidental runtime coupling.

Actions:

1. move `source-references/*` into `references/`,
2. preserve parity notes and snapshots,
3. update docs so `references/` is explicitly non-runtime.

Exit gate:

- no shipped language pack or CLI runtime depends on `source-references/` paths.

Failure mode to avoid:

- breaking migration visibility by hiding the reference evidence after moving it.

## Phase 6 — Design the future Rust pack intentionally

Objective:

- use Rust reference strengths without skipping the pack-design step.

Actions:

1. map Rust reference capabilities to the shared parity matrix,
2. identify minimal Rust base-template semantics,
3. choose initial Rust core and first feature pack(s),
4. only then create `languages/rust/`.

Exit gate:

- Rust has a real manifest-driven pack design, not just a copied reference workspace.

Failure mode to avoid:

- mistaking a good reference implementation for an already-packaged AICD language.

---

## 12. What cannot be allowed to drift

These are the hard invariants for later execution work.

1. **Core / pack / reference separation must remain explicit.**
2. **CLI discovery must be manifest-driven.**
3. **Feature packs must be discoverable and reversible.**
4. **Project-local `.agent/features/registry.json` must remain the source of truth.**
5. **Shared semantic concepts must stay aligned across languages.**
6. **No runtime dependency on archived references may emerge.**
7. **Parity must be tracked semantically, not by hand-wavy folder names.**
8. **No language may be labeled ready without satisfying `RULES.md` DoD.**
9. **No large source tree may be collapsed into vague categories without migration mapping.**
10. **TypeScript compile-time feature flags must not become the long-term end-user feature model.**
11. **Python’s current lead must not create Python-only CLI contracts.**
12. **Rust/reference richness must not be mistaken for shipped parity.**

---

## 13. Recommended immediate next execution backlog

The most leverage-positive next work items are:

1. Create shared schemas/docs for language manifests, feature manifests, and parity matrix format.
2. Relocate Python into `languages/python/` with zero behavior loss.
3. Refactor `install/` to discover languages via manifest files rather than static Python assumptions.
4. Produce a TypeScript capability decomposition matrix from the current source tree.
5. Move `source-references/` to `references/` once parity-tracking locations are ready.

This order matters. It establishes the contract first, validates one language fully, then generalizes the CLI, and only then tackles the more complex TypeScript and Rust trajectories.

---

## 14. Final planning summary

The repository is already moving in the right direction, but it is not yet uniformly aligned with `RULES.md`.

- Python is closest to the desired architecture and should become the first canonical language pack.
- `install/` already occupies the right repo role, but must stop hardcoding Python-specific assumptions and switch to manifest-driven discovery.
- TypeScript contains the richest available feature surface, but must be decomposed carefully so product source, optional pack material, and true core are not conflated.
- Rust/reference material is valuable evidence and future-language input, but must remain clearly separated from shipped pack claims until it has an intentional pack design.

The central migration discipline is simple:

> nothing gets moved, declared complete, or collapsed into boilerplate until its user-facing semantic role is known, its target bucket is explicit, and its parity status is documented.

That is the only safe way to reach a future where AICD can initialize projects, add/remove features through the CLI, and support new languages without restructuring the repository or losing original-source capabilities.
