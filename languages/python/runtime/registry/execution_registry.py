from __future__ import annotations

from .commands import command_registry, run_command
from .tool_pool import build_tool_pool


def execution_registry() -> dict[str, object]:
    return {
        "commands": command_registry(),
        "tools": build_tool_pool(),
        "run_command": run_command,
    }
