from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = REPO_ROOT / "languages" / "typescript" / "docs" / "cli-ownership.md"

EXPECTED_ROWS = [
    "references/typescript/src/cli/ndjsonSafeStringify.ts",
    "references/typescript/src/cli/transports/transportUtils.ts",
    "references/typescript/src/cli/transports/WebSocketTransport.ts",
    "references/typescript/src/cli/transports/ccrClient.ts",
    "references/typescript/src/cli/transports/SSETransport.ts",
    "references/typescript/src/cli/transports/HybridTransport.ts",
    "references/typescript/src/cli/transports/SerialBatchEventUploader.ts",
    "references/typescript/src/cli/transports/WorkerStateUploader.ts",
    "references/typescript/src/cli/structuredIO.ts",
    "references/typescript/src/cli/handlers/agents.ts",
    "references/typescript/src/cli/handlers/autoMode.ts",
    "references/typescript/src/cli/handlers/mcp.tsx",
    "references/typescript/src/cli/handlers/plugins.ts",
    "references/typescript/src/cli/handlers/util.tsx",
    "references/typescript/src/cli/handlers/auth.ts",
    "references/typescript/src/cli/print.ts",
    "references/typescript/src/cli/exit.ts",
    "references/typescript/src/cli/update.ts",
    "references/typescript/src/cli/remoteIO.ts",
]


def test_typescript_cli_rows_have_feature_pack_or_archive_rationale() -> None:
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert (
        "clear future feature-pack owner or an explicit archive-only rationale" in text
    )
    for row in EXPECTED_ROWS:
        assert row in text


def test_typescript_cli_cluster_size_matches_bead_manifest() -> None:
    assert len(EXPECTED_ROWS) == 19
