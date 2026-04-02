import { expect, mock, test } from 'bun:test'
import type { Command } from '../../src/types/command.js'

test('commands without availability are universally available', async () => {
  const { meetsAvailabilityRequirement } = await import(
    '../../src/core/registry/commands.js'
  )
  const command = { type: 'local', name: 'plain' } as unknown as Command
  expect(meetsAvailabilityRequirement(command)).toBe(true)
})

test('console availability depends on auth/provider predicates from collaborators', async () => {
  mock.module('../../src/utils/auth.js', () => ({
    isUsing3PServices: () => false,
    isClaudeAISubscriber: () => false,
  }))
  mock.module('../../src/core/providers/anthropic.js', () => ({
    isFirstPartyAnthropicBaseUrl: () => true,
  }))

  const { meetsAvailabilityRequirement } = await import(
    '../../src/core/registry/commandAvailability.ts'
  )

  const command = {
    type: 'local',
    name: 'console-only',
    availability: ['console'],
  } as unknown as Command

  expect(meetsAvailabilityRequirement(command)).toBe(true)
})
