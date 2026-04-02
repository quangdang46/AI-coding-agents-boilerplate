import type { Command } from '../../types/command.js'

export type LoadedSkillSources = {
  skillDirCommands: Command[]
  pluginSkills: Command[]
  bundledSkills: Command[]
  builtinPluginSkills: Command[]
}

export function assembleCommandSources(args: {
  skills: LoadedSkillSources
  pluginCommands: Command[]
  workflowCommands: Command[]
  builtInCommands: Command[]
}): Command[] {
  const { skills, pluginCommands, workflowCommands, builtInCommands } = args
  return [
    ...skills.bundledSkills,
    ...skills.builtinPluginSkills,
    ...skills.skillDirCommands,
    ...workflowCommands,
    ...pluginCommands,
    ...skills.pluginSkills,
    ...builtInCommands,
  ]
}

export function insertDynamicSkills(args: {
  baseCommands: Command[]
  dynamicSkills: Command[]
  builtInCommands: Command[]
  meetsAvailabilityRequirement: (cmd: Command) => boolean
  isCommandEnabled: (cmd: Command) => boolean
}): Command[] {
  const {
    baseCommands,
    dynamicSkills,
    builtInCommands,
    meetsAvailabilityRequirement,
    isCommandEnabled,
  } = args

  if (dynamicSkills.length === 0) {
    return baseCommands
  }

  const baseCommandNames = new Set(baseCommands.map(c => c.name))
  const uniqueDynamicSkills = dynamicSkills.filter(
    skill =>
      !baseCommandNames.has(skill.name) &&
      meetsAvailabilityRequirement(skill) &&
      isCommandEnabled(skill),
  )

  if (uniqueDynamicSkills.length === 0) {
    return baseCommands
  }

  const builtInNames = new Set(builtInCommands.map(c => c.name))
  const insertIndex = baseCommands.findIndex(c => builtInNames.has(c.name))

  if (insertIndex === -1) {
    return [...baseCommands, ...uniqueDynamicSkills]
  }

  return [
    ...baseCommands.slice(0, insertIndex),
    ...uniqueDynamicSkills,
    ...baseCommands.slice(insertIndex),
  ]
}
