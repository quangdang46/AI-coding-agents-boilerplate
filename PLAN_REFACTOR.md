# AICD Refactor Plan

_Date: 2026-04-04_

## Status Snapshot

This refactor plan is now partially executed, not only proposed.

Completed in the current pass:

- contract docs were rewritten around contributor authoring trees vs generated-project hidden roots
- `RULES.md` and `PLAN.md` now describe add-only feature delivery, `.agents/`, optional native hidden roots, and one feature-owned `SKILL.md`
- language manifests and schema were moved to the new add-only dual-hidden-root contract
- base templates were migrated from `.agent/` to `.agents/`
- feature authoring roots were moved out of templates into `languages/<id>/features/`
- installer feature discovery now reads from language-pack authoring registries
- `feature add` now materializes from language-pack source and writes project receipts under `.agents/features/`
- installer and fixture tests were updated to the add-only contract and are passing
- audit/governance docs were corrected to reduce overclaim risk

Still follow-up work:

- finish propagating the new contract across remaining non-critical docs and matrices
- continue migrating capability rows back toward `verified` only when new-contract evidence exists
- implement the remaining open runtime/feature-pack beads on top of the corrected contract

## 1. Mission

Refactor the repository so the project follows its intended direction:

- `languages/<language>/...` is the contributor-facing authoring tree
- generated projects may expose both a generic hidden agent root and one or more native runtime roots
- hidden roots are not treated as the source-of-truth authoring location for language packs or feature packs
- manifests, installer behavior, templates, docs, and tests all describe the same model
- feature lifecycle is add-only: users own generated code after install, and the tool must still be able to add more features later
- feature packs should ship AI-usable development skills so users can invoke best-practice workflows instead of re-explaining feature intent ad hoc

This plan corrects the current architectural drift where generated-project layout and language-pack authoring layout are partially conflated.

## 2. Problem Summary

The current repo mixes two different concerns:

1. language-pack authoring inside this repository
2. runtime/workspace layout emitted into generated projects

Today, the contract treats project-local hidden assets as if they were also the contributor-facing source tree:

- `RULES.md` defines feature packs under `template/base/.agent/features/`
- `install/src/commands/list.rs` enumerates language features from `languages/<id>/<template-base>/.agent/features`
- `install/src/commands/feature.rs` and `install/src/commands/doctor.rs` assume feature manifests live under project-local `.agent/features`
- all three manifests point `featureRegistry` at `.agent/features/registry.json`
- templates and runtime code currently blur generic agent surfaces and native runtime surfaces

That model is difficult to extend because contributors must author features inside a generated-project-shaped tree instead of a language-pack-shaped tree.

## 3. Directional Decision

The repository should adopt a strict separation between:

### 3.1 Authoring layout

This is the canonical source tree used by contributors.

Suggested target:

```text
languages/<language-id>/
├─ language.manifest.json
├─ README.md
├─ docs/
├─ runtime/
├─ template/
│  └─ base/
├─ features/
│  ├─ registry.json
│  └─ <feature-id>/
│     ├─ feature.json
│     ├─ files/
│     ├─ patches/
│     └─ docs.md
├─ prompts/         # optional language-level shared authoring assets
├─ agents/          # optional language-level shared authoring assets
├─ skills/          # optional language-level shared authoring assets
└─ tests/
```

### 3.2 Generated-project hidden-root layout

Generated projects may carry both:

- `.agents/` as a generic bootstrap/dev surface
- `.<runtime-name>/` as the native runtime surface of the shipped tool

Key rules:

- hidden roots are named by surface identity, not by the user project name
- `.agents/` is a generic interoperability/bootstrap surface
- `.agents/` is the primary home for portable agent-facing assets, especially one feature skill per installed feature
- `.<runtime-name>/` proves native self-host/runtime identity
- both roots may coexist in the same generated project
- neither root is the canonical feature authoring tree

Example shape:

```text
<generated-project>/
├─ <language-config-file>
├─ <app source>
├─ tests/
├─ README.md
├─ .agents/
│  ├─ skills/
│  │  └─ <skill-name>/
│  │     └─ SKILL.md
│  ├─ prompts/
│  ├─ agents/
│  ├─ features/
│  ├─ context/
│  ├─ sessions/
│  └─ usage/
└─ .<runtime-name>/
   ├─ prompts/
   ├─ agents/
   ├─ skills/
   ├─ features/
   ├─ context/
   ├─ sessions/
   └─ usage/
```

The exact native hidden root name can be product-specific, but the architecture must treat both hidden roots as generated runtime surfaces rather than language-pack authoring surfaces.

## 4. Refactor Goals

### 4.1 Primary goals

- make language packs easy to extend without editing generated-project-shaped trees
- remove the assumption that feature-pack source lives under `template/base/.agent/features/`
- preserve the generated project's dual hidden-root model
- preserve manifest-driven discovery and robust `feature add`

### 4.2 Non-goals

- do not redesign the entire runtime kernel in this refactor
- do not change unrelated behavior outside language-pack layout, feature-pack sourcing, and hidden-root semantics
- do not add new dependencies

## 5. Current Evidence

These files encode the current conflation and will need coordinated updates:

- `RULES.md`
- `PLAN.md`
- `languages/python/language.manifest.json`
- `languages/typescript/language.manifest.json`
- `languages/rust/language.manifest.json`
- `install/src/manifest.rs`
- `install/src/renderer.rs`
- `install/src/commands/init.rs`
- `install/src/commands/list.rs`
- `install/src/commands/feature.rs`
- `install/src/commands/doctor.rs`
- `languages/*/template/base/*`
- `languages/*/docs/*`
- `shared/docs/capability-matrix.json`
- `tests/test_completeness_controls.py`
- `install/tests/fixtures/manifests/*.json`

Important current assumptions:

- template rendering is sourced from `manifest.templates.base` via `install/src/renderer.rs`
- language features are listed from `template/base/.agent/features` in `install/src/commands/list.rs`
- project-local feature manifests are read from `.agent/features/<feature-id>/feature.json` in `install/src/commands/feature.rs`
- doctor validates required project paths under `.agent/...` in `install/src/commands/doctor.rs`

## 6. Target Contract

### 6.1 Manifest model

The manifest should stop overloading one field for two meanings.

Target shape:

- keep template location for `init`
- add an explicit language-pack feature source root for contributor authoring
- add explicit generated-project hidden-root semantics for generic and native surfaces

Suggested semantic split:

```json
{
  "templates": {
    "base": "template/base"
  },
  "authoring": {
    "featureRegistry": "features/registry.json"
  },
  "runtime": {
    "configFile": "...",
    "projectMarkers": ["..."],
    "genericWorkspaceRoot": ".agents",
    "nativeWorkspaceRoot": ".join"
  }
}
```

Exact field names can vary, but the split in meaning should be explicit.

### 6.2 Feature-pack model

Feature packs should be authored under:

- `languages/<id>/features/registry.json`
- `languages/<id>/features/<feature-id>/feature.json`
- `languages/<id>/features/<feature-id>/files/...`
- `languages/<id>/features/<feature-id>/skill/SKILL.md` when the feature ships a reusable AI workflow

At `feature add` time, the installer should:

1. load the language-pack feature source from the repo
2. copy/materialize files into the generated project
3. patch config/runtime state
4. update the generated project's local runtime registry if one exists
5. validate the result

The target model is intentionally add-only:

- feature packs are generated into user-owned project code
- after generation, users are free to modify the emitted code
- the tool must remain able to add more features later as long as bootstrap config/discovery still works
- `feature remove` is not part of the long-term contract

Feature packs are not only code bundles. In the MVP architecture they should also be able to ship an AI development interface:

- code/assets needed for the feature to function
- one `SKILL.md` file that teaches an LLM how to extend, operate, or continue work on that feature

This lets users invoke a known feature skill instead of re-describing the feature from scratch each time.

### 6.3 Hidden-root model

Generated projects should load prompts, agents, skills, sessions, usage, and related local assets from hidden roots.

Those roots should be:

- declared by contract
- referenced by runtime code and validation
- independent from the user project name

The model should distinguish:

- `.agents/` as the generic bootstrap/dev root that other tools or agents can understand
- `.<runtime-name>/` as the native root used by the runtime/tool being developed

The architecture should allow both to coexist. It should not force the system to pretend only one hidden root is valid.

For the MVP, `.agents/` should be optimized for agent-readable development assets rather than native runtime state. The most important artifact in that root is `SKILL.md`.

### 6.4 Shadcn-style customization model

The system must let users customize generated projects heavily and still keep `feature add` usable later.

The desired behavior is closer to `shadcn/ui` than to a package manager:

- the tool keeps only a small bootstrap/discovery contract
- `feature add` materializes real source files into the project
- once written, that code is user-owned
- the tool does not promise clean reversible removal of arbitrary user-edited generated code

This means customization should not depend on a rigid managed-vs-local directory split for all editable assets.

Instead, the contract should be:

- keep a fixed bootstrap surface for tool discovery and project detection
- allow native runtime roots to exist alongside the generic bootstrap surface
- keep feature source-of-truth in `languages/<id>/features/`
- generate files into normal project paths
- generate one feature-level `SKILL.md` asset into `.agents/skills/<skill-name>/...` so external LLM workflows have a reusable development entrypoint
- record minimal receipts so the tool knows what it added, but treat those receipts as advisory for future add/upgrade flows, not as strict package-manager ownership

Suggested receipt metadata per installed feature:

- feature id and version
- source manifest hash
- generated file list
- generated config mutations

Operational policy:

- user customization can happen anywhere in generated code
- `feature add` must avoid depending on untouched generated files whenever possible
- if a later add needs to touch an existing file, use narrow patches, explicit markers, or fail with a clear message rather than assuming the file is pristine
- doctor should validate bootstrap integrity across the declared hidden roots, not enforce tool ownership of all generated code
- users should be able to invoke the shipped feature skill as the preferred way to continue development on that feature

Acceptance criteria:

- a user can heavily modify generated project code and still add another feature later
- the tool requires only the declared bootstrap/config contract to keep operating
- feature add failures point to concrete missing anchors or broken bootstrap config
- the architecture does not depend on guaranteed safe feature removal
- feature best practices can be carried forward through shipped `SKILL.md` files instead of relying on repeated free-form prompting

## 7. Execution Plan

### Phase 1: Rewrite the architectural contract

Update:

- `RULES.md`
- `PLAN.md`
- affected docs under `languages/*/docs/`
- affected shared docs

Changes:

- define authoring layout separately from generated-project layout
- define hidden-root semantics explicitly
- move canonical feature-pack authoring location out of `template/base/.agent/features/`
- stop describing generated-project hidden folders as the contributor source-of-truth
- define `.agents` as the generic agent-facing asset root, with exactly one first-class `SKILL.md` deliverable per feature in the MVP

Acceptance criteria:

- the docs clearly distinguish authoring tree vs generated runtime tree
- no spec section claims feature packs live canonically under `template/base/.agent/features/`
- the hidden roots are described as generated runtime surfaces, not as the source tree
- the docs state that shipped feature skills are the preferred AI handoff surface for continuing work on a feature

### Phase 2: Introduce the new manifest contract

Update:

- `languages/*/language.manifest.json`
- `install/src/manifest.rs`
- manifest schema/tests/fixtures

Changes:

- split repo authoring paths from generated runtime paths
- add an explicit field for language-pack feature source
- add or normalize fields describing the generated hidden roots

Acceptance criteria:

- manifests validate with the new split
- loader code can resolve both template source and feature source without guessing
- tests fail clearly for missing authoring/hidden-root fields

### Phase 3: Move language feature source out of template trees

Update:

- `languages/python/features/...`
- `languages/typescript/features/...`
- `languages/rust/features/...`

Changes:

- create canonical `features/` directories at language-pack root
- move existing feature assets out of `template/base/.agent/features/`
- introduce a canonical single-skill authoring location alongside feature source
- keep `template/base/` focused on base generated-project scaffolding

Acceptance criteria:

- every existing feature has one canonical authoring location under `languages/<id>/features/`
- each feature-owned skill has one canonical authoring location under the language-pack feature tree: `features/<feature-id>/skill/SKILL.md`
- base templates remain scaffoldable without feature-authoring duplication
- no contributor workflow requires editing `template/base/.agent/features/` as the primary source

### Phase 4: Refactor installer feature flows

Update:

- `install/src/commands/list.rs`
- `install/src/commands/feature.rs`
- `install/src/commands/doctor.rs`
- any supporting installer code

Changes:

- list features from language-pack authoring registries
- add features from repo-sourced feature packs rather than project-local authoring copies
- validate generated projects against the declared hidden roots and bootstrap contract
- drop `feature remove` from the target architecture
- record lightweight feature-install receipts to support later add/upgrade diagnostics
- materialize each feature's single `SKILL.md` asset into `.agents/skills/<skill-name>/SKILL.md` during add flows

Acceptance criteria:

- `aicd list features` reads from `languages/<id>/features/`
- `aicd feature add` copies from repo authoring source into the generated project
- `aicd doctor` validates generated runtime assets without assuming they are the repo authoring source
- `aicd feature add` still works after users have modified previously generated code, as long as required bootstrap anchors still exist
- added features expose one reusable `SKILL.md` entrypoint under `.agents/skills/<skill-name>/SKILL.md` when the feature ships a skill

### Phase 5: Normalize hidden-root usage

Update:

- language templates
- runtime loaders in generated templates
- config files such as `agentkit.toml`, `boilerplate.config.ts`, `.claw.json`
- language docs and README files

Changes:

- stop hardcoding `.agent` as an accidental universal source-tree concept
- represent `.agents` and native runtime roots as separate explicit concepts
- keep Rust-specific `.claw` surfaces only where they are intentionally product-specific
- define `.agents` payloads around agent-facing assets, especially skills

Acceptance criteria:

- each generated project has one coherent hidden-root story covering both generic and native surfaces when both exist
- config/runtime code points at declared hidden roots deliberately
- docs no longer imply `.agent` is both runtime root and authoring root
- `.agents` is documented as the preferred LLM-facing development interface

### Phase 6: Rebuild tests around the corrected model

Update:

- root tests
- installer tests
- manifest fixtures
- language-pack tests

Changes:

- test authoring-tree discovery separately from generated-project validation
- test `init`
- test repeated `feature add`
- test doctor failure modes
- test hidden-root loading

Acceptance criteria:

- tests prove the contract separation is real
- tests catch regressions where authoring and generated layouts are conflated again

## 8. Proposed Order of Code Changes

Use this implementation order to keep the diff reviewable and reversible:

1. land contract/doc rewrite
2. update manifest schema and fixture tests
3. add new `languages/<id>/features/` roots
4. refactor installer feature discovery and materialization
5. migrate templates/runtime configs to explicit hidden-root semantics
6. update doctor and integration tests
7. remove dead paths and stale `.agent` assumptions

## 9. Risks and Mitigations

### Risk: partial migration leaves two competing truths

Mitigation:

- do not leave both `template/base/.agent/features/` and `languages/<id>/features/` as equal canonical sources
- if temporary compatibility is required, declare one canonical source and one transitional mirror

### Risk: generated projects lose feature add compatibility after user customization

Mitigation:

- refactor installer feature flows before deleting old template feature trees
- add integration tests around repeated add flows against drifted projects
- keep the bootstrap/discovery contract intentionally small

### Risk: later feature adds assume generated files are pristine

Mitigation:

- prefer generating isolated files and directories per feature
- when touching shared files, use narrow anchored patches
- fail clearly when anchors are missing instead of rewriting whole files

### Risk: feature skills become stale relative to generated code

Mitigation:

- keep feature skills authored alongside the feature source tree
- verify that add flows materialize both code and skill assets from the same feature pack revision
- add tests that assert shipped skills still reference valid generated paths and workflows

### Risk: hidden-root semantics become inconsistent across generic and native surfaces

Mitigation:

- encode generic and native hidden-root identity in manifest/config contract
- make docs and tests validate the same declared semantics

### Risk: Rust remains a special-case outlier

Mitigation:

- explicitly decide whether Rust keeps `.claw` as its native hidden-root family or converges on another native hidden-root concept
- document intentional divergence rather than letting it remain accidental

## 10. Verification Plan

The refactor is complete only when all of the following are true:

- docs describe separate authoring and generated-project contracts
- every language manifest encodes the same semantic split
- `aicd init --language <id>` still works
- `aicd list features` reads feature packs from language-pack authoring roots
- `aicd feature add` still works for at least one feature in each supported language
- `aicd doctor --project <dir>` validates the generated project correctly
- no remaining core contract text states that contributor-authored feature packs live canonically under a generated-project hidden runtime directory
- repeated add flows remain usable after user customization as long as bootstrap anchors remain valid
- both `.agents` and native hidden roots can coexist without breaking the documented contract
- feature packs can ship `SKILL.md` files that remain usable as AI best-practice entrypoints after generation

Suggested verification commands after implementation:

```bash
cargo test --manifest-path install/Cargo.toml
pytest
./scripts/<integration-check-if-added>
```

## 11. Completion Definition

This refactor is done when the repo communicates and implements one clear idea:

- contributors extend language packs from `languages/<language>/...`
- generated projects may load local AI/runtime assets from both `.agents/` and one or more native hidden roots analogous to `.claude/`
- those two concerns are connected by the installer, not collapsed into the same directory contract
- feature lifecycle is add-only and resilient to user-owned customization
- feature packs are both code artifacts and AI development interfaces via shipped `SKILL.md` files
