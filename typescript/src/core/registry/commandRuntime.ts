import type { Command } from '../../types/command.js'
import { insertDynamicSkills } from './commandAssembly.js'
import {
  selectSkillToolCommands,
  selectSlashCommandToolSkills,
} from './commandSelection.js'

type CacheableMemoizedFn = {
  cache?: {
    clear?: () => void
  }
}

export function filterEnabledCommands(args: {
  commands: Command[]
  meetsAvailabilityRequirement: (cmd: Command) => boolean
  isCommandEnabled: (cmd: Command) => boolean
}): Command[] {
  const { commands, meetsAvailabilityRequirement, isCommandEnabled } = args
  return commands.filter(
    command =>
      meetsAvailabilityRequirement(command) && isCommandEnabled(command),
  )
}

export async function loadVisibleCommands(args: {
  cwd: string
  loadAllCommands: (cwd: string) => Promise<Command[]>
  getDynamicSkills: () => Command[]
  builtInCommands: Command[]
  meetsAvailabilityRequirement: (cmd: Command) => boolean
  isCommandEnabled: (cmd: Command) => boolean
}): Promise<Command[]> {
  const {
    cwd,
    loadAllCommands,
    getDynamicSkills,
    builtInCommands,
    meetsAvailabilityRequirement,
    isCommandEnabled,
  } = args

  const allCommands = await loadAllCommands(cwd)
  const dynamicSkills = getDynamicSkills()
  const baseCommands = filterEnabledCommands({
    commands: allCommands,
    meetsAvailabilityRequirement,
    isCommandEnabled,
  })

  return insertDynamicSkills({
    baseCommands,
    dynamicSkills,
    builtInCommands,
    meetsAvailabilityRequirement,
    isCommandEnabled,
  })
}

export async function loadSkillToolCommands(args: {
  cwd: string
  getCommands: (cwd: string) => Promise<Command[]>
}): Promise<Command[]> {
  const allCommands = await args.getCommands(args.cwd)
  return selectSkillToolCommands(allCommands)
}

export async function loadSlashCommandToolSkills(args: {
  cwd: string
  getCommands: (cwd: string) => Promise<Command[]>
  onError: (error: unknown) => void
  onDebug: (message: string) => void
}): Promise<Command[]> {
  try {
    const allCommands = await args.getCommands(args.cwd)
    return selectSlashCommandToolSkills(allCommands)
  } catch (error) {
    args.onError(error)
    args.onDebug('Returning empty skills array due to load failure')
    return []
  }
}

export function clearMemoizedCommandCaches(args: {
  loadAllCommands: CacheableMemoizedFn
  getSkillToolCommands: CacheableMemoizedFn
  getSlashCommandToolSkills: CacheableMemoizedFn
  clearSkillIndexCache?: (() => void) | null
}): void {
  args.loadAllCommands.cache?.clear?.()
  args.getSkillToolCommands.cache?.clear?.()
  args.getSlashCommandToolSkills.cache?.clear?.()
  args.clearSkillIndexCache?.()
}
