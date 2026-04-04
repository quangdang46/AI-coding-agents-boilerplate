import { join } from 'node:path'

import { readText } from './files.ts'
import { extractString, extractStringList } from './text.ts'

export type RuntimeConfig = {
  projectName: string
  defaultProvider: string
  providerModel: string
  systemPath: string
  appendPaths: string[]
  contextPaths: string[]
  enabledTools: string[]
  bashTimeoutMs: number
  approvalMode: string
  deny: string[]
}

export function loadRuntimeConfig(root: string): RuntimeConfig {
  const configText = readText(join(root, 'boilerplate.config.ts'))
  const defaultProvider = extractString(configText, /defaultProvider:\s*'([^']+)'/)

  return {
    projectName: extractString(configText, /name:\s*'([^']+)'/),
    defaultProvider,
    providerModel: extractString(
      configText,
      new RegExp(`${defaultProvider}:\\s*{[\\s\\S]*?model:\\s*'([^']+)'`),
    ),
    systemPath: extractString(configText, /systemPath:\s*'([^']+)'/),
    appendPaths: extractStringList(configText, /appendPaths:\s*\[([\s\S]*?)\]/),
    contextPaths: extractStringList(configText, /contextPaths:\s*\[([\s\S]*?)\]/),
    enabledTools: extractStringList(configText, /enabled:\s*\[([\s\S]*?)\]/),
    bashTimeoutMs: Number(extractString(configText, /bashTimeoutMs:\s*(\d+)/)),
    approvalMode: extractString(configText, /approvalMode:\s*'([^']+)'/),
    deny: extractStringList(configText, /deny:\s*\[([\s\S]*?)\]/),
  }
}
