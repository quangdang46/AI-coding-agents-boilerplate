# Python Boilerplate Migration Notes

This document captures the first extraction boundary for turning `python/` from a porting workspace into a reusable coding-agent boilerplate.

## Current facts
- The current CLI is report/mirror oriented.
- Commands and tools currently load from snapshot JSON files.
- The query engine and runtime prove useful session and orchestration seams.
- The default Python baseline tests pass from `python/`.

## Keep
- CLI dispatch pattern
- session/transcript/persistence seams
- permission filtering pattern
- registry-driven command/tool composition pattern

## Remove from shipped boilerplate
- parity audit and manifest reporting commands
- snapshot-backed runtime registries
- placeholder remote/direct mode commands
- runtime dependency on `reference_data/`
- placeholder archive metadata packages

## Generalize
- runtime kernel into `src/agent_boilerplate/core/`
- config into `src/agent_boilerplate/config/`
- prompts into `src/agent_boilerplate/prompts/`
- registries into `src/agent_boilerplate/registry/`
- generated project assets into `templates/base/`

## Migration invariant
Introduce the new package boundaries first and move behavior behind compatibility shims instead of performing a flag-day rewrite.
