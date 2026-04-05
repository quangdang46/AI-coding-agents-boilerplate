from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT / "languages" / "typescript" / "docs" / "skill-tool-ownership.md"
)

EXPECTED_ROWS = [
    "references/typescript/src/tools/SkillTool/SkillTool.ts",
    "references/typescript/src/tools/SkillTool/prompt.ts",
    "references/typescript/src/tools/SkillTool/UI.tsx",
    "references/typescript/src/tools/SkillTool/constants.ts",
]


def test_typescript_skill_tool_rows_have_owner_or_archive_rationale() -> None:
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert "concrete shipped owner or an explicit archive-only rationale" in text
    for row in EXPECTED_ROWS:
        assert row in text


def test_typescript_skill_tool_cluster_size_matches_expected_rows() -> None:
    assert len(EXPECTED_ROWS) == 4
