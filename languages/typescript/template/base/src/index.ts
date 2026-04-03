import { readFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

type RuntimeSummary = {
  defaultProvider: string
  providerModel: string
  promptDigest: string
  contextDigest: string
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
  }
}

function runSessionLoop(): string {
  const summary = loadRuntimeSummary()
  return `__PROJECT_NAME__ session loop completed provider=${summary.defaultProvider} model=${summary.providerModel} prompt_digest=${summary.promptDigest} context_digest=${summary.contextDigest}`
}

export function main(): string {
  return runSessionLoop()
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  console.log(main())
}
