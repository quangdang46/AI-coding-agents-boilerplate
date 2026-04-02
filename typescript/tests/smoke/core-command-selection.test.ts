import { expect, test } from 'bun:test'
import {
  selectSkillToolCommands,
  selectSlashCommandToolSkills,
} from '../../src/core/registry/commands.js'
import type { Command } from '../../src/types/command.js'

test('core command selection keeps model-invocable skills for SkillTool', () => {
  const commands = [
    { type: 'prompt', source: 'bundled', loadedFrom: 'bundled', disableModelInvocation: false, name: 'bundled' },
    { type: 'prompt', source: 'builtin', loadedFrom: 'bundled', disableModelInvocation: false, name: 'builtin' },
    { type: 'prompt', source: 'userSettings', loadedFrom: 'skills', disableModelInvocation: false, name: 'skill' },
    { type: 'prompt', source: 'userSettings', loadedFrom: 'plugin', disableModelInvocation: false, hasUserSpecifiedDescription: true, name: 'plugin-prompt' },
  ] as unknown as Command[]

  expect(selectSkillToolCommands(commands).map(command => command.name)).toEqual([
    'bundled',
    'skill',
    'plugin-prompt',
  ])
})

test('core command selection keeps slash-command skills and disable-model-invocation prompts', () => {
  const commands = [
    { type: 'prompt', source: 'userSettings', loadedFrom: 'skills', disableModelInvocation: false, hasUserSpecifiedDescription: true, name: 'skill' },
    { type: 'prompt', source: 'userSettings', loadedFrom: 'plugin', disableModelInvocation: false, whenToUse: 'Use me', name: 'plugin' },
    { type: 'prompt', source: 'userSettings', loadedFrom: 'commands_DEPRECATED', disableModelInvocation: true, hasUserSpecifiedDescription: true, name: 'manual-only' },
    { type: 'prompt', source: 'builtin', loadedFrom: 'bundled', disableModelInvocation: false, whenToUse: 'builtin', name: 'builtin' },
  ] as unknown as Command[]

  expect(
    selectSlashCommandToolSkills(commands).map(command => command.name),
  ).toEqual(['skill', 'plugin', 'manual-only'])
})
