import type { BoilerplateConfig } from './schema.js'

export const DEFAULT_BOILERPLATE_CONFIG: BoilerplateConfig = {
  app: {
    name: 'agent-boilerplate',
    version: '0.1.0',
    binaryName: 'agentkit',
    defaultProvider: 'anthropic',
    defaultMode: 'interactive',
    workingDirectoryPolicy: 'project-root',
  },
  prompts: {
    systemPath: '.agent/prompts/system.md',
    appendPaths: [
      '.agent/prompts/sections/style.md',
      '.agent/prompts/sections/verification.md',
      '.agent/prompts/sections/security.md',
    ],
    contextPaths: [],
  },
  tools: {
    enabled: ['bash', 'file_read', 'file_edit', 'file_write', 'web_fetch'],
    defaults: {
      bashTimeoutMs: 120000,
      webFetchTimeoutMs: 30000,
      maxToolCallsPerTurn: 50,
    },
    policy: {
      approvalMode: 'default',
      deny: [],
    },
  },
  providers: {
    anthropic: {
      apiKeyEnv: 'ANTHROPIC_API_KEY',
      model: 'claude-sonnet-4-6',
    },
    openai: {
      apiKeyEnv: 'OPENAI_API_KEY',
      model: 'gpt-5.2-codex',
    },
  },
  agents: {
    directories: ['.agent/agents'],
    enabled: ['planner', 'executor', 'reviewer'],
    defaultAgent: 'executor',
  },
  skills: {
    directories: ['.agent/skills'],
    enabled: ['plan', 'verify', 'add-feature'],
    autoDiscover: true,
  },
  features: {
    enabled: [],
  },
}

export function getDefaultBoilerplateConfig(): BoilerplateConfig {
  return structuredClone(DEFAULT_BOILERPLATE_CONFIG)
}
