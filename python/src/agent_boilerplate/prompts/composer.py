from __future__ import annotations

from pathlib import Path

from ..config.models import BoilerplateConfig
from ..registry.features import load_feature_prompt_sections


PACKAGE_PROMPT_ROOT = Path(__file__).resolve().parent


def _read_if_exists(path: Path) -> str:
    if not path.exists():
        return ''
    return path.read_text().strip()


def compose_prompt(project_root: Path, config: BoilerplateConfig, agent_prompt: str | None = None, append_text: str | None = None) -> str:
    parts: list[str] = []

    runtime_default = _read_if_exists(PACKAGE_PROMPT_ROOT / 'system.md')
    if runtime_default:
        parts.append(runtime_default)

    project_system = _read_if_exists(project_root / config.prompts.system)
    if project_system:
        parts.append(project_system)

    section_paths = list(config.prompts.sections)
    for feature_section in load_feature_prompt_sections(project_root, config):
        if feature_section not in section_paths:
            section_paths.append(feature_section)

    for section_path in section_paths:
        content = _read_if_exists(project_root / section_path)
        if content:
            parts.append(content)

    if agent_prompt:
        content = _read_if_exists(project_root / agent_prompt)
        if content:
            parts.append(content)

    for append_path in config.prompts.append:
        content = _read_if_exists(project_root / append_path)
        if content:
            parts.append(content)

    if append_text:
        parts.append(append_text.strip())

    return '\n\n'.join(part for part in parts if part)
