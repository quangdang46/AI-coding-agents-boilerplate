import type { Command } from '../../types/command.js'

export function selectSkillToolCommands(allCommands: Command[]): Command[] {
  return allCommands.filter(
    cmd =>
      cmd.type === 'prompt' &&
      !cmd.disableModelInvocation &&
      cmd.source !== 'builtin' &&
      (cmd.loadedFrom === 'bundled' ||
        cmd.loadedFrom === 'skills' ||
        cmd.loadedFrom === 'commands_DEPRECATED' ||
        cmd.hasUserSpecifiedDescription ||
        cmd.whenToUse),
  )
}

export function selectSlashCommandToolSkills(allCommands: Command[]): Command[] {
  return allCommands.filter(
    cmd =>
      cmd.type === 'prompt' &&
      cmd.source !== 'builtin' &&
      (cmd.hasUserSpecifiedDescription || cmd.whenToUse) &&
      (cmd.loadedFrom === 'skills' ||
        cmd.loadedFrom === 'plugin' ||
        cmd.loadedFrom === 'bundled' ||
        cmd.disableModelInvocation),
  )
}
