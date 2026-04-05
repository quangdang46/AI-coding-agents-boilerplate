from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT / "languages" / "typescript" / "docs" / "tool-search-tool-ownership.md"
)

EXPECTED_ROWS = [
    "references/typescript/src/tools/ToolSearchTool/prompt.ts",
    "references/typescript/src/tools/ToolSearchTool/ToolSearchTool.ts",
    "references/typescript/src/tools/ToolSearchTool/constants.ts",
]


def test_typescript_tool_search_rows_have_feature_pack_or_archive_rationale() -> None:
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert (
        "clear future feature-pack owner or an explicit archive-only rationale" in text
    )
    for row in EXPECTED_ROWS:
        assert row in text


def test_typescript_tool_search_cluster_size_matches_expected_rows() -> None:
    assert len(EXPECTED_ROWS) == 3
