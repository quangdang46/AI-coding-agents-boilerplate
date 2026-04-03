from __future__ import annotations

from pathlib import Path


def _read_state(path: Path) -> dict[str, str]:
    if not path.exists():
        return {}
    state: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8").splitlines():
        if "=" in line:
            key, value = line.split("=", 1)
            state[key] = value
    return state


def load_latest_session(project_root: Path) -> dict[str, str]:
    return _read_state(project_root / ".agent" / "sessions" / "latest.state")


def load_named_session(project_root: Path, session_id: str) -> dict[str, str]:
    return _read_state(project_root / ".agent" / "sessions" / f"{session_id}.state")
