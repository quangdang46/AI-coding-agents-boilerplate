from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT
    / "languages"
    / "typescript"
    / "docs"
    / "model-provider-prompt-context-ownership.md"
)

EXPECTED_ROWS = [
    "references/typescript/src/utils/systemPrompt.ts",
    "references/typescript/src/utils/context.ts",
    "references/typescript/src/utils/queryContext.ts",
    "references/typescript/src/utils/model/modelCapabilities.ts",
    "references/typescript/src/utils/model/bedrock.ts",
    "references/typescript/src/utils/model/modelSupportOverrides.ts",
    "references/typescript/src/utils/model/modelOptions.ts",
    "references/typescript/src/utils/model/check1mAccess.ts",
    "references/typescript/src/utils/model/modelStrings.ts",
    "references/typescript/src/utils/model/aliases.ts",
    "references/typescript/src/utils/model/model.ts",
    "references/typescript/src/utils/model/configs.ts",
    "references/typescript/src/utils/model/validateModel.ts",
    "references/typescript/src/utils/model/agent.ts",
    "references/typescript/src/utils/model/antModels.ts",
    "references/typescript/src/utils/model/contextWindowUpgradeCheck.ts",
    "references/typescript/src/utils/model/deprecation.ts",
    "references/typescript/src/utils/model/modelAllowlist.ts",
    "references/typescript/src/utils/model/providers.ts",
    "references/typescript/src/utils/systemPromptType.ts",
]


def test_typescript_model_provider_prompt_context_rows_have_owner_rationale() -> None:
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert "explicit shipped-owner or archive-only rationale" in text
    for row in EXPECTED_ROWS:
        assert row in text


def test_typescript_model_provider_prompt_context_cluster_size_matches_manifest() -> (
    None
):
    assert len(EXPECTED_ROWS) == 20
