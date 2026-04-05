from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = REPO_ROOT / "languages" / "rust" / "docs" / "api-crate-ownership.md"

EXPECTED_ROWS = [
    "references/rust/crates/api/Cargo.toml",
    "references/rust/crates/api/tests/client_integration.rs",
    "references/rust/crates/api/tests/provider_client_integration.rs",
    "references/rust/crates/api/tests/openai_compat_integration.rs",
    "references/rust/crates/api/src/client.rs",
    "references/rust/crates/api/src/error.rs",
    "references/rust/crates/api/src/lib.rs",
    "references/rust/crates/api/src/types.rs",
    "references/rust/crates/api/src/sse.rs",
    "references/rust/crates/api/src/providers/mod.rs",
    "references/rust/crates/api/src/providers/anthropic.rs",
    "references/rust/crates/api/src/providers/openai_compat.rs",
    "references/rust/crates/api/src/prompt_cache.rs",
]


def test_rust_api_crate_rows_have_owner_rationale() -> None:
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert "explicit shipped-owner or archive-only rationale" in text
    for row in EXPECTED_ROWS:
        assert row in text


def test_rust_api_crate_cluster_size_matches_manifest() -> None:
    assert len(EXPECTED_ROWS) == 13
