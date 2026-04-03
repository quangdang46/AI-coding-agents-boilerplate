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
        "top-level-runtime-registries",
        "main-cli-runtime",
        "interactive-ui-screens",
        "skill-pipeline",
        "plugin-system",
        "bridge-system",
        "voice-system",
        "task-system",
        "services-ecosystem",
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
        }
        assert entry["reason"]


def test_gate_blocks_early_extraction() -> None:
    mapping = _load_map()
    prereqs = mapping["prerequisites"]

    assert prereqs["pythonProvingSliceRequired"] is True
    assert prereqs["installerManifestDiscoveryRequired"] is True
    assert prereqs["referenceIsolationRequired"] is True
