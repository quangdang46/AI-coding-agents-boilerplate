import { existsSync, readFileSync } from 'node:fs'
import { join } from 'node:path'

import { loadRuntimeSummary } from '../config/loadRuntimeSummary.ts'
import { runSessionLoop } from '../engine/sessionLoop.ts'
import { inferBrandRoot, instructionCandidates } from '../utils/brand.ts'

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
  const latest = readState(join(inferBrandRoot(root), 'sessions/latest.state'))
  return `session_id=${latest.session_id ?? 'missing'} turn_count=${latest.turn_count ?? '0'}`
}

function exportSummary(root: string): string {
  const brandRoot = inferBrandRoot(root)
  const exportPath = join(brandRoot, 'sessions/local-main-session.export.md')
  const exists = existsSync(exportPath)
  return `export_path=${brandRoot.split('/').at(-1) ?? '.claude'}/sessions/local-main-session.export.md export_exists=${exists}`
}

function contextSummary(root: string): string {
  const summary = loadRuntimeSummary(root)
  return `context_digest=${summary.contextDigest}`
}

function usageSummary(root: string): string {
  const usage = readState(join(inferBrandRoot(root), 'sessions/summary.state'))
  return `usage_entries=${usage.usage_entries ?? '0'} total_cost_micros=${usage.total_cost_micros ?? '0'}`
}

function permissionSummary(root: string): string {
  const summary = loadRuntimeSummary(root)
  return `approval_mode=${summary.approvalMode} bash_policy=${summary.bashPolicy} file_write_policy=${summary.fileWritePolicy}`
}

function fileSummary(root: string): string {
  const brandRoot = inferBrandRoot(root)
  const sessionState = existsSync(join(brandRoot, 'sessions/local-main-session.state'))
  const exportState = existsSync(join(brandRoot, 'sessions/local-main-session.export.md'))
  const usageState = existsSync(join(brandRoot, 'sessions/summary.state'))
  return `session_state=${sessionState} export_state=${exportState} usage_state=${usageState}`
}

function taskSummary(root: string): string {
  const latest = readState(join(inferBrandRoot(root), 'sessions/latest.state'))
  const turnCount = latest.turn_count ?? '0'
  return `task_count=1 active_task=session-loop turn_count=${turnCount}`
}

function doctorSummary(root: string): string {
  const brandRoot = inferBrandRoot(root)
  const required = [
    join(brandRoot, 'sessions/README.md'),
    join(brandRoot, 'sessions/latest.state'),
    join(brandRoot, 'sessions/summary.state'),
  ]
  const missing = required
    .filter((path) => !existsSync(path))
    .map((path) => path.replace(`${root}/`, ''))
  if (instructionCandidates(root, brandRoot).length === 0) {
    missing.push('instruction-surface')
  }
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
