from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path

from ..config.models import BoilerplateConfig
from .features import load_feature_added_skill_names


@dataclass(frozen=True)
class SkillManifest:
    name: str
    description: str
    when_to_use: str | None = None
    allowed_tools: tuple[str, ...] = ()
    argument_hint: str | None = None
    body_markdown: str = ''


def _parse_frontmatter(text: str) -> tuple[dict[str, str], str]:
    if not text.startswith('---\n'):
        return {}, text
    parts = text.split('\n---\n', 1)
    if len(parts) != 2:
        return {}, text
    frontmatter_text = parts[0][4:]
    body = parts[1]
    frontmatter: dict[str, str] = {}
    for line in frontmatter_text.splitlines():
        if ':' not in line:
            continue
        key, value = line.split(':', 1)
        frontmatter[key.strip()] = value.strip()
    return frontmatter, body


def _parse_allowed_tools(value: str | None) -> tuple[str, ...]:
    if not value:
        return ()
    cleaned = value.strip()
    if cleaned.startswith('[') and cleaned.endswith(']'):
        cleaned = cleaned[1:-1]
    return tuple(part.strip().strip("\"'") for part in cleaned.split(',') if part.strip())


def _load_skill(path: Path) -> SkillManifest:
    frontmatter, body = _parse_frontmatter(path.read_text())
    return SkillManifest(
        name=frontmatter.get('name', path.parent.name),
        description=frontmatter.get('description', ''),
        when_to_use=frontmatter.get('whenToUse'),
        allowed_tools=_parse_allowed_tools(frontmatter.get('allowed-tools')),
        argument_hint=frontmatter.get('argument-hint'),
        body_markdown=body.strip(),
    )


def load_skills(project_root: Path, config: BoilerplateConfig) -> tuple[SkillManifest, ...]:
    manifests: list[SkillManifest] = []
    for directory in config.skills.directories:
        path = project_root / directory
        if not path.exists():
            continue
        for skill_file in sorted(path.glob('*/SKILL.md')):
            manifests.append(_load_skill(skill_file))
    feature_skill_names = set(load_feature_added_skill_names(project_root, config))
    if config.skills.enabled or feature_skill_names:
        enabled = set(config.skills.enabled) | feature_skill_names
        manifests = [manifest for manifest in manifests if manifest.name in enabled]
    return tuple(manifests)
