import type { Tool, Tools } from '../../Tool.js'

export function excludeToolsByName<T extends { name: string }>(
  tools: readonly T[],
  excludedNames: ReadonlySet<string>,
): T[] {
  return tools.filter(tool => !excludedNames.has(tool.name))
}

export function keepEnabledTools<T extends { isEnabled(): boolean }>(
  tools: readonly T[],
): T[] {
  const isEnabled = tools.map(tool => tool.isEnabled())
  return tools.filter((_, i) => isEnabled[i])
}

export function hideReplOnlyToolsWhenReplEnabled(
  tools: Tools,
  replToolName: string,
  replOnlyTools: ReadonlySet<string>,
  matchesName: (tool: Tool, toolName: string) => boolean,
): Tools {
  const replEnabled = tools.some(tool => matchesName(tool, replToolName))
  if (!replEnabled) {
    return [...tools]
  }
  return tools.filter(tool => !replOnlyTools.has(tool.name))
}
