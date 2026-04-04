# Rust Language Pack Extension Model

The Rust language pack will follow the shared extension model defined in `RULES.md`:

1. runtime defaults
2. generated project code under normal source directories
3. project-local `.agents/*` assets
4. project-local native hidden roots such as `.claw/*`
5. explicit CLI overrides

In the current migration slice, Rust ships runtime defaults for `init`, `feature add`, and `doctor`.

Only the first Rust feature-pack slice is currently shipped; broader Rust feature extraction remains gated by future verification.
