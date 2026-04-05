# TypeScript MCP, LSP, ask-user, and interaction tool ownership

The archived advanced interaction tool files in bead `aicd-3ix.4.10.1.1` are not preserved as a one-for-one shipped tool package in the current AICD TypeScript proving slice.

The shipped TypeScript pack preserves the feature taxonomy for optional advanced tool families—especially `mcp-integration`, `lsp-tooling`, `interactive-clarification-tools`, `prompt-suggestion-services`, and collaboration-oriented features—but it does not ship these archived tool implementations directly under the base runtime boundary.

## Ownership and disposition

- `references/typescript/src/tools/MCPTool/prompt.ts` — future feature-pack ownership aligned to `mcp-integration`; MCP prompting is not shipped in the current proving slice.
- `references/typescript/src/tools/MCPTool/UI.tsx` — future feature-pack ownership aligned to `mcp-integration`; MCP-specific UI is not shipped in the current proving slice.
- `references/typescript/src/tools/MCPTool/classifyForCollapse.ts` — future feature-pack ownership aligned to `mcp-integration`; MCP result-classification helpers remain outside the shipped core runtime.
- `references/typescript/src/tools/MCPTool/MCPTool.ts` — future feature-pack ownership aligned to `mcp-integration`; MCP tool execution is not shipped in the base proving slice.
- `references/typescript/src/tools/SendMessageTool/SendMessageTool.ts` — future feature-pack ownership aligned to collaboration/team-memory workflow directions; messaging tool execution is not shipped in the base proving slice.
- `references/typescript/src/tools/SendMessageTool/prompt.ts` — future feature-pack ownership aligned to collaboration/team-memory workflow directions; messaging prompts are not shipped in the base proving slice.
- `references/typescript/src/tools/SendMessageTool/UI.tsx` — future feature-pack ownership aligned to collaboration/team-memory workflow directions; messaging UI is not shipped in the base proving slice.
- `references/typescript/src/tools/SendMessageTool/constants.ts` — future feature-pack ownership aligned to collaboration/team-memory workflow directions; messaging constants remain outside the shipped core runtime.
- `references/typescript/src/tools/LSPTool/LSPTool.ts` — future feature-pack ownership aligned to `lsp-tooling`; LSP tool execution is not shipped in the base proving slice.
- `references/typescript/src/tools/LSPTool/prompt.ts` — future feature-pack ownership aligned to `lsp-tooling`; LSP prompts are not shipped in the base proving slice.
- `references/typescript/src/tools/LSPTool/UI.tsx` — future feature-pack ownership aligned to `lsp-tooling`; LSP UI is not shipped in the base proving slice.
- `references/typescript/src/tools/LSPTool/formatters.ts` — future feature-pack ownership aligned to `lsp-tooling`; LSP formatting helpers remain outside the shipped core runtime.
- `references/typescript/src/tools/LSPTool/symbolContext.ts` — future feature-pack ownership aligned to `lsp-tooling`; symbol-context helpers remain outside the shipped core runtime.
- `references/typescript/src/tools/LSPTool/schemas.ts` — future feature-pack ownership aligned to `lsp-tooling`; LSP schema helpers remain outside the shipped core runtime.
- `references/typescript/src/tools/ReadMcpResourceTool/prompt.ts` — future feature-pack ownership aligned to `mcp-integration`; MCP resource-read prompting is not shipped in the base proving slice.
- `references/typescript/src/tools/ReadMcpResourceTool/ReadMcpResourceTool.ts` — future feature-pack ownership aligned to `mcp-integration`; MCP resource-read execution is not shipped in the base proving slice.
- `references/typescript/src/tools/ReadMcpResourceTool/UI.tsx` — future feature-pack ownership aligned to `mcp-integration`; MCP resource-read UI is not shipped in the base proving slice.
- `references/typescript/src/tools/ListMcpResourcesTool/prompt.ts` — future feature-pack ownership aligned to `mcp-integration`; MCP resource-list prompting is not shipped in the base proving slice.
- `references/typescript/src/tools/ListMcpResourcesTool/UI.tsx` — future feature-pack ownership aligned to `mcp-integration`; MCP resource-list UI is not shipped in the base proving slice.
- `references/typescript/src/tools/ListMcpResourcesTool/ListMcpResourcesTool.ts` — future feature-pack ownership aligned to `mcp-integration`; MCP resource-list execution is not shipped in the base proving slice.
- `references/typescript/src/tools/McpAuthTool/McpAuthTool.ts` — future feature-pack ownership aligned to `mcp-integration`; MCP auth handling is not shipped in the base proving slice.
- `references/typescript/src/tools/AskUserQuestionTool/prompt.ts` — future feature-pack ownership aligned to `interactive-clarification-tools`; ask-user prompting is not shipped in the base proving slice.
- `references/typescript/src/tools/AskUserQuestionTool/AskUserQuestionTool.tsx` — future feature-pack ownership aligned to `interactive-clarification-tools`; ask-user UI/tool execution is not shipped in the base proving slice.
- `references/typescript/src/tools/BriefTool/BriefTool.ts` — future feature-pack ownership aligned to prompt-suggestion or advanced interaction directions; briefing helpers are not shipped in the base proving slice.
- `references/typescript/src/tools/BriefTool/prompt.ts` — future feature-pack ownership aligned to `prompt-suggestion-services`; briefing prompts are not shipped in the base proving slice.
- `references/typescript/src/tools/BriefTool/UI.tsx` — future feature-pack ownership aligned to `prompt-suggestion-services`; briefing UI is not shipped in the base proving slice.
- `references/typescript/src/tools/BriefTool/attachments.ts` — future feature-pack ownership aligned to `prompt-suggestion-services`; briefing attachment helpers remain outside the shipped core runtime.
- `references/typescript/src/tools/BriefTool/upload.ts` — future feature-pack ownership aligned to `prompt-suggestion-services`; briefing upload helpers remain outside the shipped core runtime.

## Shipped-language-pack rule

This subset is complete when each archived MCP/LSP/interaction tool row has a clear future feature-pack owner or an explicit archive-only rationale. These archived advanced tools must not be mistaken for already-shipped TypeScript runtime ownership just because the repository preserves their feature-pack taxonomy.
