from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT / "languages" / "typescript" / "docs" / "migrations-ownership.md"
)

EXPECTED_ROWS = [
    "references/typescript/src/migrations/migrateBypassPermissionsAcceptedToSettings.ts",
    "references/typescript/src/migrations/migrateOpusToOpus1m.ts",
    "references/typescript/src/migrations/migrateEnableAllProjectMcpServersToSettings.ts",
    "references/typescript/src/migrations/resetAutoModeOptInForDefaultOffer.ts",
    "references/typescript/src/migrations/migrateReplBridgeEnabledToRemoteControlAtStartup.ts",
    "references/typescript/src/migrations/migrateSonnet45ToSonnet46.ts",
    "references/typescript/src/migrations/migrateLegacyOpusToCurrent.ts",
    "references/typescript/src/migrations/resetProToOpusDefault.ts",
    "references/typescript/src/migrations/migrateFennecToOpus.ts",
    "references/typescript/src/migrations/migrateSonnet1mToSonnet45.ts",
    "references/typescript/src/migrations/migrateAutoUpdatesToSettings.ts",
]


def test_typescript_migration_rows_have_owner_or_archive_rationale() -> None:
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert (
        "clear shipped config/runtime owner or an explicit archive-only rationale"
        in text
    )
    for row in EXPECTED_ROWS:
        assert row in text


def test_typescript_migration_cluster_size_matches_bead_manifest() -> None:
    assert len(EXPECTED_ROWS) == 11
