# Completeness Controls

This document defines the reference-ingestion and completeness-guarantee rules for the migration.

It is the canonical governance artifact for Task 6 of `.sisyphus/plans/canonical-migration-plan.md`.

---

## 1. Ingestion flow

The migration treats archived source material as evidence, not as shipped runtime input.

The canonical ingestion flow is:

1. archived source evidence under `references/` is read as migration-only source material.
2. archived capabilities are normalized into stable capability rows.
3. normalized rows are compared against `shared/docs/capability-matrix.json`.
4. gaps, downgrades, and uncategorized capabilities fail validation until explicitly resolved.

The capability matrix is the first-class comparison surface for completeness tracking.

---

## 2. Loss severity rules

### 2.1 Critical failure

Validation MUST fail when any of the following occurs:

- a capability row disappears without a recorded disposition
- a capability row has no `targetBucket`
- a capability row has no `implementationState`
- a broad source cluster is treated as one shippable pack instead of decomposition-guarded `reference-only`

### 2.2 State downgrade failure

Validation MUST fail when a capability’s implementation state is downgraded without explicit audit intent.

State order is:

- `declared`
- `implemented`
- `verified`

Downgrades from a higher state to a lower state require an audit trail in evidence or review artifacts.

---

## 3. Runtime/reference isolation rule

Runtime identity and generated-project behavior must not bind to archived reference material.

### 3.1 Forbidden runtime bindings

The following path forms are forbidden in runtime-facing manifests, config payloads, and generated-project contracts:

- archived reference roots under `references/`
- `reference_data/`
- `archive/claw_code_ts_snapshot/`

These paths may appear in:

- evidence logs
- migration docs
- parity/completeness controls
- archived reference utilities that are explicitly outside shipped runtime ownership

They must not appear as runtime configuration paths for generated projects or language-pack manifests.

---

## 4. Allowed evidence outputs

Derived migration status belongs in evidence and governance artifacts, not in runtime manifests.

Examples of allowed evidence outputs:

- capability matrix reports
- completeness validators
- audit logs
- migration review receipts
- rollout gate summaries

---

## 5. Validation rule summary

The completeness system is correct only when all of the following are true:

- every capability row has a `targetBucket`
- every capability row has an `implementationState`
- oversized source clusters remain decomposition-guarded
- runtime-facing manifests do not point at archived reference paths
- state downgrades require audit evidence
