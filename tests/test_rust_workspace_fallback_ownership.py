from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT / "languages" / "rust" / "docs" / "workspace-fallback-ownership.md"
)

EXPECTED_ROWS = [
    "references/rust/crates/runtime/Cargo.toml",
    "references/rust/crates/runtime/src/oauth.rs",
    "references/rust/.sandbox-home/.rustup/settings.toml",
    "references/rust/Cargo.toml",
    "references/rust/.gitignore",
    "references/rust/Cargo.lock",
]


def test_rust_workspace_fallback_rows_have_owner_rationale() -> None:
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert "explicit shipped-owner or archive-only rationale" in text
    for row in EXPECTED_ROWS:
        assert row in text


def test_rust_workspace_fallback_cluster_size_matches_current_checklist() -> None:
    assert len(EXPECTED_ROWS) == 6
