from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = REPO_ROOT / "languages" / "typescript" / "docs" / "bridge-ownership.md"

EXPECTED_ROWS = [
    "references/typescript/src/bridge/debugUtils.ts",
    "references/typescript/src/bridge/bridgeStatusUtil.ts",
    "references/typescript/src/bridge/pollConfigDefaults.ts",
    "references/typescript/src/bridge/remoteBridgeCore.ts",
    "references/typescript/src/bridge/bridgeMessaging.ts",
    "references/typescript/src/bridge/bridgeUI.ts",
    "references/typescript/src/bridge/workSecret.ts",
    "references/typescript/src/bridge/jwtUtils.ts",
    "references/typescript/src/bridge/replBridge.ts",
    "references/typescript/src/bridge/trustedDevice.ts",
    "references/typescript/src/bridge/sessionRunner.ts",
    "references/typescript/src/bridge/sessionIdCompat.ts",
    "references/typescript/src/bridge/codeSessionApi.ts",
    "references/typescript/src/bridge/bridgeEnabled.ts",
    "references/typescript/src/bridge/createSession.ts",
    "references/typescript/src/bridge/pollConfig.ts",
    "references/typescript/src/bridge/types.ts",
    "references/typescript/src/bridge/replBridgeHandle.ts",
    "references/typescript/src/bridge/initReplBridge.ts",
    "references/typescript/src/bridge/inboundMessages.ts",
    "references/typescript/src/bridge/flushGate.ts",
    "references/typescript/src/bridge/bridgePointer.ts",
    "references/typescript/src/bridge/bridgeConfig.ts",
    "references/typescript/src/bridge/bridgeMain.ts",
    "references/typescript/src/bridge/envLessBridgeConfig.ts",
    "references/typescript/src/bridge/bridgeApi.ts",
    "references/typescript/src/bridge/bridgePermissionCallbacks.ts",
    "references/typescript/src/bridge/inboundAttachments.ts",
    "references/typescript/src/bridge/capacityWake.ts",
    "references/typescript/src/bridge/replBridgeTransport.ts",
    "references/typescript/src/bridge/bridgeDebug.ts",
]


def test_typescript_bridge_rows_have_feature_pack_or_archive_rationale() -> None:
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert (
        "clear future feature-pack owner or an explicit archive-only rationale" in text
    )
    for row in EXPECTED_ROWS:
        assert row in text


def test_typescript_bridge_cluster_size_matches_bead_manifest() -> None:
    assert len(EXPECTED_ROWS) == 31
