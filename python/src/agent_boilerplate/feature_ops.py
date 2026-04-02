from __future__ import annotations

import re
import shutil
from pathlib import Path

from .config.loader import load_config
from .registry.features import FeaturePackManifest, feature_root, load_enabled_features, resolve_feature


_LIST_ASSIGNMENT = re.compile(r'(^[ \t]*{key}[ \t]*=[ \t]*\[(?P<body>.*?)\][ \t]*$)', re.MULTILINE | re.DOTALL)


def _parse_enabled_features(line: str) -> list[str]:
    return re.findall(r'"([^"]+)"', line)


def _render_list_assignment(key: str, values: list[str], multiline: bool) -> str:
    if not values:
        return f'{key} = []'
    if multiline or len(values) > 1:
        rendered_items = '\n'.join(f'  "{value}",' for value in values)
        return f'{key} = [\n{rendered_items}\n]'
    return f'{key} = ["{values[0]}"]'


def _find_list_assignment(text: str, section: str, key: str) -> tuple[int, int, str, re.Match[str]]:
    section_start = text.find(f'[{section}]')
    if section_start == -1:
        raise ValueError(f'missing section [{section}] in agentkit.toml')
    next_section = text.find('\n[', section_start + 1)
    section_end = len(text) if next_section == -1 else next_section + 1
    section_text = text[section_start:section_end]

    key_pattern = re.compile(_LIST_ASSIGNMENT.pattern.format(key=re.escape(key)), _LIST_ASSIGNMENT.flags)
    match = key_pattern.search(section_text)
    if match is None:
        raise ValueError(f'missing {section}.{key} in agentkit.toml')
    return section_start, section_end, section_text, match


def _set_list_value(text: str, section: str, key: str, values: list[str]) -> str:
    section_start, section_end, section_text, match = _find_list_assignment(text, section, key)
    original_value = match.group('body')
    replacement = _render_list_assignment(key, values, multiline='\n' in original_value)
    updated_section = section_text[:match.start(1)] + replacement + section_text[match.end(1):]
    return text[:section_start] + updated_section + text[section_end:]


def _merge_list_values(values: list[str], additions: list[str]) -> list[str]:
    merged = list(values)
    for value in additions:
        if value not in merged:
            merged.append(value)
    return merged


def _remove_list_values(values: list[str], removals: list[str]) -> list[str]:
    removal_set = set(removals)
    return [value for value in values if value not in removal_set]


def _apply_patch(text: str, path: str, op: str, raw_value: object, enable: bool) -> str:
    section, _, key = path.partition('.')
    if not section or not key:
        raise ValueError(f'unsupported patch path: {path}')
    if not isinstance(raw_value, list) or not all(isinstance(item, str) for item in raw_value):
        raise ValueError(f'unsupported patch value for {path}: expected list[str]')
    _, _, _, match = _find_list_assignment(text, section, key)
    current_values = _parse_enabled_features(match.group('body'))

    if op not in {'merge', 'append'}:
        raise ValueError(f'unsupported patch op for reversible feature flow: {op}')
    next_values = _merge_list_values(current_values, list(raw_value)) if enable else _remove_list_values(current_values, list(raw_value))
    return _set_list_value(text, section, key, next_values)


def _apply_manifest_patches(project_root: Path, manifest: FeaturePackManifest, enable: bool) -> None:
    config_path = project_root / 'agentkit.toml'
    text = config_path.read_text()
    patches = manifest.patches if enable else tuple(reversed(manifest.patches))
    for patch in patches:
        if patch.target != 'agentkit.toml' or patch.path is None:
            raise ValueError(f'unsupported feature patch target for {manifest.id}: {patch.target}')
        text = _apply_patch(text, patch.path, patch.op, patch.value, enable)
    config_path.write_text(text if text.endswith('\n') else f'{text}\n')


def _feature_files_root(project_root: Path, manifest: FeaturePackManifest) -> Path:
    config = load_config(project_root)
    entry, _ = resolve_feature(project_root, config, manifest.id)
    return feature_root(project_root, config, entry) / 'files'


def _resolve_manifest(project_root: Path, feature_id: str) -> FeaturePackManifest:
    config = load_config(project_root)
    _, manifest = resolve_feature(project_root, config, feature_id)
    return manifest


def _ensure_dependencies_satisfied(project_root: Path, manifest: FeaturePackManifest) -> None:
    enabled = set(load_config(project_root).features.enabled)
    missing = [dependency for dependency in manifest.depends_on if dependency not in enabled]
    if missing:
        raise ValueError(f'feature {manifest.id} requires enabled feature(s): {", ".join(missing)}')


def _ensure_feature_not_required(project_root: Path, feature_id: str) -> None:
    config = load_config(project_root)
    dependents = [manifest.id for manifest in load_enabled_features(project_root, config) if feature_id in manifest.depends_on and manifest.id != feature_id]
    if dependents:
        raise ValueError(f'cannot remove {feature_id}; required by enabled feature(s): {", ".join(dependents)}')


def apply_feature(project_root: Path, feature_id: str) -> None:
    manifest = _resolve_manifest(project_root, feature_id)
    _ensure_dependencies_satisfied(project_root, manifest)
    files_root = _feature_files_root(project_root, manifest)
    if files_root.exists():
        for source in sorted(files_root.rglob('*')):
            relative = source.relative_to(files_root)
            target = project_root / relative
            if source.is_dir():
                target.mkdir(parents=True, exist_ok=True)
                continue
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(source, target)
    _apply_manifest_patches(project_root, manifest, enable=True)


def remove_feature(project_root: Path, feature_id: str) -> None:
    manifest = _resolve_manifest(project_root, feature_id)
    _ensure_feature_not_required(project_root, feature_id)
    _apply_manifest_patches(project_root, manifest, enable=False)
    files_root = _feature_files_root(project_root, manifest)
    if files_root.exists():
        paths = sorted(files_root.rglob('*'), reverse=True)
        for source in paths:
            relative = source.relative_to(files_root)
            target = project_root / relative
            if source.is_dir():
                if target.exists() and not any(target.iterdir()):
                    target.rmdir()
                continue
            if target.exists():
                target.unlink()
