import { expect, test } from 'bun:test'
import {
  buildCommandCatalog,
  buildCommandNameSet,
} from '../../src/core/registry/commands.js'
import type { Command } from '../../src/types/command.js'

const command = (name: string, aliases?: string[]) =>
  ({ name, aliases } as Command)

test('core command catalog keeps primary and optional ordering while gating internal commands', () => {
  const catalog = buildCommandCatalog({
    primary: [command('alpha'), command('beta')],
    optional: [command('gamma'), null, false, command('delta')],
    internalOnly: [command('internal')],
    includeInternalOnly: true,
  })

  expect(catalog.map(entry => entry.name)).toEqual([
    'alpha',
    'beta',
    'gamma',
    'delta',
    'internal',
  ])
})

test('core command catalog omits internal commands when gating is disabled', () => {
  const catalog = buildCommandCatalog({
    primary: [command('alpha')],
    optional: [],
    internalOnly: [command('internal')],
    includeInternalOnly: false,
  })

  expect(catalog.map(entry => entry.name)).toEqual(['alpha'])
})

test('core command catalog derives command names and aliases', () => {
  const names = buildCommandNameSet([
    command('verify', ['check', 'audit']),
    command('plan'),
  ])

  expect(Array.from(names)).toEqual(['verify', 'check', 'audit', 'plan'])
})
