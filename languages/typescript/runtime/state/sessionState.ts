import { existsSync, readFileSync, writeFileSync } from 'node:fs'
import { join } from 'node:path'
import { inferBrandRoot } from '../utils/brand.ts'

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
  const state = readState(join(inferBrandRoot(root), 'sessions/latest.state'))
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
  const state = readState(join(inferBrandRoot(root), 'sessions/summary.state'))
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
  const brandRoot = inferBrandRoot(root)
  const sessionPath = join(brandRoot, 'sessions/local-main-session.state')
  const latestPath = join(brandRoot, 'sessions/latest.state')
  const exportFilePath = join(brandRoot, 'sessions/local-main-session.export.md')
  const usageSummaryPath = join(brandRoot, 'sessions/summary.state')

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

  writeState(usageSummaryPath, [
    ['usage_entries', String(nextUsage.usageEntries)],
    ['total_cost_micros', String(nextUsage.totalCostMicros)],
  ])

  return nextUsage
}
