# Archive Retirement Gate

This document defines the final evidence required before the archived source tree under `references/` can be retired.

It is the canonical archive-removal gate for the migration closeout tranche.

---

## 1. Retirement rule

The archive may be removed only when all of the following are true:

1. `files.txt` is actively maintained as the canonical per-file migration checklist.
2. `docs/porting/command-port-table.md` is present and kept current.
3. `docs/porting/tool-port-table.md` is present and kept current.
4. `docs/porting/feature-pack-port-table.md` is present and kept current.
5. Runtime-facing code under `install/` and `languages/` does not bind to archived reference paths.
6. Archive-independence tests and migration red tests are green.
7. Release-gate documentation still points to the same archive-independence evidence.

Archive deletion is blocked if any item above is missing or stale.

---

## 2. Required evidence

The required evidence set is:

- `files.txt`
- `docs/porting/command-port-table.md`
- `docs/porting/tool-port-table.md`
- `docs/porting/feature-pack-port-table.md`
- `shared/docs/completeness-controls.md`
- `shared/docs/release-gates.md`
- `tests/test_reference_isolation.py`
- `tests/test_release_gates.py`
- `install/tests/migration_red.rs`

These artifacts preserve migration truth after archive removal. They are the minimum documentation-and-test set that must remain intact.

`shared/docs/completeness-controls.md` defines the bead-close protocol for claiming and ticking `files.txt` rows. Archive retirement is blocked if `files.txt` exists but bead closeout does not follow that protocol.

---

## 3. Runtime isolation restatement

Shipped runtime behavior must remain owned by `install/` and `languages/`.

The archive remains evidence-only. runtime-facing manifests, configs, registries, and generated-project contracts must not require `references/` to exist.

---

## 4. Decision rule

If any required evidence artifact is missing, or if runtime/reference isolation tests fail, archive retirement is blocked.

If all required evidence artifacts are present and automated isolation checks are green, archive retirement may proceed to the explicit deletion review stage.

---

## 5. Evidence-only artifact classes

The following archived artifact classes are evidence-only and must not be treated as shipped runtime implementation targets:

- session dumps and conversation logs under archive-local `.claude/sessions/` or `.claw/sessions/`
- lockfiles and environment-state captures such as `Cargo.lock`, `bun.lock`, or rustup settings
- screenshots and other visual evidence assets
- plan documents, enhancement notes, and mock parity harness writeups
- parity harness scaffolding, compatibility harnesses, and archive-only runner scripts
- archive audit helpers, snapshots, and `reference_data/` inventory material

These artifacts remain useful for auditability, but they should be classified as evidence-only or tooling-only rather than active runtime port obligations.
