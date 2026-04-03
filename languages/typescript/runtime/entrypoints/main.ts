import { readFileSync, existsSync } from 'node:fs'
import { join } from 'node:path'

import { loadContextState } from '../context/loadContextState.ts'
import { loadLatestSessionState, loadUsageState } from '../state/sessionState.ts'

function readText(path: string): string {
  return readFileSync(path, 'utf8').trim()
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

function buildToolResults(root: string, approvalMode: string, deny: string[]): string {
  const usagePath = join(root, '.agent/usage/runtime-tool-smoke.txt')
  const bashPolicy = policyForOperation(approvalMode, deny, 'bash', 'bash')
  const fileReadResult = checksum([readText(join(root, '.agent/context/README.md'))])
  const fileWritePolicy = policyForOperation(approvalMode, deny, 'file_write', 'file_write')
  const fileEditPolicy = policyForOperation(approvalMode, deny, 'file_edit', 'file_edit')
  const usageExists = existsSync(usagePath)
  const usageText = usageExists ? readText(usagePath) : ''

  const fileWriteResult =
    fileWritePolicy === 'file_write=allowed' && usageExists ? 'tool-write-ok' : fileWritePolicy
  const fileEditResult =
    fileEditPolicy === 'file_edit=allowed' && usageExists ? usageText : fileEditPolicy

  return `bash_result=${bashPolicy} file_read_result=${fileReadResult} file_write_result=${fileWriteResult} file_edit_result=${fileEditResult} web_fetch_result=tool-web-fetch`
}

export function buildEntrypointSummary(root: string): string {
  const configText = readText(join(root, 'boilerplate.config.ts'))
  const projectName = extractString(configText, /name:\s*'([^']+)'/)
  const defaultProvider = extractString(configText, /defaultProvider:\s*'([^']+)'/)
  const providerModel = extractString(
    configText,
    new RegExp(`${defaultProvider}:\\s*{[\\s\\S]*?model:\\s*'([^']+)'`),
  )
  const approvalMode = extractString(configText, /approvalMode:\s*'([^']+)'/)
  const deny = extractStringList(configText, /deny:\s*\[([\s\S]*?)\]/)
  const context = loadContextState(root)
  const latest = loadLatestSessionState(root)
  const usage = loadUsageState(root)
  const bashPolicy = policyForOperation(approvalMode, deny, 'bash', 'bash')
  const fileWritePolicy = policyForOperation(approvalMode, deny, 'file_write', 'file_write')
  const toolResults = buildToolResults(root, approvalMode, deny)

  return `${projectName} session loop completed provider=${defaultProvider} model=${providerModel} prompt_digest=${context.promptDigest} context_digest=${context.contextDigest} approval_mode=${approvalMode} bash_policy=${bashPolicy} file_write_policy=${fileWritePolicy} ${toolResults} session_id=${latest.sessionId} turn_count=${latest.turnCount} export_path=.agent/sessions/local-main-session.export.md usage_entries=${usage.usageEntries} total_cost_micros=${usage.totalCostMicros}`
}
