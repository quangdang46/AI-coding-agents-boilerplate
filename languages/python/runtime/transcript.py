from __future__ import annotations

from pathlib import Path


def load_session_export(project_root: Path) -> str:
    export_path = project_root / ".agent" / "sessions" / "local-main-session.export.md"
    return export_path.read_text(encoding="utf-8") if export_path.exists() else ""
