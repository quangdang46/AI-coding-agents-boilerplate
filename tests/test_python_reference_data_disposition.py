from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
DISPOSITION_DOC = (
    REPO_ROOT / "languages" / "python" / "docs" / "reference-data-disposition.md"
)

EXPECTED_REFERENCE_DATA_ROWS = [
    "references/python/src/reference_data/archive_surface_snapshot.json",
    "references/python/src/reference_data/commands_snapshot.json",
    "references/python/src/reference_data/tools_snapshot.json",
    "references/python/src/reference_data/subsystems/skills.json",
    "references/python/src/reference_data/subsystems/voice.json",
    "references/python/src/reference_data/subsystems/vim.json",
    "references/python/src/reference_data/subsystems/state.json",
    "references/python/src/reference_data/subsystems/types.json",
    "references/python/src/reference_data/subsystems/server.json",
    "references/python/src/reference_data/subsystems/constants.json",
    "references/python/src/reference_data/subsystems/entrypoints.json",
    "references/python/src/reference_data/subsystems/keybindings.json",
    "references/python/src/reference_data/subsystems/bootstrap.json",
    "references/python/src/reference_data/subsystems/assistant.json",
    "references/python/src/reference_data/subsystems/outputStyles.json",
    "references/python/src/reference_data/subsystems/plugins.json",
    "references/python/src/reference_data/subsystems/hooks.json",
    "references/python/src/reference_data/subsystems/memdir.json",
    "references/python/src/reference_data/subsystems/utils.json",
    "references/python/src/reference_data/subsystems/remote.json",
    "references/python/src/reference_data/subsystems/upstreamproxy.json",
    "references/python/src/reference_data/subsystems/schemas.json",
    "references/python/src/reference_data/subsystems/coordinator.json",
    "references/python/src/reference_data/subsystems/migrations.json",
    "references/python/src/reference_data/subsystems/components.json",
    "references/python/src/reference_data/subsystems/screens.json",
    "references/python/src/reference_data/subsystems/native_ts.json",
    "references/python/src/reference_data/subsystems/moreright.json",
    "references/python/src/reference_data/subsystems/services.json",
    "references/python/src/reference_data/subsystems/buddy.json",
    "references/python/src/reference_data/subsystems/bridge.json",
    "references/python/src/reference_data/subsystems/cli.json",
    "references/python/src/reference_data/__init__.py",
]


def test_python_reference_data_rows_have_explicit_non_runtime_disposition() -> None:
    text = DISPOSITION_DOC.read_text(encoding="utf-8")
    assert "evidence-only migration material" in text
    assert "must never be reintroduced as a runtime dependency" in text
    for row in EXPECTED_REFERENCE_DATA_ROWS:
        assert row.removeprefix(
            "references/python/src/reference_data/"
        ) in text or row.endswith("__init__.py")


def test_python_reference_data_cluster_size_matches_coverage_manifest() -> None:
    assert len(EXPECTED_REFERENCE_DATA_ROWS) == 33
