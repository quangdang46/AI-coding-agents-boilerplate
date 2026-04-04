# TypeScript Language Pack Tests

Language-pack verification for TypeScript is currently driven from:

- `install/tests/cli.rs`
- `tests/test_typescript_classification.py`

This directory exists to establish the required language-pack test boundary defined by `RULES.md` while TypeScript extraction is still in progress.

Legacy smoke tests preserved under `references/legacy-typescript-workspace/` remain historical migration artifacts and are not the canonical language-pack verification boundary.

The current TypeScript slice covers `init`, `feature add`, and `doctor` through `install/tests/cli.rs`.
