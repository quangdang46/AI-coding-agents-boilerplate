from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT / "languages" / "typescript" / "docs" / "file-read-tool-ownership.md"
)

EXPECTED_ROWS = [
    "references/typescript/src/tools/FileReadTool/prompt.ts",
    "references/typescript/src/tools/FileReadTool/UI.tsx",
    "references/typescript/src/tools/FileReadTool/FileReadTool.ts",
    "references/typescript/src/tools/FileReadTool/limits.ts",
    "references/typescript/src/tools/FileReadTool/imageProcessor.ts",
]


def test_typescript_file_read_rows_have_owner_or_archive_rationale() -> None:
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert "concrete shipped owner or an explicit archive-only rationale" in text
    for row in EXPECTED_ROWS:
        assert row in text


def test_typescript_file_read_cluster_size_matches_expected_rows() -> None:
    assert len(EXPECTED_ROWS) == 5
