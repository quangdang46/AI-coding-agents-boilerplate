import { existsSync, readFileSync, writeFileSync } from 'node:fs'
import { join } from 'node:path'

export type SessionState = {
  sessionId: string
  turnCount: number
  provider: string
  model: string
  promptDigest: string
  contextDigest: string
}

export type UsageState = {
  usageEntries: number
  totalCostMicros: number
}

function readState(path: string): Record<string, string> {
  try {
    return Object.fromEntries(
      readFileSync(path, 'utf8')
        .trim()
        .split('\n')
        .filter((line) => line.includes('='))
        .map((line) => {
          const [key, ...rest] = line.split('=')
          return [key, rest.join('=')]
        }),
    )
  } catch {
    return {}
  }
}

function writeState(path: string, entries: Array<[string, string]>): void {
  writeFileSync(
    path,
    `${entries.map(([key, value]) => `${key}=${value}`).join('\n')}\n`,
    'utf8',
  )
}

export function loadLatestSessionState(root: string): SessionState {
  const state = readState(join(root, '.agent/sessions/latest.state'))
  return {
    sessionId: state.session_id ?? 'missing',
    turnCount: Number(state.turn_count ?? '0'),
    provider: state.provider ?? 'missing',
    model: state.model ?? 'missing',
    promptDigest: state.prompt_digest ?? 'missing',
    contextDigest: state.context_digest ?? 'missing',
  }
}

export function loadUsageState(root: string): UsageState {
  const state = readState(join(root, '.agent/usage/summary.state'))
  return {
    usageEntries: Number(state.usage_entries ?? '0'),
    totalCostMicros: Number(state.total_cost_micros ?? '0'),
  }
}

export function persistSessionState(
  root: string,
  nextState: SessionState,
  costMicros: number,
): UsageState {
  const sessionPath = join(root, '.agent/sessions/local-main-session.state')
  const latestPath = join(root, '.agent/sessions/latest.state')
  const exportFilePath = join(root, '.agent/sessions/local-main-session.export.md')
  const usageLogPath = join(root, '.agent/usage/ledger.log')
  const usageSummaryPath = join(root, '.agent/usage/summary.state')

  const stateEntries: Array<[string, string]> = [
    ['session_id', nextState.sessionId],
    ['turn_count', String(nextState.turnCount)],
    ['provider', nextState.provider],
    ['model', nextState.model],
    ['prompt_digest', nextState.promptDigest],
    ['context_digest', nextState.contextDigest],
  ]
  writeState(sessionPath, stateEntries)
  writeState(latestPath, stateEntries)

  writeFileSync(
    exportFilePath,
    `# Session Export\n\n- session_id: ${nextState.sessionId}\n- turn_count: ${nextState.turnCount}\n- provider: ${nextState.provider}\n- model: ${nextState.model}\n- prompt_digest: ${nextState.promptDigest}\n- context_digest: ${nextState.contextDigest}\n`,
    'utf8',
  )

  const previousUsage = loadUsageState(root)
  const nextUsage = {
    usageEntries: previousUsage.usageEntries + 1,
    totalCostMicros: previousUsage.totalCostMicros + costMicros,
  }

  const existingLog = existsSync(usageLogPath) ? `${readFileSync(usageLogPath, 'utf8').trim()}\n` : ''
  writeFileSync(
    usageLogPath,
    `${existingLog}session_id=${nextState.sessionId} turn_count=${nextState.turnCount} cost_micros=${costMicros}\n`,
    'utf8',
  )
  writeState(usageSummaryPath, [
    ['usage_entries', String(nextUsage.usageEntries)],
    ['total_cost_micros', String(nextUsage.totalCostMicros)],
  ])

  return nextUsage
}
