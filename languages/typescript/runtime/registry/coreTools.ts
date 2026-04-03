import { execFileSync } from 'node:child_process'
import { existsSync, readFileSync, writeFileSync } from 'node:fs'
import { join } from 'node:path'

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

function readText(path: string): string {
  return readFileSync(path, 'utf8').trim()
}

function checksum(parts: string[]): string {
  let total = 0
  for (const part of parts) {
    for (const char of part) {
      total = (total * 31 + char.charCodeAt(0)) % 0x7fffffff
    }
    total = (total * 31 + 1) % 0x7fffffff
  }
  return total.toString(16).padStart(8, '0')
}

function extractString(source: string, pattern: RegExp): string {
  const match = source.match(pattern)
  if (!match) {
    throw new Error(`missing config pattern: ${pattern.source}`)
  }
  return match[1]
}

function extractStringList(source: string, pattern: RegExp): string[] {
  const match = source.match(pattern)
  if (!match) {
    throw new Error(`missing config list pattern: ${pattern.source}`)
  }
  return [...match[1].matchAll(/'([^']+)'/g)].map((entry) => entry[1])
}

function readToolConfig(root: string): {
  enabledTools: string[]
  approvalMode: string
  deny: string[]
  bashTimeoutMs: number
} {
  const configText = readText(join(root, 'boilerplate.config.ts'))
  return {
    enabledTools: extractStringList(configText, /enabled:\s*\[([\s\S]*?)\]/),
    approvalMode: extractString(configText, /approvalMode:\s*'([^']+)'/),
    deny: extractStringList(configText, /deny:\s*\[([\s\S]*?)\]/),
    bashTimeoutMs: Number(extractString(configText, /bashTimeoutMs:\s*(\d+)/)),
  }
}

function policyForOperation(
  approvalMode: string,
  deny: string[],
  operation: string,
  toolName: string,
): string {
  if (deny.includes(toolName)) {
    return `${operation}=denied`
  }
  if (approvalMode === 'never') {
    return `${operation}=blocked`
  }
  if (approvalMode === 'default' && ['bash', 'file_edit', 'file_write'].includes(toolName)) {
    return `${operation}=approval-required`
  }
  return `${operation}=allowed`
}

function toolStatus(root: string, toolName: CoreToolName, operation: string): string {
  const { enabledTools, approvalMode, deny } = readToolConfig(root)
  if (!enabledTools.includes(toolName)) {
    return `${operation}=disabled`
  }
  return policyForOperation(approvalMode, deny, operation, toolName)
}

function usagePath(root: string): string {
  return join(root, '.agent/usage/runtime-tool-smoke.txt')
}

function runBash(root: string): string {
  const status = toolStatus(root, 'bash', 'bash')
  if (status !== 'bash=allowed') {
    return status
  }
  const { bashTimeoutMs } = readToolConfig(root)
  return execFileSync('bash', ['-lc', 'printf tool-bash-ok'], {
    cwd: root,
    encoding: 'utf8',
    timeout: bashTimeoutMs,
  }).trim()
}

function runFileRead(root: string): string {
  const status = toolStatus(root, 'file_read', 'file_read')
  if (status !== 'file_read=allowed') {
    return status
  }
  return checksum([readText(join(root, '.agent/context/README.md'))])
}

function runFileWrite(root: string): string {
  const status = toolStatus(root, 'file_write', 'file_write')
  if (status !== 'file_write=allowed') {
    return status
  }
  writeFileSync(usagePath(root), 'tool-write-ok', 'utf8')
  return 'tool-write-ok'
}

function runFileEdit(root: string): string {
  const status = toolStatus(root, 'file_edit', 'file_edit')
  if (status !== 'file_edit=allowed') {
    return status
  }
  const path = usagePath(root)
  if (!existsSync(path)) {
    writeFileSync(path, 'tool-write-ok', 'utf8')
  }
  const edited = `${readText(path)} edited`
  writeFileSync(path, edited, 'utf8')
  return edited
}

function runWebFetch(root: string): string {
  const status = toolStatus(root, 'web_fetch', 'web_fetch')
  if (status !== 'web_fetch=allowed') {
    return status
  }
  return 'tool-web-fetch'
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
