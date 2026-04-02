import { expect, test } from 'bun:test'
import {
  excludeToolsByName,
  hideReplOnlyToolsWhenReplEnabled,
  keepEnabledTools,
} from '../../src/core/registry/tools.js'

test('core tool filtering excludes tools by name', () => {
  const tools = [{ name: 'a' }, { name: 'b' }, { name: 'c' }]
  expect(excludeToolsByName(tools, new Set(['b'])).map(tool => tool.name)).toEqual([
    'a',
    'c',
  ])
})

test('core tool filtering keeps only enabled tools', () => {
  const tools = [
    { name: 'a', isEnabled: () => true },
    { name: 'b', isEnabled: () => false },
  ]
  expect(keepEnabledTools(tools).map(tool => tool.name)).toEqual(['a'])
})

test('core tool filtering hides repl-only tools only when repl tool is present', () => {
  const tools = [
    { name: 'repl' },
    { name: 'bash' },
    { name: 'file_read' },
  ] as any
  const filtered = hideReplOnlyToolsWhenReplEnabled(
    tools,
    'repl',
    new Set(['bash', 'file_read']),
    (tool, toolName) => tool.name === toolName,
  )
  expect(filtered.map(tool => tool.name)).toEqual(['repl'])
})
