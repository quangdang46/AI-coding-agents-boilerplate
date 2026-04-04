from languages.python.runtime.registry.commands import (
    command_definitions,
    command_registry,
)
from languages.python.runtime.registry.execution_registry import (
    build_execution_registry,
)
from languages.python.runtime.registry.tool_pool import (
    assemble_tool_pool,
    build_tool_pool,
)
from languages.python.runtime.registry.tools import tool_definitions, tool_registry


EXPECTED_SOURCE_ROWS = {
    "references/python/src/commands.py",
    "references/python/src/tools.py",
    "references/python/src/execution_registry.py",
    "references/python/src/tool_pool.py",
    "references/python/src/Tool.py",
}


def test_python_registry_cluster_owns_archived_registry_rows() -> None:
    registry = build_execution_registry()
    source_rows = {definition.source_file for definition in registry.commands}
    source_rows.update(definition.source_file for definition in registry.tools)
    source_rows.add("references/python/src/execution_registry.py")
    source_rows.add("references/python/src/tool_pool.py")
    source_rows.add("references/python/src/Tool.py")

    assert source_rows == EXPECTED_SOURCE_ROWS


def test_python_registry_surfaces_are_consistent() -> None:
    command_names = [definition.name for definition in command_definitions()]
    tool_names = [definition.name for definition in tool_definitions()]

    assert list(command_registry().keys()) == command_names
    assert list(tool_registry().keys()) == tool_names
    assert build_tool_pool() == tool_names
    assert assemble_tool_pool().names() == tool_names

    runtime_registry = build_execution_registry()
    assert [
        definition.name for definition in runtime_registry.commands
    ] == command_names
    assert [definition.name for definition in runtime_registry.tools] == tool_names
    assert runtime_registry.tool_pool.names() == tool_names
