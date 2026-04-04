from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path


@dataclass(frozen=True)
class PrefetchResult:
    name: str
    started: bool
    detail: str


def start_runtime_prefetch(pack_root: Path) -> PrefetchResult:
    return PrefetchResult(
        "runtime_prefetch",
        True,
        f"Prepared runtime metadata under {pack_root / 'runtime'}",
    )


def start_prompt_prefetch(pack_root: Path) -> PrefetchResult:
    return PrefetchResult(
        "prompt_prefetch",
        True,
        f"Prepared prompt and instruction surfaces under {pack_root / 'template' / 'base'}",
    )


def start_project_scan(root: Path) -> PrefetchResult:
    return PrefetchResult("project_scan", True, f"Scanned Python pack root {root}")
