from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT / "languages" / "typescript" / "docs" / "type-contract-ownership.md"
)

EXPECTED_ROWS = [
    "references/typescript/src/types/permissions.ts",
    "references/typescript/src/types/generated/events_mono/growthbook/v1/growthbook_experiment_event.ts",
    "references/typescript/src/types/generated/events_mono/common/v1/auth.ts",
    "references/typescript/src/types/generated/events_mono/claude_code/v1/claude_code_internal_event.ts",
    "references/typescript/src/types/generated/google/protobuf/timestamp.ts",
    "references/typescript/src/types/textInputTypes.ts",
    "references/typescript/src/types/connectorText.ts",
    "references/typescript/src/types/hooks.ts",
    "references/typescript/src/types/logs.ts",
    "references/typescript/src/types/command.ts",
    "references/typescript/src/types/plugin.ts",
    "references/typescript/src/types/ids.ts",
]


def test_typescript_type_contract_rows_have_archive_or_owner_rationale() -> None:
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert "explicit archive-only or shipped-owner rationale" in text
    for row in EXPECTED_ROWS:
        assert row in text


def test_typescript_type_contract_cluster_size_matches_expected_rows() -> None:
    assert len(EXPECTED_ROWS) == 12
