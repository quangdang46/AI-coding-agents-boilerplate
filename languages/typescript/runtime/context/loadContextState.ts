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
