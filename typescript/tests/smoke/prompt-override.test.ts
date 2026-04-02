import { expect, test } from 'bun:test'
import { buildEffectiveSystemPrompt } from '../../src/core/prompts/PromptComposer.js'

test('custom and append system prompts are layered predictably', () => {
  const prompt = buildEffectiveSystemPrompt({
    mainThreadAgentDefinition: undefined,
    toolUseContext: { options: {} as never },
    customSystemPrompt: 'CUSTOM PROMPT',
    defaultSystemPrompt: ['DEFAULT PROMPT'],
    appendSystemPrompt: 'APPEND PROMPT',
  })

  expect(prompt).toEqual(['CUSTOM PROMPT', 'APPEND PROMPT'])
})
