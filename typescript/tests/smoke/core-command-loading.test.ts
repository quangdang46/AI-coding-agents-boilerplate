import { expect, test } from 'bun:test'
import { loadAllCommandSources } from '../../src/core/registry/commands.js'
import type { Command } from '../../src/types/command.js'

test('core command loading combines async sources through the shared loader', async () => {
  const command = (name: string) => ({ name } as Command)
  const result = await loadAllCommandSources({
    cwd: '/tmp/project',
    loadSkills: async () => ({
      skillDirCommands: [command('skill-dir')],
      pluginSkills: [command('plugin-skill')],
      bundledSkills: [command('bundled')],
      builtinPluginSkills: [command('builtin-plugin')],
    }),
    loadPluginCommands: async () => [command('plugin-command')],
    loadWorkflowCommands: async () => [command('workflow')],
    builtInCommands: [command('builtin')],
  })

  expect(result.map(command => command.name)).toEqual([
    'bundled',
    'builtin-plugin',
    'skill-dir',
    'workflow',
    'plugin-command',
    'plugin-skill',
    'builtin',
  ])
})

test('core command loading tolerates missing workflow loader', async () => {
  const command = (name: string) => ({ name } as Command)
  const result = await loadAllCommandSources({
    cwd: '/tmp/project',
    loadSkills: async () => ({
      skillDirCommands: [],
      pluginSkills: [],
      bundledSkills: [],
      builtinPluginSkills: [],
    }),
    loadPluginCommands: async () => [command('plugin-command')],
    loadWorkflowCommands: null,
    builtInCommands: [command('builtin')],
  })

  expect(result.map(command => command.name)).toEqual([
    'plugin-command',
    'builtin',
  ])
})
