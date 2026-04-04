from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
DISPOSITION_DOC = (
    REPO_ROOT / "languages" / "typescript" / "docs" / "root-archive-disposition.md"
)

EXPECTED_ROOT_ROWS = [
    "references/typescript/README.md",
    "references/typescript/FEATURES.md",
    "references/typescript/CLAUDE.md",
    "references/typescript/package.json",
    "references/typescript/tsconfig.json",
    "references/typescript/env.d.ts",
    "references/typescript/scripts/build.ts",
    "references/typescript/install.sh",
    "references/typescript/changes.md",
    "references/typescript/assets/screenshot.png",
    "references/typescript/bun.lock",
    "references/typescript/.gitignore",
]


def test_typescript_root_archive_rows_have_explicit_disposition() -> None:
    text = DISPOSITION_DOC.read_text(encoding="utf-8")
    assert "migration evidence and historical build metadata" in text
    assert "must not depend on the archived root files at runtime" in text
    for row in EXPECTED_ROOT_ROWS:
        assert row in text


def test_typescript_root_archive_cluster_size_matches_bead_manifest() -> None:
    assert len(EXPECTED_ROOT_ROWS) == 12
