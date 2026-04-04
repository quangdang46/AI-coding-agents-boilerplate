from __future__ import annotations

from pathlib import Path


def infer_brand_root(project_root: Path) -> Path:
    candidates: list[Path] = []
    for path in project_root.iterdir():
        if not path.is_dir():
            continue
        if not path.name.startswith(".") or path.name.endswith("-plugin"):
            continue
        score = 0
        for child in ["settings.json", "settings.local.json", "instructions.md", "sessions"]:
            if (path / child).exists():
                score += 1
        if score:
            candidates.append(path)
    if candidates:
        candidates.sort(key=lambda path: path.name)
        return candidates[0]
    return project_root / ".claude"


def brand_doc_candidates(project_root: Path, brand_root: Path) -> list[Path]:
    candidates: list[Path] = []
    for name in ["AGENTS.md", "CLAUDE.md"]:
        path = project_root / name
        if path.exists():
            candidates.append(path)
    for path in project_root.glob("*.md"):
        if path.name in {"README.md", "AGENTS.md", "CLAUDE.md"}:
            continue
        candidates.insert(0, path)
    for name in ["instructions.md"]:
        path = brand_root / name
        if path.exists():
            candidates.append(path)
    for path in brand_root.glob("*.md"):
        if path.name == "instructions.md":
            continue
        candidates.append(path)
    return candidates
