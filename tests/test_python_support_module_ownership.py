import importlib
from pathlib import Path
import sys


REPO_ROOT = Path(__file__).resolve().parents[1]
PYTHON_RUNTIME_ROOT = REPO_ROOT / "languages" / "python" / "runtime"

if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))

EXPECTED_SOURCE_ROWS = {
    "references/python/src/context.py",
    "references/python/src/models.py",
    "references/python/src/bootstrap_graph.py",
    "references/python/src/command_graph.py",
    "references/python/src/port_manifest.py",
    "references/python/src/prefetch.py",
    "references/python/src/projectOnboardingState.py",
}


def test_python_support_cluster_has_shipped_runtime_owners() -> None:
    bootstrap_graph = importlib.import_module(
        "languages.python.runtime.bootstrap_graph"
    )
    command_graph = importlib.import_module("languages.python.runtime.command_graph")
    context = importlib.import_module("languages.python.runtime.context")
    port_manifest = importlib.import_module("languages.python.runtime.port_manifest")
    prefetch = importlib.import_module("languages.python.runtime.prefetch")
    project_onboarding_state = importlib.import_module(
        "languages.python.runtime.project_onboarding_state"
    )

    runtime_context = context.build_runtime_context()
    manifest = port_manifest.build_port_manifest()
    onboarding = project_onboarding_state.infer_project_onboarding_state(
        runtime_context.pack_root
    )
    runtime_command_graph = command_graph.build_command_graph()
    runtime_bootstrap_graph = bootstrap_graph.build_bootstrap_graph()
    prefetch_results = [
        prefetch.start_runtime_prefetch(runtime_context.pack_root),
        prefetch.start_prompt_prefetch(runtime_context.pack_root),
        prefetch.start_project_scan(runtime_context.pack_root),
    ]

    assert runtime_context.runtime_root == PYTHON_RUNTIME_ROOT
    assert runtime_context.runtime_root.name == "runtime"
    assert runtime_context.template_root.name == "base"
    assert "Pack root:" in context.render_context(runtime_context)
    assert manifest.total_python_files >= runtime_context.runtime_file_count
    assert onboarding.has_readme is True
    assert onboarding.has_tests is True
    assert onboarding.has_runtime_docs is True
    assert len(runtime_command_graph.builtins) > 0
    assert len(runtime_bootstrap_graph.stages) >= 3
    assert all(result.started for result in prefetch_results)


def test_python_support_cluster_covers_expected_archived_rows() -> None:
    port_manifest = importlib.import_module("languages.python.runtime.port_manifest")
    manifest = port_manifest.build_port_manifest()
    source_rows = {
        "references/python/src/context.py",
        "references/python/src/models.py",
        "references/python/src/bootstrap_graph.py",
        "references/python/src/command_graph.py",
        "references/python/src/port_manifest.py",
        "references/python/src/prefetch.py",
        "references/python/src/projectOnboardingState.py",
    }

    assert source_rows == EXPECTED_SOURCE_ROWS
    assert any(module.name == "runtime" for module in manifest.top_level_modules)
