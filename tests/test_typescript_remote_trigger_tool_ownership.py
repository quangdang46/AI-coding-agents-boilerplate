from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT / "languages" / "typescript" / "docs" / "remote-trigger-tool-ownership.md"
)

EXPECTED_ROWS = [
    "references/typescript/src/tools/RemoteTriggerTool/prompt.ts",
    "references/typescript/src/tools/RemoteTriggerTool/UI.tsx",
    "references/typescript/src/tools/RemoteTriggerTool/RemoteTriggerTool.ts",
]


def test_typescript_remote_trigger_rows_have_archive_or_owner_rationale() -> None:
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert "explicit archive-only or shipped-owner rationale" in text
    for row in EXPECTED_ROWS:
        assert row in text


def test_typescript_remote_trigger_cluster_size_matches_expected_rows() -> None:
    assert len(EXPECTED_ROWS) == 3
