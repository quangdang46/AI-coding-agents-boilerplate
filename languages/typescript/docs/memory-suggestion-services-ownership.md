# TypeScript memory, tips, prompt suggestion, and summarization services ownership

The archived service files in bead `aicd-3ix.4.11.1.3` are not preserved as a one-for-one shipped services subtree in the current AICD TypeScript proving slice.

The repository already declares optional feature-pack families that match this cluster closely: `session-memory`, `team-memory`, and `prompt-suggestion-services`. The archived service implementations therefore belong to those optional feature directions rather than to the shipped core runtime.

## Ownership and disposition

- `references/typescript/src/services/MagicDocs/magicDocs.ts` — future feature-pack ownership aligned to `prompt-suggestion-services`; Magic Docs behavior is not shipped in the base proving slice.
- `references/typescript/src/services/MagicDocs/prompts.ts` — future feature-pack ownership aligned to `prompt-suggestion-services`; Magic Docs prompting is not shipped in the base proving slice.
- `references/typescript/src/services/teamMemorySync/teamMemSecretGuard.ts` — future feature-pack ownership aligned to `team-memory`; archived team-memory secret-guard logic is not shipped in the base proving slice.
- `references/typescript/src/services/teamMemorySync/types.ts` — future feature-pack ownership aligned to `team-memory`; archived team-memory sync types are not shipped in the base proving slice.
- `references/typescript/src/services/teamMemorySync/secretScanner.ts` — future feature-pack ownership aligned to `team-memory`; archived team-memory scanning is not shipped in the base proving slice.
- `references/typescript/src/services/teamMemorySync/index.ts` — future feature-pack ownership aligned to `team-memory`; team-memory sync orchestration is not shipped in the base proving slice.
- `references/typescript/src/services/teamMemorySync/watcher.ts` — future feature-pack ownership aligned to `team-memory`; team-memory watcher behavior is not shipped in the base proving slice.
- `references/typescript/src/services/tips/tipScheduler.ts` — future feature-pack ownership aligned to `prompt-suggestion-services`; tip scheduling is not shipped in the base proving slice.
- `references/typescript/src/services/tips/tipRegistry.ts` — future feature-pack ownership aligned to `prompt-suggestion-services`; tip registry behavior is not shipped in the base proving slice.
- `references/typescript/src/services/tips/tipHistory.ts` — future feature-pack ownership aligned to `prompt-suggestion-services`; tip history behavior is not shipped in the base proving slice.
- `references/typescript/src/services/compact/compact.ts` — future feature-pack ownership aligned to `prompt-suggestion-services`; archived compact/summarization behavior is not shipped in the base proving slice.
- `references/typescript/src/services/compact/cachedMicrocompact.ts` — future feature-pack ownership aligned to `prompt-suggestion-services`; archived compact caching is not shipped in the base proving slice.
- `references/typescript/src/services/compact/compactWarningHook.ts` — future feature-pack ownership aligned to `prompt-suggestion-services`; compact warning behavior is not shipped in the base proving slice.
- `references/typescript/src/services/compact/compactWarningState.ts` — future feature-pack ownership aligned to `prompt-suggestion-services`; compact warning state is not shipped in the base proving slice.
- `references/typescript/src/services/compact/grouping.ts` — future feature-pack ownership aligned to `prompt-suggestion-services`; compact grouping helpers are not shipped in the base proving slice.
- `references/typescript/src/services/compact/autoCompact.ts` — future feature-pack ownership aligned to `prompt-suggestion-services`; auto-compact behavior is not shipped in the base proving slice.
- `references/typescript/src/services/compact/microCompact.ts` — future feature-pack ownership aligned to `prompt-suggestion-services`; micro-compact behavior is not shipped in the base proving slice.
- `references/typescript/src/services/compact/prompt.ts` — future feature-pack ownership aligned to `prompt-suggestion-services`; compact prompting is not shipped in the base proving slice.
- `references/typescript/src/services/compact/timeBasedMCConfig.ts` — future feature-pack ownership aligned to `prompt-suggestion-services`; time-based compact config is not shipped in the base proving slice.
- `references/typescript/src/services/compact/apiMicrocompact.ts` — future feature-pack ownership aligned to `prompt-suggestion-services`; API micro-compact behavior is not shipped in the base proving slice.
- `references/typescript/src/services/compact/sessionMemoryCompact.ts` — future feature-pack ownership aligned jointly to `session-memory` and `prompt-suggestion-services`; session-memory compacting is not shipped in the base proving slice.
- `references/typescript/src/services/compact/snipProjection.ts` — future feature-pack ownership aligned to `prompt-suggestion-services`; compact projection helpers are not shipped in the base proving slice.
- `references/typescript/src/services/compact/postCompactCleanup.ts` — future feature-pack ownership aligned to `prompt-suggestion-services`; post-compact cleanup is not shipped in the base proving slice.
- `references/typescript/src/services/compact/snipCompact.ts` — future feature-pack ownership aligned to `prompt-suggestion-services`; compact snipping behavior is not shipped in the base proving slice.
- `references/typescript/src/services/compact/cachedMCConfig.ts` — future feature-pack ownership aligned to `prompt-suggestion-services`; compact config caching is not shipped in the base proving slice.
- `references/typescript/src/services/toolUseSummary/toolUseSummaryGenerator.ts` — future feature-pack ownership aligned to `prompt-suggestion-services`; tool-use summarization is not shipped in the base proving slice.
- `references/typescript/src/services/extractMemories/prompts.ts` — future feature-pack ownership aligned to `session-memory`; memory extraction prompts are not shipped in the base proving slice.
- `references/typescript/src/services/extractMemories/extractMemories.ts` — future feature-pack ownership aligned to `session-memory`; memory extraction behavior is not shipped in the base proving slice.
- `references/typescript/src/services/SessionMemory/sessionMemoryUtils.ts` — future feature-pack ownership aligned to `session-memory`; archived session-memory helpers are not shipped in the base proving slice.
- `references/typescript/src/services/SessionMemory/sessionMemory.ts` — future feature-pack ownership aligned to `session-memory`; archived session-memory behavior is not shipped in the base proving slice.
- `references/typescript/src/services/SessionMemory/prompts.ts` — future feature-pack ownership aligned to `session-memory`; session-memory prompting is not shipped in the base proving slice.
- `references/typescript/src/services/PromptSuggestion/promptSuggestion.ts` — future feature-pack ownership aligned to `prompt-suggestion-services`; prompt suggestion behavior is not shipped in the base proving slice.
- `references/typescript/src/services/PromptSuggestion/speculation.ts` — future feature-pack ownership aligned to `prompt-suggestion-services`; prompt speculation behavior is not shipped in the base proving slice.

## Shipped-language-pack rule

This subset is complete when each archived memory/suggestion service row has a clear future feature-pack owner or an explicit archive-only rationale. These service files must not be mistaken for already-shipped TypeScript runtime ownership just because the repository preserves their feature-pack taxonomy.
