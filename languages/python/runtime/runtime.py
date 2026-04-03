from __future__ import annotations

from pathlib import Path

from .query_engine import QueryEngine


def run_runtime(project_root: Path, runtime_output: str) -> str:
    return QueryEngine(project_root).run(runtime_output)
