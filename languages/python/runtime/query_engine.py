from __future__ import annotations

from pathlib import Path

from .query import summarize_query_state


class QueryEngine:
    def __init__(self, project_root: Path) -> None:
        self.project_root = project_root

    def run(self, runtime_output: str) -> str:
        return summarize_query_state(self.project_root, runtime_output)
