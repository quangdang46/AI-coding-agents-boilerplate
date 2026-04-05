from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT / "languages" / "typescript" / "docs" / "search-tool-ownership.md"
)

EXPECTED_ROWS = [
    "references/typescript/src/tools/GlobTool/prompt.ts",
    "references/typescript/src/tools/GlobTool/UI.tsx",
    "references/typescript/src/tools/GlobTool/GlobTool.ts",
    "references/typescript/src/tools/GrepTool/GrepTool.ts",
    "references/typescript/src/tools/GrepTool/prompt.ts",
    "references/typescript/src/tools/GrepTool/UI.tsx",
]


def test_typescript_search_tool_rows_have_feature_pack_or_archive_rationale() -> None:
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert (
        "clear future feature-pack owner or an explicit archive-only rationale" in text
    )
    for row in EXPECTED_ROWS:
        assert row in text


def test_typescript_search_tool_cluster_size_matches_expected_rows() -> None:
    assert len(EXPECTED_ROWS) == 6
