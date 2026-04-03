# Rust Language Pack Extension Model

The Rust language pack will follow the shared extension model defined in `RULES.md`:

1. runtime defaults
2. project-local assets
3. enabled feature-pack assets
4. explicit CLI overrides

In the current migration slice, Rust ships runtime defaults for `init`, `feature add/remove`, and `doctor`.

Only the first Rust feature-pack slice is currently shipped; broader Rust feature extraction remains gated by future verification.
