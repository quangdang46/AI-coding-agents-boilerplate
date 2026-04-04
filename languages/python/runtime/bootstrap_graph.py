from __future__ import annotations

from dataclasses import dataclass


@dataclass(frozen=True)
class BootstrapGraph:
    stages: tuple[str, ...]

    def as_markdown(self) -> str:
        lines = ["# Bootstrap Graph", ""]
        lines.extend(f"- {stage}" for stage in self.stages)
        return "\n".join(lines)


def build_bootstrap_graph() -> BootstrapGraph:
    return BootstrapGraph(
        stages=(
            "load manifest-backed Python pack roots",
            "hydrate prompt, session, and usage summaries",
            "register shipped command and tool surfaces",
            "apply permission policy and direct-mode summaries",
            "run query engine against the active runtime output",
        )
    )
