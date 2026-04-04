from __future__ import annotations

from dataclasses import dataclass
from typing import TypedDict

from .commands import (
    CommandDefinition,
    command_definitions,
    command_registry,
    run_command,
)
from .tool_pool import ToolPool, assemble_tool_pool, build_tool_pool
from .tools import ToolDefinition, tool_definitions


@dataclass(frozen=True)
class ExecutionRegistry:
    commands: tuple[CommandDefinition, ...]
    tools: tuple[ToolDefinition, ...]
    tool_pool: ToolPool


class ExecutionRegistryView(TypedDict):
    commands: dict[str, str]
    tools: list[str]
    run_command: object
    command_definitions: tuple[CommandDefinition, ...]
    tool_definitions: tuple[ToolDefinition, ...]
    tool_pool: ToolPool


def build_execution_registry() -> ExecutionRegistry:
    pool = assemble_tool_pool()
    return ExecutionRegistry(
        commands=command_definitions(),
        tools=tool_definitions(),
        tool_pool=pool,
    )


def execution_registry() -> ExecutionRegistryView:
    registry = build_execution_registry()
    return {
        "commands": command_registry(),
        "tools": build_tool_pool(),
        "run_command": run_command,
        "command_definitions": registry.commands,
        "tool_definitions": registry.tools,
        "tool_pool": registry.tool_pool,
    }
