from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT / "languages" / "rust" / "docs" / "runtime-advanced-modules-ownership.md"
)

EXPECTED_ROWS = [
    "references/rust/crates/runtime/src/team_cron_registry.rs",
    "references/rust/crates/runtime/src/mcp_tool_bridge.rs",
    "references/rust/crates/runtime/src/summary_compression.rs",
    "references/rust/crates/runtime/src/plugin_lifecycle.rs",
    "references/rust/crates/runtime/src/mcp.rs",
    "references/rust/crates/runtime/src/mcp_client.rs",
    "references/rust/crates/runtime/src/task_registry.rs",
    "references/rust/crates/runtime/src/mcp_stdio.rs",
    "references/rust/crates/runtime/src/lsp_client.rs",
    "references/rust/crates/runtime/src/task_packet.rs",
    "references/rust/crates/runtime/src/remote.rs",
    "references/rust/crates/runtime/src/mcp_lifecycle_hardened.rs",
    "references/rust/crates/runtime/src/session_control.rs",
]


def test_rust_runtime_advanced_rows_have_owner_rationale() -> None:
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert "explicit shipped-owner or archive-only rationale" in text
    for row in EXPECTED_ROWS:
        assert row in text


def test_rust_runtime_advanced_cluster_size_matches_manifest() -> None:
    assert len(EXPECTED_ROWS) == 13
