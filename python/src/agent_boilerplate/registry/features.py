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


def load_feature_registry(project_root: Path, config: BoilerplateConfig) -> tuple[FeatureRegistryEntry, ...]:
    registry_path = project_root / config.features.registry
    raw = json.loads(registry_path.read_text())
    entries = raw.get('features', raw)
    return tuple(FeatureRegistryEntry(id=str(entry['id']), path=str(entry.get('path', entry['id']))) for entry in entries)


def load_feature_manifest(project_root: Path, config: BoilerplateConfig, entry: FeatureRegistryEntry) -> FeaturePackManifest:
    feature_root = (project_root / config.features.registry).parent / entry.path
    raw = json.loads((feature_root / 'feature.json').read_text())
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


def load_enabled_features(project_root: Path, config: BoilerplateConfig) -> tuple[FeaturePackManifest, ...]:
    entries = {entry.id: entry for entry in load_feature_registry(project_root, config)}
    manifests: list[FeaturePackManifest] = []
    for feature_id in config.features.enabled:
        entry = entries.get(feature_id)
        if entry is None:
            continue
        manifests.append(load_feature_manifest(project_root, config, entry))
    return tuple(manifests)
