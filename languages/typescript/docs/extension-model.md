# TypeScript Language Pack Extension Model

The TypeScript language pack follows the shared extension model defined in `RULES.md`:

1. runtime defaults
2. generated project code under normal source directories
3. project-local `.agents/*` assets
4. project-local native hidden roots when present
5. explicit CLI overrides

This pack currently exposes the template and config boundary first; deeper runtime extraction remains gated by `shared/docs/typescript-decomposition-map.json` and the pack-owned boundary notes in this directory.
