import type { Command } from '../../types/command.js'
import { loadAllCommandSources } from './commandLoading.js'
import type { LoadedSkillSources } from './commandAssembly.js'
import { loadSkillSourcesWithFallbacks } from './skillSources.js'

export async function loadCommandSkillSources(args: {
  cwd: string
  loadSkillDirCommands: (cwd: string) => Promise<Command[]>
  loadPluginSkills: () => Promise<Command[]>
  getBundledSkills: () => Command[]
  getBuiltinPluginSkillCommands: () => Command[]
  onError: (error: unknown) => void
  onDebug: (message: string) => void
}): Promise<LoadedSkillSources> {
  return loadSkillSourcesWithFallbacks(args)
}

export async function loadCommandsForCwd(args: {
  cwd: string
  loadSkills: (cwd: string) => Promise<LoadedSkillSources>
  loadPluginCommands: () => Promise<Command[]>
  loadWorkflowCommands: ((cwd: string) => Promise<Command[]>) | null
  getBuiltInCommands: () => Command[]
}): Promise<Command[]> {
  const {
    cwd,
    loadSkills,
    loadPluginCommands,
    loadWorkflowCommands,
    getBuiltInCommands,
  } = args

  return loadAllCommandSources({
    cwd,
    loadSkills,
    loadPluginCommands,
    loadWorkflowCommands,
    builtInCommands: getBuiltInCommands(),
  })
}
