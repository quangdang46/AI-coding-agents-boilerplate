from __future__ import annotations

from pathlib import Path


def load_usage_summary(project_root: Path) -> dict[str, str]:
    summary_path = project_root / ".agent" / "usage" / "summary.state"
    if not summary_path.exists():
        return {}
    state: dict[str, str] = {}
    for line in summary_path.read_text(encoding="utf-8").splitlines():
        if "=" in line:
            key, value = line.split("=", 1)
            state[key] = value
    return state


def load_usage_ledger(project_root: Path) -> list[str]:
    ledger_path = project_root / ".agent" / "usage" / "ledger.log"
    if not ledger_path.exists():
        return []
    return [
        line for line in ledger_path.read_text(encoding="utf-8").splitlines() if line
    ]
