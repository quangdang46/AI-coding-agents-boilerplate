# Release Gates

This document defines the explicit go/no-go gates for migration expansion.

It is the canonical rollout control for Task 13 of `.sisyphus/plans/canonical-migration-plan.md`.

---

## 1. Required gates before expansion

New language expansion is blocked until all of the following are true:

1. shared schemas locked and validated
2. installer manifest discovery green
3. Python proving slice green
4. reference/runtime isolation green
5. capability matrix refresh green
6. TypeScript classification complete
7. CLI lifecycle tests green in CI

---

## 2. Gate evidence sources

### Gate 1 — shared schemas locked and validated

Evidence:

- `shared/schemas/language.manifest.schema.json`
- `shared/schemas/feature-pack.schema.json`
- installer manifest-schema tests

### Gate 2 — installer manifest discovery green

Evidence:

- `install/src/manifest.rs`
- `install/src/commands/list.rs`
- tests proving `list languages` reads manifests and rejects invalid manifests

### Gate 3 — Python proving slice green

Evidence:

- `languages/python/`
- installer lifecycle tests for Python init/add/remove/doctor

### Gate 4 — reference/runtime isolation green

Evidence:

- `shared/docs/completeness-controls.md`
- `references/README.md`
- `tests/test_reference_isolation.py`

### Gate 5 — capability matrix refresh green

Evidence:

- `shared/docs/capability-matrix.json`
- `tests/test_capability_matrix.py`

### Gate 6 — TypeScript classification complete

Evidence:

- `shared/docs/typescript-decomposition-map.json`
- `tests/test_typescript_classification.py`

### Gate 7 — CLI lifecycle tests green in CI

Evidence:

- installer CLI lifecycle tests under `install/tests/cli.rs`

---

## 3. Status reporting rule

Migration status reports must distinguish three implementation states:

- `declared`
- `implemented`
- `verified`

Only `verified` counts as complete for rollout gates.

---

## 4. Decision rule

If any required gate lacks evidence, expansion is blocked.

If all required gates have evidence, expansion may proceed to the next approved migration stage.
