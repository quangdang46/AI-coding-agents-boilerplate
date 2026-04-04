from __future__ import annotations

from pathlib import Path

from ..brand import brand_doc_candidates, infer_brand_root
from ..permissions import summarize_permission_state
from ..tasks import summarize_task_state


def _read_state(path: Path) -> dict[str, str]:
    if not path.exists():
        return {}
    state: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8").splitlines():
        if "=" in line:
            key, value = line.split("=", 1)
            state[key] = value
    return state


def _extract_value(source: str, marker: str) -> str:
    start = source.index(marker) + len(marker)
    end = source.find(" ", start)
    if end == -1:
        end = len(source)
    return source[start:end]


def command_registry() -> dict[str, str]:
    return {
        "status": "status",
        "session": "session",
        "export": "export",
        "config": "config",
        "doctor": "doctor",
        "context": "context",
        "usage": "usage",
        "permissions": "permissions",
        "files": "files",
        "tasks": "tasks",
    }


def run_command(command_name: str, project_root: Path, runtime_output: str) -> str:
    brand_root = infer_brand_root(project_root)
    if command_name == "status":
        return runtime_output
    if command_name == "session":
        latest = _read_state(brand_root / "sessions" / "latest.state")
        return (
            f"session_id={latest.get('session_id', 'missing')} "
            f"turn_count={latest.get('turn_count', '0')}"
        )
    if command_name == "export":
        export_path = brand_root / "sessions" / "local-main-session.export.md"
        return (
            f"export_path={brand_root.name}/sessions/local-main-session.export.md "
            f"export_exists={export_path.exists()}"
        )
    if command_name == "config":
        return (
            f"provider={_extract_value(runtime_output, 'provider=')} "
            f"model={_extract_value(runtime_output, 'model=')} "
            f"approval_mode={_extract_value(runtime_output, 'approval_mode=')}"
        )
    if command_name == "doctor":
        required = [
            brand_root / "sessions" / "README.md",
            brand_root / "sessions" / "latest.state",
            brand_root / "sessions" / "summary.state",
        ]
        missing = [
            str(path.relative_to(project_root))
            for path in required
            if not path.exists()
        ]
        if not brand_doc_candidates(project_root, brand_root):
            missing.append("instruction-surface")
        return "doctor=ok" if not missing else f"doctor=missing:{','.join(missing)}"
    if command_name == "context":
        return f"context_digest={_extract_value(runtime_output, 'context_digest=')}"
    if command_name == "usage":
        summary = _read_state(brand_root / "sessions" / "summary.state")
        return (
            f"usage_entries={summary.get('usage_entries', '0')} "
            f"total_cost_micros={summary.get('total_cost_micros', '0')}"
        )
    if command_name == "permissions":
        return summarize_permission_state(runtime_output)
    if command_name == "files":
        session_state = (brand_root / "sessions" / "local-main-session.state").exists()
        export_state = (brand_root / "sessions" / "local-main-session.export.md").exists()
        usage_state = (brand_root / "sessions" / "summary.state").exists()
        return (
            f"session_state={session_state} export_state={export_state} "
            f"usage_state={usage_state}"
        )
    if command_name == "tasks":
        return summarize_task_state(project_root)
    raise KeyError(f"unknown command: {command_name}")
