from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT
    / "languages"
    / "typescript"
    / "docs"
    / "mcp-lsp-interaction-tools-ownership.md"
)

EXPECTED_ROWS = [
    "references/typescript/src/tools/MCPTool/prompt.ts",
    "references/typescript/src/tools/MCPTool/UI.tsx",
    "references/typescript/src/tools/MCPTool/classifyForCollapse.ts",
    "references/typescript/src/tools/MCPTool/MCPTool.ts",
    "references/typescript/src/tools/SendMessageTool/SendMessageTool.ts",
    "references/typescript/src/tools/SendMessageTool/prompt.ts",
    "references/typescript/src/tools/SendMessageTool/UI.tsx",
    "references/typescript/src/tools/SendMessageTool/constants.ts",
    "references/typescript/src/tools/LSPTool/LSPTool.ts",
    "references/typescript/src/tools/LSPTool/prompt.ts",
    "references/typescript/src/tools/LSPTool/UI.tsx",
    "references/typescript/src/tools/LSPTool/formatters.ts",
    "references/typescript/src/tools/LSPTool/symbolContext.ts",
    "references/typescript/src/tools/LSPTool/schemas.ts",
    "references/typescript/src/tools/ReadMcpResourceTool/prompt.ts",
    "references/typescript/src/tools/ReadMcpResourceTool/ReadMcpResourceTool.ts",
    "references/typescript/src/tools/ReadMcpResourceTool/UI.tsx",
    "references/typescript/src/tools/ListMcpResourcesTool/prompt.ts",
    "references/typescript/src/tools/ListMcpResourcesTool/UI.tsx",
    "references/typescript/src/tools/ListMcpResourcesTool/ListMcpResourcesTool.ts",
    "references/typescript/src/tools/McpAuthTool/McpAuthTool.ts",
    "references/typescript/src/tools/AskUserQuestionTool/prompt.ts",
    "references/typescript/src/tools/AskUserQuestionTool/AskUserQuestionTool.tsx",
    "references/typescript/src/tools/BriefTool/BriefTool.ts",
    "references/typescript/src/tools/BriefTool/prompt.ts",
    "references/typescript/src/tools/BriefTool/UI.tsx",
    "references/typescript/src/tools/BriefTool/attachments.ts",
    "references/typescript/src/tools/BriefTool/upload.ts",
]


def test_typescript_mcp_lsp_interaction_rows_have_feature_pack_or_archive_rationale() -> (
    None
):
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert (
        "clear future feature-pack owner or an explicit archive-only rationale" in text
    )
    for row in EXPECTED_ROWS:
        assert row in text


def test_typescript_mcp_lsp_interaction_cluster_size_matches_manifest() -> None:
    assert len(EXPECTED_ROWS) == 28
