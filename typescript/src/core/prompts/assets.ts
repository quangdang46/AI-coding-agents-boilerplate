export const DEFAULT_PROMPT_ASSET_PATHS = {
  system: 'src/core/prompts/default-system.md',
  sections: [
    'src/core/prompts/sections/style.md',
    'src/core/prompts/sections/verification.md',
    'src/core/prompts/sections/security.md',
  ],
} as const

export function getDefaultPromptAssetPaths() {
  return {
    system: DEFAULT_PROMPT_ASSET_PATHS.system,
    sections: [...DEFAULT_PROMPT_ASSET_PATHS.sections],
  }
}
