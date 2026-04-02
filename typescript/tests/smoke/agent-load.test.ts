import { expect, test } from 'bun:test'
import { mkdtemp, mkdir, rm, writeFile } from 'node:fs/promises'
import { join } from 'node:path'
import { tmpdir } from 'node:os'
import {
  clearAgentDefinitionsCache,
  getAgentDefinitionsWithOverrides,
} from '../../src/core/registry/agents.js'

test('project-local agent definitions are discoverable', async () => {
  const root = await mkdtemp(join(tmpdir(), 'aicd-agent-'))
  try {
    const agentsDir = join(root, '.claude', 'agents')
    await mkdir(agentsDir, { recursive: true })
    await writeFile(
      join(agentsDir, 'migration-executor.md'),
      `---\nname: migration-executor\ndescription: Use for TypeScript boilerplate migration work\ntools: Read,Edit,Bash\nmodel: inherit\n---\n\nRead the relevant files first, then implement the migration step.\n`,
    )

    clearAgentDefinitionsCache()
    const result = await getAgentDefinitionsWithOverrides(root)
    expect(
      result.activeAgents.some(agent => agent.agentType === 'migration-executor'),
    ).toBe(true)
  } finally {
    clearAgentDefinitionsCache()
    await rm(root, { recursive: true, force: true })
  }
})
