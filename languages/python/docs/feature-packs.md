# Python Language Pack Feature Packs

Canonical Python feature authoring lives under:

- `languages/python/features/registry.json`
- `languages/python/features/<feature-id>/feature.json`
- `languages/python/features/<feature-id>/agents/*.md` when the feature ships subagents
- `languages/python/features/<feature-id>/skill/SKILL.md` when the feature ships a skill

Generated Python projects then receive branded project-local assets under:

- `.<brand>/agents/`
- `.<brand>/skills/<skill-name>/SKILL.md`
- `.agents/skills/<skill-name>/SKILL.md` as an optional interoperability mirror

The proving-slice sample feature is `github-pr-review`.
