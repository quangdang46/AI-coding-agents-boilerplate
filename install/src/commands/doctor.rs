use std::path::Path;

use crate::manifest::validate_agentkit_toml;

pub fn run(project_root: &Path) -> Result<String, String> {
    for required in [
        "README.md",
        "pyproject.toml",
        ".agent/prompts/system.md",
        ".agent/prompts/sections/coding-style.md",
        ".agent/prompts/sections/verification.md",
        ".agent/agents/planner.agent.json",
        ".agent/agents/executor.agent.json",
        ".agent/agents/reviewer.agent.json",
        ".agent/skills/plan/SKILL.md",
        ".agent/skills/add-feature/SKILL.md",
        ".agent/skills/verify/SKILL.md",
        ".agent/features/registry.json",
        ".agent/features/github-pr-review/feature.json",
    ] {
        let path = project_root.join(required);
        if !path.exists() {
            return Err(format!("missing required file: {}", path.display()));
        }
    }
    validate_agentkit_toml(project_root)?;
    Ok(format!("doctor ok: {}", project_root.display()))
}
