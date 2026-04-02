# Python Agent Boilerplate Authoring Workspace

This directory contains the reusable Python coding-agent boilerplate and the template assets consumed by the Rust `aicd` installer.

## What is here
- `src/agent_boilerplate/` — reusable runtime/config/registry helpers
- `templates/base/` — generated-project template
- `docs/` — boilerplate architecture and extension contract
- `tests/` — baseline porting tests + new boilerplate tests

## Current supported workflow
- Scaffold a project with `cd ../install && cargo run -- init <name> --language python --output <dir> --yes`
- Add/remove a feature with `aicd feature add/remove <feature-id> --project <dir>`
- Validate a generated project with `aicd doctor --project <dir>`

## Python runtime utilities
Run from `python/`:
- `python -m src.agent_boilerplate.entrypoints.cli doctor --project <dir>`
- `python -m src.agent_boilerplate.entrypoints.cli show-config --project <dir>`
- `python -m src.agent_boilerplate.entrypoints.cli feature add <feature-id> --project <dir>`
- `python -m src.agent_boilerplate.entrypoints.cli feature remove <feature-id> --project <dir>`
