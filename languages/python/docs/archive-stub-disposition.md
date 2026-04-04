# Python archive stub disposition

The archived Python package stubs under `references/python/src/**/__init__.py` are source-fidelity artifacts, not shipped runtime modules.

Their final disposition is to remain archive-only compatibility markers while the actual shipped ownership lives under `languages/python/runtime/`, `languages/python/template/base/`, and repo-level verification in `tests/`.

## Final disposition

- `references/python/src/assistant/__init__.py` — archive-only namespace stub; shipped ownership replaced by Python language-pack runtime and template surfaces.
- `references/python/src/bootstrap/__init__.py` — archive-only namespace stub; shipped ownership replaced by Python language-pack runtime and template surfaces.
- `references/python/src/bridge/__init__.py` — archive-only namespace stub; no shipped runtime dependency remains.
- `references/python/src/buddy/__init__.py` — archive-only namespace stub; no shipped runtime dependency remains.
- `references/python/src/cli/__init__.py` — archive-only namespace stub; shipped CLI ownership lives in generated project template code.
- `references/python/src/components/__init__.py` — archive-only namespace stub; no shipped runtime dependency remains.
- `references/python/src/constants/__init__.py` — archive-only namespace stub; shipped config and runtime defaults live outside the archive.
- `references/python/src/coordinator/__init__.py` — archive-only namespace stub; no shipped runtime dependency remains.
- `references/python/src/entrypoints/__init__.py` — archive-only namespace stub; shipped entrypoints live under `languages/python/runtime/entrypoints/` and template code.
- `references/python/src/hooks/__init__.py` — archive-only namespace stub; advanced hook behavior remains outside the shipped core slice.
- `references/python/src/keybindings/__init__.py` — archive-only namespace stub; no shipped runtime dependency remains.
- `references/python/src/memdir/__init__.py` — archive-only namespace stub; no shipped runtime dependency remains.
- `references/python/src/migrations/__init__.py` — archive-only namespace stub; no shipped runtime dependency remains.
- `references/python/src/moreright/__init__.py` — archive-only namespace stub; no shipped runtime dependency remains.
- `references/python/src/native_ts/__init__.py` — archive-only namespace stub; no shipped runtime dependency remains.
- `references/python/src/outputStyles/__init__.py` — archive-only namespace stub; shipped output behavior is implemented elsewhere.
- `references/python/src/plugins/__init__.py` — archive-only namespace stub; plugin runtime remains outside the shipped core slice.
- `references/python/src/remote/__init__.py` — archive-only namespace stub; no shipped runtime dependency remains.
- `references/python/src/schemas/__init__.py` — archive-only namespace stub; schema-backed validation is owned by shipped repo surfaces outside the archive.
- `references/python/src/screens/__init__.py` — archive-only namespace stub; no shipped runtime dependency remains.
- `references/python/src/server/__init__.py` — archive-only namespace stub; no shipped runtime dependency remains.
- `references/python/src/services/__init__.py` — archive-only namespace stub; no shipped runtime dependency remains.
- `references/python/src/skills/__init__.py` — archive-only namespace stub; shipped skill ownership lives in language-pack docs, template assets, and feature packs.
- `references/python/src/state/__init__.py` — archive-only namespace stub; shipped state ownership is replaced by runtime/session helpers outside the archive.
- `references/python/src/types/__init__.py` — archive-only namespace stub; shipped typed ownership is replaced by language-pack runtime modules.
- `references/python/src/upstreamproxy/__init__.py` — archive-only namespace stub; upstream proxy behavior is not part of the shipped Python core slice.
- `references/python/src/utils/__init__.py` — archive-only namespace stub; utility sprawl is intentionally not preserved as a shipped monolith.
- `references/python/src/vim/__init__.py` — archive-only namespace stub; no shipped runtime dependency remains.
- `references/python/src/voice/__init__.py` — archive-only namespace stub; no shipped runtime dependency remains.
- `references/python/tests/test_porting_workspace.py` — archive-only historical harness superseded by shipped repo verification under `tests/` and language-pack verification surfaces.

## Shipped-language-pack rule

These archive stubs remain only to preserve original source identity during migration. They must not be imported by shipped runtime code, and they must not be treated as active ownership surfaces for the Python language pack.
