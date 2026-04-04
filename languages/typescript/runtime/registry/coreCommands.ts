import { existsSync, readFileSync } from 'node:fs'
import { join } from 'node:path'

import { loadRuntimeSummary } from '../config/loadRuntimeSummary.ts'
import { loadContextSummary } from '../context/loadContextState.ts'
import { loadSessionSnapshot } from '../state/sessionState.ts'
import { inferBrandRoot, instructionCandidates } from '../utils/brand.ts'
import { loadRuntimeConfigSummary } from '../utils/config.ts'

export type CoreCommandName =
  | 'status'
  | 'session'
  | 'export'
  | 'config'
  | 'doctor'
  | 'context'
  | 'memory'
  | 'plan'
  | 'review'
  | 'model'
  | 'effort'
  | 'fast'
  | 'passes'
  | 'usage'
  | 'permissions'
  | 'files'
  | 'resume'
  | 'compact'
  | 'diff'
  | 'cost'
  | 'clear'
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

function configSummary(root: string): string {
  const { defaultProvider, providerModel, approvalMode } = loadRuntimeConfigSummary(root)
  return `provider=${defaultProvider} model=${providerModel} approval_mode=${approvalMode}`
}

function sessionSummary(root: string): string {
  const latest = readState(join(inferBrandRoot(root), 'sessions/latest.state'))
  return `session_id=${latest.session_id ?? 'missing'} turn_count=${latest.turn_count ?? '0'}`
}

function statusSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  return `status=ready session_id=${session.sessionId} turn_count=${session.turnCount} usage_entries=${session.usageEntries}`
}

function exportSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  const brandRoot = inferBrandRoot(root)
  const exists = existsSync(join(root, session.exportPath))
  return `export_path=${session.exportPath} export_exists=${exists && brandRoot.length > 0}`
}

function contextSummary(root: string): string {
  const summary = loadContextSummary(root)
  return `prompt_digest=${summary.promptDigest} context_digest=${summary.contextDigest}`
}

function usageSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  return `usage_entries=${session.usageEntries} total_cost_micros=${session.totalCostMicros}`
}

function memorySummary(root: string): string {
  const session = loadSessionSnapshot(root)
  return `memory_entries=${session.usageEntries} context_digest=${session.contextDigest}`
}

function planSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  return `plan_ready=true session_id=${session.sessionId} turn_count=${session.turnCount}`
}

function reviewSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  return `review_ready=true session_id=${session.sessionId} usage_entries=${session.usageEntries}`
}

function modelSummary(root: string): string {
  const { defaultProvider, providerModel } = loadRuntimeConfigSummary(root)
  return `model_provider=${defaultProvider} model_name=${providerModel}`
}

function effortSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  return `effort_level=normal turn_count=${session.turnCount} usage_entries=${session.usageEntries}`
}

function fastSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  return `fast_mode=available session_id=${session.sessionId} context_digest=${session.contextDigest}`
}

function passesSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  return `passes_ready=true usage_entries=${session.usageEntries} total_cost_micros=${session.totalCostMicros}`
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

function resumeSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  return `resume_session_id=${session.sessionId} resume_turn_count=${session.turnCount}`
}

function compactSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  return `compact_ready=true usage_entries=${session.usageEntries} export_path=${session.exportPath}`
}

function diffSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  return `diff_ready=true context_digest=${session.contextDigest} turn_count=${session.turnCount}`
}

function costSummary(root: string): string {
  const session = loadSessionSnapshot(root)
  return `cost_entries=${session.usageEntries} total_cost_micros=${session.totalCostMicros}`
}

function clearSummary(root: string): string {
  const latest = readState(join(inferBrandRoot(root), 'sessions/latest.state'))
  const hasSession = (latest.session_id ?? '').length > 0
  return `clearable=${hasSession} retained_session_id=${latest.session_id ?? 'missing'}`
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
  { name: 'status', run: statusSummary },
  { name: 'session', run: sessionSummary },
  { name: 'export', run: exportSummary },
  { name: 'config', run: configSummary },
  { name: 'doctor', run: doctorSummary },
  { name: 'context', run: contextSummary },
  { name: 'memory', run: memorySummary },
  { name: 'plan', run: planSummary },
  { name: 'review', run: reviewSummary },
  { name: 'model', run: modelSummary },
  { name: 'effort', run: effortSummary },
  { name: 'fast', run: fastSummary },
  { name: 'passes', run: passesSummary },
  { name: 'usage', run: usageSummary },
  { name: 'permissions', run: permissionSummary },
  { name: 'files', run: fileSummary },
  { name: 'resume', run: resumeSummary },
  { name: 'compact', run: compactSummary },
  { name: 'diff', run: diffSummary },
  { name: 'cost', run: costSummary },
  { name: 'clear', run: clearSummary },
  { name: 'tasks', run: taskSummary },
]

export function getCoreCommandRegistry(): Record<CoreCommandName, CoreCommandHandler> {
  return Object.fromEntries(coreCommands.map((command) => [command.name, command.run])) as Record<
    CoreCommandName,
    CoreCommandHandler
  >
}
