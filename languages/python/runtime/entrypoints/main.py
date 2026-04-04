from __future__ import annotations

from pathlib import Path

from ..brand import brand_doc_candidates, infer_brand_root


def _read_text(path: Path) -> str:
    return path.read_text(encoding="utf-8").strip()


def _read_optional_text(path: Path) -> str:
    return _read_text(path) if path.exists() else ""


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
    brand_root = infer_brand_root(project_root)
    latest = _read_state(brand_root / "sessions" / "latest.state")
    usage = _read_state(brand_root / "sessions" / "summary.state")
    prompt_present = any(_read_optional_text(path) for path in brand_doc_candidates(project_root, brand_root))
    return (
        f"{runtime_output} "
        f"entry_session_id={latest.get('session_id', 'missing')} "
        f"entry_turn_count={latest.get('turn_count', '0')} "
        f"entry_usage_entries={usage.get('usage_entries', '0')} "
        f"entry_prompt_present={prompt_present} "
        f"entry_context_present={prompt_present}"
    )


def main(project_root: Path, runtime_output: str) -> str:
    return build_entry_summary(project_root, runtime_output)
