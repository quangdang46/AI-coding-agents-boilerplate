import { existsSync, readFileSync } from 'node:fs'
import { join } from 'node:path'

import { loadRuntimeSummary } from '../config/loadRuntimeSummary.ts'
import { runSessionLoop } from '../engine/sessionLoop.ts'

export type CoreCommandName =
  | 'status'
  | 'session'
  | 'export'
  | 'config'
  | 'doctor'
  | 'context'
  | 'usage'
  | 'permissions'
  | 'files'
  | 'tasks'

export type CoreCommandHandler = (root: string) => string

export type CoreCommandDefinition = {
  name: CoreCommandName
  run: CoreCommandHandler
}

function readText(path: string): string {
  return readFileSync(path, 'utf8').trim()
}

function readState(path: string): Record<string, string> {
  if (!existsSync(path)) {
    return {}
  }
  return Object.fromEntries(
    readText(path)
      .split('\n')
      .filter((line) => line.includes('='))
      .map((line) => {
        const [key, ...rest] = line.split('=')
        return [key, rest.join('=')]
      }),
  )
}

function extractString(source: string, pattern: RegExp): string {
  const match = source.match(pattern)
  if (!match) {
    throw new Error(`missing config pattern: ${pattern.source}`)
  }
  return match[1]
}

function configSummary(root: string): string {
  const configText = readText(join(root, 'boilerplate.config.ts'))
  const defaultProvider = extractString(configText, /defaultProvider:\s*'([^']+)'/)
  const providerModel = extractString(
    configText,
    new RegExp(`${defaultProvider}:\\s*{[\\s\\S]*?model:\\s*'([^']+)'`),
  )
  const approvalMode = extractString(configText, /approvalMode:\s*'([^']+)'/)
  return `provider=${defaultProvider} model=${providerModel} approval_mode=${approvalMode}`
}

function sessionSummary(root: string): string {
  const latest = readState(join(root, '.agent/sessions/latest.state'))
  return `session_id=${latest.session_id ?? 'missing'} turn_count=${latest.turn_count ?? '0'}`
}

function exportSummary(root: string): string {
  const exportPath = join(root, '.agent/sessions/local-main-session.export.md')
  const exists = existsSync(exportPath)
  return `export_path=.agent/sessions/local-main-session.export.md export_exists=${exists}`
}

function contextSummary(root: string): string {
  const summary = loadRuntimeSummary(root)
  return `context_digest=${summary.contextDigest}`
}

function usageSummary(root: string): string {
  const usage = readState(join(root, '.agent/usage/summary.state'))
  return `usage_entries=${usage.usage_entries ?? '0'} total_cost_micros=${usage.total_cost_micros ?? '0'}`
}

function permissionSummary(root: string): string {
  const summary = loadRuntimeSummary(root)
  return `approval_mode=${summary.approvalMode} bash_policy=${summary.bashPolicy} file_write_policy=${summary.fileWritePolicy}`
}

function fileSummary(root: string): string {
  const sessionState = existsSync(join(root, '.agent/sessions/local-main-session.state'))
  const exportState = existsSync(join(root, '.agent/sessions/local-main-session.export.md'))
  const usageState = existsSync(join(root, '.agent/usage/summary.state'))
  return `session_state=${sessionState} export_state=${exportState} usage_state=${usageState}`
}

function taskSummary(root: string): string {
  const latest = readState(join(root, '.agent/sessions/latest.state'))
  const turnCount = latest.turn_count ?? '0'
  return `task_count=1 active_task=session-loop turn_count=${turnCount}`
}

function doctorSummary(root: string): string {
  const required = [
    '.agent/sessions/README.md',
    '.agent/usage/README.md',
    '.agent/context/README.md',
    '.agent/sessions/latest.state',
    '.agent/usage/summary.state',
  ]
  const missing = required.filter((relative) => !existsSync(join(root, relative)))
  return missing.length === 0 ? 'doctor=ok' : `doctor=missing:${missing.join(',')}`
}

export const coreCommands: CoreCommandDefinition[] = [
  { name: 'status', run: (root) => runSessionLoop(root) },
  { name: 'session', run: sessionSummary },
  { name: 'export', run: exportSummary },
  { name: 'config', run: configSummary },
  { name: 'doctor', run: doctorSummary },
  { name: 'context', run: contextSummary },
  { name: 'usage', run: usageSummary },
  { name: 'permissions', run: permissionSummary },
  { name: 'files', run: fileSummary },
  { name: 'tasks', run: taskSummary },
]

export function getCoreCommandRegistry(): Record<CoreCommandName, CoreCommandHandler> {
  return Object.fromEntries(coreCommands.map((command) => [command.name, command.run])) as Record<
    CoreCommandName,
    CoreCommandHandler
  >
}
