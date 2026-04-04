---
name: use-mcp-integration
description: Use the optional MCP integration feature-pack workflow for TypeScript generated projects.
whenToUse: Use when the user needs MCP-backed resources, authentication, or tool invocation.
allowed-tools: [mcp, list_mcp_resources, read_mcp_resource, mcp_auth, file_read]
argument-hint: <mcp-task>
---

1. Confirm which MCP capability is needed before invoking tools.
2. Prefer resource listing and targeted resource reads before broad MCP tool calls.
3. Keep MCP usage scoped to the requested integration task.
