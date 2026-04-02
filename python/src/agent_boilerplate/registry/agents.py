from __future__ import annotations

import json
from dataclasses import dataclass
from pathlib import Path

from ..config.models import BoilerplateConfig
from .features import load_feature_added_agent_ids


@dataclass(frozen=True)
class AgentManifest:
    id: str
    description: str
    prompt: str
    tools: tuple[str, ...] = ()
    disallowed_tools: tuple[str, ...] = ()
    skills: tuple[str, ...] = ()
    model: str = 'inherit'
    effort: str = 'medium'
    permission_mode: str = 'default'
    mcp_servers: tuple[str, ...] = ()
    initial_prompt: str | None = None
    memory: str = 'project'
    background: bool = False
    isolation: str = 'same-worktree'



def _load_manifest(path: Path) -> AgentManifest:
    raw = json.loads(path.read_text())
    return AgentManifest(
        id=str(raw['id']),
        description=str(raw['description']),
        prompt=str(raw['prompt']),
        tools=tuple(raw.get('tools', [])),
        disallowed_tools=tuple(raw.get('disallowedTools', [])),
        skills=tuple(raw.get('skills', [])),
        model=str(raw.get('model', 'inherit')),
        effort=str(raw.get('effort', 'medium')),
        permission_mode=str(raw.get('permissionMode', 'default')),
        mcp_servers=tuple(raw.get('mcpServers', [])),
        initial_prompt=raw.get('initialPrompt'),
        memory=str(raw.get('memory', 'project')),
        background=bool(raw.get('background', False)),
        isolation=str(raw.get('isolation', 'same-worktree')),
    )


def load_agents(project_root: Path, config: BoilerplateConfig) -> tuple[AgentManifest, ...]:
    manifests: list[AgentManifest] = []
    for directory in config.agents.directories:
        path = project_root / directory
        if not path.exists():
            continue
        for file_path in sorted(path.glob('*.agent.json')):
            manifests.append(_load_manifest(file_path))
    feature_agent_ids = set(load_feature_added_agent_ids(project_root, config))
    if config.agents.enabled or feature_agent_ids:
        enabled = set(config.agents.enabled) | feature_agent_ids
        manifests = [manifest for manifest in manifests if manifest.id in enabled]
    return tuple(manifests)
