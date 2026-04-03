import { readFileSync } from 'node:fs'
import { join } from 'node:path'

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

export type ContextState = {
  promptTexts: string[]
  contextTexts: string[]
  promptDigest: string
  contextDigest: string
}

export function loadContextState(root: string): ContextState {
  const configText = readText(join(root, 'boilerplate.config.ts'))
  const systemPath = extractString(configText, /systemPath:\s*'([^']+)'/)
  const appendPaths = extractStringList(configText, /appendPaths:\s*\[([\s\S]*?)\]/)
  const contextPaths = extractStringList(configText, /contextPaths:\s*\[([\s\S]*?)\]/)

  const promptTexts = [readText(join(root, systemPath))]
  for (const path of appendPaths) {
    promptTexts.push(readText(join(root, path)))
  }
  const contextTexts = contextPaths.map((path) => readText(join(root, path)))

  return {
    promptTexts,
    contextTexts,
    promptDigest: checksum(promptTexts),
    contextDigest: checksum(contextTexts),
  }
}
