import { expect, test } from 'bun:test'
import { file } from 'bun'
import { getDefaultPromptAssetPaths } from '../../src/core/prompts/index.js'

test('core prompt assets are present at the expected paths', async () => {
  const assets = getDefaultPromptAssetPaths()

  expect(await file(assets.system).exists()).toBe(true)
  for (const section of assets.sections) {
    expect(await file(section).exists()).toBe(true)
  }
})
