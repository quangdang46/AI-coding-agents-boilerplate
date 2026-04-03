from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
TAXONOMY_PATH = REPO_ROOT / "shared" / "docs" / "migration-taxonomy.md"
RULES_PATH = REPO_ROOT / "RULES.md"
PLAN_PATH = REPO_ROOT / ".sisyphus" / "plans" / "canonical-migration-plan.md"


def _read(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def test_taxonomy_is_complete_and_non_overlapping() -> None:
    taxonomy = _read(TAXONOMY_PATH)
    rules = _read(RULES_PATH)
    plan = _read(PLAN_PATH)

    assert "# Migration Taxonomy" in taxonomy

    for term in [
        "installer",
        "language-pack",
        "feature-pack",
        "reference-pack",
        "planned",
        "experimental",
        "stable",
        "deprecated",
        "retired",
        "declared",
        "implemented",
        "verified",
        "core",
        "reference-only",
        "deferred",
        "rejected",
    ]:
        assert f"`{term}`" in taxonomy

    assert "These five backlog dispositions are exhaustive" in taxonomy
    assert "These states are ordered and distinct" in taxonomy

    assert "declared` / `implemented` / `verified` are clearly distinct" in plan
    assert (
        "`core` / `feature-pack` / `deferred` / `reference-only` / `rejected` are exhaustive"
        in plan
    )
    assert "language.manifest.json" in rules


def test_derived_status_leakage_is_prevented() -> None:
    taxonomy = _read(TAXONOMY_PATH)
    rules = _read(RULES_PATH)

    assert (
        "Manifest files define runtime identity and runtime capabilities." in taxonomy
    )
    assert "Derived migration state MUST live outside runtime manifests." in taxonomy

    for field in [
        "id",
        "displayName",
        "status",
        "templateRoot",
        "configFile",
        "featureRegistry",
        "supports",
    ]:
        assert f"`{field}`" in taxonomy

    for derived in [
        "parity completeness",
        "migration readiness",
        "capability verification counts",
        "rollout status summaries",
    ]:
        assert derived in taxonomy

    assert (
        "Optional fields may include placeholder conventions, runtime entrypoint hints, status, and capability flags."
        in rules
    )
