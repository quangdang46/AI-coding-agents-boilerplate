import { execFileSync } from 'node:child_process'
import { existsSync, writeFileSync } from 'node:fs'
import { join } from 'node:path'

import { loadRuntimeConfig } from './config.ts'
import { readText } from './files.ts'
import { inferBrandRoot } from './brand.ts'
import { checksum } from './text.ts'
import { policyForOperation } from './policy.ts'

export type ToolExecutionConfig = {
  enabledTools: string[]
  approvalMode: string
  deny: string[]
  bashTimeoutMs: number
}

export type CoreToolExecutionName =
  | 'bash'
  | 'file_read'
  | 'file_write'
  | 'file_edit'
  | 'web_fetch'

function usagePath(root: string): string {
  return join(inferBrandRoot(root), 'sessions/runtime-tool-smoke.txt')
}

function status(config: ToolExecutionConfig, toolName: string, operation: string): string {
  if (!config.enabledTools.includes(toolName)) {
    return `${operation}=disabled`
  }
  return policyForOperation(config.approvalMode, config.deny, operation, toolName)
}

export function runCoreTool(
  root: string,
  config: ToolExecutionConfig,
  toolName: CoreToolExecutionName,
): string {
  const toolUsagePath = usagePath(root)

  if (toolName === 'bash') {
    const bashStatus = status(config, 'bash', 'bash')
    if (bashStatus !== 'bash=allowed') {
      return bashStatus
    }
    return execFileSync('bash', ['-lc', 'printf tool-bash-ok'], {
      cwd: root,
      encoding: 'utf8',
      timeout: config.bashTimeoutMs,
    }).trim()
  }

  if (toolName === 'file_read') {
    const fileReadStatus = status(config, 'file_read', 'file_read')
    if (fileReadStatus !== 'file_read=allowed') {
      return fileReadStatus
    }
    const { contextPaths } = loadRuntimeConfig(root)
    return checksum(
      contextPaths.map((path) => join(root, path)).filter(existsSync).map(readText),
    )
  }

  if (toolName === 'file_write') {
    const fileWriteStatus = status(config, 'file_write', 'file_write')
    if (fileWriteStatus !== 'file_write=allowed') {
      return fileWriteStatus
    }
    writeFileSync(toolUsagePath, 'tool-write-ok', 'utf8')
    return 'tool-write-ok'
  }

  if (toolName === 'file_edit') {
    const fileEditStatus = status(config, 'file_edit', 'file_edit')
    if (fileEditStatus !== 'file_edit=allowed') {
      return fileEditStatus
    }
    if (!existsSync(toolUsagePath)) {
      writeFileSync(toolUsagePath, 'tool-write-ok', 'utf8')
    }
    const edited = `${readText(toolUsagePath)} edited`
    writeFileSync(toolUsagePath, edited, 'utf8')
    return edited
  }

  const webFetchStatus = status(config, 'web_fetch', 'web_fetch')
  if (webFetchStatus !== 'web_fetch=allowed') {
    return webFetchStatus
  }
  return 'tool-web-fetch'
}

export function runCoreTools(root: string, config: ToolExecutionConfig): string {
  return [
    `bash_result=${runCoreTool(root, config, 'bash')}`,
    `file_read_result=${runCoreTool(root, config, 'file_read')}`,
    `file_write_result=${runCoreTool(root, config, 'file_write')}`,
    `file_edit_result=${runCoreTool(root, config, 'file_edit')}`,
    `web_fetch_result=${runCoreTool(root, config, 'web_fetch')}`,
  ].join(' ')
}

export function readCoreToolResults(root: string, approvalMode: string, deny: string[]): string {
  const toolUsagePath = usagePath(root)
  const { contextPaths } = loadRuntimeConfig(root)
  const bashPolicy = policyForOperation(approvalMode, deny, 'bash', 'bash')
  const fileReadResult = checksum(
    contextPaths.map((path) => join(root, path)).filter(existsSync).map(readText),
  )
  const fileWritePolicy = policyForOperation(approvalMode, deny, 'file_write', 'file_write')
  const fileEditPolicy = policyForOperation(approvalMode, deny, 'file_edit', 'file_edit')
  const usageExists = existsSync(toolUsagePath)
  const usageText = usageExists ? readText(toolUsagePath) : ''

  const fileWriteResult =
    fileWritePolicy === 'file_write=allowed' && usageExists ? 'tool-write-ok' : fileWritePolicy
  const fileEditResult =
    fileEditPolicy === 'file_edit=allowed' && usageExists ? usageText : fileEditPolicy

  return `bash_result=${bashPolicy} file_read_result=${fileReadResult} file_write_result=${fileWriteResult} file_edit_result=${fileEditResult} web_fetch_result=tool-web-fetch`
}
