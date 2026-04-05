from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT / "languages" / "typescript" / "docs" / "mcp-lsp-services-ownership.md"
)

EXPECTED_ROWS = [
    "references/typescript/src/services/mcp/xaaIdpLogin.ts",
    "references/typescript/src/services/mcp/headersHelper.ts",
    "references/typescript/src/services/mcp/channelNotification.ts",
    "references/typescript/src/services/mcp/xaa.ts",
    "references/typescript/src/services/mcp/config.ts",
    "references/typescript/src/services/mcp/InProcessTransport.ts",
    "references/typescript/src/services/mcp/client.ts",
    "references/typescript/src/services/mcp/normalization.ts",
    "references/typescript/src/services/mcp/envExpansion.ts",
    "references/typescript/src/services/mcp/vscodeSdkMcp.ts",
    "references/typescript/src/services/mcp/useManageMCPConnections.ts",
    "references/typescript/src/services/mcp/SdkControlTransport.ts",
    "references/typescript/src/services/mcp/elicitationHandler.ts",
    "references/typescript/src/services/mcp/channelPermissions.ts",
    "references/typescript/src/services/mcp/types.ts",
    "references/typescript/src/services/mcp/claudeai.ts",
    "references/typescript/src/services/mcp/utils.ts",
    "references/typescript/src/services/mcp/officialRegistry.ts",
    "references/typescript/src/services/mcp/auth.ts",
    "references/typescript/src/services/mcp/oauthPort.ts",
    "references/typescript/src/services/mcp/MCPConnectionManager.tsx",
    "references/typescript/src/services/mcp/mcpStringUtils.ts",
    "references/typescript/src/services/mcp/channelAllowlist.ts",
    "references/typescript/src/services/lsp/LSPDiagnosticRegistry.ts",
    "references/typescript/src/services/lsp/config.ts",
    "references/typescript/src/services/lsp/LSPServerManager.ts",
    "references/typescript/src/services/lsp/passiveFeedback.ts",
    "references/typescript/src/services/lsp/LSPServerInstance.ts",
    "references/typescript/src/services/lsp/manager.ts",
    "references/typescript/src/services/lsp/LSPClient.ts",
]


def test_typescript_mcp_lsp_service_rows_have_owner_rationale() -> None:
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert (
        "clear future feature-pack owner or an explicit archive-only rationale" in text
    )
    for row in EXPECTED_ROWS:
        assert row in text


def test_typescript_mcp_lsp_service_cluster_size_matches_manifest() -> None:
    assert len(EXPECTED_ROWS) == 30
