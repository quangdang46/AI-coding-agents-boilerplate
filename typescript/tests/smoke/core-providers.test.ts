import { expect, test } from 'bun:test'
import {
  DEFAULT_CODEX_MODEL,
  getAPIProvider,
  isCodexModel,
  mapClaudeModelToCodex,
} from '../../src/core/providers/index.js'

test('core provider selection respects environment flags', () => {
  const prevOpenAI = process.env.CLAUDE_CODE_USE_OPENAI
  const prevBedrock = process.env.CLAUDE_CODE_USE_BEDROCK

  try {
    delete process.env.CLAUDE_CODE_USE_BEDROCK
    delete process.env.CLAUDE_CODE_USE_OPENAI
    expect(getAPIProvider()).toBe('firstParty')

    process.env.CLAUDE_CODE_USE_OPENAI = '1'
    expect(getAPIProvider()).toBe('openai')

    process.env.CLAUDE_CODE_USE_BEDROCK = '1'
    expect(getAPIProvider()).toBe('bedrock')
  } finally {
    if (prevOpenAI === undefined) delete process.env.CLAUDE_CODE_USE_OPENAI
    else process.env.CLAUDE_CODE_USE_OPENAI = prevOpenAI
    if (prevBedrock === undefined) delete process.env.CLAUDE_CODE_USE_BEDROCK
    else process.env.CLAUDE_CODE_USE_BEDROCK = prevBedrock
  }
})

test('core openai provider helpers expose codex model mapping', () => {
  expect(isCodexModel(DEFAULT_CODEX_MODEL)).toBe(true)
  expect(mapClaudeModelToCodex('claude-sonnet-4-6')).toBe('gpt-5.2-codex')
  expect(mapClaudeModelToCodex('claude-opus-4-6')).toBe('gpt-5.1-codex-max')
})
