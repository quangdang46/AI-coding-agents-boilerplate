from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT / "languages" / "typescript" / "docs" / "query-surface-ownership.md"
)

EXPECTED_QUERY_ROWS = [
    "references/typescript/src/query/config.ts",
    "references/typescript/src/query/tokenBudget.ts",
    "references/typescript/src/query/stopHooks.ts",
    "references/typescript/src/query/deps.ts",
]


def test_typescript_query_rows_have_owner_or_archive_rationale() -> None:
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert "concrete shipped owner or an explicit archive-only rationale" in text
    for row in EXPECTED_QUERY_ROWS:
        assert row in text


def test_typescript_query_cluster_size_matches_expected_rows() -> None:
    assert len(EXPECTED_QUERY_ROWS) == 4
