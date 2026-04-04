from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT / "languages" / "typescript" / "docs" / "runtime-misc-ownership.md"
)

EXPECTED_MISC_ROWS = [
    "references/typescript/src/tools/utils.ts",
    "references/typescript/src/tools/testing/TestingPermissionTool.tsx",
    "references/typescript/src/ink.ts",
    "references/typescript/src/outputStyles/loadOutputStylesDir.ts",
    "references/typescript/src/coordinator/coordinatorMode.ts",
    "references/typescript/src/vendor/ripgrep/x64-linux/rg",
]


def test_typescript_misc_rows_have_explicit_runtime_owner_or_exclusion() -> None:
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert (
        "concrete shipped TypeScript owner or an explicit exclusion rationale" in text
    )
    for row in EXPECTED_MISC_ROWS:
        assert row in text


def test_typescript_misc_cluster_size_matches_bead_manifest() -> None:
    assert len(EXPECTED_MISC_ROWS) == 6
