from __future__ import annotations

import hashlib
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


def _load_runtime_summary() -> str:
    project_root = _project_root()
    config = tomllib.loads((project_root / "agentkit.toml").read_text(encoding="utf-8"))

    default_provider = config["app"]["default_provider"]
    provider_model = config["providers"][default_provider]["model"]

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
        f"context_digest={_checksum(context_texts)}"
    )


def run_session_loop() -> str:
    return f"__PROJECT_NAME__ session loop completed {_load_runtime_summary()}"


def main() -> str:
    return run_session_loop()
