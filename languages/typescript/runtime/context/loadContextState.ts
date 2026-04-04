import { existsSync } from 'node:fs'
import { join } from 'node:path'

import { loadRuntimeConfig } from '../utils/config.ts'
import { readText } from '../utils/files.ts'
import { checksum } from '../utils/text.ts'

export type ContextState = {
  promptTexts: string[]
  contextTexts: string[]
  promptDigest: string
  contextDigest: string
}

export function loadContextState(root: string): ContextState {
  const { systemPath, appendPaths, contextPaths } = loadRuntimeConfig(root)

  const promptTexts = []
  const systemFull = join(root, systemPath)
  if (existsSync(systemFull)) {
    promptTexts.push(readText(systemFull))
  }
  for (const path of appendPaths) {
    const full = join(root, path)
    if (existsSync(full)) {
      promptTexts.push(readText(full))
    }
  }
  const contextTexts = contextPaths
    .map((path) => join(root, path))
    .filter(existsSync)
    .map(readText)

  return {
    promptTexts,
    contextTexts,
    promptDigest: checksum(promptTexts),
    contextDigest: checksum(contextTexts),
  }
}
