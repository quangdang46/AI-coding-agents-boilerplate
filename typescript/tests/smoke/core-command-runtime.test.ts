import { expect, test } from 'bun:test'
import {
  clearMemoizedCommandCaches,
  filterEnabledCommands,
  loadSkillToolCommands,
  loadSlashCommandToolSkills,
  loadVisibleCommands,
} from '../../src/core/registry/commands.js'
import type { Command } from '../../src/types/command.js'

const promptCommand = (
  name: string,
  overrides: Partial<Command> = {},
) =>
  ({
    type: 'prompt',
    name,
    description: `${name} description`,
    progressMessage: `${name} progress`,
    contentLength: 0,
    source: 'skills',
    getPromptForCommand: async () => [],
    ...overrides,
  }) as Command

test('core command runtime filters by availability and enabled state', () => {
  const commands = [
    promptCommand('keep'),
    promptCommand('blocked-by-availability'),
    promptCommand('blocked-by-enabled'),
  ]

  const filtered = filterEnabledCommands({
    commands,
    meetsAvailabilityRequirement: cmd => cmd.name !== 'blocked-by-availability',
    isCommandEnabled: cmd => cmd.name !== 'blocked-by-enabled',
  })

  expect(filtered.map(command => command.name)).toEqual(['keep'])
})

test('core command runtime inserts dynamic skills ahead of built-ins', async () => {
  const dynamicSkill = promptCommand('dynamic', {
    hasUserSpecifiedDescription: true,
  })
  const builtIn = promptCommand('builtin', {
    source: 'builtin',
    loadedFrom: 'bundled',
  })

  const commands = await loadVisibleCommands({
    cwd: '/tmp/project',
    loadAllCommands: async () => [builtIn],
    getDynamicSkills: () => [dynamicSkill],
    builtInCommands: [builtIn],
    meetsAvailabilityRequirement: () => true,
    isCommandEnabled: () => true,
  })

  expect(commands.map(command => command.name)).toEqual(['dynamic', 'builtin'])
})

test('core command runtime shares skill-tool selection helpers', async () => {
  const commands = [
    promptCommand('usable-skill', {
      loadedFrom: 'skills',
      hasUserSpecifiedDescription: true,
    }),
    promptCommand('hidden-skill', {
      loadedFrom: 'skills',
      disableModelInvocation: true,
      hasUserSpecifiedDescription: true,
    }),
  ]

  const selected = await loadSkillToolCommands({
    cwd: '/tmp/project',
    getCommands: async () => commands,
  })

  expect(selected.map(command => command.name)).toEqual(['usable-skill'])
})

test('core command runtime swallows slash-skill loading failures', async () => {
  const messages: string[] = []
  const errors: unknown[] = []

  const selected = await loadSlashCommandToolSkills({
    cwd: '/tmp/project',
    getCommands: async () => {
      throw new Error('boom')
    },
    onError: error => errors.push(error),
    onDebug: message => messages.push(message),
  })

  expect(selected).toEqual([])
  expect(errors).toHaveLength(1)
  expect(messages).toContain('Returning empty skills array due to load failure')
})

test('core command runtime clears memoized caches and skill index cache', () => {
  let cleared = 0

  const makeMemoized = () => ({
    cache: {
      clear: () => {
        cleared += 1
      },
    },
  })

  clearMemoizedCommandCaches({
    loadAllCommands: makeMemoized(),
    getSkillToolCommands: makeMemoized(),
    getSlashCommandToolSkills: makeMemoized(),
    clearSkillIndexCache: () => {
      cleared += 1
    },
  })

  expect(cleared).toBe(4)
})
