from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = REPO_ROOT / "languages" / "typescript" / "docs" / "screens-ownership.md"

EXPECTED_ROWS = [
    "references/typescript/src/screens/Doctor.tsx",
    "references/typescript/src/screens/ResumeConversation.tsx",
    "references/typescript/src/screens/REPL.tsx",
]


def test_typescript_screen_rows_have_archive_or_owner_rationale() -> None:
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert "explicit archive-only or shipped-owner rationale" in text
    for row in EXPECTED_ROWS:
        assert row in text


def test_typescript_screen_cluster_size_matches_expected_rows() -> None:
    assert len(EXPECTED_ROWS) == 3
