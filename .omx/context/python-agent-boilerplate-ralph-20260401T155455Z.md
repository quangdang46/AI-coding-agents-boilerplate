# Ralph Context Snapshot

- timestamp_utc: 20260401T155455Z
- task_statement: Execute `.omx/plans/python-agent-boilerplate-migration-plan-2026-04-01.md` via Ralph.
- desired_outcome: Convert the current `python/` mirror-oriented source into a reusable Python coding-agent boilerplate with config/prompts/agents/skills/features seams, and add a Rust `install/` CLI slice that can scaffold and validate the Python template.

## Known facts / evidence
- Current Python CLI is mirror/reporting oriented (`python/src/main.py`).
- Current Python commands/tools are snapshot-backed and placeholder-executed (`python/src/commands.py`, `python/src/tools.py`).
- Current test baseline passes from `python/` with `python -m unittest discover -s tests -v`.
- TypeScript docs already define target boundary concepts (`typescript/docs/config-reference.md`, `typescript/docs/architecture/core-runtime-boundaries.md`).
- Python migration plan exists at `.omx/plans/python-agent-boilerplate-migration-plan-2026-04-01.md`.
- Ralph planning gate is not yet satisfied for Python because there is no Python PRD/test-spec artifact in `.omx/plans/`.

## Constraints
- Must follow Ralph planning gate before implementation-heavy execution.
- Must preserve fresh verification evidence before claiming completion.
- No new dependencies unless clearly needed; prefer stdlib-first Python.
- Keep diffs reviewable and use compatibility shims during extraction.

## Unknowns / open questions
- Exact final binary name for the Rust installer (`aicd` used as provisional name).
- Whether Python and TypeScript will fully share manifest formats long-term.
- Whether provider scope for Python v1 should include both OpenAI and Anthropic.

## Likely codebase touchpoints
- `python/src/main.py`
- `python/src/query_engine.py`
- `python/src/runtime.py`
- `python/src/commands.py`
- `python/src/tools.py`
- `python/src/session_store.py`
- `python/src/transcript.py`
- `python/tests/test_porting_workspace.py`
- `install/` (new Rust scaffold)
- `.omx/plans/` (PRD/test-spec artifacts)
