from __future__ import annotations

from pathlib import Path

from .brand import infer_brand_root


def load_session_export(project_root: Path) -> str:
    export_path = infer_brand_root(project_root) / "sessions" / "local-main-session.export.md"
    return export_path.read_text(encoding="utf-8") if export_path.exists() else ""
