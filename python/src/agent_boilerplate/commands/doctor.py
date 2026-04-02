from __future__ import annotations

from dataclasses import dataclass, field
from pathlib import Path

from ..config.loader import load_config
from ..core.runtime import ProjectRuntime
from ..registry.features import load_feature_manifest, load_feature_registry


@dataclass(frozen=True)
class DoctorReport:
    ok: bool
    checks: tuple[str, ...]
    errors: tuple[str, ...] = field(default_factory=tuple)

    def as_text(self) -> str:
        lines = ['Doctor: OK' if self.ok else 'Doctor: FAILED', '']
        lines.extend(f'- {check}' for check in self.checks)
        if self.errors:
            lines.append('')
            lines.append('Errors:')
            lines.extend(f'- {error}' for error in self.errors)
        return '\n'.join(lines)


def run_doctor(project_root: Path) -> DoctorReport:
    checks: list[str] = []
    errors: list[str] = []

    for relative in ('agentkit.toml', 'README.md', 'pyproject.toml'):
        path = project_root / relative
        if path.exists():
            checks.append(f'found {relative}')
        else:
            errors.append(f'missing {relative}')

    try:
        config = load_config(project_root)
        checks.append('loaded agentkit.toml')
    except Exception as exc:
        return DoctorReport(ok=False, checks=tuple(checks), errors=tuple(errors + [f'config load failed: {exc}']))

    runtime = ProjectRuntime(project_root=project_root, config=config)

    prompt_files = [config.prompts.system, *config.prompts.sections, *config.prompts.append]
    for relative in prompt_files:
        path = project_root / relative
        if path.exists():
            checks.append(f'found prompt file {relative}')
        else:
            errors.append(f'missing prompt file: {relative}')

    agents = runtime.load_agents()
    loaded_agent_ids = {agent.id for agent in agents}
    if agents:
        checks.append(f'loaded {len(agents)} agents')
    else:
        errors.append('no agents loaded')
    for agent in agents:
        prompt_path = project_root / agent.prompt
        if prompt_path.exists():
            checks.append(f'found agent prompt for {agent.id}')
        else:
            errors.append(f'missing agent prompt for {agent.id}: {agent.prompt}')
    for expected in config.agents.enabled:
        if expected not in loaded_agent_ids:
            errors.append(f'enabled agent not found: {expected}')

    skills = runtime.load_skills()
    loaded_skill_names = {skill.name for skill in skills}
    if skills:
        checks.append(f'loaded {len(skills)} skills')
    else:
        errors.append('no skills loaded')
    for expected in config.skills.enabled:
        if expected not in loaded_skill_names:
            errors.append(f'enabled skill not found: {expected}')

    try:
        registry_entries = {entry.id: entry for entry in load_feature_registry(project_root, config)}
        checks.append(f'loaded {len(registry_entries)} feature registry entries')
        for feature_id in config.features.enabled:
            entry = registry_entries.get(feature_id)
            if entry is None:
                errors.append(f'enabled feature not found in registry: {feature_id}')
                continue
            manifest = load_feature_manifest(project_root, config, entry)
            checks.append(f'loaded feature manifest {feature_id}')
            for agent_name in manifest.adds.get('agents', ()): 
                agent_path = project_root / '.agent' / 'agents' / agent_name
                if agent_path.exists():
                    checks.append(f'found feature agent file: {agent_name}')
                else:
                    errors.append(f'missing feature agent file for {feature_id}: {agent_name}')
            for skill_name in manifest.adds.get('skills', ()): 
                skill_path = project_root / '.agent' / 'skills' / skill_name / 'SKILL.md'
                if skill_path.exists():
                    checks.append(f'found feature skill file: {skill_name}')
                else:
                    errors.append(f'missing feature skill file for {feature_id}: {skill_name}')
            for prompt_name in manifest.adds.get('prompts', ()): 
                prompt_path = project_root / '.agent' / 'prompts' / 'sections' / prompt_name
                if prompt_path.exists():
                    checks.append(f'found feature prompt file: {prompt_name}')
                else:
                    errors.append(f'missing feature prompt file for {feature_id}: {prompt_name}')
    except Exception as exc:
        errors.append(f'feature registry load failed: {exc}')

    return DoctorReport(ok=not errors, checks=tuple(checks), errors=tuple(errors))
