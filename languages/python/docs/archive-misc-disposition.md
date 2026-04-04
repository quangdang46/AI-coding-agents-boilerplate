# Python archive misc disposition

The remaining archived Python root docs/config files and one-off support modules below are historical migration artifacts.

They are kept for source-fidelity, audit truth, or design history, but they are not shipped runtime dependencies for the manifest-backed Python language pack.

## Final disposition

- `references/python/CLAUDE.md` — archive-only workflow guidance from the legacy workspace; superseded by current repo rules and language-pack docs.
- `references/python/ROADMAP.md` — archive-only historical product roadmap retained for migration context.
- `references/python/.claude.json` — archive-only configuration snapshot retained for historical audit reference.
- `references/python/.gitignore` — archive-only workspace ignore file retained for source fidelity.
- `references/python/src/cost_tracker.py` — archive-only support module superseded by shipped session/usage tracking outside the archive.
- `references/python/src/_archive_helper.py` — archive-only migration helper; never part of shipped runtime ownership.
- `references/python/src/deferred_init.py` — archive-only startup helper retained for historical reference.
- `references/python/src/parity_audit.py` — archive-only parity helper superseded by shipped repo verification and docs.
- `references/python/src/ink.py` — archive-only UI/runtime support file retained only as source evidence.
- `references/python/src/costHook.py` — archive-only cost-hook helper retained only as source evidence.
- `references/python/src/__init__.py` — archive-only package marker retained for source fidelity.

## Shipped-language-pack rule

These files may inform migration audits, docs, or future planning, but they must not be treated as current Python language-pack ownership surfaces and must not be imported by shipped runtime code.
