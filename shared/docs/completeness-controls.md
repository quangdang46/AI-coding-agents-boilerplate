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
- a capability row in state `implemented` or `verified` has no concrete `evidence`
- a broad source cluster is treated as one shippable pack instead of decomposition-guarded `reference-only`
- a capability row claims the current contract but still points only at known-stale contract surfaces without an explicit migration note

### 2.2 State downgrade failure

Validation MUST fail when a capability’s implementation state is downgraded without explicit audit intent.

State order is:

- `declared`
- `implemented`
- `verified`

Downgrades from a higher state to a lower state require an audit trail in evidence or review artifacts.

### 2.3 Evidence integrity rule

Capability states above `declared` require concrete proof:

- `implemented` MUST cite at least one shipped code path or generated-project runtime path plus one validating test or fixture
- `verified` MUST cite automated proof strong enough to justify the stronger state

Notes alone are not evidence.

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
- every capability row above `declared` has concrete `evidence`
- oversized source clusters remain decomposition-guarded
- runtime-facing manifests do not point at archived reference paths
- state downgrades require audit evidence

---

## 6. `files.txt` bead-close protocol

`files.txt` is the canonical per-file migration checklist.

Every implementation bead that ports, classifies, retires, or explicitly relegates archived source coverage MUST maintain `files.txt` as part of bead closeout.

### 6.1 Claim rule

Before implementation begins, the bead must identify the `files.txt` rows it owns.

Accepted ownership evidence is:

- an explicit coverage manifest in the bead comments, or
- a narrower documented checklist region referenced by the bead and understood before edits begin

Agents must not tick arbitrary rows opportunistically after coding. Row ownership must be known before the bead is closed.

### 6.2 Tick rule

A `files.txt` row may be changed from `[ ]` to `[x]` only when the owning bead has completed the final disposition for that archived path.

Accepted final dispositions include:

- the behavior is now owned by shipped code under `install/` or `languages/`
- the path is intentionally classified into a shipped governance artifact such as a command/tool/feature-pack port table
- the path is explicitly documented as evidence-only, tooling-only, deferred, rejected, or reference-only in the canonical migration artifacts

Ticking a row means the migration decision for that archived path is no longer ambiguous. It does not mean the original file was copied verbatim.

### 6.3 Close rule

An implementation bead is not complete until all of the following are true:

1. its owned `files.txt` rows are identified
2. every owned row that reached final disposition is ticked to `[x]`
3. any supporting evidence lives in the canonical docs, tests, bead comments, or port tables rather than inline in `files.txt`
4. untouched rows outside the bead's owned coverage remain unchanged

### 6.4 Safety rule

Agents must not:

- tick rows owned by another open bead
- tick rows without supporting implementation or explicit disposition evidence
- add narrative notes inline to checklist rows
- close a bead whose claimed coverage region still contains unresolved `[ ]` rows that were supposed to be finished by that bead

### 6.5 Review rule

When reviewing or closing a bead, treat `files.txt` maintenance as a required verification step, not an optional cleanup task.

If checklist ownership is unclear, the bead must remain open until its coverage manifest or equivalent ownership note is corrected.
