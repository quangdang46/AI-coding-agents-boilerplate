import { expect, test } from 'bun:test'
import {
  buildBaseToolCatalog,
  buildSimpleModeToolSet,
} from '../../src/core/registry/tools.js'

test('core tool catalog combines always/search/optional tools in order', () => {
  const tool = (name: string) => ({ name }) as any
  const result = buildBaseToolCatalog({
    always: [tool('always-a')],
    search: [tool('search-a')],
    optional: [tool('optional-a'), null, undefined],
  })

  expect(result.map(tool => tool.name)).toEqual([
    'always-a',
    'search-a',
    'optional-a',
  ])
})

test('core simple-mode tool set builds REPL-only variant when repl is enabled', () => {
  const filter = (tools: any[]) => tools
  const result = buildSimpleModeToolSet({
    replTool: { name: 'repl' } as any,
    taskStopTool: { name: 'task-stop' } as any,
    sendMessageTool: { name: 'send-message' } as any,
    bashTool: { name: 'bash' } as any,
    fileReadTool: { name: 'read' } as any,
    fileEditTool: { name: 'edit' } as any,
    replModeEnabled: true,
    coordinatorModeEnabled: true,
    permissionContext: {} as any,
    filterToolsByDenyRules: filter as any,
  })

  expect(result.map(tool => tool.name)).toEqual(['repl', 'task-stop', 'send-message'])
})
