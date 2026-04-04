import { existsSync } from 'node:fs'
import { join } from 'node:path'

import { persistSessionState } from '../state/sessionState.ts'
import { loadRuntimeConfig } from '../utils/config.ts'
import { inferBrandRoot } from '../utils/brand.ts'
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
  const costMicros = (promptTexts.concat(contextTexts).join('').length * 2) + toolResults.length * 3
  const nextState = persistSessionState(
    root,
    {
      sessionId: 'local-main-session',
      turnCount: 0,
      provider: defaultProvider,
      model: providerModel,
      promptDigest,
      contextDigest,
    },
    costMicros,
  )

  return {
    sessionId: nextState.sessionId,
    turnCount: nextState.turnCount,
    exportPath: nextState.exportPath,
    usageEntries: nextState.usageEntries,
    totalCostMicros: nextState.totalCostMicros,
  }
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

  const promptTexts = [systemPath].map((path) => join(root, path)).filter(existsSync).map(readText)
  for (const path of appendPaths) {
    const full = join(root, path)
    if (existsSync(full)) {
      promptTexts.push(readText(full))
    }
  }

  const contextTexts = contextPaths.map((path) => join(root, path)).filter(existsSync).map(readText)
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
