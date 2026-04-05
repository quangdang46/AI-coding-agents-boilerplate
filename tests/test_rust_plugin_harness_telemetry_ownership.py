from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT / "languages" / "rust" / "docs" / "plugin-harness-telemetry-ownership.md"
)

EXPECTED_ROWS = [
    "references/rust/crates/mock-anthropic-service/Cargo.toml",
    "references/rust/crates/mock-anthropic-service/src/main.rs",
    "references/rust/crates/mock-anthropic-service/src/lib.rs",
    "references/rust/crates/compat-harness/Cargo.toml",
    "references/rust/crates/compat-harness/src/lib.rs",
    "references/rust/crates/plugins/bundled/example-bundled/.claude-plugin/plugin.json",
    "references/rust/crates/plugins/bundled/example-bundled/hooks/post.sh",
    "references/rust/crates/plugins/bundled/example-bundled/hooks/pre.sh",
    "references/rust/crates/plugins/bundled/sample-hooks/.claude-plugin/plugin.json",
    "references/rust/crates/plugins/bundled/sample-hooks/hooks/post.sh",
    "references/rust/crates/plugins/bundled/sample-hooks/hooks/pre.sh",
    "references/rust/crates/plugins/Cargo.toml",
    "references/rust/crates/plugins/src/lib.rs",
    "references/rust/crates/plugins/src/hooks.rs",
    "references/rust/crates/telemetry/Cargo.toml",
    "references/rust/crates/telemetry/src/lib.rs",
]


def test_rust_plugin_harness_telemetry_rows_have_owner_rationale() -> None:
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert "explicit shipped-owner or archive-only rationale" in text
    for row in EXPECTED_ROWS:
        assert row in text


def test_rust_plugin_harness_telemetry_cluster_size_matches_manifest() -> None:
    assert len(EXPECTED_ROWS) == 16
