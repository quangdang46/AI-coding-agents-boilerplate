export type { BoilerplateConfig, BoilerplateConfigShape, ProviderId } from './schema.js'
export {
  DEFAULT_BOILERPLATE_CONFIG,
  getCurrentProjectConfig,
  getDefaultBoilerplateConfig,
  getGlobalConfig,
  saveCurrentProjectConfig,
  saveGlobalConfig,
  type GlobalConfig,
  type ProjectConfig,
} from './loadConfig.js'
