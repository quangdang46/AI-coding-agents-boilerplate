from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT
    / "languages"
    / "typescript"
    / "docs"
    / "memory-suggestion-services-ownership.md"
)

EXPECTED_ROWS = [
    "references/typescript/src/services/MagicDocs/magicDocs.ts",
    "references/typescript/src/services/MagicDocs/prompts.ts",
    "references/typescript/src/services/teamMemorySync/teamMemSecretGuard.ts",
    "references/typescript/src/services/teamMemorySync/types.ts",
    "references/typescript/src/services/teamMemorySync/secretScanner.ts",
    "references/typescript/src/services/teamMemorySync/index.ts",
    "references/typescript/src/services/teamMemorySync/watcher.ts",
    "references/typescript/src/services/tips/tipScheduler.ts",
    "references/typescript/src/services/tips/tipRegistry.ts",
    "references/typescript/src/services/tips/tipHistory.ts",
    "references/typescript/src/services/compact/compact.ts",
    "references/typescript/src/services/compact/cachedMicrocompact.ts",
    "references/typescript/src/services/compact/compactWarningHook.ts",
    "references/typescript/src/services/compact/compactWarningState.ts",
    "references/typescript/src/services/compact/grouping.ts",
    "references/typescript/src/services/compact/autoCompact.ts",
    "references/typescript/src/services/compact/microCompact.ts",
    "references/typescript/src/services/compact/prompt.ts",
    "references/typescript/src/services/compact/timeBasedMCConfig.ts",
    "references/typescript/src/services/compact/apiMicrocompact.ts",
    "references/typescript/src/services/compact/sessionMemoryCompact.ts",
    "references/typescript/src/services/compact/snipProjection.ts",
    "references/typescript/src/services/compact/postCompactCleanup.ts",
    "references/typescript/src/services/compact/snipCompact.ts",
    "references/typescript/src/services/compact/cachedMCConfig.ts",
    "references/typescript/src/services/toolUseSummary/toolUseSummaryGenerator.ts",
    "references/typescript/src/services/extractMemories/prompts.ts",
    "references/typescript/src/services/extractMemories/extractMemories.ts",
    "references/typescript/src/services/SessionMemory/sessionMemoryUtils.ts",
    "references/typescript/src/services/SessionMemory/sessionMemory.ts",
    "references/typescript/src/services/SessionMemory/prompts.ts",
    "references/typescript/src/services/PromptSuggestion/promptSuggestion.ts",
    "references/typescript/src/services/PromptSuggestion/speculation.ts",
]


def test_typescript_memory_suggestion_service_rows_have_owner_rationale() -> None:
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert (
        "clear future feature-pack owner or an explicit archive-only rationale" in text
    )
    for row in EXPECTED_ROWS:
        assert row in text


def test_typescript_memory_suggestion_service_cluster_size_matches_manifest() -> None:
    assert len(EXPECTED_ROWS) == 33
