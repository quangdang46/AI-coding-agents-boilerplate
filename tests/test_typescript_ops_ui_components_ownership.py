from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT / "languages" / "typescript" / "docs" / "ops-ui-components-ownership.md"
)

EXPECTED_ROWS = [
    "references/typescript/src/components/MCPServerApprovalDialog.tsx",
    "references/typescript/src/components/teams/TeamsDialog.tsx",
    "references/typescript/src/components/teams/TeamStatus.tsx",
    "references/typescript/src/components/mcp/utils/reconnectHelpers.tsx",
    "references/typescript/src/components/mcp/MCPToolListView.tsx",
    "references/typescript/src/components/mcp/MCPSettings.tsx",
    "references/typescript/src/components/mcp/MCPListPanel.tsx",
    "references/typescript/src/components/mcp/MCPRemoteServerMenu.tsx",
    "references/typescript/src/components/mcp/MCPAgentServerMenu.tsx",
    "references/typescript/src/components/mcp/McpParsingWarnings.tsx",
    "references/typescript/src/components/mcp/CapabilitiesSection.tsx",
    "references/typescript/src/components/mcp/ElicitationDialog.tsx",
    "references/typescript/src/components/mcp/MCPReconnect.tsx",
    "references/typescript/src/components/mcp/index.ts",
    "references/typescript/src/components/mcp/MCPToolDetailView.tsx",
    "references/typescript/src/components/mcp/MCPStdioServerMenu.tsx",
    "references/typescript/src/components/tasks/RemoteSessionDetailDialog.tsx",
    "references/typescript/src/components/tasks/InProcessTeammateDetailDialog.tsx",
    "references/typescript/src/components/tasks/BackgroundTask.tsx",
    "references/typescript/src/components/tasks/RemoteSessionProgress.tsx",
    "references/typescript/src/components/tasks/AsyncAgentDetailDialog.tsx",
    "references/typescript/src/components/tasks/ShellProgress.tsx",
    "references/typescript/src/components/tasks/DreamDetailDialog.tsx",
    "references/typescript/src/components/tasks/taskStatusUtils.tsx",
    "references/typescript/src/components/tasks/renderToolActivity.tsx",
    "references/typescript/src/components/tasks/BackgroundTasksDialog.tsx",
    "references/typescript/src/components/tasks/BackgroundTaskStatus.tsx",
    "references/typescript/src/components/tasks/ShellDetailDialog.tsx",
    "references/typescript/src/components/shell/ShellTimeDisplay.tsx",
    "references/typescript/src/components/shell/ShellProgressMessage.tsx",
    "references/typescript/src/components/shell/OutputLine.tsx",
    "references/typescript/src/components/shell/ExpandShellOutputContext.tsx",
    "references/typescript/src/components/permissions/EnterPlanModePermissionRequest/EnterPlanModePermissionRequest.tsx",
    "references/typescript/src/components/permissions/FileEditPermissionRequest/FileEditPermissionRequest.tsx",
    "references/typescript/src/components/permissions/useShellPermissionFeedback.ts",
    "references/typescript/src/components/permissions/ExitPlanModePermissionRequest/ExitPlanModePermissionRequest.tsx",
    "references/typescript/src/components/permissions/NotebookEditPermissionRequest/NotebookEditToolDiff.tsx",
    "references/typescript/src/components/permissions/NotebookEditPermissionRequest/NotebookEditPermissionRequest.tsx",
    "references/typescript/src/components/permissions/shellPermissionHelpers.tsx",
    "references/typescript/src/components/permissions/SandboxPermissionRequest.tsx",
    "references/typescript/src/components/permissions/FallbackPermissionRequest.tsx",
    "references/typescript/src/components/permissions/PermissionRequestTitle.tsx",
    "references/typescript/src/components/permissions/FilePermissionDialog/FilePermissionDialog.tsx",
    "references/typescript/src/components/permissions/FilePermissionDialog/permissionOptions.tsx",
    "references/typescript/src/components/permissions/FilePermissionDialog/useFilePermissionDialog.ts",
    "references/typescript/src/components/permissions/FilePermissionDialog/ideDiffConfig.ts",
    "references/typescript/src/components/permissions/FilePermissionDialog/usePermissionHandler.ts",
    "references/typescript/src/components/permissions/SedEditPermissionRequest/SedEditPermissionRequest.tsx",
    "references/typescript/src/components/permissions/WorkerBadge.tsx",
    "references/typescript/src/components/permissions/PermissionDecisionDebugInfo.tsx",
    "references/typescript/src/components/permissions/WebFetchPermissionRequest/WebFetchPermissionRequest.tsx",
    "references/typescript/src/components/permissions/FilesystemPermissionRequest/FilesystemPermissionRequest.tsx",
    "references/typescript/src/components/permissions/PowerShellPermissionRequest/PowerShellPermissionRequest.tsx",
    "references/typescript/src/components/permissions/PowerShellPermissionRequest/powershellToolUseOptions.tsx",
    "references/typescript/src/components/permissions/ComputerUseApproval/ComputerUseApproval.tsx",
    "references/typescript/src/components/permissions/hooks.ts",
    "references/typescript/src/components/permissions/PermissionRuleExplanation.tsx",
    "references/typescript/src/components/permissions/utils.ts",
    "references/typescript/src/components/permissions/FileWritePermissionRequest/FileWritePermissionRequest.tsx",
    "references/typescript/src/components/permissions/FileWritePermissionRequest/FileWriteToolDiff.tsx",
    "references/typescript/src/components/permissions/rules/AddPermissionRules.tsx",
    "references/typescript/src/components/permissions/rules/PermissionRuleInput.tsx",
    "references/typescript/src/components/permissions/rules/PermissionRuleDescription.tsx",
    "references/typescript/src/components/permissions/rules/RecentDenialsTab.tsx",
    "references/typescript/src/components/permissions/rules/AddWorkspaceDirectory.tsx",
    "references/typescript/src/components/permissions/rules/RemoveWorkspaceDirectory.tsx",
    "references/typescript/src/components/permissions/rules/PermissionRuleList.tsx",
    "references/typescript/src/components/permissions/rules/WorkspaceTab.tsx",
    "references/typescript/src/components/permissions/BashPermissionRequest/bashToolUseOptions.tsx",
    "references/typescript/src/components/permissions/BashPermissionRequest/BashPermissionRequest.tsx",
    "references/typescript/src/components/permissions/WorkerPendingPermission.tsx",
    "references/typescript/src/components/permissions/PermissionDialog.tsx",
    "references/typescript/src/components/permissions/PermissionPrompt.tsx",
    "references/typescript/src/components/permissions/PermissionRequest.tsx",
    "references/typescript/src/components/permissions/PermissionExplanation.tsx",
    "references/typescript/src/components/permissions/AskUserQuestionPermissionRequest/AskUserQuestionPermissionRequest.tsx",
    "references/typescript/src/components/permissions/AskUserQuestionPermissionRequest/PreviewBox.tsx",
    "references/typescript/src/components/permissions/AskUserQuestionPermissionRequest/use-multiple-choice-state.ts",
    "references/typescript/src/components/permissions/AskUserQuestionPermissionRequest/QuestionNavigationBar.tsx",
    "references/typescript/src/components/permissions/AskUserQuestionPermissionRequest/SubmitQuestionsView.tsx",
    "references/typescript/src/components/permissions/AskUserQuestionPermissionRequest/PreviewQuestionView.tsx",
    "references/typescript/src/components/permissions/AskUserQuestionPermissionRequest/QuestionView.tsx",
    "references/typescript/src/components/permissions/SkillPermissionRequest/SkillPermissionRequest.tsx",
]


def test_typescript_ops_ui_component_rows_have_owner_rationale() -> None:
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert "explicit shipped-owner or archive-only rationale" in text
    for row in EXPECTED_ROWS:
        assert row in text


def test_typescript_ops_ui_component_cluster_size_matches_manifest() -> None:
    assert len(EXPECTED_ROWS) == 83
