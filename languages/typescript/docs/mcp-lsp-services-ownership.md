# TypeScript MCP and LSP services ownership

The archived MCP and LSP service files in bead `aicd-3ix.4.11.1.2` are not preserved as a one-for-one shipped services subtree in the current AICD TypeScript proving slice.

The current TypeScript pack preserves the feature taxonomy for optional `mcp-integration` and `lsp-tooling` work, but it does not ship these archived service implementations directly under the base runtime boundary. Their final disposition is therefore future feature-pack ownership, not shipped core runtime ownership.

## Ownership and disposition

- `references/typescript/src/services/mcp/xaaIdpLogin.ts` — future feature-pack ownership aligned to `mcp-integration`; archived MCP/XAA login handling is not shipped in the base proving slice.
- `references/typescript/src/services/mcp/headersHelper.ts` — future feature-pack ownership aligned to `mcp-integration`; MCP header helpers are not shipped in the base proving slice.
- `references/typescript/src/services/mcp/channelNotification.ts` — future feature-pack ownership aligned to `mcp-integration`; channel notification handling is not shipped in the base proving slice.
- `references/typescript/src/services/mcp/xaa.ts` — future feature-pack ownership aligned to `mcp-integration`; archived MCP/XAA handling is not shipped in the base proving slice.
- `references/typescript/src/services/mcp/config.ts` — future feature-pack ownership aligned to `mcp-integration`; MCP config handling is not shipped in the base proving slice.
- `references/typescript/src/services/mcp/InProcessTransport.ts` — future feature-pack ownership aligned to `mcp-integration`; in-process MCP transport is not shipped in the base proving slice.
- `references/typescript/src/services/mcp/client.ts` — future feature-pack ownership aligned to `mcp-integration`; archived MCP client handling is not shipped in the base proving slice.
- `references/typescript/src/services/mcp/normalization.ts` — future feature-pack ownership aligned to `mcp-integration`; MCP normalization helpers are not shipped in the base proving slice.
- `references/typescript/src/services/mcp/envExpansion.ts` — future feature-pack ownership aligned to `mcp-integration`; MCP environment expansion is not shipped in the base proving slice.
- `references/typescript/src/services/mcp/vscodeSdkMcp.ts` — future feature-pack ownership aligned to `mcp-integration`; VS Code SDK MCP support is not shipped in the base proving slice.
- `references/typescript/src/services/mcp/useManageMCPConnections.ts` — future feature-pack ownership aligned to `mcp-integration`; MCP connection management UI/service behavior is not shipped in the base proving slice.
- `references/typescript/src/services/mcp/SdkControlTransport.ts` — future feature-pack ownership aligned to `mcp-integration`; SDK control transport support is not shipped in the base proving slice.
- `references/typescript/src/services/mcp/elicitationHandler.ts` — future feature-pack ownership aligned to `mcp-integration`; MCP elicitation handling is not shipped in the base proving slice.
- `references/typescript/src/services/mcp/channelPermissions.ts` — future feature-pack ownership aligned to `mcp-integration`; MCP channel permissions are not shipped in the base proving slice.
- `references/typescript/src/services/mcp/types.ts` — future feature-pack ownership aligned to `mcp-integration`; MCP service types are not preserved one-for-one in the base proving slice.
- `references/typescript/src/services/mcp/claudeai.ts` — future feature-pack ownership aligned to `mcp-integration`; archived MCP/provider integration is not shipped in the base proving slice.
- `references/typescript/src/services/mcp/utils.ts` — future feature-pack ownership aligned to `mcp-integration`; MCP helpers are not shipped in the base proving slice.
- `references/typescript/src/services/mcp/officialRegistry.ts` — future feature-pack ownership aligned to `mcp-integration`; MCP registry helpers are not shipped in the base proving slice.
- `references/typescript/src/services/mcp/auth.ts` — future feature-pack ownership aligned to `mcp-integration`; MCP auth handling is not shipped in the base proving slice.
- `references/typescript/src/services/mcp/oauthPort.ts` — future feature-pack ownership aligned to `mcp-integration`; MCP OAuth port handling is not shipped in the base proving slice.
- `references/typescript/src/services/mcp/MCPConnectionManager.tsx` — future feature-pack ownership aligned to `mcp-integration`; archived MCP connection manager UI is not shipped in the base proving slice.
- `references/typescript/src/services/mcp/mcpStringUtils.ts` — future feature-pack ownership aligned to `mcp-integration`; MCP string helpers are not shipped in the base proving slice.
- `references/typescript/src/services/mcp/channelAllowlist.ts` — future feature-pack ownership aligned to `mcp-integration`; MCP channel allowlisting is not shipped in the base proving slice.
- `references/typescript/src/services/lsp/LSPDiagnosticRegistry.ts` — future feature-pack ownership aligned to `lsp-tooling`; LSP diagnostic registry support is not shipped in the base proving slice.
- `references/typescript/src/services/lsp/config.ts` — future feature-pack ownership aligned to `lsp-tooling`; LSP config handling is not shipped in the base proving slice.
- `references/typescript/src/services/lsp/LSPServerManager.ts` — future feature-pack ownership aligned to `lsp-tooling`; LSP server management is not shipped in the base proving slice.
- `references/typescript/src/services/lsp/passiveFeedback.ts` — future feature-pack ownership aligned to `lsp-tooling`; passive LSP feedback support is not shipped in the base proving slice.
- `references/typescript/src/services/lsp/LSPServerInstance.ts` — future feature-pack ownership aligned to `lsp-tooling`; LSP server instance handling is not shipped in the base proving slice.
- `references/typescript/src/services/lsp/manager.ts` — future feature-pack ownership aligned to `lsp-tooling`; LSP manager wiring is not shipped in the base proving slice.
- `references/typescript/src/services/lsp/LSPClient.ts` — future feature-pack ownership aligned to `lsp-tooling`; LSP client handling is not shipped in the base proving slice.

## Shipped-language-pack rule

This subset is complete when each archived MCP/LSP service row has a clear future feature-pack owner or an explicit archive-only rationale. These archived service files must not be mistaken for already-shipped TypeScript runtime ownership just because the repository preserves their feature-pack taxonomy.
