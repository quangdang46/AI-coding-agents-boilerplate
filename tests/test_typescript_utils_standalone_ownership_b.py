from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT / "languages" / "typescript" / "docs" / "utils-standalone-ownership-b.md"
)

EXPECTED_ROWS = [
    "references/typescript/src/utils/queryHelpers.ts",
    "references/typescript/src/utils/completionCache.ts",
    "references/typescript/src/utils/filePersistence/filePersistence.ts",
    "references/typescript/src/utils/filePersistence/outputsScanner.ts",
    "references/typescript/src/utils/filePersistence/types.ts",
    "references/typescript/src/utils/collapseBackgroundBashNotifications.ts",
    "references/typescript/src/utils/advisor.ts",
]


def test_typescript_standalone_utils_b_rows_have_archive_or_owner_rationale() -> None:
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert "explicit archive-only or shipped-owner rationale" in text
    for row in EXPECTED_ROWS:
        assert row in text


def test_typescript_standalone_utils_b_cluster_size_matches_expected_rows() -> None:
    assert len(EXPECTED_ROWS) == 7
