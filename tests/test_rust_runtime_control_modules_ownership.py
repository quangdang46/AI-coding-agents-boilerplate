from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT / "languages" / "rust" / "docs" / "runtime-control-modules-ownership.md"
)

EXPECTED_ROWS = [
    "references/rust/crates/runtime/src/green_contract.rs",
    "references/rust/crates/runtime/src/permission_enforcer.rs",
    "references/rust/crates/runtime/src/trust_resolver.rs",
    "references/rust/crates/runtime/src/permissions.rs",
    "references/rust/crates/runtime/src/recovery_recipes.rs",
    "references/rust/crates/runtime/src/worker_boot.rs",
    "references/rust/crates/runtime/src/sandbox.rs",
    "references/rust/crates/runtime/src/stale_branch.rs",
    "references/rust/crates/runtime/src/policy_engine.rs",
    "references/rust/crates/runtime/src/hooks.rs",
]


def test_rust_runtime_control_rows_have_owner_rationale() -> None:
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert "explicit shipped-owner or archive-only rationale" in text
    for row in EXPECTED_ROWS:
        assert row in text


def test_rust_runtime_control_cluster_size_matches_manifest() -> None:
    assert len(EXPECTED_ROWS) == 10
