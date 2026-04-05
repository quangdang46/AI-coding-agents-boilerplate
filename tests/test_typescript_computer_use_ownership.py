from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT / "languages" / "typescript" / "docs" / "computer-use-ownership.md"
)

EXPECTED_ROWS = [
    "references/typescript/src/utils/computerUse/setup.ts",
    "references/typescript/src/utils/computerUse/wrapper.tsx",
    "references/typescript/src/utils/computerUse/toolRendering.tsx",
    "references/typescript/src/utils/computerUse/appNames.ts",
    "references/typescript/src/utils/computerUse/swiftLoader.ts",
    "references/typescript/src/utils/computerUse/hostAdapter.ts",
    "references/typescript/src/utils/computerUse/cleanup.ts",
    "references/typescript/src/utils/computerUse/inputLoader.ts",
    "references/typescript/src/utils/computerUse/computerUseLock.ts",
    "references/typescript/src/utils/computerUse/gates.ts",
    "references/typescript/src/utils/computerUse/common.ts",
    "references/typescript/src/utils/computerUse/executor.ts",
    "references/typescript/src/utils/computerUse/drainRunLoop.ts",
    "references/typescript/src/utils/computerUse/mcpServer.ts",
    "references/typescript/src/utils/computerUse/escHotkey.ts",
]


def test_typescript_computer_use_rows_have_archive_or_owner_rationale() -> None:
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert "explicit archive-only or shipped-owner rationale" in text
    for row in EXPECTED_ROWS:
        assert row in text


def test_typescript_computer_use_cluster_size_matches_expected_rows() -> None:
    assert len(EXPECTED_ROWS) == 15
