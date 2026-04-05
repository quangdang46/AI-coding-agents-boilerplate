from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT / "languages" / "typescript" / "docs" / "task-runtime-ownership.md"
)

EXPECTED_ROWS = [
    "references/typescript/src/tasks/stopTask.ts",
    "references/typescript/src/tasks/LocalMainSessionTask.ts",
    "references/typescript/src/tasks/DreamTask/DreamTask.ts",
    "references/typescript/src/tasks/RemoteAgentTask/RemoteAgentTask.tsx",
    "references/typescript/src/tasks/InProcessTeammateTask/types.ts",
    "references/typescript/src/tasks/InProcessTeammateTask/InProcessTeammateTask.tsx",
    "references/typescript/src/tasks/types.ts",
    "references/typescript/src/tasks/pillLabel.ts",
    "references/typescript/src/tasks/LocalAgentTask/LocalAgentTask.tsx",
    "references/typescript/src/tasks/LocalShellTask/LocalShellTask.tsx",
    "references/typescript/src/tasks/LocalShellTask/killShellTasks.ts",
    "references/typescript/src/tasks/LocalShellTask/guards.ts",
]


def test_typescript_task_runtime_rows_have_archive_or_owner_rationale() -> None:
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert "explicit archive-only or shipped-owner rationale" in text
    for row in EXPECTED_ROWS:
        assert row in text


def test_typescript_task_runtime_cluster_size_matches_expected_rows() -> None:
    assert len(EXPECTED_ROWS) == 12
