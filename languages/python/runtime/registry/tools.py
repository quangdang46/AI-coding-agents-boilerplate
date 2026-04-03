from __future__ import annotations


def tool_registry() -> dict[str, str]:
    return {
        "bash": "bash",
        "file_read": "file_read",
        "file_write": "file_write",
        "file_edit": "file_edit",
        "web_fetch": "web_fetch",
    }
