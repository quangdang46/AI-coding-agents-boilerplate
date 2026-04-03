import json
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
MATRIX_PATH = REPO_ROOT / "shared" / "docs" / "capability-matrix.json"


def _load_matrix() -> dict:
    return json.loads(MATRIX_PATH.read_text(encoding="utf-8"))


def test_capability_matrix_completeness() -> None:
    matrix = _load_matrix()
    rows = matrix["capabilities"]

    assert matrix["version"] == "1"
    assert rows, "capability matrix must not be empty"

    allowed_buckets = {"core", "feature-pack", "reference-only", "deferred", "rejected"}
    allowed_states = {"declared", "implemented", "verified"}

    for row in rows:
        assert row["capabilityId"]
        assert row["userFacingName"]
        assert row["sourceEvidence"]
        assert row["sourceSubsystems"]
        assert row["targetBucket"] in allowed_buckets
        assert row["implementationState"] in allowed_states


def test_capability_matrix_cluster_guard() -> None:
    rows = _load_matrix()["capabilities"]
    by_id = {row["capabilityId"]: row for row in rows}

    required_decomposition_guards = {
        "services-cluster-decomposition": "services",
        "components-cluster-decomposition": "components",
        "hooks-cluster-decomposition": "hooks",
        "command-inventory-decomposition": "commands",
        "tool-inventory-decomposition": "tools",
        "utils-cluster-decomposition": "utils",
    }

    for capability_id, subsystem in required_decomposition_guards.items():
        assert capability_id in by_id, f"missing decomposition guard: {capability_id}"
        row = by_id[capability_id]
        assert row["targetBucket"] == "reference-only"
        assert subsystem in row["sourceSubsystems"]

    forbidden_one_to_one_mirrors = {
        "services",
        "components",
        "hooks",
        "commands",
        "tools",
        "utils",
    }
    for row in rows:
        if row["capabilityId"] in required_decomposition_guards:
            continue
        assert row["capabilityId"] not in forbidden_one_to_one_mirrors
