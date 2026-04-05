from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT / "languages" / "typescript" / "docs" / "hooks-helper-ownership.md"
)

EXPECTED_ROWS = [
    "references/typescript/src/utils/hooks/apiQueryHookHelper.ts",
    "references/typescript/src/utils/hooks/hooksConfigSnapshot.ts",
    "references/typescript/src/utils/hooks/sessionHooks.ts",
    "references/typescript/src/utils/hooks/fileChangedWatcher.ts",
    "references/typescript/src/utils/hooks/hooksConfigManager.ts",
    "references/typescript/src/utils/hooks/hookEvents.ts",
    "references/typescript/src/utils/hooks/execPromptHook.ts",
    "references/typescript/src/utils/hooks/execHttpHook.ts",
    "references/typescript/src/utils/hooks/ssrfGuard.ts",
    "references/typescript/src/utils/hooks/postSamplingHooks.ts",
    "references/typescript/src/utils/hooks/hookHelpers.ts",
    "references/typescript/src/utils/hooks/AsyncHookRegistry.ts",
    "references/typescript/src/utils/hooks/hooksSettings.ts",
    "references/typescript/src/utils/hooks/skillImprovement.ts",
    "references/typescript/src/utils/hooks/registerFrontmatterHooks.ts",
    "references/typescript/src/utils/hooks/registerSkillHooks.ts",
    "references/typescript/src/utils/hooks/execAgentHook.ts",
]


def test_typescript_hooks_helper_rows_have_archive_or_owner_rationale() -> None:
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert "explicit archive-only or shipped-owner rationale" in text
    for row in EXPECTED_ROWS:
        assert row in text


def test_typescript_hooks_helper_cluster_size_matches_expected_rows() -> None:
    assert len(EXPECTED_ROWS) == 17
