from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT / "languages" / "typescript" / "docs" / "constants-schema-ownership.md"
)

EXPECTED_ROWS = [
    "references/typescript/src/constants/outputStyles.ts",
    "references/typescript/src/constants/errorIds.ts",
    "references/typescript/src/constants/xml.ts",
    "references/typescript/src/constants/keys.ts",
    "references/typescript/src/constants/spinnerVerbs.ts",
    "references/typescript/src/constants/apiLimits.ts",
    "references/typescript/src/constants/figures.ts",
    "references/typescript/src/constants/messages.ts",
    "references/typescript/src/constants/cyberRiskInstruction.ts",
    "references/typescript/src/constants/toolLimits.ts",
    "references/typescript/src/constants/product.ts",
    "references/typescript/src/constants/systemPromptSections.ts",
    "references/typescript/src/constants/oauth.ts",
    "references/typescript/src/constants/codex-oauth.ts",
    "references/typescript/src/constants/betas.ts",
    "references/typescript/src/constants/turnCompletionVerbs.ts",
    "references/typescript/src/constants/system.ts",
    "references/typescript/src/constants/common.ts",
    "references/typescript/src/constants/files.ts",
    "references/typescript/src/constants/prompts.ts",
    "references/typescript/src/constants/tools.ts",
    "references/typescript/src/constants/github-app.ts",
    "references/typescript/src/schemas/hooks.ts",
]


def test_typescript_constants_schema_rows_have_owner_or_archive_rationale() -> None:
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert (
        "clear shipped runtime/config owner or an explicit archive-only rationale"
        in text
    )
    for row in EXPECTED_ROWS:
        assert row in text


def test_typescript_constants_schema_cluster_size_matches_bead_manifest() -> None:
    assert len(EXPECTED_ROWS) == 23
