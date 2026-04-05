from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT
    / "languages"
    / "typescript"
    / "docs"
    / "special-surfaces-remainder-ownership.md"
)

EXPECTED_ROWS = [
    "references/typescript/src/voice/voiceModeEnabled.ts",
    "references/typescript/src/vim/transitions.ts",
    "references/typescript/src/vim/motions.ts",
    "references/typescript/src/vim/operators.ts",
    "references/typescript/src/vim/textObjects.ts",
    "references/typescript/src/vim/types.ts",
    "references/typescript/src/memdir/teamMemPaths.ts",
    "references/typescript/src/memdir/paths.ts",
    "references/typescript/src/memdir/teamMemPrompts.ts",
    "references/typescript/src/memdir/memoryAge.ts",
    "references/typescript/src/memdir/memoryScan.ts",
    "references/typescript/src/memdir/memoryTypes.ts",
    "references/typescript/src/memdir/memdir.ts",
    "references/typescript/src/memdir/findRelevantMemories.ts",
    "references/typescript/src/buddy/CompanionSprite.tsx",
    "references/typescript/src/buddy/prompt.ts",
    "references/typescript/src/buddy/types.ts",
    "references/typescript/src/buddy/useBuddyNotification.tsx",
    "references/typescript/src/buddy/sprites.ts",
    "references/typescript/src/buddy/companion.ts",
    "references/typescript/src/native-ts/file-index/index.ts",
    "references/typescript/src/native-ts/yoga-layout/enums.ts",
    "references/typescript/src/native-ts/yoga-layout/index.ts",
    "references/typescript/src/native-ts/color-diff/index.ts",
    "references/typescript/src/moreright/useMoreRight.tsx",
]


def test_typescript_special_surface_remainder_rows_have_owner_or_archive_rationale() -> (
    None
):
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert (
        "clear future feature-pack owner or an explicit archive-only rationale" in text
    )
    for row in EXPECTED_ROWS:
        assert row in text


def test_typescript_special_surface_remainder_cluster_size_matches_expected_rows() -> (
    None
):
    assert len(EXPECTED_ROWS) == 25
