import { loadContextState } from '../context/loadContextState.ts'
import { loadSessionSnapshot } from '../state/sessionState.ts'
import { loadRuntimeConfig } from '../utils/config.ts'
import { policyForOperation } from '../utils/policy.ts'
import { readCoreToolResults } from '../utils/toolExecution.ts'

export function buildEntrypointSummary(root: string): string {
  const { projectName, defaultProvider, providerModel, approvalMode, deny } = loadRuntimeConfig(root)
  const context = loadContextState(root)
  const session = loadSessionSnapshot(root)
  const bashPolicy = policyForOperation(approvalMode, deny, 'bash', 'bash')
  const fileWritePolicy = policyForOperation(approvalMode, deny, 'file_write', 'file_write')
  const toolResults = readCoreToolResults(root, approvalMode, deny)

  return `${projectName} session loop completed provider=${defaultProvider} model=${providerModel} prompt_digest=${context.promptDigest} context_digest=${context.contextDigest} approval_mode=${approvalMode} bash_policy=${bashPolicy} file_write_policy=${fileWritePolicy} ${toolResults} session_id=${session.sessionId} turn_count=${session.turnCount} export_path=${session.exportPath} usage_entries=${session.usageEntries} total_cost_micros=${session.totalCostMicros}`
}
