import { expect, test } from 'bun:test'
import { loadSkillSourcesWithFallbacks } from '../../src/core/registry/commands.js'
import type { Command } from '../../src/types/command.js'

test('core skill-source loader combines successful sources', async () => {
  const command = (name: string) => ({ name } as Command)
  const result = await loadSkillSourcesWithFallbacks({
    cwd: '/tmp/project',
    loadSkillDirCommands: async () => [command('skill-dir')],
    loadPluginSkills: async () => [command('plugin-skill')],
    getBundledSkills: () => [command('bundled')],
    getBuiltinPluginSkillCommands: () => [command('builtin-plugin')],
    onError: () => {},
    onDebug: () => {},
  })

  expect(result.skillDirCommands.map(command => command.name)).toEqual(['skill-dir'])
  expect(result.pluginSkills.map(command => command.name)).toEqual(['plugin-skill'])
  expect(result.bundledSkills.map(command => command.name)).toEqual(['bundled'])
  expect(result.builtinPluginSkills.map(command => command.name)).toEqual(['builtin-plugin'])
})

test('core skill-source loader falls back when a loader rejects', async () => {
  const debug: string[] = []
  const errors: unknown[] = []
  const result = await loadSkillSourcesWithFallbacks({
    cwd: '/tmp/project',
    loadSkillDirCommands: async () => {
      throw new Error('boom')
    },
    loadPluginSkills: async () => [],
    getBundledSkills: () => [],
    getBuiltinPluginSkillCommands: () => [],
    onError: error => errors.push(error),
    onDebug: message => debug.push(message),
  })

  expect(result.skillDirCommands).toEqual([])
  expect(errors).toHaveLength(1)
  expect(debug.some(message => message.includes('Skill directory commands failed'))).toBe(true)
})
