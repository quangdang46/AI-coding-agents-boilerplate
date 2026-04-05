from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT
    / "languages"
    / "typescript"
    / "docs"
    / "chrome-secure-storage-ownership.md"
)

EXPECTED_ROWS = [
    "references/typescript/src/utils/secureStorage/keychainPrefetch.ts",
    "references/typescript/src/utils/secureStorage/fallbackStorage.ts",
    "references/typescript/src/utils/secureStorage/plainTextStorage.ts",
    "references/typescript/src/utils/secureStorage/index.ts",
    "references/typescript/src/utils/secureStorage/macOsKeychainStorage.ts",
    "references/typescript/src/utils/secureStorage/macOsKeychainHelpers.ts",
    "references/typescript/src/utils/claudeInChrome/package.ts",
    "references/typescript/src/utils/claudeInChrome/setup.ts",
    "references/typescript/src/utils/claudeInChrome/toolRendering.tsx",
    "references/typescript/src/utils/claudeInChrome/prompt.ts",
    "references/typescript/src/utils/claudeInChrome/setupPortable.ts",
    "references/typescript/src/utils/claudeInChrome/common.ts",
    "references/typescript/src/utils/claudeInChrome/mcpServer.ts",
    "references/typescript/src/utils/claudeInChrome/chromeNativeHost.ts",
]


def test_typescript_chrome_secure_storage_rows_have_archive_or_owner_rationale() -> (
    None
):
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert "explicit archive-only or shipped-owner rationale" in text
    for row in EXPECTED_ROWS:
        assert row in text


def test_typescript_chrome_secure_storage_cluster_size_matches_expected_rows() -> None:
    assert len(EXPECTED_ROWS) == 14
