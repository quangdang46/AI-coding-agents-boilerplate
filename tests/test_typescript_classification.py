import json
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
MAP_PATH = REPO_ROOT / "shared" / "docs" / "typescript-decomposition-map.json"


def _load_map() -> dict:
    return json.loads(MAP_PATH.read_text(encoding="utf-8"))


def test_complete_map() -> None:
    mapping = _load_map()
    subsystems = mapping["subsystems"]
    assert mapping["version"] == "1"
    assert mapping["prerequisites"]["pythonProvingSliceRequired"] is True
    assert mapping["prerequisites"]["installerManifestDiscoveryRequired"] is True
    assert mapping["prerequisites"]["referenceIsolationRequired"] is True

    expected_ids = {
        "core-engine",
        "core-prompts",
        "core-config",
        "core-registry",
        "core-providers",
        "base-template-config",
        "root-archive-metadata",
        "top-level-runtime-registries",
        "main-cli-runtime",
        "interactive-ui-screens",
        "skill-pipeline",
        "plugin-system",
        "bridge-system",
        "voice-system",
        "task-system",
        "root-support-miscellany",
        "services-core-config-context",
        "services-core-session-state",
        "services-core-tool-execution",
        "services-api-auth-bootstrap",
        "services-mcp-lsp",
        "services-memory-suggestions",
        "services-plugin-management",
        "services-remote-settings-sync",
        "services-voice-and-presence",
        "services-product-support",
        "services-analytics-policy",
        "commands-auth-remote-settings",
        "commands-auth-reference-only",
        "commands-mcp-integration",
        "commands-plugin-marketplace",
        "commands-app-installs",
        "commands-remote-and-bridge",
        "commands-device-integrations",
        "commands-reference-only-product-ops",
        "commands-install-and-onboarding-ui",
        "commands-teleport-and-navigation",
        "commands-reference-only-session-debug",
        "commands-advisor-workflow",
        "state-layer",
        "schema-files",
    }

    actual_ids = {entry["id"] for entry in subsystems}
    assert actual_ids == expected_ids

    for entry in subsystems:
        assert entry["paths"]
        assert entry["destination"] in {
            "languages/typescript/runtime",
            "languages/typescript/template/base",
            "future-feature-packs",
            "reference-only",
            "languages/typescript/features/oauth-onboarding",
            "languages/typescript/features/mcp-integration and languages/typescript/features/lsp-tooling",
            "languages/typescript/features/prompt-suggestion-services plus languages/typescript/features/session-memory and languages/typescript/features/team-memory",
        }
        assert entry["reason"]


def test_services_cluster_is_explicitly_decomposed() -> None:
    mapping = _load_map()
    subsystems = {entry["id"]: entry for entry in mapping["subsystems"]}

    assert "services-ecosystem" not in subsystems

    assert (
        subsystems["services-core-config-context"]["destination"]
        == "languages/typescript/runtime"
    )
    assert (
        "languages/typescript/runtime/context/loadContextState.ts"
        in subsystems["services-core-config-context"]["paths"]
    )

    assert (
        subsystems["services-core-session-state"]["destination"]
        == "languages/typescript/runtime"
    )
    assert (
        "languages/typescript/runtime/state/sessionState.ts"
        in subsystems["services-core-session-state"]["paths"]
    )

    assert (
        subsystems["services-core-tool-execution"]["destination"]
        == "languages/typescript/runtime"
    )
    assert (
        "languages/typescript/runtime/utils/toolExecution.ts"
        in subsystems["services-core-tool-execution"]["paths"]
    )

    assert (
        subsystems["services-api-auth-bootstrap"]["destination"]
        == "languages/typescript/features/oauth-onboarding"
    )
    assert (
        "references/typescript/src/services/api"
        in subsystems["services-api-auth-bootstrap"]["paths"]
    )
    assert (
        "references/typescript/src/services/oauth"
        in subsystems["services-api-auth-bootstrap"]["paths"]
    )

    assert (
        subsystems["services-mcp-lsp"]["destination"]
        == "languages/typescript/features/mcp-integration and languages/typescript/features/lsp-tooling"
    )
    assert (
        "references/typescript/src/services/mcp"
        in subsystems["services-mcp-lsp"]["paths"]
    )
    assert (
        "references/typescript/src/services/lsp"
        in subsystems["services-mcp-lsp"]["paths"]
    )

    assert (
        subsystems["services-memory-suggestions"]["destination"]
        == "languages/typescript/features/prompt-suggestion-services plus languages/typescript/features/session-memory and languages/typescript/features/team-memory"
    )
    assert (
        "references/typescript/src/services/compact"
        in subsystems["services-memory-suggestions"]["paths"]
    )
    assert (
        "references/typescript/src/services/PromptSuggestion"
        in subsystems["services-memory-suggestions"]["paths"]
    )

    assert (
        subsystems["services-plugin-management"]["destination"]
        == "future-feature-packs"
    )
    assert (
        "references/typescript/src/services/plugins"
        in subsystems["services-plugin-management"]["paths"]
    )

    assert (
        subsystems["services-remote-settings-sync"]["destination"] == "reference-only"
    )
    assert (
        "references/typescript/src/services/remoteManagedSettings"
        in subsystems["services-remote-settings-sync"]["paths"]
    )

    assert (
        subsystems["services-voice-and-presence"]["destination"]
        == "future-feature-packs"
    )
    assert (
        "references/typescript/src/services/voice.ts"
        in subsystems["services-voice-and-presence"]["paths"]
    )

    assert subsystems["services-product-support"]["destination"] == "reference-only"
    assert (
        "references/typescript/src/services/autoDream"
        in subsystems["services-product-support"]["paths"]
    )

    assert (
        subsystems["root-support-miscellany"]["destination"] == "future-feature-packs"
    )
    assert (
        "references/typescript/src/ink.ts"
        in subsystems["root-support-miscellany"]["paths"]
    )
    assert (
        "references/typescript/src/outputStyles/loadOutputStylesDir.ts"
        in subsystems["root-support-miscellany"]["paths"]
    )
    assert (
        "references/typescript/src/coordinator/coordinatorMode.ts"
        in subsystems["root-support-miscellany"]["paths"]
    )
    assert (
        "references/typescript/src/vendor/ripgrep/x64-linux/rg"
        in subsystems["root-support-miscellany"]["paths"]
    )

    assert subsystems["services-analytics-policy"]["destination"] == "reference-only"
    assert (
        "references/typescript/src/services/analytics"
        in subsystems["services-analytics-policy"]["paths"]
    )


def test_auth_and_remote_command_families_have_explicit_disposition() -> None:
    mapping = _load_map()
    subsystems = {entry["id"]: entry for entry in mapping["subsystems"]}

    assert (
        subsystems["commands-auth-remote-settings"]["destination"]
        == "languages/typescript/features/oauth-onboarding"
    )
    for path in [
        "references/typescript/src/commands/login",
        "references/typescript/src/commands/logout",
        "references/typescript/src/commands/privacy-settings",
        "references/typescript/src/commands/remote-env",
        "references/typescript/src/commands/onboarding",
    ]:
        assert path in subsystems["commands-auth-remote-settings"]["paths"]

    assert subsystems["commands-auth-reference-only"]["destination"] == "reference-only"
    for path in [
        "references/typescript/src/commands/env",
        "references/typescript/src/commands/oauth-refresh",
    ]:
        assert path in subsystems["commands-auth-reference-only"]["paths"]


def test_integration_heavy_command_families_have_explicit_disposition() -> None:
    mapping = _load_map()
    subsystems = {entry["id"]: entry for entry in mapping["subsystems"]}

    assert (
        subsystems["commands-mcp-integration"]["destination"]
        == "languages/typescript/features/mcp-integration and languages/typescript/features/lsp-tooling"
    )
    for path in [
        "references/typescript/src/commands/mcp",
        "references/typescript/src/entrypoints/mcp.ts",
    ]:
        assert path in subsystems["commands-mcp-integration"]["paths"]

    assert (
        subsystems["commands-plugin-marketplace"]["destination"]
        == "future-feature-packs"
    )
    for path in [
        "references/typescript/src/commands/plugin",
        "references/typescript/src/commands/createMovedToPluginCommand.ts",
        "references/typescript/src/services/plugins/pluginCliCommands.ts",
    ]:
        assert path in subsystems["commands-plugin-marketplace"]["paths"]

    assert subsystems["commands-app-installs"]["destination"] == "future-feature-packs"
    for path in [
        "references/typescript/src/commands/install-github-app",
        "references/typescript/src/commands/install-slack-app",
    ]:
        assert path in subsystems["commands-app-installs"]["paths"]

    assert (
        subsystems["commands-remote-and-bridge"]["destination"]
        == "future-feature-packs"
    )
    for path in [
        "references/typescript/src/commands/bridge-kick.ts",
        "references/typescript/src/commands/bridge",
        "references/typescript/src/commands/remote-setup",
        "references/typescript/src/bridge",
        "references/typescript/src/remote",
    ]:
        assert path in subsystems["commands-remote-and-bridge"]["paths"]

    assert (
        subsystems["commands-device-integrations"]["destination"]
        == "future-feature-packs"
    )
    for path in [
        "references/typescript/src/commands/chrome",
        "references/typescript/src/commands/desktop",
        "references/typescript/src/commands/mobile",
        "references/typescript/src/commands/voice",
    ]:
        assert path in subsystems["commands-device-integrations"]["paths"]

    assert (
        subsystems["commands-reference-only-product-ops"]["destination"]
        == "reference-only"
    )
    for path in [
        "references/typescript/src/commands/mock-limits",
        "references/typescript/src/commands/heapdump",
        "references/typescript/src/commands/debug-tool-call",
    ]:
        assert path in subsystems["commands-reference-only-product-ops"]["paths"]

    assert (
        subsystems["commands-install-and-onboarding-ui"]["destination"]
        == "future-feature-packs"
    )
    for path in [
        "references/typescript/src/commands/install-github-app",
        "references/typescript/src/commands/install-slack-app",
        "references/typescript/src/commands/remote-setup",
    ]:
        assert path in subsystems["commands-install-and-onboarding-ui"]["paths"]

    assert (
        subsystems["commands-teleport-and-navigation"]["destination"]
        == "future-feature-packs"
    )
    for path in [
        "references/typescript/src/commands/teleport",
        "references/typescript/src/utils/teleport.tsx",
        "references/typescript/src/utils/teleport",
    ]:
        assert path in subsystems["commands-teleport-and-navigation"]["paths"]

    assert (
        subsystems["commands-reference-only-session-debug"]["destination"]
        == "reference-only"
    )
    for path in [
        "references/typescript/src/commands/summary",
        "references/typescript/src/commands/debug-tool-call",
    ]:
        assert path in subsystems["commands-reference-only-session-debug"]["paths"]

    assert (
        subsystems["commands-advisor-workflow"]["destination"] == "future-feature-packs"
    )
    for path in [
        "references/typescript/src/commands/advisor.ts",
        "references/typescript/src/utils/advisor.ts",
    ]:
        assert path in subsystems["commands-advisor-workflow"]["paths"]


def test_gate_blocks_early_extraction() -> None:
    mapping = _load_map()
    prereqs = mapping["prerequisites"]

    assert prereqs["pythonProvingSliceRequired"] is True
    assert prereqs["installerManifestDiscoveryRequired"] is True
    assert prereqs["referenceIsolationRequired"] is True
