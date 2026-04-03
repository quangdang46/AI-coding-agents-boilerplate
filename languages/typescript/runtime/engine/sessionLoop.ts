import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

import { loadRuntimeSummary } from '../config/loadRuntimeSummary.ts'

export function runtimeProjectRoot(fromModuleUrl: string = import.meta.url): string {
  return join(dirname(fileURLToPath(fromModuleUrl)), '..', '..', 'template', 'base')
}

export function runSessionLoop(root: string): string {
  const summary = loadRuntimeSummary(root)
  return `__PROJECT_NAME__ session loop completed provider=${summary.defaultProvider} model=${summary.providerModel} prompt_digest=${summary.promptDigest} context_digest=${summary.contextDigest} approval_mode=${summary.approvalMode} bash_policy=${summary.bashPolicy} file_write_policy=${summary.fileWritePolicy} ${summary.toolResults}`
}
