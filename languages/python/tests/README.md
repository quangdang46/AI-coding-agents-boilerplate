# Python Language Pack Tests

Language-pack verification for Python is currently driven from:

- `install/tests/cli.rs`
- `tests/test_reference_isolation.py`

This directory exists to establish the required language-pack test boundary defined by `RULES.md`.

Legacy tests preserved under `references/legacy-python-workspace/` remain migration artifacts and are not the canonical language-pack verification boundary.
