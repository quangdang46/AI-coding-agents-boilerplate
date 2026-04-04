# Python Language Pack Tests

Language-pack verification for Python is currently driven from:

- `install/tests/cli.rs`
- `tests/test_reference_isolation.py`

The installer CLI suite now also validates Python doctor parity against real generated runtime artifacts, including regressions where session, export, or usage state becomes incoherent after runtime execution.

This directory exists to establish the required language-pack test boundary defined by `RULES.md`.

Legacy tests preserved under `references/legacy-python-workspace/` remain migration artifacts and are not the canonical language-pack verification boundary.
