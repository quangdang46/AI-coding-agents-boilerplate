import { execFileSync } from 'node:child_process'
import { readFileSync, writeFileSync, existsSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

type RuntimeSummary = {
  defaultProvider: string
  providerModel: string
  promptDigest: string
  contextDigest: string
  approvalMode: string
  bashPolicy: string
  fileWritePolicy: string
  toolResults: string
}

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

function projectRoot(): string {
  return dirname(dirname(fileURLToPath(import.meta.url)))
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

function loadRuntimeSummary(): RuntimeSummary {
  const root = projectRoot()
  const configText = readText(join(root, 'boilerplate.config.ts'))
  const defaultProvider = extractString(configText, /defaultProvider:\s*'([^']+)'/)
  const providerModel = extractString(
    configText,
    new RegExp(`${defaultProvider}:\\s*{[\\s\\S]*?model:\\s*'([^']+)'`),
  )
  const systemPath = extractString(configText, /systemPath:\s*'([^']+)'/)
  const appendPaths = extractStringList(configText, /appendPaths:\s*\[([\s\S]*?)\]/)
  const contextPaths = extractStringList(configText, /contextPaths:\s*\[([\s\S]*?)\]/)
  const enabledTools = extractStringList(configText, /enabled:\s*\[([\s\S]*?)\]/)
  const bashTimeoutMs = Number(extractString(configText, /bashTimeoutMs:\s*(\d+)/))
  const approvalMode = extractString(configText, /approvalMode:\s*'([^']+)'/)
  const deny = extractStringList(configText, /deny:\s*\[([\s\S]*?)\]/)

  const promptTexts = [readText(join(root, systemPath))]
  for (const path of appendPaths) {
    promptTexts.push(readText(join(root, path)))
  }

  const contextTexts = contextPaths.map((path) => readText(join(root, path)))

  return {
    defaultProvider,
    providerModel,
    promptDigest: checksum(promptTexts),
    contextDigest: checksum(contextTexts),
    approvalMode,
    bashPolicy: policyForOperation(approvalMode, deny, 'bash', 'bash'),
    fileWritePolicy: policyForOperation(approvalMode, deny, 'file_write', 'file_write'),
    toolResults: runCoreTools(root, enabledTools, approvalMode, deny, bashTimeoutMs),
  }
}

function runSessionLoop(): string {
  const summary = loadRuntimeSummary()
  return `__PROJECT_NAME__ session loop completed provider=${summary.defaultProvider} model=${summary.providerModel} prompt_digest=${summary.promptDigest} context_digest=${summary.contextDigest} approval_mode=${summary.approvalMode} bash_policy=${summary.bashPolicy} file_write_policy=${summary.fileWritePolicy} ${summary.toolResults}`
}

export function main(): string {
  return runSessionLoop()
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  console.log(main())
}
