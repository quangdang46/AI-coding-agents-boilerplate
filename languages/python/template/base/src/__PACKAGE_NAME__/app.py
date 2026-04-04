from __future__ import annotations

import hashlib
import subprocess
import tomllib
from pathlib import Path

BRAND_ROOT = "__BRAND_ROOT__"
BRAND_DOC = "__BRAND_DOC__"


def _project_root() -> Path:
    return Path(__file__).resolve().parents[2]


def _read_text(path: Path) -> str:
    return path.read_text(encoding="utf-8").strip()


def _read_optional_text(path: Path) -> str:
    return _read_text(path) if path.exists() else ""


def _checksum(parts: list[str]) -> str:
    digest = hashlib.sha256()
    for part in parts:
        digest.update(part.encode("utf-8"))
        digest.update(b"\0")
    return digest.hexdigest()[:12]


def _read_state(path: Path) -> dict[str, str]:
    if not path.exists():
        return {}
    state: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8").splitlines():
        if "=" in line:
            key, value = line.split("=", 1)
            state[key] = value
    return state


def _write_state(path: Path, items: list[tuple[str, str]]) -> None:
    path.write_text(
        "\n".join(f"{key}={value}" for key, value in items) + "\n",
        encoding="utf-8",
    )


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
    context_paths: list[Path],
    enabled: set[str],
    approval_mode: str,
    deny: list[str],
    bash_timeout_ms: int,
) -> str:
    usage_path = project_root / BRAND_ROOT / "sessions" / "runtime-tool-smoke.txt"

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
        readable = [_read_optional_text(path) for path in context_paths]
        file_read_result = _checksum([text for text in readable if text])
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


def _persist_session_and_usage(
    project_root: Path,
    provider: str,
    model: str,
    prompt_digest: str,
    context_digest: str,
    prompt_texts: list[str],
    context_texts: list[str],
    tool_results: str,
) -> str:
    session_dir = project_root / BRAND_ROOT / "sessions"
    session_dir.mkdir(parents=True, exist_ok=True)

    session_id = "local-main-session"
    session_path = session_dir / f"{session_id}.state"
    latest_path = session_dir / "latest.state"
    export_path = session_dir / f"{session_id}.export.md"
    usage_summary_path = session_dir / "summary.state"

    previous_session = _read_state(session_path)
    turn_count = int(previous_session.get("turn_count", "0")) + 1
    previous_summary = _read_state(usage_summary_path)
    usage_entries = int(previous_summary.get("usage_entries", "0")) + 1
    cost_micros = (sum(len(text) for text in prompt_texts + context_texts) * 2) + (
        len(tool_results) * 3
    )
    total_cost_micros = (
        int(previous_summary.get("total_cost_micros", "0")) + cost_micros
    )

    state_items = [
        ("session_id", session_id),
        ("turn_count", str(turn_count)),
        ("provider", provider),
        ("model", model),
        ("prompt_digest", prompt_digest),
        ("context_digest", context_digest),
        ("usage_entries", str(usage_entries)),
        ("total_cost_micros", str(total_cost_micros)),
    ]
    _write_state(session_path, state_items)
    _write_state(latest_path, state_items)

    export_path.write_text(
        "# Session Export\n\n"
        f"- session_id: {session_id}\n"
        f"- turn_count: {turn_count}\n"
        f"- provider: {provider}\n"
        f"- model: {model}\n"
        f"- prompt_digest: {prompt_digest}\n"
        f"- context_digest: {context_digest}\n",
        encoding="utf-8",
    )

    _write_state(
        usage_summary_path,
        [
            ("usage_entries", str(usage_entries)),
            ("total_cost_micros", str(total_cost_micros)),
        ],
    )

    return (
        f"session_id={session_id} "
        f"turn_count={turn_count} "
        f"export_path={BRAND_ROOT}/sessions/{session_id}.export.md "
        f"usage_entries={usage_entries} "
        f"total_cost_micros={total_cost_micros}"
    )


def _existing_paths(project_root: Path, paths: list[str]) -> list[Path]:
    return [project_root / path for path in paths if (project_root / path).exists()]


def _load_runtime_summary() -> str:
    project_root = _project_root()
    config = tomllib.loads((project_root / "agentkit.toml").read_text(encoding="utf-8"))

    default_provider = config["app"]["default_provider"]
    provider_model = config["providers"][default_provider]["model"]
    enabled = set(config["tools"]["enabled"])
    approval_mode = config["tools"]["approval_mode"]
    deny = list(config["tools"]["deny"])
    bash_timeout_ms = int(config["tools"]["bash_timeout_ms"])

    prompt_paths = _existing_paths(project_root, [config["prompts"]["system"]])
    prompt_paths.extend(
        _existing_paths(project_root, list(config["prompts"]["sections"]))
    )
    prompt_texts = [_read_text(path) for path in prompt_paths]

    context_paths = _existing_paths(project_root, list(config["prompts"]["context"]))
    context_texts = [_read_text(path) for path in context_paths]
    prompt_digest = _checksum(prompt_texts)
    context_digest = _checksum(context_texts)
    tool_results = _run_core_tools(
        project_root, context_paths, enabled, approval_mode, deny, bash_timeout_ms
    )
    session_summary = _persist_session_and_usage(
        project_root,
        default_provider,
        provider_model,
        prompt_digest,
        context_digest,
        prompt_texts,
        context_texts,
        tool_results,
    )

    return (
        f"provider={default_provider} "
        f"model={provider_model} "
        f"prompt_digest={prompt_digest} "
        f"context_digest={context_digest} "
        f"approval_mode={approval_mode} "
        f"bash_policy={_policy_for_operation(approval_mode, deny, 'bash', 'bash')} "
        f"file_write_policy={_policy_for_operation(approval_mode, deny, 'file_write', 'file_write')} "
        f"{tool_results} "
        f"{session_summary}"
    )


def run_session_loop() -> str:
    return f"__PROJECT_NAME__ session loop completed {_load_runtime_summary()}"


def main() -> str:
    return run_session_loop()
