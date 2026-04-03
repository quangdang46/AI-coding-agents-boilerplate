# Rust Language Pack Feature Packs

The Rust pack currently ships one reversible feature pack:

- `local-plugins`

This boundary exists so future Rust extraction work can add reversible feature packs under `languages/rust/` without mixing shipped artifacts with archived Rust reference material.

The first Rust feature-pack slice creates a workspace-local `.claw/plugins/` boundary and records the feature in `.claw.json` without claiming the full upstream plugin lifecycle.
