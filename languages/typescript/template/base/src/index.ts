import { execFileSync } from 'node:child_process'
import { existsSync, readFileSync, writeFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

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

function projectRoot(): string {
  return dirname(dirname(fileURLToPath(import.meta.url)))
}

function runCoreTools(
  root: string,
  enabledTools: string[],
  approvalMode: string,
  deny: string[],
  bashTimeoutMs: number,
): string {
  const usagePath = join(root, '.agent/usage/runtime-tool-smoke.txt')
  const status = (toolName: string, operation: string) => {
    if (!enabledTools.includes(toolName)) {
      return `${operation}=disabled`
    }
    return policyForOperation(approvalMode, deny, operation, toolName)
  }

  const results: string[] = []

  const bashStatus = status('bash', 'bash')
  if (bashStatus === 'bash=allowed') {
    const bashResult = execFileSync('bash', ['-lc', 'printf tool-bash-ok'], {
      cwd: root,
      encoding: 'utf8',
      timeout: bashTimeoutMs,
    }).trim()
    results.push(`bash_result=${bashResult}`)
  } else {
    results.push(`bash_result=${bashStatus}`)
  }

  const fileReadStatus = status('file_read', 'file_read')
  if (fileReadStatus === 'file_read=allowed') {
    results.push(`file_read_result=${checksum([readText(join(root, '.agent/context/README.md'))])}`)
  } else {
    results.push(`file_read_result=${fileReadStatus}`)
  }

  const fileWriteStatus = status('file_write', 'file_write')
  if (fileWriteStatus === 'file_write=allowed') {
    writeFileSync(usagePath, 'tool-write-ok', 'utf8')
    results.push('file_write_result=tool-write-ok')
  } else {
    results.push(`file_write_result=${fileWriteStatus}`)
  }

  const fileEditStatus = status('file_edit', 'file_edit')
  if (fileEditStatus === 'file_edit=allowed') {
    if (!existsSync(usagePath)) {
      writeFileSync(usagePath, 'tool-write-ok', 'utf8')
    }
    const edited = `${readText(usagePath)} edited`
    writeFileSync(usagePath, edited, 'utf8')
    results.push(`file_edit_result=${edited}`)
  } else {
    results.push(`file_edit_result=${fileEditStatus}`)
  }

  const webFetchStatus = status('web_fetch', 'web_fetch')
  if (webFetchStatus === 'web_fetch=allowed') {
    results.push('web_fetch_result=tool-web-fetch')
  } else {
    results.push(`web_fetch_result=${webFetchStatus}`)
  }

  return results.join(' ')
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
): string {
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

  return `session_id=${sessionId} turn_count=${turnCount} export_path=${exportPath} usage_entries=${usageEntries} total_cost_micros=${totalCostMicros}`
}

function buildTemplateLoopSummary(root: string): string {
  const configText = readText(join(root, 'boilerplate.config.ts'))
  const defaultProvider = extractString(configText, /defaultProvider:\s*'([^']+)'/)
  const providerModel = extractString(
    configText,
    new RegExp(`${defaultProvider}:\\s*{[\\s\\S]*?model:\\s*'([^']+)'`),
  )
  const systemPath = extractString(configText, /systemPath:\s*'([^']+)'/)
  const appendPaths = extractStringList(configText, /appendPaths:\s*\[([\s\S]*?)\]/)
  const contextPaths = extractStringList(configText, /contextPaths:\s*\[([\s\S]*?)\]/)
  const approvalMode = extractString(configText, /approvalMode:\s*'([^']+)'/)
  const deny = extractStringList(configText, /deny:\s*\[([\s\S]*?)\]/)
  const enabledTools = extractStringList(configText, /enabled:\s*\[([\s\S]*?)\]/)
  const bashTimeoutMs = Number(extractString(configText, /bashTimeoutMs:\s*(\d+)/))

  const promptTexts = [readText(join(root, systemPath))]
  for (const path of appendPaths) {
    promptTexts.push(readText(join(root, path)))
  }

  const contextTexts = contextPaths.map((path) => readText(join(root, path)))
  const promptDigest = checksum(promptTexts)
  const contextDigest = checksum(contextTexts)
  const toolResults = runCoreTools(root, enabledTools, approvalMode, deny, bashTimeoutMs)
  const sessionSummary = persistSessionAndUsage(
    root,
    defaultProvider,
    providerModel,
    promptDigest,
    contextDigest,
    promptTexts,
    contextTexts,
    toolResults,
  )

  return `__PROJECT_NAME__ session loop completed provider=${defaultProvider} model=${providerModel} prompt_digest=${promptDigest} context_digest=${contextDigest} approval_mode=${approvalMode} bash_policy=${policyForOperation(approvalMode, deny, 'bash', 'bash')} file_write_policy=${policyForOperation(approvalMode, deny, 'file_write', 'file_write')} ${toolResults} ${sessionSummary}`
}

export function main(): string {
  return buildTemplateLoopSummary(projectRoot())
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  console.log(main())
}
