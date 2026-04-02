import type { Tool, ToolPermissionContext, Tools } from '../../Tool.js'

export function buildBaseToolCatalog(parts: {
  always: Tool[]
  search: Tool[]
  optional: Array<Tool | null | false | undefined>
}): Tools {
  return [
    ...parts.always,
    ...parts.search,
    ...parts.optional.filter(Boolean),
  ] as Tools
}

export function buildSimpleModeToolSet(args: {
  replTool: Tool | null
  taskStopTool: Tool
  sendMessageTool: Tool
  bashTool: Tool
  fileReadTool: Tool
  fileEditTool: Tool
  replModeEnabled: boolean
  coordinatorModeEnabled: boolean
  permissionContext: ToolPermissionContext
  filterToolsByDenyRules: <T extends { name: string; mcpInfo?: { serverName: string; toolName: string } }>(tools: readonly T[], permissionContext: ToolPermissionContext) => T[]
}): Tools {
  const {
    replTool,
    taskStopTool,
    sendMessageTool,
    bashTool,
    fileReadTool,
    fileEditTool,
    replModeEnabled,
    coordinatorModeEnabled,
    permissionContext,
    filterToolsByDenyRules,
  } = args

  if (replModeEnabled && replTool) {
    const replSimple: Tool[] = [replTool]
    if (coordinatorModeEnabled) {
      replSimple.push(taskStopTool, sendMessageTool)
    }
    return filterToolsByDenyRules(replSimple, permissionContext)
  }

  const simpleTools: Tool[] = [bashTool, fileReadTool, fileEditTool]
  if (coordinatorModeEnabled) {
    simpleTools.push(taskStopTool, sendMessageTool)
  }
  return filterToolsByDenyRules(simpleTools, permissionContext)
}
