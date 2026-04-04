from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path

from ..brand import brand_doc_candidates, infer_brand_root
from ..permissions import summarize_permission_state
from ..tasks import summarize_task_state


@dataclass(frozen=True)
class CommandDefinition:
    name: str
    purpose: str
    source_file: str


def _read_state(path: Path) -> dict[str, str]:
    if not path.exists():
        return {}
    state: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8").splitlines():
        if "=" in line:
            key, value = line.split("=", 1)
            state[key] = value
    return state


def _extract_value(source: str, marker: str) -> str:
    start = source.index(marker) + len(marker)
    end = source.find(" ", start)
    if end == -1:
        end = len(source)
    return source[start:end]


def _extract_value_optional(source: str, marker: str) -> str | None:
    if marker not in source:
        return None
    return _extract_value(source, marker)


def _validate_runtime_artifacts(
    project_root: Path, brand_root: Path, runtime_output: str
) -> list[str]:
    issues: list[str] = []
    sessions_root = brand_root / "sessions"
    latest_path = sessions_root / "latest.state"
    summary_path = sessions_root / "summary.state"
    session_id = "local-main-session"
    if latest_path.exists():
        latest_preview = _read_state(latest_path)
        session_id = latest_preview.get("session_id", session_id)
    session_path = sessions_root / f"{session_id}.state"
    export_path = sessions_root / f"{session_id}.export.md"

    required = [
        sessions_root / "README.md",
        latest_path,
        summary_path,
        session_path,
        export_path,
    ]
    for path in required:
        if not path.exists():
            issues.append(f"missing:{path.relative_to(project_root)}")
    if not brand_doc_candidates(project_root, brand_root):
        issues.append("missing:instruction-surface")
    if issues:
        return issues

    latest = _read_state(latest_path)
    session = _read_state(session_path)
    summary = _read_state(summary_path)
    export_text = export_path.read_text(encoding="utf-8")

    for key in [
        "session_id",
        "turn_count",
        "provider",
        "model",
        "prompt_digest",
        "context_digest",
        "usage_entries",
        "total_cost_micros",
    ]:
        latest_value = latest.get(key)
        session_value = session.get(key)
        if not latest_value:
            issues.append(f"invalid:latest-state:{key}")
            continue
        if session_value != latest_value:
            issues.append(f"invalid:session-state:{key}")

    for key in ["usage_entries", "total_cost_micros"]:
        summary_value = summary.get(key)
        latest_value = latest.get(key)
        if not summary_value:
            issues.append(f"invalid:summary-state:{key}")
            continue
        if latest_value != summary_value:
            issues.append(f"invalid:summary-mismatch:{key}")
        try:
            int(summary_value)
        except ValueError:
            issues.append(f"invalid:summary-state:{key}:not-int")

    for key in [
        "session_id",
        "turn_count",
        "provider",
        "model",
        "prompt_digest",
        "context_digest",
    ]:
        expected = latest.get(key)
        if not expected or f"- {key}: {expected}" not in export_text:
            issues.append(f"invalid:export:{key}")

    runtime_expectations = {
        "provider": _extract_value_optional(runtime_output, "provider="),
        "model": _extract_value_optional(runtime_output, "model="),
        "prompt_digest": _extract_value_optional(runtime_output, "prompt_digest="),
        "context_digest": _extract_value_optional(runtime_output, "context_digest="),
        "turn_count": _extract_value_optional(runtime_output, "turn_count="),
        "usage_entries": _extract_value_optional(runtime_output, "usage_entries="),
        "total_cost_micros": _extract_value_optional(
            runtime_output, "total_cost_micros="
        ),
    }
    for key, expected in runtime_expectations.items():
        if expected and latest.get(key) != expected:
            issues.append(f"invalid:runtime-mismatch:{key}")

    return issues


DEFAULT_COMMANDS: tuple[CommandDefinition, ...] = (
    CommandDefinition(
        "status",
        "Render the runtime status summary",
        "references/python/src/commands.py",
    ),
    CommandDefinition(
        "session",
        "Summarize the latest persisted session state",
        "references/python/src/commands.py",
    ),
    CommandDefinition(
        "export",
        "Report the exported session artifact path",
        "references/python/src/commands.py",
    ),
    CommandDefinition(
        "config",
        "Summarize active provider, model, and approval configuration",
        "references/python/src/commands.py",
    ),
    CommandDefinition(
        "doctor",
        "Validate the shipped Python instruction and session surfaces",
        "references/python/src/commands.py",
    ),
    CommandDefinition(
        "context",
        "Report the current context digest for the active runtime turn",
        "references/python/src/commands.py",
    ),
    CommandDefinition(
        "usage",
        "Summarize usage and accumulated runtime cost",
        "references/python/src/commands.py",
    ),
    CommandDefinition(
        "permissions",
        "Summarize the effective runtime permission posture",
        "references/python/src/commands.py",
    ),
    CommandDefinition(
        "files",
        "Report persisted session, export, and usage file presence",
        "references/python/src/commands.py",
    ),
    CommandDefinition(
        "tasks",
        "Summarize the active runtime task surface",
        "references/python/src/commands.py",
    ),
)


def command_definitions() -> tuple[CommandDefinition, ...]:
    return DEFAULT_COMMANDS


def command_registry() -> dict[str, str]:
    return {definition.name: definition.purpose for definition in command_definitions()}


def run_command(command_name: str, project_root: Path, runtime_output: str) -> str:
    brand_root = infer_brand_root(project_root)
    if command_name == "status":
        return runtime_output
    if command_name == "session":
        latest = _read_state(brand_root / "sessions" / "latest.state")
        return (
            f"session_id={latest.get('session_id', 'missing')} "
            f"turn_count={latest.get('turn_count', '0')}"
        )
    if command_name == "export":
        export_path = brand_root / "sessions" / "local-main-session.export.md"
        return (
            f"export_path={brand_root.name}/sessions/local-main-session.export.md "
            f"export_exists={export_path.exists()}"
        )
    if command_name == "config":
        return (
            f"provider={_extract_value(runtime_output, 'provider=')} "
            f"model={_extract_value(runtime_output, 'model=')} "
            f"approval_mode={_extract_value(runtime_output, 'approval_mode=')}"
        )
    if command_name == "doctor":
        issues = _validate_runtime_artifacts(project_root, brand_root, runtime_output)
        return "doctor=ok" if not issues else f"doctor={';'.join(issues)}"
    if command_name == "context":
        return f"context_digest={_extract_value(runtime_output, 'context_digest=')}"
    if command_name == "usage":
        summary = _read_state(brand_root / "sessions" / "summary.state")
        return (
            f"usage_entries={summary.get('usage_entries', '0')} "
            f"total_cost_micros={summary.get('total_cost_micros', '0')}"
        )
    if command_name == "permissions":
        return summarize_permission_state(runtime_output)
    if command_name == "files":
        session_state = (brand_root / "sessions" / "local-main-session.state").exists()
        export_state = (
            brand_root / "sessions" / "local-main-session.export.md"
        ).exists()
        usage_state = (brand_root / "sessions" / "summary.state").exists()
        return (
            f"session_state={session_state} export_state={export_state} "
            f"usage_state={usage_state}"
        )
    if command_name == "tasks":
        return summarize_task_state(project_root)
    raise KeyError(f"unknown command: {command_name}")
