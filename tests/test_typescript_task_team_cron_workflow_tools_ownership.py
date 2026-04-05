from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT
    / "languages"
    / "typescript"
    / "docs"
    / "task-team-cron-workflow-tools-ownership.md"
)

EXPECTED_ROWS = [
    "references/typescript/src/tools/TaskOutputTool/constants.ts",
    "references/typescript/src/tools/TaskOutputTool/TaskOutputTool.tsx",
    "references/typescript/src/tools/TaskUpdateTool/prompt.ts",
    "references/typescript/src/tools/TaskUpdateTool/TaskUpdateTool.ts",
    "references/typescript/src/tools/TaskUpdateTool/constants.ts",
    "references/typescript/src/tools/TeamCreateTool/TeamCreateTool.ts",
    "references/typescript/src/tools/TeamCreateTool/prompt.ts",
    "references/typescript/src/tools/TeamCreateTool/UI.tsx",
    "references/typescript/src/tools/TeamCreateTool/constants.ts",
    "references/typescript/src/tools/TeamDeleteTool/TeamDeleteTool.ts",
    "references/typescript/src/tools/TeamDeleteTool/prompt.ts",
    "references/typescript/src/tools/TeamDeleteTool/UI.tsx",
    "references/typescript/src/tools/TeamDeleteTool/constants.ts",
    "references/typescript/src/tools/ScheduleCronTool/CronCreateTool.ts",
    "references/typescript/src/tools/ScheduleCronTool/CronListTool.ts",
    "references/typescript/src/tools/ScheduleCronTool/prompt.ts",
    "references/typescript/src/tools/ScheduleCronTool/UI.tsx",
    "references/typescript/src/tools/ScheduleCronTool/CronDeleteTool.ts",
    "references/typescript/src/tools/WorkflowTool/constants.ts",
    "references/typescript/src/tools/TaskGetTool/prompt.ts",
    "references/typescript/src/tools/TaskGetTool/TaskGetTool.ts",
    "references/typescript/src/tools/TaskGetTool/constants.ts",
    "references/typescript/src/tools/TaskStopTool/TaskStopTool.ts",
    "references/typescript/src/tools/TaskStopTool/prompt.ts",
    "references/typescript/src/tools/TaskStopTool/UI.tsx",
    "references/typescript/src/tools/TaskCreateTool/prompt.ts",
    "references/typescript/src/tools/TaskCreateTool/TaskCreateTool.ts",
    "references/typescript/src/tools/TaskCreateTool/constants.ts",
    "references/typescript/src/tools/TaskListTool/TaskListTool.ts",
    "references/typescript/src/tools/TaskListTool/prompt.ts",
    "references/typescript/src/tools/TaskListTool/constants.ts",
]


def test_typescript_task_team_cron_workflow_rows_have_feature_pack_or_archive_rationale() -> (
    None
):
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert (
        "clear future feature-pack owner or an explicit archive-only rationale" in text
    )
    for row in EXPECTED_ROWS:
        assert row in text


def test_typescript_task_team_cron_workflow_cluster_size_matches_manifest() -> None:
    assert len(EXPECTED_ROWS) == 31
