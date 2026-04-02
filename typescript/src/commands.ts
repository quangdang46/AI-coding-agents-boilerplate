import { feature } from 'bun:bundle'
import memoize from 'lodash-es/memoize.js'
import { type LoadedSkillSources } from './core/registry/commandAssembly.js'
import {
  BRIDGE_SAFE_COMMANDS,
  INTERNAL_ONLY_COMMANDS,
  REMOTE_SAFE_COMMANDS,
  builtInCommandNames,
  getBuiltInCommands,
} from './core/registry/builtInCommands.js'
import {
  findCommand,
  formatDescriptionWithSource,
  getCommand,
  getMcpSkillCommands,
  hasCommand,
} from './core/registry/commandHelpers.js'
import {
  clearMemoizedCommandCaches,
  loadSkillToolCommands,
  loadSlashCommandToolSkills,
  loadVisibleCommands,
} from './core/registry/commandRuntime.js'
import {
  loadCommandsForCwd,
  loadCommandSkillSources,
} from './core/registry/commandSources.js'
import { meetsAvailabilityRequirement } from './core/registry/commandAvailability.js'
import {
  type Command,
  getCommandName,
  isCommandEnabled,
} from './types/command.js'
import { logForDebugging } from './utils/debug.js'
import { toError } from './utils/errors.js'
import { logError } from './utils/log.js'
import {
  clearPluginCommandCache,
  clearPluginSkillsCache,
  getPluginCommands,
  getPluginSkills,
} from './utils/plugins/loadPluginCommands.js'
import {
  clearSkillCaches,
  getDynamicSkills,
  getSkillDirCommands,
} from './skills/loadSkillsDir.js'
import { getBundledSkills } from './skills/bundledSkills.js'
import { getBuiltinPluginSkillCommands } from './plugins/builtinPlugins.js'

/* eslint-disable @typescript-eslint/no-require-imports */
const clearSkillIndexCache = feature('EXPERIMENTAL_SKILL_SEARCH')
  ? (require('./services/skillSearch/localSearch.js').clearSkillIndexCache as
      | (() => void)
      | undefined)
  : null

const getWorkflowCommands = feature('WORKFLOW_SCRIPTS')
  ? (() => {
      const workflowModule = require('./tools/WorkflowTool/createWorkflowCommand.js')
      return workflowModule.getWorkflowCommands as (cwd: string) => Promise<Command[]>
    })()
  : null
/* eslint-enable @typescript-eslint/no-require-imports */

export type {
  Command,
  CommandBase,
  CommandResultDisplay,
  LocalCommandResult,
  LocalJSXCommandContext,
  PromptCommand,
  ResumeEntrypoint,
} from './types/command.js'
export { getCommandName, isCommandEnabled } from './types/command.js'
export {
  BRIDGE_SAFE_COMMANDS,
  INTERNAL_ONLY_COMMANDS,
  REMOTE_SAFE_COMMANDS,
  builtInCommandNames,
} from './core/registry/builtInCommands.js'
export {
  findCommand,
  formatDescriptionWithSource,
  getCommand,
  getMcpSkillCommands,
  hasCommand,
} from './core/registry/commandHelpers.js'

async function getSkills(cwd: string): Promise<LoadedSkillSources> {
  return loadCommandSkillSources({
    cwd,
    loadSkillDirCommands: getSkillDirCommands,
    loadPluginSkills: getPluginSkills,
    getBundledSkills,
    getBuiltinPluginSkillCommands,
    onError: err => logError(toError(err)),
    onDebug: logForDebugging,
  })
}

const loadAllCommands = memoize(async (cwd: string): Promise<Command[]> => {
  return loadCommandsForCwd({
    cwd,
    loadSkills: getSkills,
    loadPluginCommands: getPluginCommands,
    loadWorkflowCommands: getWorkflowCommands,
    getBuiltInCommands,
  })
})

export async function getCommands(cwd: string): Promise<Command[]> {
  return loadVisibleCommands({
    cwd,
    loadAllCommands,
    getDynamicSkills,
    builtInCommands: getBuiltInCommands(),
    meetsAvailabilityRequirement,
    isCommandEnabled,
  })
}

export function clearCommandMemoizationCaches(): void {
  clearMemoizedCommandCaches({
    loadAllCommands,
    getSkillToolCommands,
    getSlashCommandToolSkills,
    clearSkillIndexCache,
  })
}

export function clearCommandsCache(): void {
  clearCommandMemoizationCaches()
  clearPluginCommandCache()
  clearPluginSkillsCache()
  clearSkillCaches()
}

export const getSkillToolCommands = memoize(
  async (cwd: string): Promise<Command[]> => {
    return loadSkillToolCommands({ cwd, getCommands })
  },
)

export const getSlashCommandToolSkills = memoize(
  async (cwd: string): Promise<Command[]> => {
    return loadSlashCommandToolSkills({
      cwd,
      getCommands,
      onError: error => logError(toError(error)),
      onDebug: logForDebugging,
    })
  },
)

export function isBridgeSafeCommand(cmd: Command): boolean {
  if (cmd.type === 'local-jsx') return false
  if (cmd.type === 'prompt') return true
  return BRIDGE_SAFE_COMMANDS.has(cmd)
}

export function filterCommandsForRemoteMode(commands: Command[]): Command[] {
  return commands.filter(cmd => REMOTE_SAFE_COMMANDS.has(cmd))
}
