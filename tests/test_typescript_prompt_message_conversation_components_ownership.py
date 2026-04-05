from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT
    / "languages"
    / "typescript"
    / "docs"
    / "prompt-message-conversation-components-ownership.md"
)

EXPECTED_ROWS = [
    "references/typescript/src/components/Message.tsx",
    "references/typescript/src/components/messages/CollapsedReadSearchContent.tsx",
    "references/typescript/src/components/messages/ShutdownMessage.tsx",
    "references/typescript/src/components/messages/UserResourceUpdateMessage.tsx",
    "references/typescript/src/components/messages/AssistantTextMessage.tsx",
    "references/typescript/src/components/messages/UserToolResultMessage/UserToolSuccessMessage.tsx",
    "references/typescript/src/components/messages/UserToolResultMessage/RejectedToolUseMessage.tsx",
    "references/typescript/src/components/messages/UserToolResultMessage/UserToolResultMessage.tsx",
    "references/typescript/src/components/messages/UserToolResultMessage/UserToolRejectMessage.tsx",
    "references/typescript/src/components/messages/UserToolResultMessage/RejectedPlanMessage.tsx",
    "references/typescript/src/components/messages/UserToolResultMessage/UserToolCanceledMessage.tsx",
    "references/typescript/src/components/messages/UserToolResultMessage/utils.tsx",
    "references/typescript/src/components/messages/UserToolResultMessage/UserToolErrorMessage.tsx",
    "references/typescript/src/components/messages/HighlightedThinkingText.tsx",
    "references/typescript/src/components/messages/UserBashOutputMessage.tsx",
    "references/typescript/src/components/messages/AttachmentMessage.tsx",
    "references/typescript/src/components/messages/UserAgentNotificationMessage.tsx",
    "references/typescript/src/components/messages/UserPlanMessage.tsx",
    "references/typescript/src/components/messages/teamMemCollapsed.tsx",
    "references/typescript/src/components/messages/UserTeammateMessage.tsx",
    "references/typescript/src/components/messages/GroupedToolUseContent.tsx",
    "references/typescript/src/components/messages/UserImageMessage.tsx",
    "references/typescript/src/components/messages/AssistantRedactedThinkingMessage.tsx",
    "references/typescript/src/components/messages/CompactBoundaryMessage.tsx",
    "references/typescript/src/components/messages/HookProgressMessage.tsx",
    "references/typescript/src/components/messages/SystemTextMessage.tsx",
    "references/typescript/src/components/messages/UserMemoryInputMessage.tsx",
    "references/typescript/src/components/messages/UserCommandMessage.tsx",
    "references/typescript/src/components/messages/AssistantThinkingMessage.tsx",
    "references/typescript/src/components/messages/UserBashInputMessage.tsx",
    "references/typescript/src/components/messages/SystemAPIErrorMessage.tsx",
    "references/typescript/src/components/messages/UserPromptMessage.tsx",
    "references/typescript/src/components/messages/AssistantToolUseMessage.tsx",
    "references/typescript/src/components/messages/TaskAssignmentMessage.tsx",
    "references/typescript/src/components/messages/UserChannelMessage.tsx",
    "references/typescript/src/components/messages/teamMemSaved.ts",
    "references/typescript/src/components/messages/RateLimitMessage.tsx",
    "references/typescript/src/components/messages/nullRenderingAttachments.ts",
    "references/typescript/src/components/messages/PlanApprovalMessage.tsx",
    "references/typescript/src/components/messages/AdvisorMessage.tsx",
    "references/typescript/src/components/messages/UserLocalCommandOutputMessage.tsx",
    "references/typescript/src/components/messages/UserTextMessage.tsx",
    "references/typescript/src/components/PromptInput/PromptInputFooter.tsx",
    "references/typescript/src/components/PromptInput/PromptInputModeIndicator.tsx",
    "references/typescript/src/components/PromptInput/SandboxPromptFooterHint.tsx",
    "references/typescript/src/components/PromptInput/useMaybeTruncateInput.ts",
    "references/typescript/src/components/PromptInput/Notifications.tsx",
    "references/typescript/src/components/PromptInput/HistorySearchInput.tsx",
    "references/typescript/src/components/PromptInput/PromptInputHelpMenu.tsx",
    "references/typescript/src/components/PromptInput/IssueFlagBanner.tsx",
    "references/typescript/src/components/PromptInput/useShowFastIconHint.ts",
    "references/typescript/src/components/PromptInput/useSwarmBanner.ts",
    "references/typescript/src/components/PromptInput/VoiceIndicator.tsx",
    "references/typescript/src/components/PromptInput/inputPaste.ts",
    "references/typescript/src/components/PromptInput/PromptInputStashNotice.tsx",
    "references/typescript/src/components/PromptInput/utils.ts",
    "references/typescript/src/components/PromptInput/usePromptInputPlaceholder.ts",
    "references/typescript/src/components/PromptInput/PromptInput.tsx",
    "references/typescript/src/components/PromptInput/inputModes.ts",
    "references/typescript/src/components/PromptInput/ShimmeredInput.tsx",
    "references/typescript/src/components/PromptInput/PromptInputQueuedCommands.tsx",
    "references/typescript/src/components/PromptInput/PromptInputFooterSuggestions.tsx",
    "references/typescript/src/components/PromptInput/PromptInputFooterLeftSide.tsx",
    "references/typescript/src/components/Messages.tsx",
    "references/typescript/src/components/MessageRow.tsx",
    "references/typescript/src/components/MarkdownTable.tsx",
    "references/typescript/src/components/Markdown.tsx",
]


def test_typescript_prompt_message_component_rows_have_owner_rationale() -> None:
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert "explicit shipped-owner or archive-only rationale" in text
    for row in EXPECTED_ROWS:
        assert row in text


def test_typescript_prompt_message_component_cluster_size_matches_manifest() -> None:
    assert len(EXPECTED_ROWS) == 67
