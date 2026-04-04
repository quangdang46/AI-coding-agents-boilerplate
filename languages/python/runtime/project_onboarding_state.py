from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path


@dataclass(frozen=True)
class ProjectOnboardingState:
    has_readme: bool
    has_tests: bool
    has_runtime_docs: bool
    python_first: bool = True


def infer_project_onboarding_state(pack_root: Path) -> ProjectOnboardingState:
    return ProjectOnboardingState(
        has_readme=(pack_root / "README.md").exists(),
        has_tests=(pack_root / "template" / "base" / "tests").exists(),
        has_runtime_docs=(pack_root / "docs" / "migration-baseline.md").exists(),
    )
