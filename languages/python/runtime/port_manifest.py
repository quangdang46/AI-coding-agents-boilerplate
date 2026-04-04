from __future__ import annotations

from collections import Counter
from dataclasses import dataclass
from pathlib import Path


@dataclass(frozen=True)
class Subsystem:
    name: str
    path: str
    file_count: int
    notes: str


DEFAULT_PACK_ROOT = Path(__file__).resolve().parents[1]


@dataclass(frozen=True)
class PortManifest:
    pack_root: Path
    total_python_files: int
    top_level_modules: tuple[Subsystem, ...]

    def to_markdown(self) -> str:
        lines = [
            f"Pack root: `{self.pack_root}`",
            f"Total Python files: **{self.total_python_files}**",
            "",
            "Top-level Python modules:",
        ]
        for module in self.top_level_modules:
            lines.append(
                f"- `{module.name}` ({module.file_count} files) — {module.notes}"
            )
        return "\n".join(lines)


def build_port_manifest(pack_root: Path | None = None) -> PortManifest:
    root = pack_root or DEFAULT_PACK_ROOT
    files = [path for path in root.rglob("*.py") if path.is_file()]
    counter = Counter(
        path.relative_to(root).parts[0]
        if len(path.relative_to(root).parts) > 1
        else path.name
        for path in files
    )
    notes = {
        "runtime": "Python runtime ownership boundary",
        "template": "Generated project template surface",
        "tests": "Language-pack verification surface",
    }
    modules = tuple(
        Subsystem(
            name=name,
            path=f"languages/python/{name}",
            file_count=count,
            notes=notes.get(name, "Python language-pack support module"),
        )
        for name, count in counter.most_common()
    )
    return PortManifest(
        pack_root=root,
        total_python_files=len(files),
        top_level_modules=modules,
    )
