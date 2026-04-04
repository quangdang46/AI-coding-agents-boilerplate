from __future__ import annotations

from dataclasses import dataclass

from .tools import ToolDefinition, tool_definitions


@dataclass(frozen=True)
class ToolPool:
    tools: tuple[ToolDefinition, ...]

    def names(self) -> list[str]:
        return [tool.name for tool in self.tools]


def assemble_tool_pool() -> ToolPool:
    return ToolPool(tools=tool_definitions())


def build_tool_pool() -> list[str]:
    return assemble_tool_pool().names()
