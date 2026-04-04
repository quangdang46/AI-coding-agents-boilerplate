from __future__ import annotations

from pathlib import Path

from .brand import infer_brand_root


def load_usage_summary(project_root: Path) -> dict[str, str]:
    summary_path = infer_brand_root(project_root) / "sessions" / "summary.state"
    if not summary_path.exists():
        return {}
    state: dict[str, str] = {}
    for line in summary_path.read_text(encoding="utf-8").splitlines():
        if "=" in line:
            key, value = line.split("=", 1)
            state[key] = value
    return state


def load_usage_ledger(project_root: Path) -> list[str]:
    summary = load_usage_summary(project_root)
    if not summary:
        return []
    return [
        f"usage_entries={summary.get('usage_entries', '0')}",
        f"total_cost_micros={summary.get('total_cost_micros', '0')}",
    ]
