from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT / "languages" / "typescript" / "docs" / "dialog-settings-ownership.md"
)

EXPECTED_ROWS = [
    "references/typescript/src/components/Onboarding.tsx",
    "references/typescript/src/components/ConsoleOAuthFlow.tsx",
    "references/typescript/src/components/ApproveApiKey.tsx",
    "references/typescript/src/components/InvalidConfigDialog.tsx",
    "references/typescript/src/components/InvalidSettingsDialog.tsx",
    "references/typescript/src/components/ManagedSettingsSecurityDialog/ManagedSettingsSecurityDialog.tsx",
    "references/typescript/src/components/ManagedSettingsSecurityDialog/utils.ts",
    "references/typescript/src/components/Settings/Settings.tsx",
    "references/typescript/src/components/Settings/Config.tsx",
    "references/typescript/src/components/Settings/Usage.tsx",
    "references/typescript/src/components/Settings/Status.tsx",
    "references/typescript/src/components/ConfigurableShortcutHint.tsx",
    "references/typescript/src/components/KeybindingWarnings.tsx",
]


def test_typescript_dialog_settings_rows_have_archive_or_owner_rationale() -> None:
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert "explicit archive-only or shipped-owner rationale" in text
    for row in EXPECTED_ROWS:
        assert row in text


def test_typescript_dialog_settings_cluster_size_matches_expected_rows() -> None:
    assert len(EXPECTED_ROWS) == 13
