import uniqBy from 'lodash-es/uniqBy.js'
import type { Tool, ToolPermissionContext, Tools } from '../../Tool.js'
import { getDenyRuleForTool } from '../../utils/permissions/permissions.js'

export const TOOL_PRESETS = ['default'] as const

export type ToolPreset = (typeof TOOL_PRESETS)[number]

export function parseToolPreset(preset: string): ToolPreset | null {
  const presetString = preset.toLowerCase()
  if (!TOOL_PRESETS.includes(presetString as ToolPreset)) {
    return null
  }
  return presetString as ToolPreset
}

export function getEnabledToolNames<T extends { name: string; isEnabled(): boolean }>(
  tools: readonly T[],
): string[] {
  const isEnabled = tools.map(tool => tool.isEnabled())
  return tools.filter((_, i) => isEnabled[i]).map(tool => tool.name)
}

export function filterToolsByDenyRules<
  T extends {
    name: string
    mcpInfo?: { serverName: string; toolName: string }
  },
>(tools: readonly T[], permissionContext: ToolPermissionContext): T[] {
  return tools.filter(tool => !getDenyRuleForTool(permissionContext, tool))
}

export function assembleToolPoolFromLists(
  permissionContext: ToolPermissionContext,
  builtInTools: Tools,
  mcpTools: Tools,
): Tools {
  const allowedMcpTools = filterToolsByDenyRules(mcpTools, permissionContext)
  const byName = (a: Tool, b: Tool) => a.name.localeCompare(b.name)
  return uniqBy(
    [...builtInTools].sort(byName).concat(allowedMcpTools.sort(byName)),
    'name',
  )
}

export function getMergedToolsFromLists(
  builtInTools: Tools,
  mcpTools: Tools,
): Tools {
  return [...builtInTools, ...mcpTools]
}
