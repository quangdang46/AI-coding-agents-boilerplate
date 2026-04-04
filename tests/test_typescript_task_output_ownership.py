from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT / "languages" / "typescript" / "docs" / "task-output-ownership.md"
)

EXPECTED_ROWS = [
    "references/typescript/src/utils/task/outputFormatting.ts",
    "references/typescript/src/utils/task/TaskOutput.ts",
    "references/typescript/src/utils/task/sdkProgress.ts",
    "references/typescript/src/utils/task/framework.ts",
    "references/typescript/src/utils/task/diskOutput.ts",
]


def test_typescript_task_output_rows_have_archive_or_owner_rationale() -> None:
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert "explicit archive-only or shipped-owner rationale" in text
    for row in EXPECTED_ROWS:
        assert row in text


def test_typescript_task_output_cluster_size_matches_expected_rows() -> None:
    assert len(EXPECTED_ROWS) == 5
