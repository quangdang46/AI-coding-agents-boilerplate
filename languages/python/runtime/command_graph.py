from __future__ import annotations

from dataclasses import dataclass

from .registry.commands import command_definitions


@dataclass(frozen=True)
class CommandNode:
    name: str
    responsibility: str
    source_file: str


@dataclass(frozen=True)
class CommandGraph:
    builtins: tuple[CommandNode, ...]
    plugin_like: tuple[CommandNode, ...]
    skill_like: tuple[CommandNode, ...]

    def flattened(self) -> tuple[CommandNode, ...]:
        return self.builtins + self.plugin_like + self.skill_like

    def as_markdown(self) -> str:
        lines = [
            "# Command Graph",
            "",
            f"Builtins: {len(self.builtins)}",
            f"Plugin-like commands: {len(self.plugin_like)}",
            f"Skill-like commands: {len(self.skill_like)}",
        ]
        return "\n".join(lines)


def build_command_graph() -> CommandGraph:
    builtins = tuple(
        CommandNode(
            name=definition.name,
            responsibility=definition.purpose,
            source_file="references/python/src/command_graph.py",
        )
        for definition in command_definitions()
    )
    return CommandGraph(builtins=builtins, plugin_like=(), skill_like=())
