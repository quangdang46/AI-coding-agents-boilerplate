from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path

from ..config.loader import load_config
from ..config.models import BoilerplateConfig
from ..prompts.composer import compose_prompt
from ..registry.agents import load_agents
from ..registry.features import load_enabled_features
from ..registry.skills import load_skills


@dataclass(frozen=True)
class ProjectRuntime:
    project_root: Path
    config: BoilerplateConfig

    @classmethod
    def from_project(cls, project_root: Path) -> 'ProjectRuntime':
        return cls(project_root=project_root, config=load_config(project_root))

    def load_agents(self):
        return load_agents(self.project_root, self.config)

    def load_skills(self):
        return load_skills(self.project_root, self.config)

    def load_enabled_features(self):
        return load_enabled_features(self.project_root, self.config)

    def compose_system_prompt(self, agent_prompt: str | None = None, append_text: str | None = None) -> str:
        return compose_prompt(self.project_root, self.config, agent_prompt=agent_prompt, append_text=append_text)
