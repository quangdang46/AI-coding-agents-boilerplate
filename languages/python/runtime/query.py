from __future__ import annotations

from pathlib import Path

from .history import load_usage_summary
from .session_store import load_latest_session


def summarize_query_state(project_root: Path, runtime_output: str) -> str:
    latest = load_latest_session(project_root)
    usage = load_usage_summary(project_root)
    return (
        f"{runtime_output} "
        f"query_session_id={latest.get('session_id', 'missing')} "
        f"query_turn_count={latest.get('turn_count', '0')} "
        f"query_usage_entries={usage.get('usage_entries', '0')}"
    )
