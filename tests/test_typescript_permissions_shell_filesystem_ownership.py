from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT
    / "languages"
    / "typescript"
    / "docs"
    / "permissions-shell-filesystem-ownership.md"
)

EXPECTED_ROWS = [
    "references/typescript/src/utils/shell/powershellProvider.ts",
    "references/typescript/src/utils/shell/resolveDefaultShell.ts",
    "references/typescript/src/utils/shell/prefix.ts",
    "references/typescript/src/utils/shell/shellProvider.ts",
    "references/typescript/src/utils/shell/readOnlyCommandValidation.ts",
    "references/typescript/src/utils/shell/outputLimits.ts",
    "references/typescript/src/utils/shell/specPrefix.ts",
    "references/typescript/src/utils/shell/shellToolUtils.ts",
    "references/typescript/src/utils/shell/bashProvider.ts",
    "references/typescript/src/utils/shell/powershellDetection.ts",
    "references/typescript/src/utils/permissions/autoModeState.ts",
    "references/typescript/src/utils/permissions/dangerousPatterns.ts",
    "references/typescript/src/utils/permissions/PermissionRule.ts",
    "references/typescript/src/utils/permissions/denialTracking.ts",
    "references/typescript/src/utils/permissions/shadowedRuleDetection.ts",
    "references/typescript/src/utils/permissions/permissions.ts",
    "references/typescript/src/utils/permissions/permissionRuleParser.ts",
    "references/typescript/src/utils/permissions/filesystem.ts",
    "references/typescript/src/utils/permissions/getNextPermissionMode.ts",
    "references/typescript/src/utils/permissions/classifierShared.ts",
    "references/typescript/src/utils/permissions/bypassPermissionsKillswitch.ts",
    "references/typescript/src/utils/permissions/PermissionPromptToolResultSchema.ts",
    "references/typescript/src/utils/permissions/bashClassifier.ts",
    "references/typescript/src/utils/permissions/PermissionResult.ts",
    "references/typescript/src/utils/permissions/permissionsLoader.ts",
    "references/typescript/src/utils/permissions/PermissionUpdateSchema.ts",
    "references/typescript/src/utils/permissions/permissionExplainer.ts",
    "references/typescript/src/utils/permissions/classifierDecision.ts",
    "references/typescript/src/utils/permissions/PermissionMode.ts",
    "references/typescript/src/utils/permissions/PermissionUpdate.ts",
    "references/typescript/src/utils/permissions/permissionSetup.ts",
    "references/typescript/src/utils/permissions/pathValidation.ts",
    "references/typescript/src/utils/permissions/shellRuleMatching.ts",
    "references/typescript/src/utils/permissions/yoloClassifier.ts",
]


def test_typescript_permissions_shell_rows_have_owner_rationale() -> None:
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert "explicit shipped-owner or archive-only rationale" in text
    for row in EXPECTED_ROWS:
        assert row in text


def test_typescript_permissions_shell_cluster_size_matches_manifest() -> None:
    assert len(EXPECTED_ROWS) == 34
