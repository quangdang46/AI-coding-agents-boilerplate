from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT
    / "languages"
    / "typescript"
    / "docs"
    / "bootstrap-entrypoint-state-ownership.md"
)

EXPECTED_ROWS = [
    "references/typescript/src/entrypoints/mcp.ts",
    "references/typescript/src/entrypoints/init.ts",
    "references/typescript/src/entrypoints/agentSdkTypes.ts",
    "references/typescript/src/entrypoints/cli.tsx",
    "references/typescript/src/entrypoints/sdk/coreTypes.ts",
    "references/typescript/src/entrypoints/sdk/coreTypes.generated.ts",
    "references/typescript/src/entrypoints/sdk/controlSchemas.ts",
    "references/typescript/src/entrypoints/sdk/toolTypes.ts",
    "references/typescript/src/entrypoints/sdk/runtimeTypes.ts",
    "references/typescript/src/entrypoints/sdk/coreSchemas.ts",
    "references/typescript/src/entrypoints/sandboxTypes.ts",
    "references/typescript/src/context/fpsMetrics.tsx",
    "references/typescript/src/context/mailbox.tsx",
    "references/typescript/src/context/overlayContext.tsx",
    "references/typescript/src/context/stats.tsx",
    "references/typescript/src/context/promptOverlayContext.tsx",
    "references/typescript/src/context/QueuedMessageContext.tsx",
    "references/typescript/src/context/notifications.tsx",
    "references/typescript/src/context/modalContext.tsx",
    "references/typescript/src/context/voice.tsx",
    "references/typescript/src/state/AppState.tsx",
    "references/typescript/src/state/store.ts",
    "references/typescript/src/state/onChangeAppState.ts",
    "references/typescript/src/state/teammateViewHelpers.ts",
    "references/typescript/src/state/AppStateStore.ts",
    "references/typescript/src/state/selectors.ts",
    "references/typescript/src/bootstrap/state.ts",
]


def test_typescript_bootstrap_entrypoint_state_rows_have_archive_or_owner_rationale() -> (
    None
):
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert "explicit archive-only or shipped-owner rationale" in text
    for row in EXPECTED_ROWS:
        assert row in text


def test_typescript_bootstrap_entrypoint_state_cluster_size_matches_expected_rows() -> (
    None
):
    assert len(EXPECTED_ROWS) == 27
