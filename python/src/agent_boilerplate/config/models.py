from __future__ import annotations

from dataclasses import dataclass, field
from typing import Literal

ApprovalMode = Literal["default", "always-ask", "never-ask"]
ProviderId = Literal["openai", "anthropic", "local"]


@dataclass(frozen=True)
class ProviderSettings:
    api_key_env: str
    model: str
    base_url: str | None = None


@dataclass(frozen=True)
class AppConfig:
    name: str
    package_name: str
    version: str
    default_provider: ProviderId
    default_mode: Literal["interactive", "headless"]
    working_directory_policy: Literal["project-root", "cwd"] = "project-root"


@dataclass(frozen=True)
class PromptConfig:
    system: str
    sections: tuple[str, ...] = ()
    append: tuple[str, ...] = ()


@dataclass(frozen=True)
class ToolConfig:
    enabled: tuple[str, ...]
    approval_mode: ApprovalMode = "default"
    deny: tuple[str, ...] = ()
    bash_timeout_ms: int = 15000
    web_fetch_timeout_ms: int = 10000
    max_tool_calls_per_turn: int = 12


@dataclass(frozen=True)
class RegistryConfig:
    directories: tuple[str, ...]
    enabled: tuple[str, ...] = ()
    auto_discover: bool = True
    default: str | None = None


@dataclass(frozen=True)
class FeatureConfig:
    enabled: tuple[str, ...] = ()
    registry: str = ".agent/features/registry.json"


@dataclass(frozen=True)
class BoilerplateConfig:
    app: AppConfig
    prompts: PromptConfig
    tools: ToolConfig
    providers: dict[str, ProviderSettings]
    agents: RegistryConfig
    skills: RegistryConfig
    features: FeatureConfig = field(default_factory=FeatureConfig)
