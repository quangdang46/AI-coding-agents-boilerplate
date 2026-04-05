# Rust workspace artifacts ownership

The archived Rust workspace artifacts in bead `aicd-3ix.5.11.1` are not preserved as shipped runtime modules in the current Rust language-pack slice. Instead, these rows are retained as archive-only planning history, parity evidence, session history, or verification support so the shipped Rust pack can keep a clean runtime boundary.

## Ownership and disposition

- `references/rust/TUI-ENHANCEMENT-PLAN.md` — archive-only planning artifact retained for source fidelity; the current shipped Rust language pack does not ship this historical enhancement plan one-for-one.
- `references/rust/.omc/plans/tui-enhancement-plan.md` — archive-only planning artifact retained for source fidelity; the current shipped Rust language pack does not ship this local planning record one-for-one.
- `references/rust/.claude/sessions/session-1775009431231.json` — archive-only session log retained for source fidelity; the current shipped Rust language pack does not expose archived Claude session history as a runtime module.
- `references/rust/.claude/sessions/session-1775009769569.json` — archive-only session log retained for source fidelity; the current shipped Rust language pack does not expose archived Claude session history as a runtime module.
- `references/rust/.claude/sessions/session-1775008071886.json` — archive-only session log retained for source fidelity; the current shipped Rust language pack does not expose archived Claude session history as a runtime module.
- `references/rust/.claude/sessions/session-1775008464519.json` — archive-only session log retained for source fidelity; the current shipped Rust language pack does not expose archived Claude session history as a runtime module.
- `references/rust/.claude/sessions/session-1775008308936.json` — archive-only session log retained for source fidelity; the current shipped Rust language pack does not expose archived Claude session history as a runtime module.
- `references/rust/.claude/sessions/session-1775011146355.json` — archive-only session log retained for source fidelity; the current shipped Rust language pack does not expose archived Claude session history as a runtime module.
- `references/rust/.claude/sessions/session-1775012687059.json` — archive-only session log retained for source fidelity; the current shipped Rust language pack does not expose archived Claude session history as a runtime module.
- `references/rust/.claude/sessions/session-1775010047738.json` — archive-only session log retained for source fidelity; the current shipped Rust language pack does not expose archived Claude session history as a runtime module.
- `references/rust/.claude/sessions/session-1775008137143.json` — archive-only session log retained for source fidelity; the current shipped Rust language pack does not expose archived Claude session history as a runtime module.
- `references/rust/.claude/sessions/session-1775008007069.json` — archive-only session log retained for source fidelity; the current shipped Rust language pack does not expose archived Claude session history as a runtime module.
- `references/rust/.claude/sessions/session-1775012674485.json` — archive-only session log retained for source fidelity; the current shipped Rust language pack does not expose archived Claude session history as a runtime module.
- `references/rust/.claude/sessions/session-1775010384918.json` — archive-only session log retained for source fidelity; the current shipped Rust language pack does not expose archived Claude session history as a runtime module.
- `references/rust/.claude/sessions/session-1775009841982.json` — archive-only session log retained for source fidelity; the current shipped Rust language pack does not expose archived Claude session history as a runtime module.
- `references/rust/.claude/sessions/session-1775011562247.json` — archive-only session log retained for source fidelity; the current shipped Rust language pack does not expose archived Claude session history as a runtime module.
- `references/rust/.claude/sessions/session-1775010333630.json` — archive-only session log retained for source fidelity; the current shipped Rust language pack does not expose archived Claude session history as a runtime module.
- `references/rust/.claude/sessions/session-1775007981374.json` — archive-only session log retained for source fidelity; the current shipped Rust language pack does not expose archived Claude session history as a runtime module.
- `references/rust/.claude/sessions/session-1775010909274.json` — archive-only session log retained for source fidelity; the current shipped Rust language pack does not expose archived Claude session history as a runtime module.
- `references/rust/.claude/sessions/session-1775008161929.json` — archive-only session log retained for source fidelity; the current shipped Rust language pack does not expose archived Claude session history as a runtime module.
- `references/rust/.claude/sessions/session-1775013221875.json` — archive-only session log retained for source fidelity; the current shipped Rust language pack does not expose archived Claude session history as a runtime module.
- `references/rust/.claude/sessions/session-1775007484031.json` — archive-only session log retained for source fidelity; the current shipped Rust language pack does not expose archived Claude session history as a runtime module.
- `references/rust/.claude/sessions/session-1775009126336.json` — archive-only session log retained for source fidelity; the current shipped Rust language pack does not expose archived Claude session history as a runtime module.
- `references/rust/.claude/sessions/session-1775008997307.json` — archive-only session log retained for source fidelity; the current shipped Rust language pack does not expose archived Claude session history as a runtime module.
- `references/rust/.claude/sessions/session-1775007453382.json` — archive-only session log retained for source fidelity; the current shipped Rust language pack does not expose archived Claude session history as a runtime module.
- `references/rust/.claude/sessions/session-1775008427969.json` — archive-only session log retained for source fidelity; the current shipped Rust language pack does not expose archived Claude session history as a runtime module.
- `references/rust/.claude/sessions/session-1775009119214.json` — archive-only session log retained for source fidelity; the current shipped Rust language pack does not expose archived Claude session history as a runtime module.
- `references/rust/.claude/sessions/session-1775007490104.json` — archive-only session log retained for source fidelity; the current shipped Rust language pack does not expose archived Claude session history as a runtime module.
- `references/rust/.claude/sessions/session-1775009145469.json` — archive-only session log retained for source fidelity; the current shipped Rust language pack does not expose archived Claude session history as a runtime module.
- `references/rust/.claude/sessions/session-1775009869734.json` — archive-only session log retained for source fidelity; the current shipped Rust language pack does not expose archived Claude session history as a runtime module.
- `references/rust/scripts/run_mock_parity_diff.py` — archive-only parity helper retained for source fidelity and verification reference; the current shipped Rust language pack does not ship this historical diff script one-for-one.
- `references/rust/scripts/run_mock_parity_harness.sh` — archive-only parity harness helper retained for source fidelity and verification reference; the current shipped Rust language pack does not ship this historical shell harness one-for-one.
- `references/rust/.clawd-todos.json` — archive-only task-history artifact retained for source fidelity; the current shipped Rust language pack does not expose local todo state as a runtime surface.
- `references/rust/PARITY.md` — archive-only parity evidence document retained for verification history; the current shipped Rust language pack does not ship this archived parity note one-for-one.
- `references/rust/mock_parity_scenarios.json` — archive-only parity scenario fixture retained for verification history; the current shipped Rust language pack does not expose this archived scenario file as a shipped runtime asset.
- `references/rust/MOCK_PARITY_HARNESS.md` — archive-only parity harness document retained for verification history; the current shipped Rust language pack does not ship this archived harness document one-for-one.

## Shipped-language-pack rule

This subset is complete when each archived workspace artifact row above has an explicit shipped-owner or archive-only rationale. These planning files, session logs, parity documents, scenario fixtures, and helper scripts must remain clearly separated from shipped Rust runtime ownership so archive noise does not get mistaken for a required language-pack module.
