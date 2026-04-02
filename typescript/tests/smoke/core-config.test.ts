import { expect, test } from 'bun:test'
import { getDefaultBoilerplateConfig } from '../../src/core/config/index.js'

test('default boilerplate config exposes the base template surface', () => {
  const config = getDefaultBoilerplateConfig()

  expect(config.app?.binaryName).toBe('agentkit')
  expect(config.prompts?.systemPath).toBe('.agent/prompts/system.md')
  expect(config.tools?.enabled).toContain('bash')
  expect(config.agents?.enabled).toContain('executor')
  expect(config.skills?.enabled).toContain('verify')
})
