from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT / "languages" / "rust" / "docs" / "workspace-artifacts-ownership.md"
)

EXPECTED_ROWS = [
    "references/rust/TUI-ENHANCEMENT-PLAN.md",
    "references/rust/.omc/plans/tui-enhancement-plan.md",
    "references/rust/.claude/sessions/session-1775009431231.json",
    "references/rust/.claude/sessions/session-1775009769569.json",
    "references/rust/.claude/sessions/session-1775008071886.json",
    "references/rust/.claude/sessions/session-1775008464519.json",
    "references/rust/.claude/sessions/session-1775008308936.json",
    "references/rust/.claude/sessions/session-1775011146355.json",
    "references/rust/.claude/sessions/session-1775012687059.json",
    "references/rust/.claude/sessions/session-1775010047738.json",
    "references/rust/.claude/sessions/session-1775008137143.json",
    "references/rust/.claude/sessions/session-1775008007069.json",
    "references/rust/.claude/sessions/session-1775012674485.json",
    "references/rust/.claude/sessions/session-1775010384918.json",
    "references/rust/.claude/sessions/session-1775009841982.json",
    "references/rust/.claude/sessions/session-1775011562247.json",
    "references/rust/.claude/sessions/session-1775010333630.json",
    "references/rust/.claude/sessions/session-1775007981374.json",
    "references/rust/.claude/sessions/session-1775010909274.json",
    "references/rust/.claude/sessions/session-1775008161929.json",
    "references/rust/.claude/sessions/session-1775013221875.json",
    "references/rust/.claude/sessions/session-1775007484031.json",
    "references/rust/.claude/sessions/session-1775009126336.json",
    "references/rust/.claude/sessions/session-1775008997307.json",
    "references/rust/.claude/sessions/session-1775007453382.json",
    "references/rust/.claude/sessions/session-1775008427969.json",
    "references/rust/.claude/sessions/session-1775009119214.json",
    "references/rust/.claude/sessions/session-1775007490104.json",
    "references/rust/.claude/sessions/session-1775009145469.json",
    "references/rust/.claude/sessions/session-1775009869734.json",
    "references/rust/scripts/run_mock_parity_diff.py",
    "references/rust/scripts/run_mock_parity_harness.sh",
    "references/rust/.clawd-todos.json",
    "references/rust/PARITY.md",
    "references/rust/mock_parity_scenarios.json",
    "references/rust/MOCK_PARITY_HARNESS.md",
]


def test_rust_workspace_artifact_rows_have_owner_rationale() -> None:
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert "explicit shipped-owner or archive-only rationale" in text
    for row in EXPECTED_ROWS:
        assert row in text


def test_rust_workspace_artifact_cluster_size_matches_manifest() -> None:
    assert len(EXPECTED_ROWS) == 36
