from __future__ import annotations

import json
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

    def _write_feature(self, project: Path, feature_id: str, feature_path: str, *, depends_on: list[str] | None = None, marker: str) -> None:
        registry_path = project / '.agent' / 'features' / 'registry.json'
        registry = json.loads(registry_path.read_text())
        registry['features'].append({'id': feature_id, 'path': feature_path})
        registry_path.write_text(json.dumps(registry, indent=2) + '\n')

        feature_root = registry_path.parent / feature_path
        (feature_root / 'files' / 'markers').mkdir(parents=True, exist_ok=True)
        (feature_root / 'files' / 'markers' / f'{feature_id}.txt').write_text(marker)
        (feature_root / 'feature.json').write_text(
            json.dumps(
                {
                    'id': feature_id,
                    'name': feature_id,
                    'description': f'{feature_id} feature',
                    'version': '0.1.0',
                    'dependsOn': depends_on or [],
                    'adds': {},
                    'patches': [
                        {
                            'target': 'agentkit.toml',
                            'op': 'merge',
                            'path': 'features.enabled',
                            'value': [feature_id],
                        }
                    ],
                },
                indent=2,
            )
            + '\n'
        )

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
        config = load_config(project)
        runtime = ProjectRuntime.from_project(project)
        self.assertTrue((project / '.agent/prompts/sections/github-review.md').exists())
        self.assertTrue((project / '.agent/skills/review-pr/SKILL.md').exists())
        self.assertTrue((project / '.agent/agents/review-agent.agent.json').exists())
        self.assertIn('github-pr-review', config.features.enabled)
        self.assertIn('.agent/prompts/sections/github-review.md', config.prompts.sections)
        self.assertIn('review-agent', config.agents.enabled)
        self.assertIn('review-pr', config.skills.enabled)
        self.assertIn('mcp', config.tools.enabled)
        self.assertIn('review-agent', {agent.id for agent in runtime.load_agents()})
        self.assertIn('review-pr', {skill.name for skill in runtime.load_skills()})
        remove_feature(project, 'github-pr-review')
        config = load_config(project)
        self.assertFalse((project / '.agent/prompts/sections/github-review.md').exists())
        self.assertFalse((project / '.agent/skills/review-pr/SKILL.md').exists())
        self.assertFalse((project / '.agent/agents/review-agent.agent.json').exists())
        self.assertEqual(config.features.enabled, ())
        self.assertNotIn('.agent/prompts/sections/github-review.md', config.prompts.sections)
        self.assertNotIn('review-agent', config.agents.enabled)
        self.assertNotIn('review-pr', config.skills.enabled)
        self.assertNotIn('mcp', config.tools.enabled)

    def test_enabled_feature_contributes_runtime_prompt_skill_and_agent(self) -> None:
        project = self._scaffold()
        apply_feature(project, 'github-pr-review')

        runtime = ProjectRuntime.from_project(project)
        self.assertIn('review-pr', {skill.name for skill in runtime.load_skills()})
        self.assertIn('review-agent', {agent.id for agent in runtime.load_agents()})

        composed = runtime.compose_system_prompt()
        self.assertIn('GitHub Review', composed)

    def test_feature_add_uses_registry_paths_and_dependency_checks(self) -> None:
        project = self._scaffold()
        self._write_feature(project, 'base-review', 'packs/base-review', marker='base ready')
        self._write_feature(
            project,
            'dependent-review',
            'packs/advanced/dependent-review',
            depends_on=['base-review'],
            marker='dependent ready',
        )

        with self.assertRaisesRegex(ValueError, 'requires enabled feature'):
            apply_feature(project, 'dependent-review')

        apply_feature(project, 'base-review')
        apply_feature(project, 'dependent-review')
        self.assertEqual((project / 'markers/base-review.txt').read_text(), 'base ready')
        self.assertEqual((project / 'markers/dependent-review.txt').read_text(), 'dependent ready')

        with self.assertRaisesRegex(ValueError, 'required by enabled feature'):
            remove_feature(project, 'base-review')

        remove_feature(project, 'dependent-review')
        remove_feature(project, 'base-review')
        self.assertFalse((project / 'markers/base-review.txt').exists())
        self.assertFalse((project / 'markers/dependent-review.txt').exists())

    def test_doctor_passes_on_scaffold(self) -> None:
        project = self._scaffold()
        report = run_doctor(project)
        self.assertTrue(report.ok)
        self.assertIn('loaded 3 agents', report.as_text())

    def test_doctor_passes_on_feature_enabled_scaffold(self) -> None:
        project = self._scaffold()
        apply_feature(project, 'github-pr-review')
        report = run_doctor(project)
        self.assertTrue(report.ok)
        self.assertIn('loaded 4 agents', report.as_text())
        self.assertIn('loaded 4 skills', report.as_text())
        self.assertIn('feature prompt enabled in config: github-review.md', report.as_text())
        self.assertIn('feature tool enabled in config: mcp', report.as_text())

    def test_doctor_fails_when_enabled_feature_files_are_missing(self) -> None:
        project = self._scaffold()
        apply_feature(project, 'github-pr-review')
        (project / '.agent/skills/review-pr/SKILL.md').unlink()
        report = run_doctor(project)
        self.assertFalse(report.ok)
        self.assertIn('missing feature skill file for github-pr-review: review-pr', report.as_text())

    def test_doctor_fails_when_feature_wiring_is_missing_from_config(self) -> None:
        project = self._scaffold()
        apply_feature(project, 'github-pr-review')
        config_path = project / 'agentkit.toml'
        config_path.write_text(config_path.read_text().replace('  "mcp",\n', '', 1))
        report = run_doctor(project)
        self.assertFalse(report.ok)
        self.assertIn('feature tool not enabled in config for github-pr-review: mcp', report.as_text())

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
