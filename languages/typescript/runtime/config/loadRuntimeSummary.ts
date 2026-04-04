import { existsSync, readFileSync, writeFileSync } from 'node:fs'
import { join } from 'node:path'

import { loadRuntimeConfig } from '../utils/config.ts'
import { readText } from '../utils/files.ts'
import { policyForOperation } from '../utils/policy.ts'
import { checksum } from '../utils/text.ts'
import { runCoreTools } from '../utils/toolExecution.ts'

export type RuntimeSummary = {
  defaultProvider: string
  providerModel: string
  promptDigest: string
  contextDigest: string
  approvalMode: string
  bashPolicy: string
  fileWritePolicy: string
  toolResults: string
  sessionId: string
  turnCount: number
  exportPath: string
  usageEntries: number
  totalCostMicros: number
}

function readState(path: string): Record<string, string> {
  try {
    return Object.fromEntries(
      readText(path)
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


function persistSessionAndUsage(
  root: string,
  defaultProvider: string,
  providerModel: string,
  promptDigest: string,
  contextDigest: string,
  promptTexts: string[],
  contextTexts: string[],
  toolResults: string,
): Pick<RuntimeSummary, 'sessionId' | 'turnCount' | 'exportPath' | 'usageEntries' | 'totalCostMicros'> {
  const sessionId = 'local-main-session'
  const sessionPath = join(root, '.agent/sessions/local-main-session.state')
  const latestPath = join(root, '.agent/sessions/latest.state')
  const exportPath = '.agent/sessions/local-main-session.export.md'
  const exportFilePath = join(root, exportPath)
  const usageLogPath = join(root, '.agent/usage/ledger.log')
  const usageSummaryPath = join(root, '.agent/usage/summary.state')

  const previousSession = readState(sessionPath)
  const turnCount = Number(previousSession.turn_count ?? '0') + 1
  const previousSummary = readState(usageSummaryPath)
  const usageEntries = Number(previousSummary.usage_entries ?? '0') + 1
  const costMicros = (promptTexts.concat(contextTexts).join('').length * 2) + toolResults.length * 3
  const totalCostMicros = Number(previousSummary.total_cost_micros ?? '0') + costMicros

  const stateEntries: Array<[string, string]> = [
    ['session_id', sessionId],
    ['turn_count', String(turnCount)],
    ['provider', defaultProvider],
    ['model', providerModel],
    ['prompt_digest', promptDigest],
    ['context_digest', contextDigest],
  ]
  writeState(sessionPath, stateEntries)
  writeState(latestPath, stateEntries)

  writeFileSync(
    exportFilePath,
    `# Session Export\n\n- session_id: ${sessionId}\n- turn_count: ${turnCount}\n- provider: ${defaultProvider}\n- model: ${providerModel}\n- prompt_digest: ${promptDigest}\n- context_digest: ${contextDigest}\n`,
    'utf8',
  )

  const existingLog = existsSync(usageLogPath) ? `${readText(usageLogPath)}\n` : ''
  writeFileSync(
    usageLogPath,
    `${existingLog}session_id=${sessionId} turn_count=${turnCount} cost_micros=${costMicros}\n`,
    'utf8',
  )
  writeState(usageSummaryPath, [
    ['usage_entries', String(usageEntries)],
    ['total_cost_micros', String(totalCostMicros)],
  ])

  return { sessionId, turnCount, exportPath, usageEntries, totalCostMicros }
}

export function loadRuntimeSummary(root: string): RuntimeSummary {
  const {
    defaultProvider,
    providerModel,
    systemPath,
    appendPaths,
    contextPaths,
    enabledTools,
    bashTimeoutMs,
    approvalMode,
    deny,
  } = loadRuntimeConfig(root)

  const promptTexts = [readText(join(root, systemPath))]
  for (const path of appendPaths) {
    promptTexts.push(readText(join(root, path)))
  }

  const contextTexts = contextPaths.map((path) => readText(join(root, path)))
  const promptDigest = checksum(promptTexts)
  const contextDigest = checksum(contextTexts)
  const toolResults = runCoreTools(root, {
    enabledTools,
    approvalMode,
    deny,
    bashTimeoutMs,
  })
  const sessionState = persistSessionAndUsage(
    root,
    defaultProvider,
    providerModel,
    promptDigest,
    contextDigest,
    promptTexts,
    contextTexts,
    toolResults,
  )

  return {
    defaultProvider,
    providerModel,
    promptDigest,
    contextDigest,
    approvalMode,
    bashPolicy: policyForOperation(approvalMode, deny, 'bash', 'bash'),
    fileWritePolicy: policyForOperation(approvalMode, deny, 'file_write', 'file_write'),
    toolResults,
    ...sessionState,
  }
}
