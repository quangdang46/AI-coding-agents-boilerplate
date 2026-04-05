from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT / "languages" / "typescript" / "docs" / "root-runtime-shell-ownership.md"
)

EXPECTED_ROWS = [
    "references/typescript/src/main.tsx",
    "references/typescript/src/QueryEngine.ts",
    "references/typescript/src/Task.ts",
    "references/typescript/src/Tool.ts",
    "references/typescript/src/commands.ts",
    "references/typescript/src/tools.ts",
    "references/typescript/src/context.ts",
    "references/typescript/src/history.ts",
    "references/typescript/src/setup.ts",
    "references/typescript/src/query.ts",
    "references/typescript/src/tasks.ts",
    "references/typescript/src/dialogLaunchers.tsx",
    "references/typescript/src/interactiveHelpers.tsx",
    "references/typescript/src/replLauncher.tsx",
    "references/typescript/src/projectOnboardingState.ts",
    "references/typescript/src/cost-tracker.ts",
    "references/typescript/src/costHook.ts",
]


def test_typescript_root_runtime_shell_rows_have_archive_or_owner_rationale() -> None:
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert "explicit shipped-owner or archive-only rationale" in text
    for row in EXPECTED_ROWS:
        assert row in text


def test_typescript_root_runtime_shell_cluster_size_matches_expected_rows() -> None:
    assert len(EXPECTED_ROWS) == 17
