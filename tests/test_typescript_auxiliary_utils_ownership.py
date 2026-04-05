from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT / "languages" / "typescript" / "docs" / "auxiliary-utils-ownership.md"
)

EXPECTED_ROWS = [
    "references/typescript/src/utils/powershell/dangerousCmdlets.ts",
    "references/typescript/src/utils/powershell/staticPrefix.ts",
    "references/typescript/src/utils/powershell/parser.ts",
    "references/typescript/src/utils/sandbox/sandbox-adapter.ts",
    "references/typescript/src/utils/sandbox/sandbox-ui-utils.ts",
    "references/typescript/src/utils/memory/versions.ts",
    "references/typescript/src/utils/memory/types.ts",
    "references/typescript/src/utils/filePersistence/filePersistence.ts",
    "references/typescript/src/utils/filePersistence/outputsScanner.ts",
    "references/typescript/src/utils/filePersistence/types.ts",
    "references/typescript/src/utils/dxt/zip.ts",
    "references/typescript/src/utils/dxt/helpers.ts",
    "references/typescript/src/utils/git/gitFilesystem.ts",
    "references/typescript/src/utils/git/gitignore.ts",
    "references/typescript/src/utils/git/gitConfigParser.ts",
    "references/typescript/src/utils/mcp/elicitationValidation.ts",
    "references/typescript/src/utils/mcp/dateTimeParser.ts",
    "references/typescript/src/utils/background/remote/preconditions.ts",
    "references/typescript/src/utils/background/remote/remoteSession.ts",
    "references/typescript/src/utils/teleport/environmentSelection.ts",
    "references/typescript/src/utils/teleport/gitBundle.ts",
    "references/typescript/src/utils/teleport/environments.ts",
    "references/typescript/src/utils/teleport/api.ts",
    "references/typescript/src/utils/ultraplan/ccrSession.ts",
    "references/typescript/src/utils/ultraplan/prompt.txt",
    "references/typescript/src/utils/ultraplan/keyword.ts",
]


def test_typescript_auxiliary_utils_rows_have_archive_or_owner_rationale() -> None:
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert "explicit archive-only or shipped-owner rationale" in text
    for row in EXPECTED_ROWS:
        assert row in text


def test_typescript_auxiliary_utils_cluster_size_matches_expected_rows() -> None:
    assert len(EXPECTED_ROWS) == 26
