# Canonical AICD Migration Plan

## TL;DR

This is the single source of truth for migrating AICD from its current mixed, Python-biased repository layout into the manifest-driven, multi-language boilerplate architecture defined in `RULES.md`.

The migration has four hard goals:

1. make `install/` the only user-facing orchestration layer,
2. standardize each supported language as a pack under `languages/<id>/`,
3. keep shipped core minimal and move optional capability into reversible feature packs,
4. preserve the original-source capability surface from `FEATURES.md` without silently losing anything during migration.

### Primary deliverables

- manifest-driven installer and CLI workflows
- standardized `languages/python/` as the proving slice
- staged `languages/typescript/` migration after classification, not before
- `references/` as analysis-only source material, never runtime input
- shared schemas and parity/completeness tracking
- TDD-oriented migration harness, rollout gates, and atomic commit strategy

### Critical path

taxonomy and schemas -> capability matrix -> red migration harness -> manifest-driven installer -> Python proving slice -> reference separation -> TypeScript decomposition gate -> rollout enforcement

### Non-negotiable migration rules

- no full-parity claims without evidence and tests
- no runtime dependency on `references/`
- no big-bang TypeScript move
- no new language expansion before rollout gates are green
- no feature counted complete unless it reaches `verified`

---

## 1. Purpose, scope, and non-goals

### Purpose

This plan merges the architecture decisions from `.sisyphus/plans/multi-language-boilerplate-migration.md` with the execution structure from `.sisyphus/plans/migration-execution-plan.md`.

It exists so later work does not have to choose between two planning artifacts or reinterpret intent. This file is the canonical migration reference.

Before being treated as canonical, this plan was revised until Momus returned `[OKAY]`. The repo-local audit record for that review is stored at `.sisyphus/review-evidence/canonical-migration-plan-momus-okay.md`, including a SHA-256 digest binding the review record to the reviewed plan revision.

### In scope

- repository restructuring path toward the `RULES.md` target model
- Python, TypeScript, installer/CLI, and Rust/reference migration strategy
- core vs feature-pack vs reference-only classification
- manifest design and feature-pack contract normalization
- parity tracking against `FEATURES.md`
- TDD migration verification
- rollout gates and commit cadence

### Out of scope

- code implementation in this document
- claiming current parity for Python, TypeScript, or Rust
- treating reference-source trees as shipped runtime dependencies
- adding new languages before the proving slice and installer gates pass

### Success definition

The migration is successful only when:

- `install/` is manifest-driven,
- Python is a verified canonical language pack,
- TypeScript has an evidence-based decomposition map before extraction proceeds,
- the original-source capability inventory is machine-tracked and loss-detected,
- and expansion to additional languages is blocked until readiness gates pass.

---

## 2. Evidence base and current repo reality

This plan is grounded in checked-in evidence, especially:

- `RULES.md`
- `FEATURES.md`
- `python/README.md`
- `python/docs/migration.md`
- `python/docs/feature-packs.md`
- `python/docs/extension-model.md`
- `python/src/agent_boilerplate/*`
- `python/templates/base/agentkit.toml`
- `python/templates/base/.agent/features/registry.json`
- `python/templates/base/.agent/features/github-pr-review/feature.json`
- `python/tests/test_agent_boilerplate.py`
- `typescript/README.md`
- `typescript/docs/migration.md`
- `typescript/docs/config-reference.md`
- `typescript/docs/architecture/core-runtime-boundaries.md`
- `typescript/src/core/config/*`
- `typescript/src/core/registry/*`
- `typescript/src/templates/base/*`
- `install/src/catalog.rs`
- `install/src/commands/init.rs`
- `install/src/commands/feature.rs`
- `install/src/commands/doctor.rs`
- `install/src/commands/list.rs`
- `source-references/source-typescript/PARITY.md`
- `source-references/source-rust/README.md`
- `source-references/source-python/src/reference_data/*`

Important path note:

- `source-references/` is the **current** location of archived source material in this repository.
- `references/` is the **target** destination defined by `RULES.md` and by this migration plan.
- Until Task 10 is executed, all evidence collection and parity work should continue reading from `source-references/`.

### Current repo reality

#### Python

Python is closest to the target model today.

Evidence shows:

- a reusable boilerplate authoring workspace,
- a base template under `python/templates/base/`,
- a real config contract in `agentkit.toml`,
- project-local feature registry support,
- reversible feature add/remove,
- and a doctor path already tested in `python/tests/test_agent_boilerplate.py`.

#### TypeScript

TypeScript still mixes product source, future-boilerplate extraction work, and feature-flag-heavy packaging behavior.

Evidence shows:

- broad product/runtime surface in `typescript/src/`,
- partial extraction boundaries under `typescript/src/core/`,
- template assets under `typescript/src/templates/base/`,
- but no clean, finished language-pack separation yet.

#### Installer / CLI

`install/` has the right role but the wrong implementation strategy.

Evidence shows:

- command families already resemble the target workflow,
- but language discovery and lifecycle logic are still materially Python-specific,
- and static catalogs remain a second source of truth.

#### Rust and source references

Rust and the archived source trees are strong capability evidence, not shipped AICD parity.

Evidence shows:

- rich crate boundaries on the Rust side,
- explicit non-parity warnings in `source-references/source-typescript/PARITY.md`,
- and a much larger original-source capability inventory captured in `FEATURES.md`.

### Planning interpretation

- Python should be the first proving slice.
- TypeScript must be decomposed by capability, not moved wholesale.
- `install/` must become manifest-driven before language expansion.
- reference trees must be preserved for analysis but forbidden from runtime ownership.

---

## 3. Canonical target architecture

The long-term repository shape is:

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

### Architectural properties

1. `install/` is the only user-facing orchestration layer.
2. Each language pack is discoverable via a manifest.
3. Core semantics are shared across languages even if formats differ.
4. Feature packs are first-class, self-contained, reversible units.
5. Reference material is analysis-only, never runtime input.
6. Users can `init`, `feature add/remove`, and `doctor` without hand-editing kernel internals.

### Repository roles

#### `install/`

Owns:

- CLI argument parsing
- language discovery
- project initialization
- feature add/remove orchestration
- doctor dispatch
- user-facing error messages and workflow

Must not own:

- language-specific internal path assumptions beyond manifest-driven resolution
- hidden static catalogs duplicating manifest truth

#### `languages/<id>/`

Each language pack owns:

- language manifest
- docs
- runtime helpers
- template/base
- feature registry and feature packs
- language-pack tests

#### `shared/`

Owns:

- manifest schemas
- feature-pack schemas
- shared semantic contract docs
- parity matrix formats and vocabulary

#### `references/`

Owns:

- source snapshots
- parity notes
- imported reference trees
- migration evidence

Nothing under `references/` may be required by `install/`, generated projects, or shipped runtime helpers.

### Concrete folder moves

#### Python

- `python/` -> `languages/python/`
- `python/src/agent_boilerplate/` -> `languages/python/runtime/`
- `python/templates/base/` -> `languages/python/template/base/`
- `python/docs/` -> `languages/python/docs/`
- language-pack tests -> `languages/python/tests/`

Migration-only/reporting helpers that describe the old Python porting workspace must move to `references/`, `docs/`, or `scripts/`, not remain in the shipped pack path.

#### TypeScript

TypeScript cannot be moved in one shot.

Required sequence:

1. classify current TS subsystems,
2. extract pack-ready runtime and template material,
3. isolate product-only and reference-only material,
4. only then create `languages/typescript/`.

#### Source references

- `source-references/source-python/` -> `references/source-python/`
- `source-references/source-typescript/` -> `references/source-typescript/`
- `source-references/source-rust/` -> `references/source-rust/`

#### Rust

Rust becomes `languages/rust/` only after a deliberate language-pack design exists.

---

## 4. Classification model: core vs feature-pack vs reference-only

The migration must preserve exactly three primary buckets, plus two workflow statuses for backlog control.

### 4.1 Shipped boilerplate core

Definition:

- minimal language-pack content required to generate a valid, editable, smoke-tested project,
- shared semantics across languages,
- no product-specific bloat.

Every language-pack core must include:

1. `language.manifest.json`
2. base template
3. config contract
4. prompt layering contract
5. agent discovery/editability contract
6. skill discovery/editability contract
7. project-local feature registry contract
8. runtime validation path used by CLI
9. one smoke-testable runtime entrypoint

Shared semantic categories that must not drift:

- app
- prompts
- tools
- providers
- agents
- skills
- features

### 4.2 Optional / dynamic feature packs

Definition:

- self-contained, discoverable, reversible capability bundles layered onto a valid base project,
- installed and removed through manifest- and registry-driven CLI flows.

Likely feature-pack families from current evidence:

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
- `advanced-review`
- `rich-doctor-ui`

### 4.3 Reference-only material

Definition:

- capability evidence,
- parity notes,
- archived source trees,
- subsystem inventories,
- reconstruction notes,
- imported snapshots.

These must remain non-runtime until intentionally broken into coherent packs.

Examples:

- giant `components/` surfaces
- giant `services/` ecosystems
- giant `utils/` ecosystems
- full command inventory as one monolith
- full tool inventory as one monolith
- compile-time feature-flag inventories used only as historical evidence

### 4.4 Deferred and rejected statuses

These are backlog dispositions, not runtime buckets.

- `deferred`: capability acknowledged and intentionally postponed
- `rejected`: capability intentionally not planned for AICD shipped scope

Every `deferred` or `rejected` capability must have evidence and rationale.

---

## 5. Contract normalization requirements

### 5.1 Language manifest contract

Every language pack must define `language.manifest.json`.

Minimum required fields:

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

Rules:

- installer discovery must come only from manifests,
- manifests must be the source of truth for template roots and project detection,
- parity/completeness state must not live in runtime manifests,
- language status must reflect evidence, not aspiration.

### 5.2 Feature registry contract

Every generated project must own:

- `.agent/features/registry.json`

Rules:

- registry is the source of truth for project-scoped feature discovery,
- feature ids must be stable,
- every registered feature must have a valid `feature.json`,
- `doctor` must validate registry integrity.

### 5.3 Feature manifest contract

Every feature pack must declare at least:

- `id`
- `name`
- `version`
- `description`
- `dependsOn`
- `conflictsWith` when relevant
- `adds`
- patch behavior required to wire the feature

Rules:

- feature packs must be reversible,
- dependencies and conflicts must be explicit,
- manifest presence alone is not proof of completion.

### 5.4 Config semantic normalization

Config syntax may differ by language, but these meanings may not drift:

- app metadata
- prompt system and section layering
- enabled tools and policy
- provider definitions
- enabled agents and skill directories
- enabled features and feature registry path

### 5.5 Capability state model

Every capability tracked by the migration must carry exactly one implementation state:

- `declared`: manifest or catalog entry exists
- `implemented`: code path and assets exist
- `verified`: automated tests prove the behavior

Rollout and status reporting use only `verified` for progress claims.

---

## 6. Language surface strategy

### 6.1 Installer / orchestrator strategy

Current state:

- command families already resemble the target CLI,
- but discovery and lifecycle logic remain Python-specific in practice.

Target state:

- manifest-driven language discovery,
- manifest-driven init/feature/doctor dispatch,
- no static second registry,
- capability-aware error handling.

Migration policy:

- do not add new languages by copying special cases into `install/`,
- eliminate `catalog.rs`-style hardcoded language discovery,
- keep compatibility shims only if they have explicit removal criteria.

### 6.2 Python strategy

Python is the first proving slice.

Target role:

- first language pack to satisfy the `RULES.md` Definition of Done,
- reference implementation for manifest-driven init/add/remove/doctor lifecycle.

Python core should include:

- base runtime entrypoint
- config loader
- prompt loading and layering
- manifest-backed agents and skills
- baseline tool enablement
- provider config semantics
- doctor
- smoke test
- one sample feature pack

Python should not keep in shipped core:

- snapshot-backed parity helpers
- migration-reporting commands that are not part of the generated-project contract
- direct coupling to reference data

### 6.3 TypeScript strategy

TypeScript is the highest-risk migration area.

Migration principle:

Do not move `typescript/` wholesale.

Required decomposition buckets:

1. boilerplate core kernel
2. optional feature packs
3. reference/product-only material

TypeScript core extraction candidates:

- typed config contract from `src/core/config/*`
- prompt composition and file-backed prompt loading
- manifest-backed agent loading
- file-backed skill loading
- provider-neutral base provider interfaces
- minimal registry composition required for generated projects
- doctor/validation path
- one smoke-testable runtime entrypoint

TypeScript features that should not remain hardwired in core:

- plugin lifecycle
- hook execution system
- MCP service breadth
- multi-agent task/team orchestration
- bridge/remote-control stack
- voice mode
- GitHub/Slack onboarding
- advanced planning/review flows
- remote/structured transports
- team memory and expanded session memory
- rich UI panels beyond the base interaction shell

TypeScript reference-only until intentionally extracted:

- product branding and distribution surfaces
- giant compile-time feature-flag inventory as user packaging model
- broad service ecosystem without pack boundaries
- broad command families not yet mapped to stable AICD semantics
- source-only conveniences with no stable cross-language meaning

### 6.4 Rust and reference strategy

Near-term role:

- capability evidence and future pack-design input

Not allowed:

- treating Rust reference richness as current AICD shipped parity

Future role:

- deliberate `languages/rust/` pack only after it satisfies the same semantic contract as Python and TypeScript.

---

## 7. Parity tracking and anti-loss safeguards

### 7.1 What parity means in this migration

Parity does not mean every language implements every historical source feature today.

Parity means:

1. the original-source capability is inventoried,
2. its current status per language is known,
3. its intended destination bucket is known,
4. it cannot disappear silently during restructuring.

### 7.2 Required capability matrix

Create a machine-readable capability matrix keyed by semantic capability id.

Suggested fields:

- `capabilityId`
- `userFacingName`
- `sourceEvidence`
- `sourceSubsystems`
- `targetBucket` (`core`, `feature-pack`, `reference-only`, `deferred`, `rejected`)
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

### 7.3 Mandatory safeguards

#### A. No silent source-tree deletion

No large subtree may be deleted or rehomed until its semantic capabilities are classified in the matrix.

#### B. No giant undifferentiated misc pack

Broad subsystems must be decomposed by user-facing capability.

#### C. No unsupported parity claims

No document may claim full parity without evidence and tests.

#### D. No runtime/reference coupling

Any runtime path that reaches into `references/` is a migration failure.

#### E. Stable feature ids across languages

If two languages implement the same user-facing capability, they should reuse the same feature id.

#### F. No installer path sprawl

All language-specific path knowledge must come from manifests and shared logic, not scattered CLI branching.

#### G. Reversibility is part of feature completeness

If a feature cannot be removed safely through the CLI, it is not done.

---

## 8. Verification strategy

This migration is TDD-oriented.

### 8.1 Test rhythm

- **RED**: add failing tests/fixtures that express the target behavior
- **GREEN**: implement the smallest change that makes the tests pass
- **REFACTOR**: remove transitional duplication or dead paths after green

### 8.2 Verification layers

#### Language-pack verification

Every language pack must prove:

1. manifest is valid
2. template scaffolds correctly
3. generated project is immediately valid
4. doctor succeeds on fresh scaffold
5. at least one feature adds correctly
6. the same feature removes correctly
7. doctor catches broken wiring

#### CLI/orchestrator verification

Repo-level integration tests under `tests/` must cover:

- language discovery from manifests
- `aicd init --language <id>`
- `aicd features list --language <id>`
- `aicd features list --project <dir>`
- `aicd feature add/remove`
- `aicd doctor`
- invalid manifest handling
- unsupported feature/language errors

#### Completeness verification

Migration work fails review if:

- a folder moved but no parity mapping exists
- a feature is declared shipped without manifest, registry, and tests
- a language status was upgraded without DoD evidence
- a capability row disappears or is downgraded without audit trail

### 8.3 Evidence model

Every implementation task should produce evidence artifacts under `.sisyphus/evidence/`.

Status reporting must distinguish:

- `declared`
- `implemented`
- `verified`

Only `verified` counts for rollout gates.

---

## 9. Execution strategy

### 9.1 Program streams

1. **Contracts & Taxonomy**
2. **Installer / CLI**
3. **Python Pack**
4. **References & Completeness**
5. **TypeScript Decomposition**
6. **Repo Governance**

### 9.2 Milestones

- **M1 — Contract Lock**: schemas, taxonomy, capability IDs, and red harness approved
- **M2 — Installer Decoupled**: manifest-driven discovery and no hidden second registry
- **M3 — Python Proven**: Python passes the full installer-driven lifecycle
- **M4 — Completeness Controls Live**: capability matrix and reference-isolation rules enforced
- **M5 — TypeScript Extraction Gate**: TypeScript classified before extraction
- **M6 — Expansion Ready**: no more languages until all rollout gates are green

### 9.3 Parallel waves

- **Wave 1**: taxonomy, schemas, capability matrix, red harness
- **Wave 2**: installer decoupling and CLI contract evolution
- **Wave 3**: Python pack standardization and reference separation
- **Wave 4**: feature catalog governance and TypeScript classification
- **Wave 5**: repo-level proving-slice enforcement and rollout gates

### 9.4 Dependency matrix

| Task | Depends On | Blocks |
|---|---|---|
| 1 | - | 2,3,4,5,6 |
| 2 | 1 | 7,8,9 |
| 3 | 1 | 4,5,6,11 |
| 4 | 1,3 | 7,8,10,11 |
| 5 | 1,3 | 9,10,11 |
| 6 | 1,3 | 10,11 |
| 7 | 2,4 | 8,12 |
| 8 | 2,4,7 | 12,13 |
| 9 | 2,5 | 13 |
| 10 | 4,5,6 | 11,13 |
| 11 | 4,5,6,10 | 13 |
| 12 | 7,8 | 13 |
| 13 | 8,9,10,11,12 | final readiness |

---

## 10. Canonical execution backlog

This backlog is the execution engine for the migration. Architecture rationale lives in earlier sections; tasks below focus on ordered delivery.

### Task 1 — Lock taxonomy, states, and capability identifiers

**Goal**
- define shared vocabulary for pack types, lifecycle states, capability states, and backlog dispositions

**Outputs**
- canonical taxonomy docs in `shared/`
- namespaced capability-id convention derived from `FEATURES.md`

**Acceptance**
- no conflicting terms remain across later tasks
- `declared` / `implemented` / `verified` are clearly distinct
- `core` / `feature-pack` / `deferred` / `reference-only` / `rejected` are exhaustive

**QA scenarios**

```text
Scenario: Taxonomy is complete and non-overlapping
Tool: `python -m pytest tests/test_migration_taxonomy.py`
Steps:
1. create or update `tests/test_migration_taxonomy.py`
2. validate the taxonomy artifact against `RULES.md`
3. scan later migration docs/tasks for vocabulary drift
Expected:
- all migration artifacts use the locked status/category terms
- no conflicting term pairs remain

Scenario: Derived-status leakage is prevented
Tool: `python -m pytest tests/test_migration_taxonomy.py -k derived_status`
Steps:
1. validate example manifest payloads produced by the taxonomy work
2. verify parity or runtime health fields are not embedded as handwritten manifest truth
Expected:
- parity/completeness state is modeled outside manifest identity
```

**Commit**
- `docs(migration): lock taxonomy and capability states`

### Task 2 — Define shared language-manifest and feature-pack schemas

**Goal**
- make manifests the only installer discovery contract

**Outputs**
- `shared/schemas/language.manifest.schema.json`
- `shared/schemas/feature-pack.schema.json`

**Acceptance**
- valid examples pass schema validation
- incomplete manifests fail clearly
- parity metadata is not embedded in runtime manifest truth

**QA scenarios**

```text
Scenario: Schema accepts valid manifest examples
Tool: `cargo test -p install manifest_schema_accepts_valid_examples`
Steps:
1. validate a Python manifest fixture
2. validate a TypeScript manifest fixture
Expected:
- both fixtures pass validation

Scenario: Schema rejects incomplete runtime contracts
Tool: `cargo test -p install manifest_schema_rejects_incomplete_examples`
Steps:
1. validate a fixture missing `featureRegistry` and `supports`
Expected:
- validation fails with field-level errors
```

**Commit**
- `feat(shared): add language and feature pack schemas`

### Task 3 — Build the capability matrix from `FEATURES.md`

**Goal**
- convert narrative inventory into machine-readable, auditable parity tracking

**Outputs**
- capability matrix keyed by stable ids
- decomposed backlog slices for oversized source clusters

**Acceptance**
- every feature cluster in `FEATURES.md` maps to a matrix row or decomposition set
- every row has one disposition category
- uncategorized capability rows fail validation

**QA scenarios**

```text
Scenario: No uncategorized capabilities remain
Tool: `python -m pytest tests/test_capability_matrix.py -k completeness`
Steps:
1. load the capability matrix
2. assert every row has a disposition and verification-state field
Expected:
- zero uncategorized rows

Scenario: Oversized clusters are decomposed instead of mirrored
Tool: `python -m pytest tests/test_capability_matrix.py -k cluster_guard`
Steps:
1. inspect rows derived from broad inventory clusters like services/components/utils
2. fail if any such cluster is represented as one shippable pack
Expected:
- broad clusters are decomposed into smaller capability slices
```

**Commit**
- `feat(parity): add capability matrix and backlog classification`

### Task 4 — Create the red migration harness

**Goal**
- express the target architecture as failing tests before refactors begin

**Outputs**
- repo-level tests for discovery, init, doctor, feature add/remove, invalid manifest handling, unsupported language handling, and no-runtime-reference coupling

**Acceptance**
- tests fail on the current hardcoded installer behavior
- tests express manifest-driven target behavior
- at least one reference-coupling guard exists

**QA scenarios**

```text
Scenario: Red harness fails for the current hardcoded state
Tool: `cargo test -p install migration_red_harness_fails_on_hardcoded_python`
Steps:
1. run the new migration-focused test suite against the pre-migration codebase
Expected:
- tests fail specifically on manifest discovery and hardcoded Python assumptions

Scenario: Reference/runtime coupling guard fires
Tool: `cargo test -p install migration_guard_rejects_reference_runtime_paths`
Steps:
1. run the suite with a fixture that points a runtime path at reference data
Expected:
- validation fails before runtime execution
```

**Commit**
- `test(migration): add red harness for target architecture`

### Task 5 — Define core-vs-feature-pack policy and first catalog shape

**Goal**
- lock what stays minimal core and what must become dynamic feature packs

**Outputs**
- policy doc for core semantics
- initial proving-slice feature catalog

**Acceptance**
- core remains minimal and aligned with `RULES.md`
- optional pack policy includes dependencies, conflicts, reversibility, and stable ids
- broad original-source systems are not promoted to core without product reason

**QA scenarios**

```text
Scenario: Core remains minimal
Tool: `python -m pytest tests/test_core_feature_policy.py -k minimal_core`
Steps:
1. compare approved core list against capability-matrix rows
Expected:
- only the semantics required by `RULES.md` are classified as core

Scenario: Feature-pack reversibility is enforceable
Tool: `python -m pytest tests/test_core_feature_policy.py -k reversibility`
Steps:
1. validate initial feature-catalog entries against dependency/conflict/patch rules
Expected:
- invalid or irreversible entries fail validation
```

**Commit**
- `docs(catalog): define core and feature pack policy`

### Task 6 — Define the reference-ingestion and completeness-guarantee system

**Goal**
- prevent silent feature loss over time

**Outputs**
- ingestion flow from references -> normalized capability records -> current matrix comparison
- loss severity rules and validators

**Acceptance**
- capability disappearance without logged disposition fails validation
- runtime cannot bind to `references/`
- downgraded verification state requires audit trail

**QA scenarios**

```text
Scenario: Silent capability loss is blocked
Tool: `python -m pytest tests/test_completeness_controls.py -k silent_loss`
Steps:
1. remove a capability row from a normalized matrix fixture
2. run completeness validation
Expected:
- validation fails with critical-loss reporting

Scenario: Runtime cannot bind to references
Tool: `python -m pytest tests/test_completeness_controls.py -k runtime_reference_binding`
Steps:
1. inject a runtime manifest or config fixture that points at reference paths
2. run validation
Expected:
- validation fails before runtime execution
```

**Commit**
- `feat(governance): add completeness and reference isolation controls`

### Task 7 — Refactor `install/` to manifest-driven language discovery

**Goal**
- kill hardcoded language catalogs and make manifests the single source of truth

**Outputs**
- installer manifest scanning over `languages/*/language.manifest.json`
- centralized path resolution from manifest fields

**Acceptance**
- `aicd languages list` is manifest-driven
- invalid manifests fail clearly
- no second installer language registry remains

**QA scenarios**

```text
Scenario: Manifest-driven discovery works
Tool: `cargo test -p install languages_list_reads_manifests`
Steps:
1. add two language-manifest fixtures
2. run `aicd languages list`
Expected:
- output includes both languages with display name and status from manifests

Scenario: Invalid manifest is rejected safely
Tool: `cargo test -p install languages_list_rejects_invalid_manifest`
Steps:
1. add a malformed language-manifest fixture
2. run language discovery
Expected:
- CLI exits non-zero with clear human-readable and machine-readable errors
```

**Commit**
- `refactor(install): switch language discovery to manifests`

### Task 8 — Evolve the CLI workflow contract end to end

**Goal**
- make the user-facing CLI stable across languages

**Outputs**
- manifest-driven versions of:
  - `aicd init`
  - `aicd languages list`
  - `aicd features list --language`
  - `aicd features list --project`
  - `aicd feature add`
  - `aicd feature remove`
  - `aicd doctor`

**Acceptance**
- all required CLI commands have integration coverage
- unsupported capabilities fail explicitly
- validation runs automatically on init/add/remove

**QA scenarios**

```text
Scenario: Full Python lifecycle succeeds through the CLI
Tool: `cargo test -p install python_cli_lifecycle_end_to_end`
Steps:
1. initialize a Python project
2. list features by language and by project
3. add a sample feature
4. remove the feature
5. run doctor
Expected:
- every command succeeds and the project remains valid throughout

Scenario: Unsupported capability fails clearly
Tool: `cargo test -p install cli_reports_unsupported_capability`
Steps:
1. use a manifest fixture with `featureRemove` disabled
2. run `aicd feature remove`
Expected:
- CLI exits non-zero with explicit unsupported-capability messaging
```

**Commit**
- `feat(cli): align command workflows with manifest contracts`

### Task 9 — Standardize Python as the first complete language pack

**Goal**
- produce the first canonical pack under `languages/python/`

**Outputs**
- `languages/python/language.manifest.json`
- docs, runtime, template/base, tests in the standard shape
- manifest-backed sample feature pack proving add/remove behavior

**Acceptance**
- Python exposes required semantic concepts
- Python init/add/remove/doctor work solely through the manifest-driven installer flow
- Python pack does not depend on reference data

**QA scenarios**

```text
Scenario: Python pack scaffold works end to end
Tool: `cargo test -p install python_pack_init_generates_expected_structure`
Steps:
1. initialize a project from `languages/python/`
2. inspect the generated project structure
Expected:
- config file, `.agent/`, `src/`, `tests/`, and README are all present and valid

Scenario: Python sample feature is reversible
Tool: `cargo test -p install python_sample_feature_is_reversible`
Steps:
1. add the sample feature
2. verify runtime/config wiring
3. remove the feature
4. rerun doctor
Expected:
- the project returns to a valid baseline state with no orphaned wiring
```

**Commit**
- `feat(python): standardize first language pack`

### Task 10 — Separate shipped code from reference/parity material

**Goal**
- make the architecture honest and enforceable

**Outputs**
- current `source-references/` assets relocated under future `references/`
- shipped runtime paths decoupled from reference trees

**Acceptance**
- shipped flows function without runtime access to reference paths
- parity/completeness workflows still function through normalized analysis artifacts
- Python porting helpers are no longer implicitly bundled into shipped runtime ownership

**QA scenarios**

```text
Scenario: Shipped runtime is independent of references
Tool: `cargo test -p install runtime_ignores_reference_paths && python -m pytest languages/python/tests -k no_reference_dependency`
Steps:
1. run shipped runtime tests in a fixture where reference trees are absent from runtime resolution
Expected:
- runtime and installer flows still pass

Scenario: Completeness workflow still works after relocation
Tool: `python -m pytest tests/test_completeness_controls.py -k relocated_reference_tree`
Steps:
1. relocate source evidence into the future `references/` layout in a migration fixture
2. rerun capability-matrix refresh and completeness checks
Expected:
- analysis succeeds without any shipped-runtime imports from reference trees
```

**Commit**
- `refactor(repo): separate references from shipped language code`

### Task 11 — Classify and stage TypeScript decomposition before extraction

**Goal**
- stop accidental loss and false parity claims when migrating TypeScript

**Outputs**
- TypeScript split map assigning each major subsystem to:
  - `languages/typescript/runtime/`
  - `languages/typescript/template/base/`
  - future TS feature packs
  - `references/`

**Acceptance**
- every major TS subsystem has one approved destination category
- no TS extraction begins before Python proving slice and installer gates are green
- TS self-installer and updater concerns are explicitly classified

**QA scenarios**

```text
Scenario: No unclassified TypeScript subsystems remain
Tool: `python -m pytest tests/test_typescript_classification.py -k complete_map`
Steps:
1. run a validation pass over the approved TypeScript split map
Expected:
- all required TS directories and files are assigned a destination category and linked capability row

Scenario: Premature TypeScript migration is gated off
Tool: `python -m pytest tests/test_typescript_classification.py -k gate_blocks_early_extraction`
Steps:
1. leave the Python proving slice incomplete
2. attempt to enable TypeScript extraction stage
Expected:
- the gate blocks advancement with explicit unmet prerequisites
```

**Commit**
- `docs(typescript): classify decomposition and rollout gates`

### Task 12 — Enforce the Python proving slice at repo level

**Goal**
- make the first language-pack lifecycle a permanent readiness gate

**Outputs**
- repo-level integration suite covering manifest discovery, init, features list, feature add/remove, doctor, invalid manifest failures, and unsupported feature/language failures

**Acceptance**
- full Python lifecycle passes through the installer
- broken feature registry wiring is detected
- CI uses these tests as merge gates

**QA scenarios**

```text
Scenario: Canonical Python proving slice passes
Tool: `cargo test -p install python_proving_slice_repo_gate`
Steps:
1. run the full Python lifecycle suite at repo root
Expected:
- all required lifecycle and failure-mode tests pass

Scenario: Broken feature registry wiring is detected
Tool: `cargo test -p install python_doctor_detects_broken_feature_registry`
Steps:
1. corrupt the project-local registry fixture
2. rerun doctor and lifecycle tests
Expected:
- doctor reports invalid wiring and the suite fails
```

**Commit**
- `test(repo): enforce python proving slice at root`

### Task 13 — Enforce rollout gates and release discipline

**Goal**
- block premature expansion and keep migration honest

**Outputs**
- explicit go/no-go gates for expansion
- evidence-based migration status reporting

**Required rollout gates**

1. shared schemas locked and validated
2. installer manifest discovery green
3. Python proving slice green
4. reference/runtime isolation green
5. capability matrix refresh green
6. TypeScript classification complete
7. CLI lifecycle tests green in CI

**Acceptance**
- new language work is procedurally blocked until all seven gates pass
- release reporting distinguishes `declared`, `implemented`, and `verified`

**QA scenarios**

```text
Scenario: Expansion is blocked when gates are incomplete
Tool: `python -m pytest tests/test_release_gates.py -k expansion_blocked`
Steps:
1. intentionally fail one required gate
2. run expansion readiness validation
Expected:
- readiness fails and new-language expansion is blocked

Scenario: Release report is evidence-based
Tool: `python -m pytest tests/test_release_gates.py -k evidence_based_status_report`
Steps:
1. generate a status report from the capability matrix and gate outputs
Expected:
- the report distinguishes `declared`, `implemented`, and `verified` and highlights blockers
```

**Commit**
- `docs(release): enforce migration gates before expansion`

---

## 11. Risk register

| Risk | Severity | Trigger | Mitigation | Gate |
|---|---|---|---|---|
| Installer remains partly hardcoded | High | language list still sourced from constants | kill second registry, add discovery tests | M2 |
| Python migration drags reference data into shipped pack | High | pack imports or templates depend on reference data | runtime-reference isolation tests | M3/M4 |
| TypeScript treated as a simple move | High | task proposals skip classification map | enforce Task 11 before TS extraction | M5 |
| Capability loss goes unnoticed | Critical | matrix rows disappear or downgrade without audit | completeness validator + severity rules | M4 |
| Feature packs become hidden dependency bundles | Medium | catalog grouping replaces explicit dependencies | schema validation + policy checks | M1/M4 |
| New languages added too early | High | expansion starts before proving slice passes | rollout gates block expansion | M6 |

---

## 12. Atomic commit strategy

### Rules

- one commit per task by default
- each commit must preserve a runnable and testable repository state
- never mix taxonomy/schema work with installer refactors in the same commit
- never mix Python proving-slice restructuring with reference relocation in the same commit
- never mix TypeScript classification artifacts with TypeScript implementation extraction in the same commit

### Recommended commit sequence

1. `docs(migration): lock taxonomy and capability states`
2. `feat(shared): add language and feature pack schemas`
3. `feat(parity): add capability matrix and backlog classification`
4. `test(migration): add red harness for target architecture`
5. `docs(catalog): define core and feature pack policy`
6. `feat(governance): add completeness and reference isolation controls`
7. `refactor(install): switch language discovery to manifests`
8. `feat(cli): align command workflows with manifest contracts`
9. `feat(python): standardize first language pack`
10. `refactor(repo): separate references from shipped language code`
11. `docs(typescript): classify decomposition and rollout gates`
12. `test(repo): enforce python proving slice at root`
13. `docs(release): enforce migration gates before expansion`

### TDD commit rhythm

- **RED commit**: add failing tests or fixtures proving the missing behavior
- **GREEN commit**: minimal implementation to satisfy tests
- **REFACTOR commit**: remove transitional duplication after green

If a task is governance-only, schema/validator additions are the RED phase and cleanup/gate enforcement is the REFACTOR phase.

---

## 13. Final rollout gate and review discipline

Before migration work is declared ready for execution handoff or broad expansion, run a final review wave over the implemented artifacts.

Required final review lenses:

1. **Plan compliance audit** — architecture still matches `RULES.md`
2. **Code quality review** — manifests, schemas, tests, and shims do not recreate hardcoded registries or runtime-reference coupling
3. **Manual QA of CLI lifecycle** — full user-style flow passes
4. **Scope fidelity check** — no reference-source subsystem was silently promoted to shipped core without explicit approval

No migration stream is complete until the review wave is green.

---

## 14. Success criteria

The migration is complete only when all of the following are true:

- repository architecture matches the target contract in `RULES.md`
- Python is the first verified language pack and all lifecycle flows work through `install/`
- TypeScript has a complete, evidence-based decomposition map before extraction proceeds
- `FEATURES.md` is backed by a machine-readable capability matrix and loss-detection controls
- no important original-source capability can disappear silently because uncategorized or downgraded rows fail validation
- `references/` is analysis-only and does not participate in runtime behavior
- installer discovery is manifest-driven and has no hidden second registry
- feature packs are discoverable, dependency-aware, and reversible
- expansion to additional languages is blocked until all rollout gates pass

### Final migration discipline

Nothing gets moved, declared complete, or collapsed into boilerplate until:

1. its user-facing semantic role is known,
2. its target bucket is explicit,
3. its parity status is documented,
4. and its behavior is verified.

That is the only safe path to a clean, future-proof, multi-language AICD boilerplate that stays easy to use, easy to extend, and exact about feature coverage.
