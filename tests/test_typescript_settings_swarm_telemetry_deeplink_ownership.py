from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT
    / "languages"
    / "typescript"
    / "docs"
    / "settings-swarm-telemetry-deeplink-ownership.md"
)

EXPECTED_ROWS = [
    "references/typescript/src/utils/settings/validation.ts",
    "references/typescript/src/utils/settings/pluginOnlyPolicy.ts",
    "references/typescript/src/utils/settings/mdm/rawRead.ts",
    "references/typescript/src/utils/settings/mdm/settings.ts",
    "references/typescript/src/utils/settings/mdm/constants.ts",
    "references/typescript/src/utils/settings/toolValidationConfig.ts",
    "references/typescript/src/utils/settings/changeDetector.ts",
    "references/typescript/src/utils/settings/types.ts",
    "references/typescript/src/utils/settings/permissionValidation.ts",
    "references/typescript/src/utils/settings/applySettingsChange.ts",
    "references/typescript/src/utils/settings/internalWrites.ts",
    "references/typescript/src/utils/settings/validationTips.ts",
    "references/typescript/src/utils/settings/settingsCache.ts",
    "references/typescript/src/utils/settings/allErrors.ts",
    "references/typescript/src/utils/settings/managedPath.ts",
    "references/typescript/src/utils/settings/settings.ts",
    "references/typescript/src/utils/settings/constants.ts",
    "references/typescript/src/utils/settings/schemaOutput.ts",
    "references/typescript/src/utils/settings/validateEditTool.ts",
    "references/typescript/src/utils/swarm/teammateLayoutManager.ts",
    "references/typescript/src/utils/swarm/teammateModel.ts",
    "references/typescript/src/utils/swarm/It2SetupPrompt.tsx",
    "references/typescript/src/utils/swarm/teammatePromptAddendum.ts",
    "references/typescript/src/utils/swarm/teammateInit.ts",
    "references/typescript/src/utils/swarm/reconnection.ts",
    "references/typescript/src/utils/swarm/spawnInProcess.ts",
    "references/typescript/src/utils/swarm/leaderPermissionBridge.ts",
    "references/typescript/src/utils/swarm/inProcessRunner.ts",
    "references/typescript/src/utils/swarm/permissionSync.ts",
    "references/typescript/src/utils/swarm/spawnUtils.ts",
    "references/typescript/src/utils/swarm/teamHelpers.ts",
    "references/typescript/src/utils/swarm/constants.ts",
    "references/typescript/src/utils/swarm/backends/detection.ts",
    "references/typescript/src/utils/swarm/backends/PaneBackendExecutor.ts",
    "references/typescript/src/utils/swarm/backends/ITermBackend.ts",
    "references/typescript/src/utils/swarm/backends/types.ts",
    "references/typescript/src/utils/swarm/backends/registry.ts",
    "references/typescript/src/utils/swarm/backends/it2Setup.ts",
    "references/typescript/src/utils/swarm/backends/teammateModeSnapshot.ts",
    "references/typescript/src/utils/swarm/backends/InProcessBackend.ts",
    "references/typescript/src/utils/swarm/backends/TmuxBackend.ts",
    "references/typescript/src/utils/deepLink/registerProtocol.ts",
    "references/typescript/src/utils/deepLink/banner.ts",
    "references/typescript/src/utils/deepLink/protocolHandler.ts",
    "references/typescript/src/utils/deepLink/terminalLauncher.ts",
    "references/typescript/src/utils/deepLink/parseDeepLink.ts",
    "references/typescript/src/utils/deepLink/terminalPreference.ts",
    "references/typescript/src/utils/telemetry/logger.ts",
    "references/typescript/src/utils/telemetry/instrumentation.ts",
    "references/typescript/src/utils/telemetry/events.ts",
    "references/typescript/src/utils/telemetry/perfettoTracing.ts",
    "references/typescript/src/utils/telemetry/skillLoadedEvent.ts",
    "references/typescript/src/utils/telemetry/bigqueryExporter.ts",
    "references/typescript/src/utils/telemetry/betaSessionTracing.ts",
    "references/typescript/src/utils/telemetry/pluginTelemetry.ts",
    "references/typescript/src/utils/telemetry/sessionTracing.ts",
]


def test_typescript_settings_swarm_telemetry_deeplink_rows_have_owner_rationale() -> (
    None
):
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert "explicit shipped-owner or archive-only rationale" in text
    for row in EXPECTED_ROWS:
        assert row in text


def test_typescript_settings_swarm_telemetry_deeplink_cluster_size_matches_manifest() -> (
    None
):
    assert len(EXPECTED_ROWS) == 56
