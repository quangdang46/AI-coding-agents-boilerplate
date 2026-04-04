# Build Your Own AI Coding Agent

Create a **branded AI coding agent** in **Python**, **TypeScript**, or **Rust** with an open source boilerplate built for real products, not toy demos.

**AI Coding Agents Boilerplate** helps you start faster when you want to build:

- your own AI coding assistant
- your own internal developer agent
- your own branded code agent product
- your own coding-agent runtime you can extend over time

## Why This Repo

Most AI agent repos give you one of two things:

- a prompt demo
- a thin wrapper around an API

This repo gives you a **real starting point for building your own coding agent**.

You get a generated project with:

- a local coding-agent runtime
- branded config and instruction surfaces
- markdown subagents
- skills and feature packs
- doctor checks for generated projects
- session and runtime scaffolding you can evolve into a real product

## Who It's For

This project is for:

- founders building an AI coding product
- teams building internal coding agents for developers
- OSS maintainers who want a custom agent workflow
- engineers who want a serious coding-agent boilerplate instead of starting from zero

## What You Generate

When you initialize a project, you get a coding-agent workspace with:

- a top-level branded instruction file like `<BRAND>.md`
- a branded compat config like `.<brand>.json`
- a branded runtime root like `.<brand>/`
- Claude-compatible `settings.json` and `settings.local.json`
- local markdown subagents under `.<brand>/agents/`
- skills under `.<brand>/skills/`
- commands under `.<brand>/commands/`
- sessions under `.<brand>/sessions/`

The result is a boilerplate you can fork, brand, extend, and ship.

## Quick Start

```bash
aicd list languages
aicd init my-agent --language python
aicd feature add github-pr-review --project ./my-agent
aicd doctor --project ./my-agent
```

Supported language packs:

- Python
- TypeScript
- Rust

## Why Use This Instead Of Starting From Scratch

Because the hard part is not calling a model.

The hard part is designing a coding-agent project that has:

- a maintainable runtime structure
- clean config and instruction surfaces
- extension points for subagents, skills, and feature packs
- enough scaffolding to become a product instead of staying a prototype

This repo gives you that foundation.

## Product Angle

If your goal is:

- “I want to build my own AI coding agent”
- “I want a coding agent boilerplate I can customize”
- “I want to launch a branded AI coding assistant”

then this repo is designed for exactly that path.

## Feature Direction

The boilerplate is built around:

- a core runtime first
- optional capability growth through feature packs
- local extensibility through subagents and skills
- branded surfaces instead of hardcoded upstream identities

That keeps the generated project usable on day one while still leaving room to grow into a larger agent platform.

## Repository Structure

```text
install/      Installer and project generation logic
languages/    Python, TypeScript, and Rust language packs
shared/       Shared schemas and cross-language docs
docs/         Porting and architecture docs
tests/        Repo-level verification
references/   Archived reference evidence used during migration
```

## Current Status

The project already supports:

- manifest-driven language discovery
- project initialization
- feature add flows
- doctor validation
- branded runtime/config scaffolding

It is still expanding, but the generated project contract is already real and test-backed.

## Keywords This Repo Intentionally Serves

- build your own AI coding agent
- coding agent boilerplate
- open source AI coding assistant
- custom coding agent framework
- branded AI developer tool
- Python TypeScript Rust AI agent boilerplate

## License

See [LICENSE](LICENSE).
