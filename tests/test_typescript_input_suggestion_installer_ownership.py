from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT
    / "languages"
    / "typescript"
    / "docs"
    / "input-suggestion-installer-ownership.md"
)

EXPECTED_ROWS = [
    "references/typescript/src/utils/processUserInput/processUserInput.ts",
    "references/typescript/src/utils/processUserInput/processTextPrompt.ts",
    "references/typescript/src/utils/processUserInput/processBashCommand.tsx",
    "references/typescript/src/utils/processUserInput/processSlashCommand.tsx",
    "references/typescript/src/utils/suggestions/slackChannelSuggestions.ts",
    "references/typescript/src/utils/suggestions/skillUsageTracking.ts",
    "references/typescript/src/utils/suggestions/commandSuggestions.ts",
    "references/typescript/src/utils/suggestions/shellHistoryCompletion.ts",
    "references/typescript/src/utils/suggestions/directoryCompletion.ts",
    "references/typescript/src/utils/nativeInstaller/pidLock.ts",
    "references/typescript/src/utils/nativeInstaller/installer.ts",
    "references/typescript/src/utils/nativeInstaller/packageManagers.ts",
    "references/typescript/src/utils/nativeInstaller/download.ts",
    "references/typescript/src/utils/nativeInstaller/index.ts",
]


def test_typescript_input_suggestion_installer_rows_have_owner_or_archive_rationale() -> (
    None
):
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert (
        "clear shipped interaction/runtime owner or an explicit archive-only rationale"
        in text
    )
    for row in EXPECTED_ROWS:
        assert row in text


def test_typescript_input_suggestion_installer_cluster_size_matches_bead_manifest() -> (
    None
):
    assert len(EXPECTED_ROWS) == 14
