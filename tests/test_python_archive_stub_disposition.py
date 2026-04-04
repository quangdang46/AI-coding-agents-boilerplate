from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
DISPOSITION_DOC = (
    REPO_ROOT / "languages" / "python" / "docs" / "archive-stub-disposition.md"
)

EXPECTED_ARCHIVE_STUB_ROWS = [
    "references/python/src/assistant/__init__.py",
    "references/python/src/bootstrap/__init__.py",
    "references/python/src/bridge/__init__.py",
    "references/python/src/buddy/__init__.py",
    "references/python/src/cli/__init__.py",
    "references/python/src/components/__init__.py",
    "references/python/src/constants/__init__.py",
    "references/python/src/coordinator/__init__.py",
    "references/python/src/entrypoints/__init__.py",
    "references/python/src/hooks/__init__.py",
    "references/python/src/keybindings/__init__.py",
    "references/python/src/memdir/__init__.py",
    "references/python/src/migrations/__init__.py",
    "references/python/src/moreright/__init__.py",
    "references/python/src/native_ts/__init__.py",
    "references/python/src/outputStyles/__init__.py",
    "references/python/src/plugins/__init__.py",
    "references/python/src/remote/__init__.py",
    "references/python/src/schemas/__init__.py",
    "references/python/src/screens/__init__.py",
    "references/python/src/server/__init__.py",
    "references/python/src/services/__init__.py",
    "references/python/src/skills/__init__.py",
    "references/python/src/state/__init__.py",
    "references/python/src/types/__init__.py",
    "references/python/src/upstreamproxy/__init__.py",
    "references/python/src/utils/__init__.py",
    "references/python/src/vim/__init__.py",
    "references/python/src/voice/__init__.py",
    "references/python/tests/test_porting_workspace.py",
]


def test_python_archive_stub_rows_have_explicit_archive_only_disposition() -> None:
    text = DISPOSITION_DOC.read_text(encoding="utf-8")
    assert "archive-only compatibility markers" in text
    assert "must not be imported by shipped runtime code" in text
    for row in EXPECTED_ARCHIVE_STUB_ROWS:
        assert row in text


def test_python_archive_stub_cluster_size_matches_coverage_manifest() -> None:
    assert len(EXPECTED_ARCHIVE_STUB_ROWS) == 30
