from __future__ import annotations

from .tools import tool_registry


def build_tool_pool() -> list[str]:
    return list(tool_registry().keys())
