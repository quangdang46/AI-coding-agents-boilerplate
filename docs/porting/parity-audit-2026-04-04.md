# Parity Audit 2026-04-04

This audit records the current trust gaps between:

- the original-source capability inventory under `references/`
- the current capability and feature-pack docs
- the bead graph
- the runtime and installer code that actually exists today

The purpose of this audit is not to reopen all migration planning. It is to make parity claims truthful enough that implementation can continue safely.

---

## 1. What is not missing

At the feature-family level, the repository is not obviously missing large optional capability buckets from the source inventory.

`FEATURES.md`, `shared/docs/capability-matrix.json`, and `docs/porting/feature-pack-port-table.md` currently cover the major optional families:

- planning
- multi-agent workflows
- MCP
- LSP
- Git/GitHub workflows
- plugin runtime and marketplace behavior
- hooks
- memory and team-memory
- onboarding/auth flows
- remote transport / bridge / proxy / direct-connect
- prompt suggestion services

The main risk is not missing taxonomy. The main risk is overstated implementation state and stale contract assumptions.

---

## 2. Capabilities that were overstated or contract-stale

The following rows previously claimed `verified` while still depending on the interim `.agent/*` contract or other pre-refactor assumptions:

- `workspace-bootstrap-init`
- `local-agent-discovery`
- `local-skill-discovery`
- `workspace-instruction-memory`
- `system-prompt-layering`
- `workspace-editable-extension-seams`
- `schema-backed-validation`

Audit decision:

- downgrade these rows from `verified` to `implemented`
- attach concrete evidence
- update notes to say they are implemented for the current shipped slice, but not yet aligned with the new `.agents` + native-hidden-root contract

The following rows were not downgraded, but their notes were corrected because they implied “reserved seam only” despite real runtime code existing:

- `usage-and-cost-tracking`
- `saved-sessions-and-resume`
- `workspace-context-building`

Audit decision:

- keep them at `implemented`
- add concrete runtime evidence
- replace “for later” wording with wording that matches the existing Python/TypeScript/Rust runtime foundations

---

## 3. Beads that were closed too early and must be reopened

The following beads must be reopened because the repository contract changed materially after they were closed, or because their acceptance criteria are no longer sufficient for the new target architecture:

- `aicd-3ix.1.5` — feature-pack port table still needs add-only + feature-skill contract alignment
- `aicd-3ix.1.6` — plan, matrix, and bead graph were no longer mutually consistent after the contract shift
- `aicd-3ix.2.1` — generated-project contract changed to `.agents` + optional native hidden roots
- `aicd-3ix.2.7` — init/doctor/feature lifecycle is now add-only and must validate new bootstrap surfaces
- `aicd-3ix.2.8` — parity tests were accepted against the old contract and need follow-up proof for the new one

These reopenings are not reversions of the useful work already done. They are contract-alignment reopenings.

---

## 4. Optional feature families still lacking implementation evidence

The following feature-pack families remain planned but unevidenced as shipped implementation:

- `assistant-session-history`
- `bridge-remote-control`
- `bundled-skills`
- `direct-connect-server`
- `doctor-ui`
- `external-app-integrations`
- `git-workflows`
- `hooks-runtime`
- `multi-agent-workflows`
- `oauth-onboarding`
- `plugin-marketplace-ui`
- `plugin-runtime`
- `prompt-suggestion-services`
- `session-compaction`
- `session-memory`
- `specialist-agents`
- `streaming-responses`
- `structured-remote-transport`
- `swarm-permissions`
- `team-memory`
- `upstream-proxy`

This does not mean the repo forgot these families.

It means:

- they are represented in the inventory and port tables
- they are assigned feature ids and owning beads
- they are not yet backed by implementation evidence strong enough to claim parity

---

## 5. Operational conclusion

The repo can continue implementing beads safely if it follows these rules:

- trust `FEATURES.md` as source inventory, not implementation proof
- trust `feature-pack-port-table.md` as optional-family coverage, not implementation proof
- trust `capability-matrix.json` only after state and evidence are audited
- reopen contract/governance/test beads whenever the generated-project contract changes materially

This audit exists so future bead completion can be measured against truthful migration state rather than inherited overclaims.
