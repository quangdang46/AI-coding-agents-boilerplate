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


def test_gate_blocks_early_extraction() -> None:
    mapping = _load_map()
    prereqs = mapping["prerequisites"]

    assert prereqs["pythonProvingSliceRequired"] is True
    assert prereqs["installerManifestDiscoveryRequired"] is True
    assert prereqs["referenceIsolationRequired"] is True
