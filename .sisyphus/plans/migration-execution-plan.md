# AICD Migration Execution Plan

## TL;DR
> **Summary**: Migrate AICD from a mixed, Python-biased repository into a manifest-driven multi-language boilerplate system defined by `RULES.md`, using Python as the first standardized language pack, decomposing TypeScript behind explicit gates, and converting original-source capability coverage in `FEATURES.md` into a verified backlog plus parity-control system.
> **Deliverables**:
> - manifest-driven installer/CLI architecture in `install/`
> - standardized `languages/python/` and `languages/typescript/` migration path
> - shared schemas/contracts and reference-only capability tracking model
> - feature-pack catalog and core-vs-pack classification rules
> - TDD migration harness, rollout gates, and feature-completeness governance
> **Effort**: XL
> **Parallel**: YES - 5 waves
> **Critical Path**: taxonomy/schema lock -> installer decoupling -> Python pack standardization -> reference separation + parity system -> TypeScript decomposition -> CLI feature workflows -> rollout gate approval

## Context

### Original Request
Build a detailed execution-oriented migration plan aligning the repo with `/home/quangdang/projects/aicd/RULES.md`, using `/home/quangdang/projects/aicd/FEATURES.md` as the original-source capability inventory. The plan must cover Python, TypeScript, installer/CLI, manifest design, feature-pack strategy, parity/backlog tracking, CLI workflow evolution, TDD, atomic commits, rollout gates, and guarantees against silently losing important original-source capabilities.

### Interview Summary
- No implementation was requested; planning only.
- `RULES.md` is the target operating model.
- `FEATURES.md` is the inventory of original-source capability surface, not a parity claim.
- Known repo realities to preserve in sequencing:
  - Python currently has the strongest boilerplate boundary.
  - TypeScript still mixes product source with extraction work.
  - The installer must become manifest-driven instead of language-hardcoded.
  - Future feature completeness must be exact and auditable.

### Grounded Repo Findings
- `install/src/catalog.rs` hardcodes languages and feature catalogs.
- `install/src/commands/init.rs` is effectively Python-only.
- `install/src/commands/doctor.rs` validates a Python-specific generated-project shape.
- `install/src/manifest.rs` is a generated-project config helper, not the required repo-level language manifest system.
- `python/src/agent_boilerplate/` is the best current candidate for the first true language pack.
- `python/src/reference_data/`, `python/src/parity_audit.py`, `python/src/port_manifest.py`, and `python/tests/test_porting_workspace.py` show Python still mixes shipped boilerplate and reference/parity concerns.
- `typescript/src/entrypoints/cli.tsx`, `typescript/src/utils/localInstaller.ts`, `typescript/src/utils/nativeInstaller/installer.ts`, and `typescript/src/services/extractMemories/extractMemories.ts` show TypeScript remains a monolithic source snapshot with product, installer, and extraction concerns mixed together.
- Existing tests are split across `install/tests/`, `python/tests/`, and `typescript/tests/`; there is no evident repo-level cross-language test suite enforcing the target architecture.

### Metis Review (gaps addressed)
- Metis consultation was attempted twice and timed out both times. The plan therefore bakes in additional explicit guardrails to compensate:
  - declaration vs implementation vs verified-state separation
  - no big-bang TypeScript migration
  - no runtime dependence on `references/`
  - mandatory parity evidence before any feature is marked complete
  - rollout gates that block new-language expansion until the core migration harness is proven

## Work Objectives

### Core Objective
Restructure the repository and its delivery model so `install/` becomes a manifest-driven orchestrator, each supported language becomes a standardized pack under `languages/<id>/`, reference-origin material is isolated under `references/`, and the original-source capability inventory in `FEATURES.md` is converted into a tracked, test-backed backlog that prevents silent capability loss.

### Deliverables
- Shared language-manifest schema and feature-pack taxonomy.
- Migration target structure for `install/`, `languages/`, `shared/`, `references/`, and repo-level `tests/`.
- Standardized Python language pack as the proving slice.
- Structured TypeScript extraction/decomposition plan with go/no-go gates.
- Manifest-driven CLI workflow for:
  - `aicd init`
  - `aicd languages list`
  - `aicd features list`
  - `aicd feature add`
  - `aicd feature remove`
  - `aicd doctor`
- Capability matrix and parity-control system sourced from `FEATURES.md` and `references/` evidence.
- TDD harness and migration verification strategy.
- Atomic commit plan aligned to migration streams.

### Definition of Done
- `install/` discovers languages from checked-in manifests instead of `install/src/catalog.rs`-style hardcoded lists.
- Python and TypeScript have an approved target shape under `languages/<id>/` with explicit ownership boundaries.
- The installer no longer embeds Python-specific internal path knowledge in multiple places.
- Reference-origin data is consumed only by analysis/parity tooling, never by shipped runtime flows.
- A machine-readable capability matrix exists that maps original-source capabilities to one of: `core`, `feature-pack`, `deferred`, `reference-only`, `rejected`.
- Every migrated capability has a status of `declared`, `implemented`, or `verified`, with only `verified` counted as parity progress.
- CLI workflows have integration tests and failure-mode tests.
- Rollout gates explicitly block adding more languages until the Python proving slice and manifest-driven installer are green.

### Must Have
- Manifest-first architecture.
- Python-first standardization.
- TypeScript handled as staged decomposition, not simple relocation.
- Core-vs-pack decision framework.
- Exact feature completeness governance over time.
- TDD-oriented sequencing.
- Atomic commit strategy.

### Must NOT Have
- No assumption of current parity.
- No runtime dependency on `references/` or source-reference trees.
- No new long-term architecture that preserves top-level `python/` or `typescript/` as permanent pack roots.
- No feature counted complete based only on copied files or manifest presence.
- No addition of new languages before migration gates pass.
- No CLI logic that hardcodes per-language internal paths in multiple locations.
- No feature bundles used as hidden dependency mechanisms.

## Verification Strategy
> ZERO HUMAN INTERVENTION — all verification is agent-executed.
- **Test decision**: TDD (RED -> GREEN -> REFACTOR) for migration seams, manifests, CLI discovery, and feature lifecycle flows.
- **Frameworks in play**:
  - Rust CLI/integration tests in `install/tests/`
  - Python tests in `python/tests/` until moved, then pack-local tests under `languages/python/tests/`
  - TypeScript tests in `typescript/tests/` until moved, then pack-local tests under `languages/typescript/tests/`
  - new repo-level integration tests under `tests/`
- **Evidence rule**: Every task writes artifacts under `.sisyphus/evidence/task-{N}-{slug}.*` during execution.
- **Verification state model**:
  - `declared`: manifest/catalog entry exists
  - `implemented`: code path and assets exist
  - `verified`: automated tests prove behavior
- **Gate rule**: rollout decisions use only `verified` capability counts, never `declared` or `implemented` counts.

## Execution Strategy

### Program Streams
1. **Contracts & Taxonomy** — schemas, manifest contracts, pack semantics, capability identifiers.
2. **Installer/CLI** — manifest-driven discovery and command workflow migration.
3. **Python Pack** — first standardized language-pack proving slice.
4. **References & Completeness** — move reference-origin material out of shipped paths and build parity tracking.
5. **TypeScript Decomposition** — split reusable runtime, optional feature packs, and reference-only product surface.
6. **Repo Governance** — rollout gates, tests, and backlog discipline.

### Major Milestones
- **M1 — Contract Lock**: shared schemas, taxonomy, capability IDs, and migration test harness approved.
- **M2 — Installer Decoupled**: `install/` reads manifests; hardcoded language branching removed.
- **M3 — Python Proven**: Python exists as the first standardized pack and passes full CLI lifecycle tests.
- **M4 — Completeness Controls Live**: capability matrix, parity backlog, and no-runtime-reference rule are enforced.
- **M5 — TypeScript Extraction Gate**: TypeScript target split is classified and only then scheduled for staged implementation.
- **M6 — Expansion Ready**: more languages remain blocked until all rollout gates pass.

### Parallel Execution Waves
Wave 1: contracts, taxonomy, parity model, baseline red tests

Wave 2: installer decoupling and repo-level test harness

Wave 3: Python pack standardization and reference separation

Wave 4: feature catalog workflow, completeness governance, and TypeScript classification

Wave 5: TypeScript extraction planning artifacts, rollout gates, and release-readiness verification

### Dependency Matrix
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
| 12 | 7,8 | F1-F4 |
| 13 | 8,9,10,11 | F1-F4 |

### Agent Dispatch Summary
- Wave 1: 3 tasks — schemas, classification, red-test harness
- Wave 2: 3 tasks — installer discovery, CLI command contract, root integration tests
- Wave 3: 3 tasks — Python pack shape, Python/reference split, manifest-backed doctor/init lifecycle
- Wave 4: 2 tasks — feature catalog governance, capability matrix/parity governance
- Wave 5: 2 tasks — TypeScript decomposition package, rollout gates/release discipline

## TODOs

- [ ] 1. Lock shared migration taxonomy, status model, and capability identifiers

  **What to do**:
  - Define the canonical taxonomy in `shared/` planning outputs for:
    - pack type: `language-pack`, `feature-pack`, `shared-schema`, `reference-snapshot`
    - lifecycle: `experimental`, `stable`, `deprecated`, `retired`
    - capability state: `declared`, `implemented`, `verified`
    - backlog disposition: `core`, `feature-pack`, `deferred`, `reference-only`, `rejected`
  - Lock a namespaced capability-ID convention derived from `FEATURES.md` clusters so future parity tracking is stable.
  - Define one rule: bundles/catalog groups may improve discoverability but never replace explicit dependency declarations.

  **Must NOT do**:
  - Do not encode implementation-specific paths into taxonomy.
  - Do not mix runtime health/status fields into identity manifests.

  **Recommended Agent Profile**:
  - Category: `architecture-contract` — Reason: this task fixes the vocabulary used by every later stream.
  - Skills: `flywheel-planner` — why needed: taxonomy and contract synthesis.
  - Omitted: `flywheel-swarm` — why not needed: no execution orchestration yet.

  **Parallelization**: Can Parallel: NO | Wave 1 | Blocks: 2,3,4,5,6 | Blocked By: none

  **References**:
  - Pattern: `RULES.md` — target roles, pack semantics, CLI contract, testing rules.
  - Pattern: `FEATURES.md` — original-source capability clusters and extraction caution areas.
  - Pattern: `install/src/catalog.rs` — anti-pattern to replace.
  - External: `https://packaging.python.org/en/latest/specifications/core-metadata/` — minimal stable manifest guidance.
  - External: `https://backstage.io/docs/features/software-catalog/descriptor-format/` — declaration vs derived status separation.

  **Acceptance Criteria**:
  - [ ] Taxonomy decisions are documented and unambiguous enough that later tasks do not redefine terms.
  - [ ] Capability states clearly distinguish declared vs verified progress.
  - [ ] Core-vs-pack classification categories are stable and exhaustive.

  **QA Scenarios**:
  ```
  Scenario: Taxonomy is complete and non-overlapping
    Tool: Bash
    Steps: Validate the taxonomy artifact against RULES.md and confirm every later plan task uses only locked status/category terms.
    Expected: No conflicting terms remain; all task wording matches the taxonomy.
    Evidence: .sisyphus/evidence/task-1-taxonomy-check.txt

  Scenario: Derived-status leakage is prevented
    Tool: Bash
    Steps: Review manifest examples in the taxonomy artifact and check that parity or runtime health fields are not embedded as handwritten manifest truth.
    Expected: Status/projection fields are modeled outside the identity envelope.
    Evidence: .sisyphus/evidence/task-1-derived-status-check.txt
  ```

  **Commit**: YES | Message: `docs(migration): lock taxonomy and capability states` | Files: `shared/*`, planning docs, tests if added

- [ ] 2. Define the shared language manifest and feature-pack schemas

  **What to do**:
  - Design `shared/schemas/language.manifest.schema.json` and `shared/schemas/feature-pack.schema.json`.
  - Make `language.manifest.json` the only installer discovery contract for languages.
  - Require fields from `RULES.md`: `id`, `displayName`, `status`, `templateRoot`, `configFile`, `featureRegistry`, `supports`.
  - Add explicit optional fields for placeholder conventions, runtime entrypoint hints, test commands, and migration status.
  - Keep parity/completeness metadata out of runtime manifests; instead, define separate derived analysis artifacts.

  **Must NOT do**:
  - Do not add ad hoc free-form fields that will later become operationally required.
  - Do not represent source-reference file paths as runtime dependencies.

  **Recommended Agent Profile**:
  - Category: `schema-design` — Reason: installer behavior depends on these contracts.
  - Skills: `mcp-builder` — why needed: strong contract/schema design discipline.
  - Omitted: `khuym:debugging` — why not needed: this is definition, not failure analysis.

  **Parallelization**: Can Parallel: YES | Wave 1 | Blocks: 7,8,9 | Blocked By: 1

  **References**:
  - Pattern: `RULES.md` sections 4, 5, and 8.
  - Pattern: `install/src/manifest.rs` — current generated-project config handling boundary.
  - Pattern: `python/docs/feature-packs.md` — current Python feature-pack semantics to normalize.
  - External: `https://packaging.python.org/en/latest/specifications/entry-points/` — discovery contract pattern.
  - External: `https://code.visualstudio.com/api/references/extension-manifest` — extension contribution and dependency separation.

  **Acceptance Criteria**:
  - [ ] Schema validates all required RULES.md fields.
  - [ ] Feature-pack schema requires explicit dependencies and conflicts.
  - [ ] Installer can eventually read these schemas without language-specific branching.

  **QA Scenarios**:
  ```
  Scenario: Schema accepts valid manifest examples
    Tool: Bash
    Steps: Run schema validation tests against one valid Python manifest example and one valid TypeScript manifest example fixture.
    Expected: Both examples validate successfully.
    Evidence: .sisyphus/evidence/task-2-valid-schema.txt

  Scenario: Schema rejects incomplete runtime contracts
    Tool: Bash
    Steps: Run schema validation tests against a manifest missing featureRegistry/supports fields.
    Expected: Validation fails with clear field-level errors.
    Evidence: .sisyphus/evidence/task-2-invalid-schema.txt
  ```

  **Commit**: YES | Message: `feat(shared): add language and feature pack schemas` | Files: `shared/schemas/*`, schema tests

- [ ] 3. Build the source-capability matrix and classification backlog from FEATURES.md

  **What to do**:
  - Convert `FEATURES.md` into a machine-readable capability matrix keyed by stable capability IDs.
  - For each capability cluster, record:
    - source evidence
    - intended destination (`core`, `feature-pack`, `deferred`, `reference-only`, `rejected`)
    - target languages
    - verification state
    - blocking dependencies
    - risk/severity if omitted
  - Split oversized clusters (`services`, `components`, `utils`, full commands/tools inventories) into coherent migration backlog slices rather than mirroring source trees.
  - Define a rule that only matrix items with explicit evidence can enter the backlog.

  **Must NOT do**:
  - Do not equate inventory presence with product commitment.
  - Do not allow markdown prose to remain the sole parity source of truth.

  **Recommended Agent Profile**:
  - Category: `capability-governance` — Reason: this task prevents silent feature loss.
  - Skills: `khuym:gkg` — why needed: codebase intelligence and mapping discipline.
  - Omitted: `rust-cli-scaffold` — why not needed: irrelevant to backlog governance.

  **Parallelization**: Can Parallel: YES | Wave 1 | Blocks: 4,5,6,11 | Blocked By: 1

  **References**:
  - Pattern: `FEATURES.md` — full original-source capability inventory and caution areas.
  - Pattern: `python/src/reference_data/archive_surface_snapshot.json` — quantitative evidence source.
  - Pattern: `source-references/source-typescript/PARITY.md` — explicit non-parity signal to preserve.
  - External: `https://developer.mozilla.org/en-US/docs/MDN/Writing_guidelines/Page_structures/Compatibility_tables` — structured compatibility-data principle.

  **Acceptance Criteria**:
  - [ ] Every feature cluster in `FEATURES.md` is mapped to a matrix record or justified decomposition set.
  - [ ] Every matrix row has one and only one disposition category.
  - [ ] Silent omission risk is measurable because uncategorized rows fail validation.

  **QA Scenarios**:
  ```
  Scenario: No uncategorized capabilities remain
    Tool: Bash
    Steps: Run a matrix validation script/test that checks every imported capability row has a disposition and verification-state field.
    Expected: Validation passes with zero uncategorized capabilities.
    Evidence: .sisyphus/evidence/task-3-matrix-complete.txt

  Scenario: Oversized clusters are decomposed instead of mirrored
    Tool: Bash
    Steps: Run a guard test that fails if top-level inventory buckets like services/components/utils are classified as single shippable packs.
    Expected: Guard test passes because broad clusters are decomposed.
    Evidence: .sisyphus/evidence/task-3-cluster-guard.txt
  ```

  **Commit**: YES | Message: `feat(parity): add capability matrix and backlog classification` | Files: parity/catalog artifacts, validation tests

- [ ] 4. Create the migration red-test harness before moving architecture

  **What to do**:
  - Add failing integration tests under repo-level `tests/` that codify target-state requirements before refactors begin.
  - Cover discovery, init, doctor, feature add/remove, invalid manifest handling, unsupported language handling, and no-runtime-reference dependence.
  - Add parity-control tests that fail when matrix rows are uncategorized or marked complete without evidence.
  - Explicitly encode rollout gates as tests where practical.

  **Must NOT do**:
  - Do not start structural moves before the red harness exists.
  - Do not keep target-architecture rules only in prose.

  **Recommended Agent Profile**:
  - Category: `test-architecture` — Reason: migration must be driven by failing target-state tests.
  - Skills: `khuym:validating` — why needed: gate discipline.
  - Omitted: `book-sft-pipeline` — why not needed: unrelated.

  **Parallelization**: Can Parallel: YES | Wave 1 | Blocks: 7,8,10,11 | Blocked By: 1,3

  **References**:
  - Pattern: `RULES.md` sections 9 and 10.
  - Pattern: `install/tests/cli.rs` — current CLI coverage to extend beyond Python-specific behavior.
  - Pattern: `python/tests/test_agent_boilerplate.py` — current Python behavior to preserve.
  - Pattern: `typescript/tests/smoke/*` — existing TS smoke harness to later normalize.

  **Acceptance Criteria**:
  - [ ] Repo-level tests fail on current hardcoded installer behavior.
  - [ ] Tests assert target manifest-driven discovery and lifecycle workflows.
  - [ ] Tests include at least one no-runtime-reference guard.

  **QA Scenarios**:
  ```
  Scenario: Red harness fails for current hardcoded state
    Tool: Bash
    Steps: Run the new repo-level migration test suite against the pre-migration codebase.
    Expected: Tests fail specifically on manifest discovery and hardcoded Python assumptions.
    Evidence: .sisyphus/evidence/task-4-red-harness.txt

  Scenario: Guard catches reference-runtime coupling
    Tool: Bash
    Steps: Run the no-runtime-reference guard test with a fixture that points runtime behavior at reference data.
    Expected: The test fails with a clear explanation that references are analysis-only inputs.
    Evidence: .sisyphus/evidence/task-4-reference-guard.txt
  ```

  **Commit**: YES | Message: `test(migration): add red harness for target architecture` | Files: `tests/*`, updated CI/test runners

- [ ] 5. Define core-vs-feature-pack classification policy and first catalog shape

  **What to do**:
  - Turn the `FEATURES.md` base-candidate and optional-pack guidance into explicit policy.
  - Define what remains minimal core across languages:
    - base config
    - `.agent` extension seams
    - smoke validation
    - minimal runtime entrypoint
    - starter prompts/agents/skills/features structure
  - Classify optional families such as plugins, hooks, MCP integration, Git/GitHub workflows, multi-agent orchestration, bridge/remote control, memory systems, onboarding/auth flows, advanced planning/review flows, and rich dashboards as feature-pack candidates unless later promoted.
  - Define a small initial feature catalog for Python proving-slice tests.

  **Must NOT do**:
  - Do not promote large original-source subsystems to core just because they existed in the source inventory.
  - Do not classify UX-rich optional systems as mandatory base without a written product reason.

  **Recommended Agent Profile**:
  - Category: `product-architecture` — Reason: this task controls long-term boilerplate complexity.
  - Skills: `flywheel-planner` — why needed: consistent product-boundary reasoning.
  - Omitted: `cicd-to-cron` — why not needed: unrelated feature domain.

  **Parallelization**: Can Parallel: YES | Wave 1 | Blocks: 9,10,11 | Blocked By: 1,3

  **References**:
  - Pattern: `RULES.md` sections 6, 7, and 8.
  - Pattern: `FEATURES.md` sections 11 and 12.
  - Pattern: `python/docs/feature-packs.md` — current feature-pack organization.

  **Acceptance Criteria**:
  - [ ] Minimal core is small, language-agnostic, and aligned to RULES.md semantics.
  - [ ] Optional pack policy covers dependencies, conflicts, reversibility, and naming reuse across languages.
  - [ ] Initial proving-slice feature catalog is intentionally narrow and testable.

  **QA Scenarios**:
  ```
  Scenario: Core remains minimal
    Tool: Bash
    Steps: Run policy checks comparing the approved core list against classified capability matrix rows.
    Expected: Only the minimal semantics required by RULES.md are classified as core.
    Evidence: .sisyphus/evidence/task-5-core-policy.txt

  Scenario: Feature-pack reversibility is enforceable
    Tool: Bash
    Steps: Validate initial feature-pack catalog entries against schema rules for dependsOn/conflictsWith/patch behavior/reversible removal.
    Expected: Invalid or irreversible entries fail validation.
    Evidence: .sisyphus/evidence/task-5-feature-policy.txt
  ```

  **Commit**: YES | Message: `docs(catalog): define core and feature pack policy` | Files: policy docs, catalog fixtures, tests

- [ ] 6. Define the reference-ingestion and completeness-guarantee system

  **What to do**:
  - Establish the permanent rule that `references/` is analysis-only and never a runtime dependency.
  - Design a repeatable ingestion flow:
    1. ingest source-reference snapshots
    2. normalize into machine-readable capability records
    3. compare against current shipped matrix
    4. fail if important capabilities become uncategorized, downgraded without approval, or disappear without audit trail
  - Add severity rules for capability loss detection:
    - critical loss: original-source capability removed from matrix without disposition
    - medium loss: capability downgraded from verified to implemented/declared without linked reason
    - low loss: evidence freshness expired and needs revalidation
  - Require signed-off backlog records for any `rejected` or `reference-only` disposition.

  **Must NOT do**:
  - Do not load runtime features from `source-references/` or `references/`.
  - Do not allow untracked capability disappearance during migration.

  **Recommended Agent Profile**:
  - Category: `governance-and-risk` — Reason: this is the anti-regression backbone.
  - Skills: `khuym:reviewing` — why needed: verification and evidence mindset.
  - Omitted: `mcp-builder` — why not needed: not a protocol-design task.

  **Parallelization**: Can Parallel: YES | Wave 1 | Blocks: 10,11 | Blocked By: 1,3

  **References**:
  - Pattern: `RULES.md` sections 3.4 and 11.4.
  - Pattern: `FEATURES.md` sections 12 and conclusion.
  - Pattern: `python/src/reference_data/*` — example of current reference material living too close to shipped code.
  - External: `https://martinfowler.com/bliki/StranglerFigApplication.html` — incremental migration and transitional seams.

  **Acceptance Criteria**:
  - [ ] There is an automated path from source evidence to capability matrix refresh.
  - [ ] Capability disappearance without logged disposition fails validation.
  - [ ] Runtime code paths do not depend on reference-origin files.

  **QA Scenarios**:
  ```
  Scenario: Silent capability loss is blocked
    Tool: Bash
    Steps: Remove a capability row from the normalized matrix fixture and run completeness validation.
    Expected: Validation fails with critical-loss reporting.
    Evidence: .sisyphus/evidence/task-6-loss-detection.txt

  Scenario: Runtime cannot bind to references
    Tool: Bash
    Steps: Run guard tests that inject a runtime manifest pointing to a reference-path resource.
    Expected: Validation fails before runtime execution.
    Evidence: .sisyphus/evidence/task-6-runtime-isolation.txt
  ```

  **Commit**: YES | Message: `feat(governance): add completeness and reference isolation controls` | Files: governance artifacts, validators, tests

- [ ] 7. Refactor install/ to manifest-driven language discovery

  **What to do**:
  - Replace `install/src/catalog.rs` hardcoded language discovery with manifest scanning over `languages/*/language.manifest.json`.
  - Centralize all installer path resolution behind manifest fields.
  - Remove multi-place per-language path branching from `init`, `doctor`, and feature workflows.
  - Keep compatibility adapters only where needed during transition, with explicit expiry criteria.

  **Must NOT do**:
  - Do not preserve `catalog.rs` as a second source of truth.
  - Do not add new languages through Rust constant edits.

  **Recommended Agent Profile**:
  - Category: `installer-core` — Reason: highest-leverage change and critical path.
  - Skills: `khuym:debugging` — why needed: CLI refactors will surface breakages across tests.
  - Omitted: `book-sft-pipeline` — why not needed: unrelated.

  **Parallelization**: Can Parallel: NO | Wave 2 | Blocks: 8,12 | Blocked By: 2,4

  **References**:
  - Pattern: `install/src/catalog.rs` — hardcoded catalog anti-pattern to eliminate.
  - Pattern: `install/src/cli.rs` — command entrypoint to preserve.
  - Pattern: `RULES.md` sections 2, 3.1, 5, and 9.2.

  **Acceptance Criteria**:
  - [ ] `aicd languages list` is driven entirely by checked-in manifests.
  - [ ] No second language registry remains in installer code.
  - [ ] Unsupported or invalid manifests fail clearly.

  **QA Scenarios**:
  ```
  Scenario: Manifest-driven discovery works
    Tool: Bash
    Steps: Run installer integration tests that add two manifest fixtures and execute the language-list command.
    Expected: Output includes both languages with display name and status from manifests.
    Evidence: .sisyphus/evidence/task-7-language-discovery.txt

  Scenario: Invalid manifest is rejected safely
    Tool: Bash
    Steps: Run tests with a malformed language manifest fixture.
    Expected: CLI exits non-zero with machine-readable and human-readable errors.
    Evidence: .sisyphus/evidence/task-7-invalid-manifest.txt
  ```

  **Commit**: YES | Message: `refactor(install): switch language discovery to manifests` | Files: `install/src/*`, `install/tests/*`

- [ ] 8. Evolve the CLI workflow contract end-to-end on the new discovery model

  **What to do**:
  - Implement the migration path for these commands under manifest-driven orchestration:
    - `aicd init`
    - `aicd languages list`
    - `aicd features list --language`
    - `aicd features list --project`
    - `aicd feature add`
    - `aicd feature remove`
    - `aicd doctor`
  - Make `init`, `feature add`, and `feature remove` always validate as part of the flow.
  - Ensure clear errors for unsupported operations when a language manifest disables a capability.
  - Add machine-readable failure output for automation.

  **Must NOT do**:
  - Do not leave `doctor` as a Python-shaped validator.
  - Do not treat validation as optional after `init` or feature lifecycle commands.

  **Recommended Agent Profile**:
  - Category: `cli-contract` — Reason: user-facing workflow must be stable across languages.
  - Skills: `khuym:validating` — why needed: strong command-contract verification.
  - Omitted: `flywheel-swarm` — why not needed: implementation, not orchestration.

  **Parallelization**: Can Parallel: NO | Wave 2 | Blocks: 12,13 | Blocked By: 2,4,7

  **References**:
  - Pattern: `RULES.md` section 9.
  - Pattern: `install/src/commands/init.rs` — current Python-only behavior.
  - Pattern: `install/src/commands/feature.rs` — current feature lifecycle baseline.
  - Pattern: `install/src/commands/doctor.rs` — current Python-specific validation anti-pattern.

  **Acceptance Criteria**:
  - [ ] Every required CLI command in RULES.md has integration coverage.
  - [ ] Capability flags in manifests govern whether commands are offered/allowed.
  - [ ] Validation runs automatically on init/add/remove.

  **QA Scenarios**:
  ```
  Scenario: Full Python command lifecycle succeeds
    Tool: Bash
    Steps: Run integration tests that initialize a Python project, list features, add a sample feature, remove it, and run doctor.
    Expected: All commands succeed and project validation remains green throughout.
    Evidence: .sisyphus/evidence/task-8-python-cli-lifecycle.txt

  Scenario: Unsupported capability fails clearly
    Tool: Bash
    Steps: Run tests against a manifest fixture with featureRemove disabled and invoke `aicd feature remove`.
    Expected: CLI exits non-zero with explicit unsupported-capability messaging.
    Evidence: .sisyphus/evidence/task-8-unsupported-capability.txt
  ```

  **Commit**: YES | Message: `feat(cli): align command workflows with manifest contracts` | Files: `install/src/commands/*`, `install/tests/*`, `tests/*`

- [ ] 9. Standardize Python as the first complete language pack

  **What to do**:
  - Create the target `languages/python/` structure as the proving pack.
  - Move or map current Python boilerplate assets into the required shape:
    - `language.manifest.json`
    - `README.md`
    - `docs/`
    - `runtime/`
    - `template/base/`
    - `tests/`
  - Preserve current strongest boundaries from `python/src/agent_boilerplate/` and avoid dragging reference/parity content into the new pack.
  - Add a manifest-backed sample feature pack that proves add/remove behavior.

  **Must NOT do**:
  - Do not migrate `python/src/reference_data/` into shipped runtime/template paths.
  - Do not treat top-level `python/` as the long-term architecture after this task.

  **Recommended Agent Profile**:
  - Category: `python-pack-standardization` — Reason: first proving slice for the entire architecture.
  - Skills: `khuym:executing` — why needed: disciplined pack restructuring.
  - Omitted: `prompt-leverage` — why not needed: not a prompting task.

  **Parallelization**: Can Parallel: YES | Wave 3 | Blocks: 13 | Blocked By: 2,5

  **References**:
  - Pattern: `RULES.md` section 4.
  - Pattern: `python/src/agent_boilerplate/scaffold.py` — current template/scaffold logic.
  - Pattern: `python/src/agent_boilerplate/feature_ops.py` — current reversible feature behavior.
  - Pattern: `python/src/agent_boilerplate/entrypoints/cli.py` — current runtime validation entrypoint.
  - Pattern: `python/docs/feature-packs.md` — feature-pack documentation baseline.

  **Acceptance Criteria**:
  - [ ] `languages/python/language.manifest.json` exists and validates.
  - [ ] Python pack exposes the required semantic concepts from RULES.md.
  - [ ] Python init/add/remove/doctor can be exercised solely through manifest-driven installer flow.

  **QA Scenarios**:
  ```
  Scenario: Python pack scaffold works end to end
    Tool: Bash
    Steps: Run CLI integration tests that initialize from the new Python pack path and verify generated project structure.
    Expected: Generated project includes config file, `.agent/` structure, `src/`, `tests/`, and README.
    Evidence: .sisyphus/evidence/task-9-python-pack-init.txt

  Scenario: Python sample feature is reversible
    Tool: Bash
    Steps: Add the sample feature, assert runtime/config wiring, then remove the feature and rerun doctor.
    Expected: Project returns to a valid baseline state with no orphaned wiring.
    Evidence: .sisyphus/evidence/task-9-python-pack-feature.txt
  ```

  **Commit**: YES | Message: `feat(python): standardize first language pack` | Files: `languages/python/*`, compatibility shims if required, tests

- [ ] 10. Separate shipped code from reference/parity material

  **What to do**:
  - Move archive, parity, and imported-reference concerns out of active shipped trees toward `references/` and analysis-only locations.
  - Priority separations:
    - `python/src/reference_data/`
    - `python/src/parity_audit.py`
    - `python/src/port_manifest.py`
    - `python/tests/test_porting_workspace.py`
    - long-term `source-references/` -> `references/`
  - Introduce explicit compatibility shims only if necessary and only with removal criteria.
  - Update tests to prove shipped runtime functions without reference trees present.

  **Must NOT do**:
  - Do not let parity tooling quietly continue importing from shipped runtime internals if that recreates coupling.
  - Do not delete evidence sources before they are normalized into the capability matrix.

  **Recommended Agent Profile**:
  - Category: `boundary-enforcement` — Reason: architectural correctness depends on this separation.
  - Skills: `khuym:debugging` — why needed: path moves and import rewiring will be failure-prone.
  - Omitted: `flywheel-beads` — why not needed: this is still planning scope.

  **Parallelization**: Can Parallel: YES | Wave 3 | Blocks: 11,13 | Blocked By: 4,5,6

  **References**:
  - Pattern: `RULES.md` sections 3.4, 11.3, 11.4, and 12.
  - Pattern: `FEATURES.md` introduction and evidence basis.
  - Pattern: `python/src/reference_data/*`, `python/src/parity_audit.py`, `python/src/port_manifest.py`, `python/tests/test_porting_workspace.py`.

  **Acceptance Criteria**:
  - [ ] Shipped language-pack/runtime flows function without runtime access to reference paths.
  - [ ] Reference-origin materials live under non-runtime ownership.
  - [ ] Parity/completeness workflows still function through normalized analysis artifacts.

  **QA Scenarios**:
  ```
  Scenario: Shipped runtime is independent of references
    Tool: Bash
    Steps: Run installer and Python pack tests in an environment/fixture where reference trees are absent from runtime resolution.
    Expected: Runtime tests still pass.
    Evidence: .sisyphus/evidence/task-10-runtime-separation.txt

  Scenario: Completeness workflow still works after relocation
    Tool: Bash
    Steps: Run capability-matrix refresh and parity validation against relocated reference assets.
    Expected: Analysis succeeds without any shipped-runtime imports.
    Evidence: .sisyphus/evidence/task-10-analysis-separation.txt
  ```

  **Commit**: YES | Message: `refactor(repo): separate references from shipped language code` | Files: `references/*`, affected tooling/tests, compatibility shims if used

- [ ] 11. Classify and stage TypeScript decomposition before implementation-scale extraction

  **What to do**:
  - Produce a concrete TypeScript split map that assigns every major TS subsystem to one of:
    - future `languages/typescript/runtime/`
    - future `languages/typescript/template/base/`
    - future TS feature packs
    - `references/` only
  - Specifically classify:
    - `typescript/src/entrypoints/cli.tsx`
    - `typescript/src/core/registry/*`
    - `typescript/src/services/extractMemories/*`
    - `typescript/src/utils/localInstaller.ts`
    - `typescript/src/utils/nativeInstaller/installer.ts`
    - major product/service/UI/plugin surfaces
  - Require each TypeScript extraction candidate to trace back to a capability-matrix row and an approved destination category.
  - Schedule TS implementation only after Python proving-slice and installer gates are green.

  **Must NOT do**:
  - Do not perform a blind directory move from `typescript/` to `languages/typescript/`.
  - Do not import TS product self-installer/update code into the AICD installer contract without an explicit product decision.

  **Recommended Agent Profile**:
  - Category: `typescript-decomposition` — Reason: highest ambiguity and risk area.
  - Skills: `khuym:gkg` — why needed: large codebase mapping and dependency analysis.
  - Omitted: `rust-cli-scaffold` — why not needed: unrelated domain.

  **Parallelization**: Can Parallel: YES | Wave 4 | Blocks: 13 | Blocked By: 4,5,6,10

  **References**:
  - Pattern: `RULES.md` sections 2, 3.2, 11.3, and 12.
  - Pattern: `typescript/src/entrypoints/cli.tsx` — current monolithic product entrypoint.
  - Pattern: `typescript/src/core/registry/commandCatalog.ts` and `typescript/src/core/registry/toolCatalog.ts` — possible reusable primitives.
  - Pattern: `typescript/src/services/extractMemories/extractMemories.ts` — extraction logic mixed into product source.
  - Pattern: `typescript/src/utils/localInstaller.ts` and `typescript/src/utils/nativeInstaller/installer.ts` — separate installer/update concerns.
  - Pattern: `source-references/source-typescript/PARITY.md` — explicit parity caveats.

  **Acceptance Criteria**:
  - [ ] Every major TS subsystem has one approved destination category.
  - [ ] No TS extraction starts before Python proving slice and installer gates are green.
  - [ ] TS self-installer concerns are explicitly classified as product-runtime, optional feature, or reference-only.

  **QA Scenarios**:
  ```
  Scenario: No unclassified TS subsystems remain
    Tool: Bash
    Steps: Run a classification audit against the approved TS split map.
    Expected: All required TS directories/files are assigned a destination category and linked capability row.
    Evidence: .sisyphus/evidence/task-11-ts-classification.txt

  Scenario: Premature TS migration is gated off
    Tool: Bash
    Steps: Run rollout-gate tests before Python proving-slice completion and attempt to enable TS extraction stage.
    Expected: Gate blocks advancement with explicit unmet prerequisites.
    Evidence: .sisyphus/evidence/task-11-ts-gate.txt
  ```

  **Commit**: YES | Message: `docs(typescript): classify decomposition and rollout gates` | Files: TS decomposition docs, matrix links, gate tests

- [ ] 12. Add repo-level integration verification for the canonical Python proving slice

  **What to do**:
  - Promote the proving-slice tests into the permanent repo-level `tests/` suite.
  - Verify:
    - manifest discovery
    - Python init
    - project structure validity
    - features list by language and by project
    - feature add/remove
    - doctor success
    - invalid manifest failure
    - unsupported feature/language failure
  - Make these tests mandatory CI gates before merge.

  **Must NOT do**:
  - Do not rely only on pack-local tests once `install/` orchestration becomes central.
  - Do not allow a green Python pack without a green installer-driven lifecycle.

  **Recommended Agent Profile**:
  - Category: `integration-qa` — Reason: this becomes the first permanent readiness gate.
  - Skills: `khuym:reviewing` — why needed: acceptance and regression discipline.
  - Omitted: `mcp-builder` — why not needed: not protocol-related.

  **Parallelization**: Can Parallel: YES | Wave 5 | Blocks: F1-F4 | Blocked By: 7,8

  **References**:
  - Pattern: `RULES.md` section 10.
  - Pattern: `install/tests/cli.rs` — current lifecycle baseline.
  - Pattern: `languages/python/*` — new proving-slice target.

  **Acceptance Criteria**:
  - [ ] Repo-level tests cover the entire Python workflow contract.
  - [ ] CI fails on unsupported/invalid-manifest regressions.
  - [ ] Python is only considered migration-ready when all proving-slice tests pass.

  **QA Scenarios**:
  ```
  Scenario: Canonical Python proving slice passes
    Tool: Bash
    Steps: Run the repo-level integration suite focused on Python lifecycle coverage.
    Expected: All required lifecycle and failure-mode tests pass.
    Evidence: .sisyphus/evidence/task-12-python-proving-slice.txt

  Scenario: Broken feature registry wiring is detected
    Tool: Bash
    Steps: Corrupt the project-local registry fixture and rerun doctor/integration tests.
    Expected: Doctor reports invalid wiring and the suite fails.
    Evidence: .sisyphus/evidence/task-12-doctor-failure.txt
  ```

  **Commit**: YES | Message: `test(repo): enforce python proving slice at root` | Files: `tests/*`, CI wiring

- [ ] 13. Enforce rollout gates and release discipline before adding more languages

  **What to do**:
  - Define mandatory go/no-go gates for post-migration expansion:
    1. shared schemas locked and validated
    2. installer manifest discovery green
    3. Python proving slice green
    4. reference/runtime isolation green
    5. capability matrix refresh green
    6. TypeScript classification complete
    7. CLI lifecycle tests green in CI
  - Add a hard rule: no new language pack work starts until all seven gates pass.
  - Define release/readiness metrics:
    - zero uncategorized capability rows
    - zero runtime-reference dependencies
    - zero hardcoded installer language registries
    - one verified sample feature add/remove cycle in Python
  - Publish the atomic commit strategy below as the only approved implementation cadence.

  **Must NOT do**:
  - Do not start Rust or other future language packs while TypeScript classification or completeness controls are incomplete.
  - Do not call migration done based on repo moves alone.

  **Recommended Agent Profile**:
  - Category: `release-governance` — Reason: prevents premature expansion and drift.
  - Skills: `khuym:validating` — why needed: explicit gate enforcement.
  - Omitted: `flywheel-swarm` — why not needed: no multi-agent execution at planning stage.

  **Parallelization**: Can Parallel: YES | Wave 5 | Blocks: F1-F4 | Blocked By: 8,9,10,11

  **References**:
  - Pattern: `RULES.md` sections 10, 12, and 13.
  - Pattern: `FEATURES.md` section 12.
  - Pattern: all prior migration-stream artifacts.

  **Acceptance Criteria**:
  - [ ] Expansion gates are explicit, measurable, and test-backed.
  - [ ] Future language work is procedurally blocked until readiness criteria pass.
  - [ ] Release reporting distinguishes declared/implemented/verified progress.

  **QA Scenarios**:
  ```
  Scenario: Expansion is blocked when gates are incomplete
    Tool: Bash
    Steps: Run release-readiness checks with one required gate intentionally failing.
    Expected: Readiness fails and new-language expansion is blocked.
    Evidence: .sisyphus/evidence/task-13-expansion-block.txt

  Scenario: Release report is evidence-based
    Tool: Bash
    Steps: Generate a migration status report from the capability matrix and gate outputs.
    Expected: Report distinguishes declared, implemented, and verified counts and highlights blockers.
    Evidence: .sisyphus/evidence/task-13-status-report.txt
  ```

  **Commit**: YES | Message: `docs(release): enforce migration gates before expansion` | Files: release/gate docs, CI/gate tests

## Final Verification Wave (MANDATORY — after ALL implementation tasks)
> 4 review agents run in PARALLEL. ALL must APPROVE. Present consolidated results to user and get explicit "okay" before completing.
> **Do NOT auto-proceed after verification. Wait for user's explicit approval before marking work complete.**
> **Never mark F1-F4 as checked before getting user's okay.** Rejection or user feedback -> fix -> re-run -> present again -> wait for okay.

- [ ] F1. Plan Compliance Audit — oracle
  - Verify the execution artifacts match `RULES.md` and do not leave top-level `python/` or `typescript/` as permanent architecture assumptions.

- [ ] F2. Code Quality Review — unspecified-high
  - Verify manifests, schemas, tests, and compatibility shims do not recreate hardcoded registries or runtime-reference coupling.

- [ ] F3. Real Manual QA — unspecified-high (+ playwright if UI)
  - Run the full CLI lifecycle and failure-mode checks as a human-style scripted validation, even if automated suites already pass.

- [ ] F4. Scope Fidelity Check — deep
  - Verify no migration work accidentally promoted reference-source subsystems into shipped core without explicit product approval.

## Commit Strategy

### Atomic Commit Rules
- One commit per task above, except Tasks 7+8 may be split into two commits each if the RED/GREEN boundary is large.
- Each commit must preserve a runnable and testable repository state.
- Never mix taxonomy/schema work with installer refactors in the same commit.
- Never mix Python proving-slice restructuring with reference relocation in the same commit.
- Never mix TypeScript classification artifacts with TypeScript implementation extraction in the same commit.

### Recommended Commit Sequence
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

### TDD Commit Rhythm
- **RED commit**: add failing tests/fixtures proving the missing behavior.
- **GREEN commit**: smallest implementation that makes the new tests pass.
- **REFACTOR commit**: remove transitional duplication, dead paths, or compatibility shims only after green.
- If a task is documentation/governance-only, treat schema/validator additions as the RED phase and gate/report cleanup as REFACTOR.

## Risk Register and Handling

| Risk | Severity | Trigger | Mitigation | Gate |
|---|---|---|---|---|
| Installer remains partly hardcoded | High | language list still sourced from constants | kill second registry, add discovery tests | M2 |
| Python migration drags reference data into shipped pack | High | pack imports or templates depend on `reference_data` | runtime-reference isolation tests | M3/M4 |
| TypeScript treated as simple move | High | task proposals skip classification map | enforce Task 11 before TS extraction | M5 |
| Capability loss goes unnoticed | Critical | matrix rows disappear or downgrade without audit | completeness validator + severity rules | M4 |
| Feature packs become hidden dependency bundles | Medium | catalog grouping used instead of explicit dependsOn | schema validation + policy checks | M1/M4 |
| New languages added too early | High | new language work starts before proving slice passes | release gates block expansion | M6 |

## Success Criteria
- The repository architecture matches the long-term contract in `RULES.md`.
- Python is the first verified language pack and all lifecycle flows work through the installer.
- TypeScript has a complete, evidence-based decomposition map before extraction proceeds.
- `FEATURES.md` is no longer just a narrative inventory; it is backed by a machine-readable matrix and loss-detection controls.
- No important original-source capability can disappear silently because uncategorized or downgraded rows fail validation.
- Expansion to additional languages is blocked until the verified migration gates pass.
