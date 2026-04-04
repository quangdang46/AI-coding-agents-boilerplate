from __future__ import annotations

from dataclasses import dataclass


@dataclass(frozen=True)
class PortingTask:
    task_id: str
    description: str


def active_runtime_task() -> PortingTask:
    return PortingTask("session-loop", "Execute the local runtime session loop")
