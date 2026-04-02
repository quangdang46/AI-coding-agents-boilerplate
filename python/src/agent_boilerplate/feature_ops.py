from __future__ import annotations

import shutil
from pathlib import Path


def _feature_root(project_root: Path, feature_id: str) -> Path:
    return project_root / '.agent' / 'features' / feature_id


def _feature_files_root(project_root: Path, feature_id: str) -> Path:
    return _feature_root(project_root, feature_id) / 'files'


def _feature_manifest_path(project_root: Path, feature_id: str) -> Path:
    return _feature_root(project_root, feature_id) / 'feature.json'


def set_feature_enabled(project_root: Path, feature_id: str, enabled: bool) -> None:
    config_path = project_root / 'agentkit.toml'
    lines = config_path.read_text().splitlines()
    in_features = False
    for index, line in enumerate(lines):
        stripped = line.strip()
        if stripped.startswith('['):
            in_features = stripped == '[features]'
            continue
        if in_features and stripped.startswith('enabled = ['):
            entries = _parse_enabled_features(stripped)
            if enabled and feature_id not in entries:
                entries.append(feature_id)
            if not enabled:
                entries = [entry for entry in entries if entry != feature_id]
            rendered = ', '.join(f'"{entry}"' for entry in entries)
            lines[index] = f'enabled = [{rendered}]'
            config_path.write_text('\n'.join(lines) + '\n')
            return
    raise ValueError(f'missing features.enabled in {config_path}')


def _parse_enabled_features(line: str) -> list[str]:
    start = line.find('[')
    end = line.rfind(']')
    if start == -1 or end == -1 or end <= start:
        return []
    raw = line[start + 1:end].strip()
    if not raw:
        return []
    return [part.strip().strip('"') for part in raw.split(',') if part.strip()]


def apply_feature(project_root: Path, feature_id: str) -> None:
    manifest_path = _feature_manifest_path(project_root, feature_id)
    if not manifest_path.exists():
        raise FileNotFoundError(f'unknown feature: {manifest_path}')
    files_root = _feature_files_root(project_root, feature_id)
    if files_root.exists():
        for source in sorted(files_root.rglob('*')):
            relative = source.relative_to(files_root)
            target = project_root / relative
            if source.is_dir():
                target.mkdir(parents=True, exist_ok=True)
                continue
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(source, target)
    set_feature_enabled(project_root, feature_id, True)


def remove_feature(project_root: Path, feature_id: str) -> None:
    manifest_path = _feature_manifest_path(project_root, feature_id)
    if not manifest_path.exists():
        raise FileNotFoundError(f'unknown feature: {manifest_path}')
    files_root = _feature_files_root(project_root, feature_id)
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
    set_feature_enabled(project_root, feature_id, False)
