from __future__ import annotations

import json
from dataclasses import dataclass, field
from pathlib import Path
from typing import Literal

from ..config.models import BoilerplateConfig


@dataclass(frozen=True)
class FeaturePatch:
    target: str
    op: Literal['merge', 'replace', 'append']
    path: str | None = None
    value: object | None = None


@dataclass(frozen=True)
class FeaturePackManifest:
    id: str
    name: str
    description: str
    version: str
    depends_on: tuple[str, ...] = ()
    adds: dict[str, tuple[str, ...]] = field(default_factory=dict)
    patches: tuple[FeaturePatch, ...] = ()


@dataclass(frozen=True)
class FeatureRegistryEntry:
    id: str
    path: str


def feature_root(project_root: Path, config: BoilerplateConfig, entry: FeatureRegistryEntry) -> Path:
    return (project_root / config.features.registry).parent / entry.path


def load_feature_registry(project_root: Path, config: BoilerplateConfig) -> tuple[FeatureRegistryEntry, ...]:
    registry_path = project_root / config.features.registry
    raw = json.loads(registry_path.read_text())
    entries = raw.get('features', raw)
    return tuple(FeatureRegistryEntry(id=str(entry['id']), path=str(entry.get('path', entry['id']))) for entry in entries)


def load_feature_manifest(project_root: Path, config: BoilerplateConfig, entry: FeatureRegistryEntry) -> FeaturePackManifest:
    raw = json.loads((feature_root(project_root, config, entry) / 'feature.json').read_text())
    return FeaturePackManifest(
        id=str(raw['id']),
        name=str(raw['name']),
        description=str(raw['description']),
        version=str(raw['version']),
        depends_on=tuple(raw.get('dependsOn', [])),
        adds={key: tuple(value) for key, value in raw.get('adds', {}).items()},
        patches=tuple(
            FeaturePatch(
                target=str(patch['target']),
                op=str(patch['op']),
                path=patch.get('path'),
                value=patch.get('value'),
            )
            for patch in raw.get('patches', [])
        ),
    )


def resolve_feature(project_root: Path, config: BoilerplateConfig, feature_id: str) -> tuple[FeatureRegistryEntry, FeaturePackManifest]:
    for entry in load_feature_registry(project_root, config):
        if entry.id == feature_id:
            return entry, load_feature_manifest(project_root, config, entry)
    raise FileNotFoundError(f'unknown feature: {feature_id}')


def load_enabled_features(project_root: Path, config: BoilerplateConfig) -> tuple[FeaturePackManifest, ...]:
    entries = {entry.id: entry for entry in load_feature_registry(project_root, config)}
    manifests: list[FeaturePackManifest] = []
    for feature_id in config.features.enabled:
        entry = entries.get(feature_id)
        if entry is None:
            continue
        manifests.append(load_feature_manifest(project_root, config, entry))
    return tuple(manifests)


def load_feature_added_skill_names(project_root: Path, config: BoilerplateConfig) -> tuple[str, ...]:
    names: list[str] = []
    for manifest in load_enabled_features(project_root, config):
        for skill_name in manifest.adds.get('skills', ()):
            if skill_name not in names:
                names.append(skill_name)
    return tuple(names)


def load_feature_prompt_sections(project_root: Path, config: BoilerplateConfig) -> tuple[str, ...]:
    sections: list[str] = []
    for manifest in load_enabled_features(project_root, config):
        for prompt_name in manifest.adds.get('prompts', ()):
            relative = prompt_name if '/' in prompt_name else f'.agent/prompts/sections/{prompt_name}'
            if relative not in sections:
                sections.append(relative)
    return tuple(sections)


def _agent_id_from_manifest_entry(entry: str) -> str:
    if entry.endswith('.agent.json'):
        return entry[: -len('.agent.json')]
    return Path(entry).stem


def load_feature_added_agent_ids(project_root: Path, config: BoilerplateConfig) -> tuple[str, ...]:
    agent_ids: list[str] = []
    for manifest in load_enabled_features(project_root, config):
        for agent_entry in manifest.adds.get('agents', ()):
            agent_id = _agent_id_from_manifest_entry(agent_entry)
            if agent_id not in agent_ids:
                agent_ids.append(agent_id)
    return tuple(agent_ids)
