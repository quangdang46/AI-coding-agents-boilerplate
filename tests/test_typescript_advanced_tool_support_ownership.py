from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT
    / "languages"
    / "typescript"
    / "docs"
    / "advanced-tool-support-ownership.md"
)

EXPECTED_ADVANCED_TOOL_ROWS = [
    "references/typescript/src/tools/VerifyPlanExecutionTool/VerifyPlanExecutionTool.ts",
    "references/typescript/src/tools/VerifyPlanExecutionTool/constants.ts",
    "references/typescript/src/tools/TungstenTool/TungstenTool.ts",
    "references/typescript/src/tools/TungstenTool/TungstenLiveMonitor.tsx",
    "references/typescript/src/tools/shared/spawnMultiAgent.ts",
    "references/typescript/src/tools/shared/gitOperationTracking.ts",
]


def test_typescript_advanced_tool_rows_have_owner_or_exclusion() -> None:
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert "explicit feature-pack owner or an explicit exclusion rationale" in text
    for row in EXPECTED_ADVANCED_TOOL_ROWS:
        assert row in text


def test_typescript_advanced_tool_cluster_size_matches_bead_manifest() -> None:
    assert len(EXPECTED_ADVANCED_TOOL_ROWS) == 6
