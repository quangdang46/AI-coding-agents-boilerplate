import { expect, test } from 'bun:test'
import { getCommand, getMcpSkillCommands, hasCommand } from '../../src/core/registry/commands.js'
import { getActiveAgentsFromList } from '../../src/core/registry/agents.js'
import type { Command } from '../../src/types/command.js'
import type { AgentDefinition } from '../../src/core/registry/agents.js'

test('core command registry helpers resolve commands by name and alias', () => {
  const commands = [
    {
      type: 'local',
      name: 'verify',
      description: 'Run verification',
      aliases: ['check'],
      isEnabled: () => true,
      call: async () => ({ type: 'message', messageType: 'system', content: 'ok' }),
    },
  ] as unknown as Command[]

  expect(hasCommand('verify', commands)).toBe(true)
  expect(hasCommand('check', commands)).toBe(true)
  expect(getCommand('check', commands)?.name).toBe('verify')
})

test('core agent registry helper keeps the latest agent definition per type', () => {
  const builtIn = {
    agentType: 'executor',
    whenToUse: 'Built-in executor',
    source: 'built-in',
    baseDir: 'built-in',
    getSystemPrompt: () => 'builtin',
  } as AgentDefinition

  const project = {
    agentType: 'executor',
    whenToUse: 'Project executor',
    source: 'projectSettings',
    getSystemPrompt: () => 'project',
  } as AgentDefinition

  const active = getActiveAgentsFromList([builtIn, project])
  expect(active).toHaveLength(1)
  expect(active[0]?.whenToUse).toBe('Project executor')
})


test('core registry MCP helper returns an array result without mutating input', () => {
  const commands = [
    { type: 'prompt', loadedFrom: 'mcp', disableModelInvocation: false, name: 'usable' },
    { type: 'prompt', loadedFrom: 'mcp', disableModelInvocation: true, name: 'hidden' },
  ] as unknown as Command[]

  const before = JSON.stringify(commands)
  const filtered = getMcpSkillCommands(commands)
  expect(Array.isArray(filtered)).toBe(true)
  expect(JSON.stringify(commands)).toBe(before)
})
