import type { Tool, ToolPermissionContext, Tools } from '../../Tool.js'
import { filterToolsByDenyRules } from './toolHelpers.js'
import { excludeToolsByName, hideReplOnlyToolsWhenReplEnabled, keepEnabledTools } from './toolFiltering.js'

export function buildStandardModeToolSet(args: {
  allBaseTools: Tools
  permissionContext: ToolPermissionContext
  specialToolNames: ReadonlySet<string>
  replModeEnabled: boolean
  replToolName: string
  replOnlyTools: ReadonlySet<string>
  matchesName: (tool: Tool, toolName: string) => boolean
}): Tools {
  const {
    allBaseTools,
    permissionContext,
    specialToolNames,
    replModeEnabled,
    replToolName,
    replOnlyTools,
    matchesName,
  } = args

  const tools = excludeToolsByName(allBaseTools, specialToolNames)
  let allowedTools = filterToolsByDenyRules(tools, permissionContext)

  if (replModeEnabled) {
    allowedTools = hideReplOnlyToolsWhenReplEnabled(
      allowedTools,
      replToolName,
      replOnlyTools,
      matchesName,
    )
  }

  return keepEnabledTools(allowedTools)
}
