from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
OWNERSHIP_DOC = (
    REPO_ROOT / "languages" / "typescript" / "docs" / "bash-helper-ownership.md"
)

EXPECTED_ROWS = [
    "references/typescript/src/utils/bash/shellCompletion.ts",
    "references/typescript/src/utils/bash/prefix.ts",
    "references/typescript/src/utils/bash/bashParser.ts",
    "references/typescript/src/utils/bash/treeSitterAnalysis.ts",
    "references/typescript/src/utils/bash/shellQuote.ts",
    "references/typescript/src/utils/bash/registry.ts",
    "references/typescript/src/utils/bash/ast.ts",
    "references/typescript/src/utils/bash/specs/alias.ts",
    "references/typescript/src/utils/bash/specs/nohup.ts",
    "references/typescript/src/utils/bash/specs/time.ts",
    "references/typescript/src/utils/bash/specs/timeout.ts",
    "references/typescript/src/utils/bash/specs/index.ts",
    "references/typescript/src/utils/bash/specs/srun.ts",
    "references/typescript/src/utils/bash/specs/pyright.ts",
    "references/typescript/src/utils/bash/specs/sleep.ts",
    "references/typescript/src/utils/bash/shellQuoting.ts",
    "references/typescript/src/utils/bash/heredoc.ts",
    "references/typescript/src/utils/bash/ParsedCommand.ts",
    "references/typescript/src/utils/bash/parser.ts",
    "references/typescript/src/utils/bash/commands.ts",
    "references/typescript/src/utils/bash/bashPipeCommand.ts",
    "references/typescript/src/utils/bash/ShellSnapshot.ts",
    "references/typescript/src/utils/bash/shellPrefix.ts",
]


def test_typescript_bash_helper_rows_have_archive_or_owner_rationale() -> None:
    text = OWNERSHIP_DOC.read_text(encoding="utf-8")
    assert "explicit archive-only or shipped-owner rationale" in text
    for row in EXPECTED_ROWS:
        assert row in text


def test_typescript_bash_helper_cluster_size_matches_expected_rows() -> None:
    assert len(EXPECTED_ROWS) == 23
