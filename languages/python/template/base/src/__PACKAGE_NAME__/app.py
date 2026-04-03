from __future__ import annotations

import hashlib
import subprocess
import tomllib
from pathlib import Path


def _project_root() -> Path:
    return Path(__file__).resolve().parents[2]


def _read_text(path: Path) -> str:
    return path.read_text(encoding="utf-8").strip()


def _checksum(parts: list[str]) -> str:
    digest = hashlib.sha256()
    for part in parts:
        digest.update(part.encode("utf-8"))
        digest.update(b"\0")
    return digest.hexdigest()[:12]


def _policy_for_operation(
    approval_mode: str, deny: list[str], operation: str, tool_name: str
) -> str:
    if tool_name in deny:
        return f"{operation}=denied"
    if approval_mode == "never":
        return f"{operation}=blocked"
    if approval_mode == "default" and tool_name in {"bash", "file_edit", "file_write"}:
        return f"{operation}=approval-required"
    return f"{operation}=allowed"


def _run_core_tools(
    project_root: Path,
    enabled: set[str],
    approval_mode: str,
    deny: list[str],
    bash_timeout_ms: int,
) -> str:
    usage_path = project_root / ".agent/usage/runtime-tool-smoke.txt"

    def status(tool_name: str, operation: str) -> str:
        if tool_name not in enabled:
            return f"{operation}=disabled"
        return _policy_for_operation(approval_mode, deny, operation, tool_name)

    results: list[str] = []

    bash_status = status("bash", "bash")
    if bash_status == "bash=allowed":
        bash_result = subprocess.run(
            ["bash", "-lc", "printf tool-bash-ok"],
            capture_output=True,
            text=True,
            timeout=max(1, bash_timeout_ms // 1000),
            check=True,
        ).stdout.strip()
        results.append(f"bash_result={bash_result}")
    else:
        results.append(f"bash_result={bash_status}")

    file_read_status = status("file_read", "file_read")
    if file_read_status == "file_read=allowed":
        file_read_result = _checksum(
            [_read_text(project_root / ".agent/context/README.md")]
        )
        results.append(f"file_read_result={file_read_result}")
    else:
        results.append(f"file_read_result={file_read_status}")

    file_write_status = status("file_write", "file_write")
    if file_write_status == "file_write=allowed":
        usage_path.write_text("tool-write-ok", encoding="utf-8")
        results.append("file_write_result=tool-write-ok")
    else:
        results.append(f"file_write_result={file_write_status}")

    file_edit_status = status("file_edit", "file_edit")
    if file_edit_status == "file_edit=allowed":
        if not usage_path.exists():
            usage_path.write_text("tool-write-ok", encoding="utf-8")
        edited = usage_path.read_text(encoding="utf-8") + " edited"
        usage_path.write_text(edited, encoding="utf-8")
        results.append(f"file_edit_result={edited}")
    else:
        results.append(f"file_edit_result={file_edit_status}")

    web_fetch_status = status("web_fetch", "web_fetch")
    if web_fetch_status == "web_fetch=allowed":
        web_fetch_result = "tool-web-fetch"
        results.append(f"web_fetch_result={web_fetch_result}")
    else:
        results.append(f"web_fetch_result={web_fetch_status}")

    return " ".join(results)


def _load_runtime_summary() -> str:
    project_root = _project_root()
    config = tomllib.loads((project_root / "agentkit.toml").read_text(encoding="utf-8"))

    default_provider = config["app"]["default_provider"]
    provider_model = config["providers"][default_provider]["model"]
    enabled = set(config["tools"]["enabled"])
    approval_mode = config["tools"]["approval_mode"]
    deny = list(config["tools"]["deny"])
    bash_timeout_ms = int(config["tools"]["bash_timeout_ms"])

    prompt_paths = [project_root / config["prompts"]["system"]]
    prompt_paths.extend(project_root / path for path in config["prompts"]["sections"])
    prompt_texts = [_read_text(path) for path in prompt_paths]

    context_texts = [
        _read_text(project_root / path) for path in config["prompts"]["context"]
    ]

    return (
        f"provider={default_provider} "
        f"model={provider_model} "
        f"prompt_digest={_checksum(prompt_texts)} "
        f"context_digest={_checksum(context_texts)} "
        f"approval_mode={approval_mode} "
        f"bash_policy={_policy_for_operation(approval_mode, deny, 'bash', 'bash')} "
        f"file_write_policy={_policy_for_operation(approval_mode, deny, 'file_write', 'file_write')} "
        f"{_run_core_tools(project_root, enabled, approval_mode, deny, bash_timeout_ms)}"
    )


def run_session_loop() -> str:
    return f"__PROJECT_NAME__ session loop completed {_load_runtime_summary()}"


def main() -> str:
    return run_session_loop()
