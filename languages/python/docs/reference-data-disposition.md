# Python reference_data disposition

The archived `references/python/src/reference_data/` subtree is evidence-only migration material.

It is not part of the shipped Python runtime contract under `languages/python/`, and it must never be reintroduced as a runtime dependency.

## Final disposition

- `archive_surface_snapshot.json` — evidence-only snapshot used for parity audit history.
- `commands_snapshot.json` — evidence-only command inventory snapshot retained for migration audit truth.
- `tools_snapshot.json` — evidence-only tool inventory snapshot retained for migration audit truth.
- `subsystems/skills.json` — evidence-only subsystem inventory fixture.
- `subsystems/voice.json` — evidence-only subsystem inventory fixture.
- `subsystems/vim.json` — evidence-only subsystem inventory fixture.
- `subsystems/state.json` — evidence-only subsystem inventory fixture.
- `subsystems/types.json` — evidence-only subsystem inventory fixture.
- `subsystems/server.json` — evidence-only subsystem inventory fixture.
- `subsystems/constants.json` — evidence-only subsystem inventory fixture.
- `subsystems/entrypoints.json` — evidence-only subsystem inventory fixture.
- `subsystems/keybindings.json` — evidence-only subsystem inventory fixture.
- `subsystems/bootstrap.json` — evidence-only subsystem inventory fixture.
- `subsystems/assistant.json` — evidence-only subsystem inventory fixture.
- `subsystems/outputStyles.json` — evidence-only subsystem inventory fixture.
- `subsystems/plugins.json` — evidence-only subsystem inventory fixture.
- `subsystems/hooks.json` — evidence-only subsystem inventory fixture.
- `subsystems/memdir.json` — evidence-only subsystem inventory fixture.
- `subsystems/utils.json` — evidence-only subsystem inventory fixture.
- `subsystems/remote.json` — evidence-only subsystem inventory fixture.
- `subsystems/upstreamproxy.json` — evidence-only subsystem inventory fixture.
- `subsystems/schemas.json` — evidence-only subsystem inventory fixture.
- `subsystems/coordinator.json` — evidence-only subsystem inventory fixture.
- `subsystems/migrations.json` — evidence-only subsystem inventory fixture.
- `subsystems/components.json` — evidence-only subsystem inventory fixture.
- `subsystems/screens.json` — evidence-only subsystem inventory fixture.
- `subsystems/native_ts.json` — evidence-only subsystem inventory fixture.
- `subsystems/moreright.json` — evidence-only subsystem inventory fixture.
- `subsystems/services.json` — evidence-only subsystem inventory fixture.
- `subsystems/buddy.json` — evidence-only subsystem inventory fixture.
- `subsystems/bridge.json` — evidence-only subsystem inventory fixture.
- `subsystems/cli.json` — evidence-only subsystem inventory fixture.
- `__init__.py` — evidence-only archive package marker retained only for source fidelity.

## Shipped-language-pack rule

Useful behavior from these archived snapshots may inform docs, tests, port tables, or future feature-pack planning, but shipped runtime code under `languages/python/runtime/` must not read from `reference_data/` at runtime.
