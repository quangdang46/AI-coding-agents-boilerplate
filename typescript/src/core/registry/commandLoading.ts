import type { Command } from '../../types/command.js'
import {
  assembleCommandSources,
  type LoadedSkillSources,
} from './commandAssembly.js'

export async function loadAllCommandSources(args: {
  cwd: string
  loadSkills: (cwd: string) => Promise<LoadedSkillSources>
  loadPluginCommands: () => Promise<Command[]>
  loadWorkflowCommands: ((cwd: string) => Promise<Command[]>) | null
  builtInCommands: Command[]
}): Promise<Command[]> {
  const {
    cwd,
    loadSkills,
    loadPluginCommands,
    loadWorkflowCommands,
    builtInCommands,
  } = args

  const [skills, pluginCommands, workflowCommands] = await Promise.all([
    loadSkills(cwd),
    loadPluginCommands(),
    loadWorkflowCommands ? loadWorkflowCommands(cwd) : Promise.resolve([]),
  ])

  return assembleCommandSources({
    skills,
    pluginCommands,
    workflowCommands,
    builtInCommands,
  })
}
