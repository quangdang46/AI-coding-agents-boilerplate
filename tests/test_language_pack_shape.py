from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
LANGUAGES_ROOT = REPO_ROOT / "languages"
LANGUAGE_IDS = ("python", "typescript", "rust")
RUNTIME_SUBDIRS = ("config", "registry", "prompts", "entrypoints")


def test_language_packs_include_runtime_boundary() -> None:
    for language_id in LANGUAGE_IDS:
        language_root = LANGUAGES_ROOT / language_id
        runtime_root = language_root / "runtime"

        assert runtime_root.is_dir(), f"missing runtime boundary for {language_id}"
        assert (runtime_root / "README.md").is_file(), (
            f"missing runtime README for {language_id}"
        )

        for subdir in RUNTIME_SUBDIRS:
            subdir_root = runtime_root / subdir
            assert subdir_root.is_dir(), f"missing runtime/{subdir} for {language_id}"
            assert (subdir_root / "README.md").is_file(), (
                f"missing runtime/{subdir}/README.md for {language_id}"
            )
