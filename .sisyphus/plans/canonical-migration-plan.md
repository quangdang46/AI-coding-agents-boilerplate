# Canonical Migration Plan

This file is the canonical plan alias for the tracked AICD migration program.

The detailed source of truth remains `PLAN.md`. This file exists so tests, tooling, and migration workflows have a stable plan path under `.sisyphus/plans/`.

## Taxonomy lock

The migration taxonomy used by this plan is intentionally strict.

- `declared` / `implemented` / `verified` are clearly distinct
- only `verified` counts as complete for rollout and parity gates
- `core` / `feature-pack` / `deferred` / `reference-only` / `rejected` are exhaustive

## Contract summary

- generated-project contract decisions are tracked in `PLAN.md`
- capability state decisions are tracked in `shared/docs/capability-matrix.json`
- execution sequencing is tracked in the bead graph under `.beads/`

These three surfaces must remain mutually consistent as migration work proceeds.
