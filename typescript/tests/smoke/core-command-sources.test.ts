import { expect, test } from 'bun:test'
import {
  loadCommandSkillSources,
  loadCommandsForCwd,
} from '../../src/core/registry/commands.js'
import type { Command } from '../../src/types/command.js'

test('core command sources wrap skill-source loading with shared fallbacks', async () => {
  const command = (name: string) => ({ name } as Command)
  const result = await loadCommandSkillSources({
    cwd: '/tmp/project',
    loadSkillDirCommands: async () => [command('skill-dir')],
    loadPluginSkills: async () => [command('plugin-skill')],
    getBundledSkills: () => [command('bundled')],
    getBuiltinPluginSkillCommands: () => [command('builtin-plugin')],
    onError: () => {},
    onDebug: () => {},
  })

  expect(result.skillDirCommands.map(entry => entry.name)).toEqual(['skill-dir'])
  expect(result.pluginSkills.map(entry => entry.name)).toEqual(['plugin-skill'])
  expect(result.bundledSkills.map(entry => entry.name)).toEqual(['bundled'])
  expect(result.builtinPluginSkills.map(entry => entry.name)).toEqual([
    'builtin-plugin',
  ])
})

test('core command sources load combined commands from built-ins and async sources', async () => {
  const command = (name: string) => ({ name } as Command)
  const result = await loadCommandsForCwd({
    cwd: '/tmp/project',
    loadSkills: async () => ({
      skillDirCommands: [command('skill-dir')],
      pluginSkills: [command('plugin-skill')],
      bundledSkills: [command('bundled')],
      builtinPluginSkills: [command('builtin-plugin')],
    }),
    loadPluginCommands: async () => [command('plugin-command')],
    loadWorkflowCommands: async () => [command('workflow')],
    getBuiltInCommands: () => [command('builtin')],
  })

  expect(result.map(entry => entry.name)).toEqual([
    'bundled',
    'builtin-plugin',
    'skill-dir',
    'workflow',
    'plugin-command',
    'plugin-skill',
    'builtin',
  ])
})
