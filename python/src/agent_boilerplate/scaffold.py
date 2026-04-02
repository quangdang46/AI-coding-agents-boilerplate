from __future__ import annotations

import shutil
from pathlib import Path


PLACEHOLDERS = {
    '__PROJECT_NAME__': 'project_name',
    '__PACKAGE_NAME__': 'package_name',
    '__BINARY_NAME__': 'binary_name',
}


def normalize_package_name(project_name: str) -> str:
    return project_name.lower().replace('-', '_').replace(' ', '_')


def scaffold_template(template_root: Path, output_root: Path, project_name: str, package_name: str | None = None, binary_name: str | None = None) -> Path:
    package_name = package_name or normalize_package_name(project_name)
    binary_name = binary_name or project_name

    values = {
        'project_name': project_name,
        'package_name': package_name,
        'binary_name': binary_name,
    }

    if output_root.exists():
        shutil.rmtree(output_root)
    output_root.mkdir(parents=True, exist_ok=True)

    for source in sorted(template_root.rglob('*')):
        relative_parts = []
        for part in source.relative_to(template_root).parts:
            replacement = part
            for needle, key in PLACEHOLDERS.items():
                replacement = replacement.replace(needle, values[key])
            relative_parts.append(replacement)
        target = output_root.joinpath(*relative_parts)
        if source.is_dir():
            target.mkdir(parents=True, exist_ok=True)
            continue
        target.parent.mkdir(parents=True, exist_ok=True)
        text = source.read_text()
        for needle, key in PLACEHOLDERS.items():
            text = text.replace(needle, values[key])
        target.write_text(text)

    return output_root
