import json
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
MATRIX_PATH = REPO_ROOT / "shared" / "docs" / "capability-matrix.json"
CONTROLS_PATH = REPO_ROOT / "shared" / "docs" / "completeness-controls.md"


def _load_matrix() -> dict:
    return json.loads(MATRIX_PATH.read_text(encoding="utf-8"))


def _read_controls() -> str:
    return CONTROLS_PATH.read_text(encoding="utf-8")


def test_silent_loss() -> None:
    controls = _read_controls()
    matrix = _load_matrix()
    rows = matrix["capabilities"]

    assert "# Completeness Controls" in controls
    assert "Validation MUST fail when any of the following occurs:" in controls

    assert rows, "capability matrix must not be empty"
    for row in rows:
        assert row.get("targetBucket"), (
            f"missing targetBucket for {row.get('capabilityId')}"
        )
        assert row.get("implementationState"), (
            f"missing implementationState for {row.get('capabilityId')}"
        )

    required_reference_only_guards = {
        "services-cluster-decomposition",
        "components-cluster-decomposition",
        "hooks-cluster-decomposition",
        "command-inventory-decomposition",
        "tool-inventory-decomposition",
        "utils-cluster-decomposition",
    }
    seen = {
        row["capabilityId"] for row in rows if row["targetBucket"] == "reference-only"
    }
    assert required_reference_only_guards.issubset(seen)


def test_runtime_reference_binding() -> None:
    controls = _read_controls()

    assert (
        "The following path forms are forbidden in runtime-facing manifests, config payloads, and generated-project contracts:"
        in controls
    )

    forbidden_fragments = (
        "references/",
        "reference_data/",
        "archive/claw_code_ts_snapshot/",
    )

    runtime_like_payloads = [
        {
            "schemaVersion": "1",
            "kind": "language-pack",
            "id": "python",
            "runtime": {
                "configFile": "agentkit.toml",
                "genericWorkspaceRoot": ".agents",
            },
            "templates": {"base": "template/base"},
            "supports": {
                "init": True,
                "featureAdd": True,
                "doctor": True,
            },
            "featureRegistry": "features/registry.json",
        },
        {
            "schemaVersion": "1",
            "kind": "language-pack",
            "id": "python",
            "runtime": {
                "configFile": "references/config.json",
                "genericWorkspaceRoot": ".agents",
            },
            "templates": {"base": "template/base"},
            "supports": {
                "init": True,
                "featureAdd": True,
                "doctor": True,
            },
            "featureRegistry": "features/registry.json",
        },
        {
            "schemaVersion": "1",
            "kind": "language-pack",
            "id": "python",
            "runtime": {
                "configFile": "agentkit.toml",
                "genericWorkspaceRoot": ".agents",
            },
            "templates": {"base": "archive/claw_code_ts_snapshot/template/base"},
            "supports": {
                "init": True,
                "featureAdd": True,
                "doctor": True,
            },
            "featureRegistry": "features/registry.json",
        },
    ]

    def has_forbidden_runtime_binding(payload: object) -> bool:
        if isinstance(payload, dict):
            return any(
                has_forbidden_runtime_binding(value) for value in payload.values()
            )
        if isinstance(payload, list):
            return any(has_forbidden_runtime_binding(value) for value in payload)
        if isinstance(payload, str):
            return any(fragment in payload for fragment in forbidden_fragments)
        return False

    assert has_forbidden_runtime_binding(runtime_like_payloads[0]) is False
    assert has_forbidden_runtime_binding(runtime_like_payloads[1]) is True
    assert has_forbidden_runtime_binding(runtime_like_payloads[2]) is True
