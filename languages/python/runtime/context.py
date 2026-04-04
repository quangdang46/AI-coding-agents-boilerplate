from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path


@dataclass(frozen=True)
class RuntimeContext:
    pack_root: Path
    runtime_root: Path
    template_root: Path
    docs_root: Path
    tests_root: Path
    runtime_file_count: int
    template_file_count: int
    test_file_count: int


def build_runtime_context(base: Path | None = None) -> RuntimeContext:
    pack_root = base or Path(__file__).resolve().parents[1]
    runtime_root = pack_root / "runtime"
    template_root = pack_root / "template" / "base"
    docs_root = pack_root / "docs"
    tests_root = pack_root / "tests"
    return RuntimeContext(
        pack_root=pack_root,
        runtime_root=runtime_root,
        template_root=template_root,
        docs_root=docs_root,
        tests_root=tests_root,
        runtime_file_count=sum(
            1 for path in runtime_root.rglob("*.py") if path.is_file()
        ),
        template_file_count=sum(
            1 for path in template_root.rglob("*.py") if path.is_file()
        ),
        test_file_count=sum(1 for path in tests_root.rglob("*.py") if path.is_file()),
    )


def render_context(context: RuntimeContext) -> str:
    return "\n".join(
        [
            f"Pack root: {context.pack_root}",
            f"Runtime root: {context.runtime_root}",
            f"Template root: {context.template_root}",
            f"Docs root: {context.docs_root}",
            f"Tests root: {context.tests_root}",
            f"Runtime files: {context.runtime_file_count}",
            f"Template files: {context.template_file_count}",
            f"Test files: {context.test_file_count}",
        ]
    )
