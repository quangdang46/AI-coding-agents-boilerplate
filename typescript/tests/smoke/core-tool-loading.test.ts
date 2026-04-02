import { expect, test } from 'bun:test'
import { getEmptyToolPermissionContext } from '../../src/Tool.js'
import { buildSimpleModeToolSet, buildStandardModeToolSet } from '../../src/core/registry/tools.js'

const enabledTool = (name: string) => ({
  name,
  isEnabled: () => true,
}) as any

const disabledTool = (name: string) => ({
  name,
  isEnabled: () => false,
}) as any

test('core tool loading prefers repl tool in simple mode when repl is enabled', () => {
  const tools = buildSimpleModeToolSet({
    permissionContext: getEmptyToolPermissionContext(),
    replModeEnabled: true,
    replTool: enabledTool('repl'),
    coordinatorModeEnabled: true,
    taskStopTool: enabledTool('task_stop'),
    sendMessageTool: enabledTool('send_message'),
    bashTool: enabledTool('bash'),
    fileReadTool: enabledTool('file_read'),
    fileEditTool: enabledTool('file_edit'),
    filterToolsByDenyRules: input => [...input],
  })

  expect(tools.map(tool => tool.name)).toEqual(['repl', 'task_stop', 'send_message'])
})

test('core tool loading filters standard tools by exclusions, repl visibility, and enabled state', () => {
  const tools = buildStandardModeToolSet({
    allBaseTools: [
      enabledTool('repl'),
      enabledTool('bash'),
      enabledTool('file_read'),
      enabledTool('keep_me'),
      disabledTool('disabled_tool'),
      enabledTool('special_tool'),
    ] as any,
    permissionContext: getEmptyToolPermissionContext(),
    specialToolNames: new Set(['special_tool']),
    replModeEnabled: true,
    replToolName: 'repl',
    replOnlyTools: new Set(['bash', 'file_read']),
    matchesName: (tool, toolName) => tool.name === toolName,
  })

  expect(tools.map(tool => tool.name)).toEqual(['repl', 'keep_me'])
})
