# Migration Taxonomy

This document locks the shared vocabulary for the AICD migration program.

It is the canonical terminology source for:

- pack types
- lifecycle states
- capability states
- backlog dispositions
- capability identifiers

The terms in this document are normative for migration work. Later migration artifacts, schemas, matrices, and tests MUST use this vocabulary consistently.

---

## 1. Terminology boundaries

These categories are intentionally distinct and MUST NOT be collapsed into one another.

### 1.1 Pack types

Pack types describe what kind of repository-owned unit is being discussed.

- `installer`
  - the user-facing CLI/orchestration layer under `install/`
- `language-pack`
  - a per-language package under `languages/<id>/`
- `feature-pack`
  - an add-only capability bundle layered onto a generated project
- `reference-pack`
  - archived or imported evidence-only material under `references/`

### 1.2 Lifecycle states

Lifecycle states describe the maturity of a language-pack or feature-pack as a shipped artifact.

- `planned`
- `experimental`
- `stable`
- `deprecated`
- `retired`

Lifecycle state is about rollout maturity. It is not a claim about parity completeness.

### 1.3 Capability states

Capability states describe the implementation and proof status of a specific capability row.

- `declared`
  - a capability is named in docs, manifests, catalogs, or matrices
- `implemented`
  - code/assets/config exist for the capability
- `verified`
  - automated or explicitly defined validation proves the capability behavior

These states are ordered and distinct:

- `declared` does not imply `implemented`
- `implemented` does not imply `verified`
- only `verified` counts for rollout gates and completion reporting

### 1.4 Backlog dispositions

Backlog dispositions describe the intended destination or disposition of a capability in the migration.

- `core`
  - required shipped boilerplate functionality
- `feature-pack`
  - optional shipped capability that must remain dynamic and add-only
- `reference-only`
  - evidence retained for research, migration, or comparison; not shipped runtime behavior
- `deferred`
  - acknowledged future work, intentionally postponed
- `rejected`
  - intentionally excluded from planned shipped scope

These five backlog dispositions are exhaustive for the migration program.

---

## 2. Manifest identity vs derived verification state

Manifest files define runtime identity and runtime capabilities. They MUST NOT be used to store derived migration or parity status.

### 2.1 Manifest identity fields

Identity/config fields include examples such as:

- `id`
- `displayName`
- `status`
- `templateRoot`
- `configFile`
- `featureRegistry`
- `supports`
- feature metadata such as `dependsOn`, `conflictsWith`, `adds`, and `patches`

### 2.2 Derived verification fields

Derived migration state MUST live outside runtime manifests.

Examples:

- parity completeness
- migration readiness
- capability verification counts
- rollout status summaries
- audit-only evidence metadata

These belong in migration matrices, evidence logs, tests, and reports, not in runtime manifest truth.

---

## 3. Capability identifier convention

Every capability tracked by the migration MUST have a stable capability identifier.

### 3.1 Format

Capability identifiers MUST:

- be lowercase
- use hyphen-separated words
- describe one user-facing capability or one bounded technical capability
- avoid language names unless the capability is intentionally language-specific

Examples:

- `interactive-repl`
- `session-resume`
- `project-local-skill-discovery`
- `mcp-resource-tooling`
- `plugin-lifecycle`
- `hooks-runtime`
- `git-pr-review`
- `team-memory`
- `structured-remote-transport`

### 3.2 Anti-patterns

Capability identifiers MUST NOT:

- encode implementation file paths
- encode temporary migration task numbers
- encode verification state in the identifier
- collapse broad source clusters like `services` or `components` into one capability id

---

## 4. Usage rules

### 4.1 Rollout reporting

Rollout and readiness reporting MUST use capability state language:

- `declared`
- `implemented`
- `verified`

Only `verified` counts as complete for migration gates.

### 4.2 Feature extraction

Broad original-source areas such as large `services`, `components`, `utils`, or `hooks` surfaces MUST be decomposed into capability ids before being classified as `core` or `feature-pack`.

### 4.3 Reference isolation

Anything classified as `reference-only` MUST remain outside shipped runtime ownership unless it is intentionally re-extracted into `core` or `feature-pack` form.

---

## 5. Required vocabulary summary

The migration’s required locked vocabulary is:

- pack types: `installer`, `language-pack`, `feature-pack`, `reference-pack`
- lifecycle states: `planned`, `experimental`, `stable`, `deprecated`, `retired`
- capability states: `declared`, `implemented`, `verified`
- backlog dispositions: `core`, `feature-pack`, `reference-only`, `deferred`, `rejected`

If later artifacts need more terms, this document must be updated first.
