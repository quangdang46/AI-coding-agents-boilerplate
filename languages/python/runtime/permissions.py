from __future__ import annotations

from dataclasses import dataclass, field


@dataclass(frozen=True)
class ToolPermissionContext:
    deny_names: frozenset[str] = field(default_factory=frozenset)
    deny_prefixes: tuple[str, ...] = ()

    @classmethod
    def from_iterables(
        cls,
        deny_names: list[str] | None = None,
        deny_prefixes: list[str] | None = None,
    ) -> "ToolPermissionContext":
        return cls(
            deny_names=frozenset(name.lower() for name in (deny_names or [])),
            deny_prefixes=tuple(prefix.lower() for prefix in (deny_prefixes or [])),
        )

    def blocks(self, tool_name: str) -> bool:
        lowered = tool_name.lower()
        return lowered in self.deny_names or any(
            lowered.startswith(prefix) for prefix in self.deny_prefixes
        )


def policy_for_operation(
    approval_mode: str, deny: list[str], operation: str, tool_name: str
) -> str:
    context = ToolPermissionContext.from_iterables(deny_names=deny)
    if context.blocks(tool_name):
        return f"{operation}=denied"
    if approval_mode == "never":
        return f"{operation}=blocked"
    if approval_mode == "default" and tool_name in {"bash", "file_edit", "file_write"}:
        return f"{operation}=approval-required"
    return f"{operation}=allowed"


def summarize_permission_state(runtime_output: str) -> str:
    return " ".join(
        [
            f"approval_mode={extract_runtime_value(runtime_output, 'approval_mode=')}",
            f"bash_policy={extract_runtime_value(runtime_output, 'bash_policy=')}",
            f"file_write_policy={extract_runtime_value(runtime_output, 'file_write_policy=')}",
        ]
    )


def extract_runtime_value(source: str, marker: str) -> str:
    start = source.index(marker) + len(marker)
    end = source.find(" ", start)
    if end == -1:
        end = len(source)
    return source[start:end]
