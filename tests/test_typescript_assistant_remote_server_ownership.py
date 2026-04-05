from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT
    / "languages"
    / "typescript"
    / "docs"
    / "assistant-remote-server-ownership.md"
)

EXPECTED_ROWS = [
    "references/typescript/src/assistant/AssistantSessionChooser.tsx",
    "references/typescript/src/assistant/sessionHistory.ts",
    "references/typescript/src/upstreamproxy/upstreamproxy.ts",
    "references/typescript/src/upstreamproxy/relay.ts",
    "references/typescript/src/remote/remotePermissionBridge.ts",
    "references/typescript/src/remote/RemoteSessionManager.ts",
    "references/typescript/src/remote/sdkMessageAdapter.ts",
    "references/typescript/src/remote/SessionsWebSocket.ts",
    "references/typescript/src/server/createDirectConnectSession.ts",
    "references/typescript/src/server/types.ts",
    "references/typescript/src/server/directConnectManager.ts",
]


def test_typescript_special_surface_rows_have_feature_pack_rationale() -> None:
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert (
        "clear future feature-pack owner or an explicit archive-only rationale" in text
    )
    for row in EXPECTED_ROWS:
        assert row in text


def test_typescript_special_surface_cluster_size_matches_remaining_rows() -> None:
    assert len(EXPECTED_ROWS) == 11
