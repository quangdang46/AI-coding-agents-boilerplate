import { loadRuntimeConfig } from '../utils/config.ts'
import { runCoreTool } from '../utils/toolExecution.ts'

export type CoreToolName =
  | 'bash'
  | 'file_read'
  | 'file_write'
  | 'file_edit'
  | 'web_fetch'

export type CoreToolHandler = (root: string) => string

export type CoreToolDefinition = {
  name: CoreToolName
  run: CoreToolHandler
}

function runBash(root: string): string {
  const config = loadRuntimeConfig(root)
  return runCoreTool(root, config, 'bash')
}

function runFileRead(root: string): string {
  const config = loadRuntimeConfig(root)
  return runCoreTool(root, config, 'file_read')
}

function runFileWrite(root: string): string {
  const config = loadRuntimeConfig(root)
  return runCoreTool(root, config, 'file_write')
}

function runFileEdit(root: string): string {
  const config = loadRuntimeConfig(root)
  return runCoreTool(root, config, 'file_edit')
}

function runWebFetch(root: string): string {
  const config = loadRuntimeConfig(root)
  return runCoreTool(root, config, 'web_fetch')
}

export const coreTools: CoreToolDefinition[] = [
  { name: 'bash', run: runBash },
  { name: 'file_read', run: runFileRead },
  { name: 'file_write', run: runFileWrite },
  { name: 'file_edit', run: runFileEdit },
  { name: 'web_fetch', run: runWebFetch },
]

export function getCoreToolRegistry(): Record<CoreToolName, CoreToolHandler> {
  return Object.fromEntries(coreTools.map((tool) => [tool.name, tool.run])) as Record<
    CoreToolName,
    CoreToolHandler
  >
}
