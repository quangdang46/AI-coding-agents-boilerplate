from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT / "languages" / "typescript" / "docs" / "utils-standalone-ownership-a.md"
)

EXPECTED_ROWS = [
    "references/typescript/src/utils/xml.ts",
    "references/typescript/src/utils/sessionUrl.ts",
    "references/typescript/src/utils/detectRepository.ts",
    "references/typescript/src/utils/errorLogSink.ts",
    "references/typescript/src/utils/treeify.ts",
    "references/typescript/src/utils/generators.ts",
    "references/typescript/src/utils/sliceAnsi.ts",
    "references/typescript/src/utils/handlePromptSubmit.ts",
    "references/typescript/src/utils/sessionActivity.ts",
]


def test_typescript_standalone_utils_a_rows_have_archive_or_owner_rationale() -> None:
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert "explicit archive-only or shipped-owner rationale" in text
    for row in EXPECTED_ROWS:
        assert row in text


def test_typescript_standalone_utils_a_cluster_size_matches_expected_rows() -> None:
    assert len(EXPECTED_ROWS) == 9
