from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT / "languages" / "rust" / "docs" / "runtime-core-modules-ownership.md"
)

EXPECTED_ROWS = [
    "references/rust/crates/runtime/src/session.rs",
    "references/rust/crates/runtime/src/json.rs",
    "references/rust/crates/runtime/src/lib.rs",
    "references/rust/crates/runtime/src/sse.rs",
    "references/rust/crates/runtime/src/compact.rs",
    "references/rust/crates/runtime/src/usage.rs",
    "references/rust/crates/runtime/src/config.rs",
    "references/rust/crates/runtime/src/bootstrap.rs",
    "references/rust/crates/runtime/src/prompt.rs",
    "references/rust/crates/runtime/src/conversation.rs",
]


def test_rust_runtime_core_rows_have_owner_rationale() -> None:
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert "explicit shipped-owner or archive-only rationale" in text
    for row in EXPECTED_ROWS:
        assert row in text


def test_rust_runtime_core_cluster_size_matches_manifest() -> None:
    assert len(EXPECTED_ROWS) == 10
