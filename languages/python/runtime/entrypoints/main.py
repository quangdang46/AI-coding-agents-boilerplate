from __future__ import annotations

from pathlib import Path


def _read_text(path: Path) -> str:
    return path.read_text(encoding="utf-8").strip()


def _read_state(path: Path) -> dict[str, str]:
    if not path.exists():
        return {}
    state: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8").splitlines():
        if "=" in line:
            key, value = line.split("=", 1)
            state[key] = value
    return state


def build_entry_summary(project_root: Path, runtime_output: str) -> str:
    latest = _read_state(project_root / ".agent" / "sessions" / "latest.state")
    usage = _read_state(project_root / ".agent" / "usage" / "summary.state")
    system_prompt = _read_text(project_root / ".agent" / "prompts" / "system.md")
    context_readme = _read_text(project_root / ".agent" / "context" / "README.md")
    return (
        f"{runtime_output} "
        f"entry_session_id={latest.get('session_id', 'missing')} "
        f"entry_turn_count={latest.get('turn_count', '0')} "
        f"entry_usage_entries={usage.get('usage_entries', '0')} "
        f"entry_prompt_present={bool(system_prompt)} "
        f"entry_context_present={bool(context_readme)}"
    )


def main(project_root: Path, runtime_output: str) -> str:
    return build_entry_summary(project_root, runtime_output)
