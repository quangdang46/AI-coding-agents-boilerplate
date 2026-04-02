import { expect, test } from 'bun:test'
import {
  assembleCommandSources,
  insertDynamicSkills,
} from '../../src/core/registry/commands.js'
import type { Command } from '../../src/types/command.js'

test('core command assembly keeps source ordering stable', () => {
  const command = (name: string) => ({ name } as Command)
  const assembled = assembleCommandSources({
    skills: {
      bundledSkills: [command('bundled')],
      builtinPluginSkills: [command('builtin-plugin')],
      skillDirCommands: [command('skill-dir')],
      pluginSkills: [command('plugin-skill')],
    },
    pluginCommands: [command('plugin-command')],
    workflowCommands: [command('workflow')],
    builtInCommands: [command('builtin')],
  })

  expect(assembled.map(command => command.name)).toEqual([
    'bundled',
    'builtin-plugin',
    'skill-dir',
    'workflow',
    'plugin-command',
    'plugin-skill',
    'builtin',
  ])
})

test('core command assembly inserts dynamic skills before built-ins', () => {
  const command = (name: string) => ({ name } as Command)
  const result = insertDynamicSkills({
    baseCommands: [command('plugin-skill'), command('builtin-a'), command('builtin-b')],
    dynamicSkills: [command('dynamic')],
    builtInCommands: [command('builtin-a'), command('builtin-b')],
    meetsAvailabilityRequirement: () => true,
    isCommandEnabled: () => true,
  })

  expect(result.map(command => command.name)).toEqual([
    'plugin-skill',
    'dynamic',
    'builtin-a',
    'builtin-b',
  ])
})
