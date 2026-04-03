from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]


def _iter_text_files(root: Path, suffixes: tuple[str, ...]) -> list[Path]:
    out: list[Path] = []
    for suffix in suffixes:
        out.extend(path for path in root.rglob(f"*{suffix}") if path.is_file())
    return out


def test_shipped_runtime_has_no_reference_bound_paths() -> None:
    forbidden = (
        "reference_data/",
        "archive/claw_code_ts_snapshot/",
    )
    shipped_roots = [
        REPO_ROOT / "install",
        REPO_ROOT / "languages",
    ]

    for root in shipped_roots:
        for path in _iter_text_files(
            root, (".rs", ".py", ".md", ".json", ".toml", ".ts")
        ):
            text = path.read_text(encoding="utf-8")
            assert not any(fragment in text for fragment in forbidden), (
                f"forbidden reference binding in shipped file: {path}"
            )


def test_reference_and_porting_boundaries_are_documented() -> None:
    references_readme = (REPO_ROOT / "references" / "README.md").read_text(
        encoding="utf-8"
    )
    python_pack_boundary = (
        REPO_ROOT / "languages" / "python" / "docs" / "migration-baseline.md"
    ).read_text(encoding="utf-8")

    assert "canonical home for archived source references" in references_readme
    assert (
        "shipped runtime surfaces live under `install/` and `languages/`"
        in references_readme
    )
    assert (REPO_ROOT / "references" / "source-python" / "README.md").exists()
    assert (REPO_ROOT / "references" / "source-typescript" / "README.md").exists()
    assert (REPO_ROOT / "references" / "source-rust" / "README.md").exists()
    assert (REPO_ROOT / "references" / "parity" / "README.md").exists()
    assert "canonical migration baseline" in python_pack_boundary
    assert "languages/python/" in python_pack_boundary
    assert "must remain manifest-driven" in python_pack_boundary
