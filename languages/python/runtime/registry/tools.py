from __future__ import annotations

from dataclasses import dataclass


@dataclass(frozen=True)
class ToolDefinition:
    name: str
    purpose: str
    source_file: str


DEFAULT_TOOLS: tuple[ToolDefinition, ...] = (
    ToolDefinition(
        "bash",
        "Run shell commands within the configured approval policy",
        "references/python/src/tools.py",
    ),
    ToolDefinition(
        "file_read",
        "Read project files for runtime context and inspection",
        "references/python/src/tools.py",
    ),
    ToolDefinition(
        "file_write",
        "Write project files when the runtime policy allows it",
        "references/python/src/tools.py",
    ),
    ToolDefinition(
        "file_edit",
        "Edit existing project files when the runtime policy allows it",
        "references/python/src/tools.py",
    ),
    ToolDefinition(
        "web_fetch",
        "Fetch remote content through the shipped Python web surface",
        "references/python/src/tools.py",
    ),
)


def tool_definitions() -> tuple[ToolDefinition, ...]:
    return DEFAULT_TOOLS


def tool_registry() -> dict[str, str]:
    return {definition.name: definition.purpose for definition in tool_definitions()}
