import { expect, test } from 'bun:test'
import { getAllBaseTools, getEnabledToolNames, parseToolPreset } from '../../src/core/registry/tools.js'

test('core tools registry exposes a non-empty built-in tool set', () => {
  const tools = getAllBaseTools()
  expect(tools.length).toBeGreaterThan(0)
  expect(tools.some(tool => tool.name.toLowerCase().includes('bash'))).toBe(true)
  expect(tools.some(tool => tool.name.toLowerCase().includes('read'))).toBe(true)
})


test('core tools helpers parse presets and derive enabled tool names', () => {
  expect(parseToolPreset('DEFAULT')).toBe('default')
  expect(parseToolPreset('unknown')).toBeNull()

  const names = getEnabledToolNames([
    { name: 'alpha', isEnabled: () => true },
    { name: 'beta', isEnabled: () => false },
  ])
  expect(names).toEqual(['alpha'])
})
