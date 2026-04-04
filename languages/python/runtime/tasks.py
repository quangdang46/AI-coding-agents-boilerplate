from __future__ import annotations

from pathlib import Path

from .session_store import load_latest_session
from .task import PortingTask, active_runtime_task


def default_tasks() -> list[PortingTask]:
    return [active_runtime_task()]


def summarize_task_state(project_root: Path) -> str:
    latest = load_latest_session(project_root)
    tasks = default_tasks()
    active = tasks[0] if tasks else PortingTask("none", "No active runtime task")
    return (
        f"task_count={len(tasks)} active_task={active.task_id} "
        f"turn_count={latest.get('turn_count', '0')}"
    )
