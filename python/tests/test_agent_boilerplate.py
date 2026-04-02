from __future__ import annotations

import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

from src.agent_boilerplate.commands.doctor import run_doctor
from src.agent_boilerplate.config.loader import load_config
from src.agent_boilerplate.core.runtime import ProjectRuntime
from src.agent_boilerplate.feature_ops import apply_feature, remove_feature
from src.agent_boilerplate.prompts.composer import compose_prompt
from src.agent_boilerplate.registry.features import load_feature_manifest, load_feature_registry
from src.agent_boilerplate.scaffold import scaffold_template


class AgentBoilerplateTests(unittest.TestCase):
    def setUp(self) -> None:
        self.template_root = Path(__file__).resolve().parent.parent / 'templates' / 'base'

    def _scaffold(self) -> Path:
        target = Path(tempfile.mkdtemp(prefix='aicd-python-template-'))
        return scaffold_template(self.template_root, target, project_name='demo-agent')

    def test_template_config_loads(self) -> None:
        project = self._scaffold()
        config = load_config(project)
        self.assertEqual(config.app.package_name, 'demo_agent')
        self.assertIn('file_read', config.tools.enabled)
        self.assertIn('planner', config.agents.enabled)
        self.assertEqual(config.features.enabled, ())

    def test_prompt_composition_reads_runtime_and_project_prompts(self) -> None:
        project = self._scaffold()
        config = load_config(project)
        composed = compose_prompt(project, config)
        self.assertIn('reusable coding-agent boilerplate runtime', composed)
        self.assertIn('project-local coding agent', composed)
        self.assertIn('Prefer small, reviewable changes', composed)
        self.assertIn('Run tests and read their output', composed)

    def test_runtime_loads_agents_and_skills(self) -> None:
        project = self._scaffold()
        runtime = ProjectRuntime.from_project(project)
        agents = runtime.load_agents()
        skills = runtime.load_skills()
        self.assertEqual({agent.id for agent in agents}, {'planner', 'executor', 'reviewer'})
        self.assertEqual({skill.name for skill in skills}, {'plan', 'add-feature', 'verify'})

    def test_feature_registry_and_manifest_load(self) -> None:
        project = self._scaffold()
        config = load_config(project)
        entries = load_feature_registry(project, config)
        self.assertEqual(entries[0].id, 'github-pr-review')
        manifest = load_feature_manifest(project, config, entries[0])
        self.assertIn('mcp', manifest.adds['tools'])
        self.assertEqual(manifest.patches[0].target, 'agentkit.toml')

    def test_feature_apply_and_remove_updates_scaffold(self) -> None:
        project = self._scaffold()
        apply_feature(project, 'github-pr-review')
        self.assertTrue((project / '.agent/prompts/sections/github-review.md').exists())
        self.assertTrue((project / '.agent/skills/review-pr/SKILL.md').exists())
        self.assertIn('github-pr-review', load_config(project).features.enabled)
        remove_feature(project, 'github-pr-review')
        self.assertFalse((project / '.agent/prompts/sections/github-review.md').exists())
        self.assertFalse((project / '.agent/skills/review-pr/SKILL.md').exists())
        self.assertEqual(load_config(project).features.enabled, ())

    def test_doctor_passes_on_scaffold(self) -> None:
        project = self._scaffold()
        report = run_doctor(project)
        self.assertTrue(report.ok)
        self.assertIn('loaded 3 agents', report.as_text())

    def test_doctor_fails_when_enabled_feature_files_are_missing(self) -> None:
        project = self._scaffold()
        apply_feature(project, 'github-pr-review')
        (project / '.agent/skills/review-pr/SKILL.md').unlink()
        report = run_doctor(project)
        self.assertFalse(report.ok)
        self.assertIn('missing feature skill file for github-pr-review: review-pr', report.as_text())

    def test_doctor_fails_when_enabled_agent_file_is_missing(self) -> None:
        project = self._scaffold()
        (project / '.agent/agents/executor.agent.json').unlink()
        report = run_doctor(project)
        self.assertFalse(report.ok)
        self.assertIn('enabled agent not found: executor', report.as_text())

    def test_cli_doctor_runs(self) -> None:
        project = self._scaffold()
        result = subprocess.run(
            [sys.executable, '-m', 'src.agent_boilerplate.entrypoints.cli', 'doctor', '--project', str(project)],
            check=True,
            capture_output=True,
            text=True,
        )
        self.assertIn('Doctor: OK', result.stdout)

    def test_cli_feature_add_and_remove_runs(self) -> None:
        project = self._scaffold()
        subprocess.run(
            [sys.executable, '-m', 'src.agent_boilerplate.entrypoints.cli', 'feature', 'add', 'github-pr-review', '--project', str(project)],
            check=True,
            capture_output=True,
            text=True,
        )
        self.assertTrue((project / '.agent/prompts/sections/github-review.md').exists())
        subprocess.run(
            [sys.executable, '-m', 'src.agent_boilerplate.entrypoints.cli', 'feature', 'remove', 'github-pr-review', '--project', str(project)],
            check=True,
            capture_output=True,
            text=True,
        )
        self.assertFalse((project / '.agent/prompts/sections/github-review.md').exists())

    def test_scaffolded_project_cli_runs(self) -> None:
        project = self._scaffold()
        result = subprocess.run(
            [sys.executable, '-m', 'src.demo_agent.cli'],
            cwd=project,
            check=True,
            capture_output=True,
            text=True,
        )
        self.assertIn('demo-agent ready', result.stdout)


if __name__ == '__main__':
    unittest.main()
