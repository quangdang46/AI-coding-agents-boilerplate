from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT
    / "languages"
    / "typescript"
    / "docs"
    / "utils-runtime-helper-ownership.md"
)

EXPECTED_ROWS = [
    "references/typescript/src/utils/exampleCommands.ts",
    "references/typescript/src/utils/memoize.ts",
    "references/typescript/src/utils/set.ts",
    "references/typescript/src/utils/memory/versions.ts",
    "references/typescript/src/utils/memory/types.ts",
    "references/typescript/src/utils/cronScheduler.ts",
    "references/typescript/src/utils/ripgrep.ts",
]


def test_typescript_runtime_helper_rows_have_archive_or_owner_rationale() -> None:
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert "explicit archive-only or shipped-owner rationale" in text
    for row in EXPECTED_ROWS:
        assert row in text


def test_typescript_runtime_helper_cluster_size_matches_expected_rows() -> None:
    assert len(EXPECTED_ROWS) == 7
