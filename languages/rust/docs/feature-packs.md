# Rust Language Pack Feature Packs

The Rust pack currently ships one add-only feature pack:

- `local-plugins`

Canonical Rust feature authoring lives under:

- `languages/rust/features/registry.json`
- `languages/rust/features/<feature-id>/feature.json`

The first Rust feature-pack slice creates a branded local plugin manifest at:

- `.<brand>-plugin/plugin.json`

and records the feature in the branded compat config `.<brand>.json`.
