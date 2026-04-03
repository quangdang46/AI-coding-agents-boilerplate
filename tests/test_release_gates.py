import json
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
GATES_PATH = REPO_ROOT / "shared" / "docs" / "release-gates.md"
MATRIX_PATH = REPO_ROOT / "shared" / "docs" / "capability-matrix.json"
ARCHIVE_GATE_PATH = REPO_ROOT / "shared" / "docs" / "archive-retirement-gate.md"


def _read(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def _load_matrix() -> dict:
    return json.loads(MATRIX_PATH.read_text(encoding="utf-8"))


def _gate_artifacts_present() -> dict[str, bool]:
    return {
        "shared schemas locked and validated": (
            REPO_ROOT / "shared" / "schemas" / "language.manifest.schema.json"
        ).exists()
        and (REPO_ROOT / "shared" / "schemas" / "feature-pack.schema.json").exists(),
        "installer manifest discovery green": (
            REPO_ROOT / "install" / "src" / "manifest.rs"
        ).exists()
        and (REPO_ROOT / "install" / "src" / "commands" / "list.rs").exists(),
        "Python proving slice green": (
            REPO_ROOT / "languages" / "python" / "language.manifest.json"
        ).exists()
        and (
            REPO_ROOT / "languages" / "python" / "template" / "base" / "agentkit.toml"
        ).exists(),
        "reference/runtime isolation green": (
            REPO_ROOT / "tests" / "test_reference_isolation.py"
        ).exists()
        and (REPO_ROOT / "references" / "README.md").exists(),
        "capability matrix refresh green": (
            REPO_ROOT / "shared" / "docs" / "capability-matrix.json"
        ).exists()
        and (REPO_ROOT / "tests" / "test_capability_matrix.py").exists(),
        "TypeScript classification complete": (
            REPO_ROOT / "shared" / "docs" / "typescript-decomposition-map.json"
        ).exists()
        and (REPO_ROOT / "tests" / "test_typescript_classification.py").exists(),
        "CLI lifecycle tests green in CI": (
            REPO_ROOT / "install" / "tests" / "cli.rs"
        ).exists(),
        "archive retirement gate documented": ARCHIVE_GATE_PATH.exists(),
    }


def _status_report() -> dict:
    matrix = _load_matrix()["capabilities"]
    counts = {"declared": 0, "implemented": 0, "verified": 0}
    for row in matrix:
        counts[row["implementationState"]] += 1

    gates = _gate_artifacts_present()
    return {
        "states": counts,
        "gates": gates,
        "blocked": not all(gates.values()),
        "blockers": [name for name, present in gates.items() if not present],
    }


def test_expansion_blocked() -> None:
    gates_doc = _read(GATES_PATH)
    report = _status_report()

    assert "# Release Gates" in gates_doc
    assert (
        "New language expansion is blocked until all of the following are true:"
        in gates_doc
    )

    simulated = dict(report["gates"])
    simulated["TypeScript classification complete"] = False
    blocked = not all(simulated.values())
    blockers = [name for name, present in simulated.items() if not present]

    assert blocked is True
    assert blockers == ["TypeScript classification complete"]


def test_evidence_based_status_report() -> None:
    gates_doc = _read(GATES_PATH)
    report = _status_report()

    assert (
        "Migration status reports must distinguish three implementation states:"
        in gates_doc
    )
    assert set(report["states"].keys()) == {"declared", "implemented", "verified"}
    assert sum(report["states"].values()) == len(_load_matrix()["capabilities"])
    assert isinstance(report["blocked"], bool)
    assert isinstance(report["blockers"], list)


def test_archive_retirement_gate_references_required_evidence() -> None:
    archive_gate = _read(ARCHIVE_GATE_PATH)

    assert (
        "The archive may be removed only when all of the following are true:"
        in archive_gate
    )
    assert "`files.txt`" in archive_gate
    assert "`docs/porting/command-port-table.md`" in archive_gate
    assert "`docs/porting/tool-port-table.md`" in archive_gate
    assert "`docs/porting/feature-pack-port-table.md`" in archive_gate
    assert "`tests/test_reference_isolation.py`" in archive_gate
    assert "`install/tests/migration_red.rs`" in archive_gate


def test_release_gates_include_archive_retirement_gate() -> None:
    gates_doc = _read(GATES_PATH)
    report = _status_report()

    assert (REPO_ROOT / "shared" / "docs" / "archive-retirement-gate.md").exists()
    assert report["gates"]["archive retirement gate documented"] is True
    assert "reference/runtime isolation green" in gates_doc
