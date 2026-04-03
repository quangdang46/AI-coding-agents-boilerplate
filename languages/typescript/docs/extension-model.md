# TypeScript Language Pack Extension Model

The TypeScript language pack follows the shared extension model defined in `RULES.md`:

1. runtime defaults
2. project-local `.agent/*` assets
3. enabled feature-pack assets
4. explicit CLI overrides

This pack currently exposes the template and config boundary first; deeper runtime extraction remains gated by `shared/docs/typescript-decomposition-map.json` and the pack-owned boundary notes in this directory.
