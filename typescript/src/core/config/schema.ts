export type ProviderId = 'anthropic' | 'openai' | 'bedrock' | 'vertex' | 'foundry'

export interface BoilerplateConfig {
  app?: {
    name?: string
    version?: string
    binaryName?: string
    defaultProvider?: ProviderId
    defaultMode?: 'interactive' | 'headless'
    workingDirectoryPolicy?: 'project-root' | 'cwd'
  }
  prompts?: {
    systemPath?: string
    appendPaths?: string[]
    contextPaths?: string[]
  }
  tools?: {
    enabled?: string[]
    defaults?: {
      bashTimeoutMs?: number
      webFetchTimeoutMs?: number
      maxToolCallsPerTurn?: number
    }
    policy?: {
      approvalMode?: 'default' | 'always-ask' | 'never-ask'
      deny?: string[]
    }
  }
  providers?: Partial<
    Record<
      ProviderId,
      {
        apiKeyEnv?: string
        model?: string
        baseUrl?: string
      }
    >
  >
  agents?: {
    directories?: string[]
    enabled?: string[]
    defaultAgent?: string
  }
  skills?: {
    directories?: string[]
    enabled?: string[]
    autoDiscover?: boolean
  }
  features?: {
    enabled?: string[]
  }
}

export type BoilerplateConfigShape = BoilerplateConfig
