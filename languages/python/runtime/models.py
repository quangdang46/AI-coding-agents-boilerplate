from __future__ import annotations

from dataclasses import dataclass, field


@dataclass(frozen=True)
class Subsystem:
    name: str
    path: str
    file_count: int
    notes: str


@dataclass(frozen=True)
class RuntimeSupportModule:
    name: str
    responsibility: str
    source_file: str
    status: str = "implemented"


@dataclass
class RuntimeSupportBacklog:
    title: str
    modules: list[RuntimeSupportModule] = field(default_factory=list)

    def summary_lines(self) -> list[str]:
        return [
            f"- {module.name} [{module.status}] — {module.responsibility} (from {module.source_file})"
            for module in self.modules
        ]
