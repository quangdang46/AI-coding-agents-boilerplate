import type { Command } from '../../types/command.js'
import type { LoadedSkillSources } from './commandAssembly.js'

export async function loadSkillSourcesWithFallbacks(args: {
  cwd: string
  loadSkillDirCommands: (cwd: string) => Promise<Command[]>
  loadPluginSkills: () => Promise<Command[]>
  getBundledSkills: () => Command[]
  getBuiltinPluginSkillCommands: () => Command[]
  onError: (error: unknown) => void
  onDebug: (message: string) => void
}): Promise<LoadedSkillSources> {
  const {
    cwd,
    loadSkillDirCommands,
    loadPluginSkills,
    getBundledSkills,
    getBuiltinPluginSkillCommands,
    onError,
    onDebug,
  } = args

  try {
    const [skillDirCommands, pluginSkills] = await Promise.all([
      loadSkillDirCommands(cwd).catch(err => {
        onError(err)
        onDebug('Skill directory commands failed to load, continuing without them')
        return []
      }),
      loadPluginSkills().catch(err => {
        onError(err)
        onDebug('Plugin skills failed to load, continuing without them')
        return []
      }),
    ])

    const bundledSkills = getBundledSkills()
    const builtinPluginSkills = getBuiltinPluginSkillCommands()

    onDebug(
      `loadSkillSourcesWithFallbacks returning: ${skillDirCommands.length} skill dir commands, ${pluginSkills.length} plugin skills, ${bundledSkills.length} bundled skills, ${builtinPluginSkills.length} builtin plugin skills`,
    )

    return {
      skillDirCommands,
      pluginSkills,
      bundledSkills,
      builtinPluginSkills,
    }
  } catch (err) {
    onError(err)
    onDebug('Unexpected error in loadSkillSourcesWithFallbacks, returning empty')
    return {
      skillDirCommands: [],
      pluginSkills: [],
      bundledSkills: [],
      builtinPluginSkills: [],
    }
  }
}
