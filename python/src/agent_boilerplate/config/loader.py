from __future__ import annotations

import tomllib
from pathlib import Path

from .models import AppConfig, BoilerplateConfig, FeatureConfig, PromptConfig, ProviderSettings, RegistryConfig, ToolConfig


def _tupleize(value: object) -> tuple[str, ...]:
    if value is None:
        return ()
    if isinstance(value, (list, tuple)):
        return tuple(str(item) for item in value)
    return (str(value),)


def load_config(project_root: Path) -> BoilerplateConfig:
    config_path = project_root / 'agentkit.toml'
    raw = tomllib.loads(config_path.read_text())

    app_raw = raw['app']
    prompts_raw = raw.get('prompts', {})
    tools_raw = raw.get('tools', {})
    providers_raw = raw.get('providers', {})
    agents_raw = raw.get('agents', {})
    skills_raw = raw.get('skills', {})
    features_raw = raw.get('features', {})

    providers = {
        provider_id: ProviderSettings(
            api_key_env=str(values['api_key_env']),
            model=str(values['model']),
            base_url=str(values['base_url']) if values.get('base_url') not in (None, '') else None,
        )
        for provider_id, values in providers_raw.items()
    }

    return BoilerplateConfig(
        app=AppConfig(
            name=str(app_raw['name']),
            package_name=str(app_raw['package_name']),
            version=str(app_raw['version']),
            default_provider=str(app_raw['default_provider']),
            default_mode=str(app_raw['default_mode']),
            working_directory_policy=str(app_raw.get('working_directory_policy', 'project-root')),
        ),
        prompts=PromptConfig(
            system=str(prompts_raw['system']),
            sections=_tupleize(prompts_raw.get('sections')),
            append=_tupleize(prompts_raw.get('append')),
        ),
        tools=ToolConfig(
            enabled=_tupleize(tools_raw.get('enabled')),
            approval_mode=str(tools_raw.get('approval_mode', 'default')),
            deny=_tupleize(tools_raw.get('deny')),
            bash_timeout_ms=int(tools_raw.get('bash_timeout_ms', 15000)),
            web_fetch_timeout_ms=int(tools_raw.get('web_fetch_timeout_ms', 10000)),
            max_tool_calls_per_turn=int(tools_raw.get('max_tool_calls_per_turn', 12)),
        ),
        providers=providers,
        agents=RegistryConfig(
            directories=_tupleize(agents_raw.get('directories')),
            enabled=_tupleize(agents_raw.get('enabled')),
            auto_discover=bool(agents_raw.get('auto_discover', True)),
            default=str(agents_raw['default']) if agents_raw.get('default') else None,
        ),
        skills=RegistryConfig(
            directories=_tupleize(skills_raw.get('directories')),
            enabled=_tupleize(skills_raw.get('enabled')),
            auto_discover=bool(skills_raw.get('auto_discover', True)),
            default=str(skills_raw['default']) if skills_raw.get('default') else None,
        ),
        features=FeatureConfig(
            enabled=_tupleize(features_raw.get('enabled')),
            registry=str(features_raw.get('registry', '.agent/features/registry.json')),
        ),
    )
