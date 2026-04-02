from __future__ import annotations

from pathlib import Path

from ..feature_ops import apply_feature, remove_feature


def run_feature_add(project_root: Path, feature_id: str) -> str:
    apply_feature(project_root, feature_id)
    return f'added feature {feature_id} to {project_root}'


def run_feature_remove(project_root: Path, feature_id: str) -> str:
    remove_feature(project_root, feature_id)
    return f'removed feature {feature_id} from {project_root}'
