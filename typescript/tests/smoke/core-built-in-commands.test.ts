import { expect, test } from 'bun:test'
import {
  BRIDGE_SAFE_COMMANDS,
  REMOTE_SAFE_COMMANDS,
  builtInCommandNames,
  getBuiltInCommands,
} from '../../src/core/registry/commands.js'

function withAuthEnv<T>(fn: () => T): T {
  const previous = process.env.ANTHROPIC_API_KEY
  process.env.ANTHROPIC_API_KEY = previous ?? 'test-key'
  try {
    return fn()
  } finally {
    if (previous === undefined) {
      delete process.env.ANTHROPIC_API_KEY
    } else {
      process.env.ANTHROPIC_API_KEY = previous
    }
  }
}

test('core built-in command helper exposes a non-empty command catalog', () => {
  const commands = withAuthEnv(() => getBuiltInCommands())

  expect(commands.length).toBeGreaterThan(0)
  expect(commands.some(command => command.name === 'help')).toBe(true)
  expect(commands.some(command => command.name === 'plan')).toBe(true)
})

test('core built-in command helper exposes alias/name index and safe command sets', () => {
  expect(withAuthEnv(() => builtInCommandNames()).has('help')).toBe(true)
  expect(Array.from(REMOTE_SAFE_COMMANDS).some(command => command.name === 'session')).toBe(true)
  expect(Array.from(BRIDGE_SAFE_COMMANDS).some(command => command.name === 'compact')).toBe(true)
})
