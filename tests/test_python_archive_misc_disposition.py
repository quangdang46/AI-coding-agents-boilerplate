from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
DISPOSITION_DOC = (
    REPO_ROOT / "languages" / "python" / "docs" / "archive-misc-disposition.md"
)

EXPECTED_ARCHIVE_MISC_ROWS = [
    "references/python/CLAUDE.md",
    "references/python/ROADMAP.md",
    "references/python/.claude.json",
    "references/python/.gitignore",
    "references/python/src/cost_tracker.py",
    "references/python/src/_archive_helper.py",
    "references/python/src/deferred_init.py",
    "references/python/src/parity_audit.py",
    "references/python/src/ink.py",
    "references/python/src/costHook.py",
    "references/python/src/__init__.py",
]


def test_python_archive_misc_rows_have_explicit_archive_only_disposition() -> None:
    text = DISPOSITION_DOC.read_text(encoding="utf-8")
    assert "historical migration artifacts" in text
    assert "must not be imported by shipped runtime code" in text
    for row in EXPECTED_ARCHIVE_MISC_ROWS:
        assert row in text


def test_python_archive_misc_cluster_size_matches_expected_rows() -> None:
    assert len(EXPECTED_ARCHIVE_MISC_ROWS) == 11
