# Rust Language Pack Tests

Rust language-pack verification is currently driven from:

- `install/tests/cli.rs`

This directory exists to establish the required language-pack test boundary defined by `RULES.md` while Rust feature-pack extraction is still in progress.

The current Rust slice covers `init`, `feature add`, and `doctor` through `install/tests/cli.rs`.

That verification now includes workspace instruction-memory files under `.agents/prompts/`.

That verification also includes layered prompt sections under `.agents/prompts/sections/`.

That verification now includes local file-backed agent roots under `.agents/agents/`.

That verification now includes local file-backed skill roots under `.agents/skills/`.
