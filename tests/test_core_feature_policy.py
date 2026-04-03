import json
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
POLICY_PATH = REPO_ROOT / "shared" / "docs" / "core-feature-policy.md"
MATRIX_PATH = REPO_ROOT / "shared" / "docs" / "capability-matrix.json"
FEATURE_SCHEMA_PATH = REPO_ROOT / "shared" / "schemas" / "feature-pack.schema.json"


def _read(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def _load_matrix() -> dict:
    return json.loads(MATRIX_PATH.read_text(encoding="utf-8"))


def test_minimal_core_policy() -> None:
    policy = _read(POLICY_PATH)
    rows = {row["capabilityId"]: row for row in _load_matrix()["capabilities"]}

    assert "# Core vs Feature-Pack Policy" in policy
    assert "Shipped core must stay minimal." in policy
    assert (
        "The following capability families are allowed in core when they are implemented:"
        in policy
    )

    allowed_core = {
        "interactive-repl",
        "local-agent-discovery",
        "session-management-and-export",
        "workspace-bootstrap-init",
        "workspace-file-shell-web-tools",
        "web-fetch-and-search",
        "local-skill-discovery",
        "workspace-instruction-memory",
        "system-prompt-layering",
        "provider-and-model-selection",
        "usage-and-cost-tracking",
        "saved-sessions-and-resume",
        "workspace-context-building",
        "permissions-and-sandbox-safety",
        "workspace-editable-extension-seams",
        "schema-backed-validation",
    }

    actual_core = {
        row_id for row_id, row in rows.items() if row["targetBucket"] == "core"
    }
    assert actual_core == allowed_core

    for forbidden in [
        "plugin-runtime-manifests",
        "plugin-marketplace-lifecycle",
        "mcp-integration-subsystem",
        "hook-runtime",
        "git-and-github-workflow-assistance",
        "bridge-remote-session-control",
        "task-team-orchestration",
    ]:
        assert rows[forbidden]["targetBucket"] != "core"


def test_feature_pack_reversibility_policy() -> None:
    policy = _read(POLICY_PATH)
    schema = json.loads(FEATURE_SCHEMA_PATH.read_text(encoding="utf-8"))
    rows = _load_matrix()["capabilities"]

    assert (
        "If a capability is classified as `feature-pack`, its manifest and implementation must be compatible with reversible add/remove flows."
        in policy
    )
    assert (
        "The first proving-slice catalog for AICD should treat the following as feature-pack families:"
        in policy
    )

    required_fields = set(schema["required"])
    assert {
        "schemaVersion",
        "kind",
        "id",
        "displayName",
        "version",
        "description",
        "appliesTo",
        "dependsOn",
        "adds",
        "patches",
    }.issubset(required_fields)
    assert "conflictsWith" in schema["properties"]

    feature_pack_rows = [row for row in rows if row["targetBucket"] == "feature-pack"]
    assert feature_pack_rows, "expected at least one feature-pack row"
    for row in feature_pack_rows:
        feature_id = row["featureId"]
        assert feature_id is not None
        assert isinstance(feature_id, str) and feature_id
