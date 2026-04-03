# Python Language Pack Config Reference

The Python language pack currently generates projects with `agentkit.toml` as the base config file.

The active proving-slice template lives under `languages/python/template/base/`.

Generated Python projects include provider and model selection through `default_provider` plus provider-specific sections such as `[providers.openai]` and `[providers.anthropic]`.

Generated Python projects also include permission and sandbox controls through `[tools]`, including `approval_mode` and `deny`.

Generated Python projects also reserve `.agent/sessions/` as the local persisted-session root for future session and resume flows.

Generated Python projects also reserve `.agent/context/` as the local workspace-context root for prompt and instruction composition.

Generated Python projects also reserve `.agent/usage/` as the local usage and cost tracking root for future accounting flows.

Generated Python projects also expose local agent discovery through `[agents]`, including agent directories, enabled agents, and the default agent.
