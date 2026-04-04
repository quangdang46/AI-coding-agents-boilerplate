# Rust Runtime Registry

Owned location for Rust language-pack registry helpers and feature or asset lookup concerns.

Current shipped ownership for the Rust registry slice is exercised through the generated-project template:

- `languages/rust/template/base/src/commands.rs` owns the shipped command registry summary and slash-command surface metadata.
- `languages/rust/template/base/src/tools.rs` owns the shipped tool registry summary, enabled-tool filtering, and alias normalization metadata.
- `languages/rust/template/base/src/bootstrap.rs` and `src/conversation.rs` expose those owned registries in the runtime summary surfaced by generated Rust projects.
