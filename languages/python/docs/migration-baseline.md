# Python Language Pack Migration Baseline

This document is the canonical migration baseline for the manifest-backed Python language pack under `languages/python/`.

Any archived legacy Python workspace belongs under `references/legacy-python-workspace/`. It is not the long-term shipped ownership location for the Python language pack.

## Current owned surfaces
- `languages/python/` — canonical Python language-pack ownership root
- `install/` — manifest-driven installer/orchestration layer consuming the language pack
- `references/source-python/` — archived Python source evidence used for migration analysis only

## Migration invariants
1. Generated-project template ownership lives under `languages/python/template/base/`.
2. Installer-driven init/add/remove/doctor flows must remain manifest-driven.
3. Archived Python migration material must not be treated as runtime-facing generated-project input.
4. Python language-pack verification should be expressed through pack-owned or cross-repo tests, not root-workspace authority.
