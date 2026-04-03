use std::path::Path;

use crate::manifest::{detect_project_language, read_agentkit_toml, validate_agentkit_toml};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct FeatureManifest {
    adds: FeatureAdds,
}

#[derive(Debug, Deserialize)]
struct FeatureAdds {
    #[serde(default)]
    agents: Vec<String>,
    #[serde(default)]
    skills: Vec<String>,
    #[serde(default)]
    prompts: Vec<String>,
    #[serde(default)]
    tools: Vec<String>,
}

pub fn run(project_root: &Path) -> Result<String, String> {
    let language = match detect_project_language(project_root) {
        Ok(language) => language,
        Err(_) => {
            let agentkit_path = project_root.join("agentkit.toml");
            return Err(format!(
                "missing required file: {}",
                agentkit_path.display()
            ));
        }
    };
    if !language.supports.doctor {
        return Err(format!(
            "unsupported capability doctor for language: {}",
            language.id
        ));
    }
    for required in required_paths(&language.id) {
        let path = project_root.join(required);
        if !path.exists() {
            return Err(format!("missing required file: {}", path.display()));
        }
    }
    if language.id == "python" {
        validate_agentkit_toml(project_root)?;
    }
    validate_provider_selection(project_root, &language.id)?;
    validate_web_fetch_support(project_root, &language.id)?;
    validate_permission_controls(project_root, &language.id)?;
    validate_tool_runtime_defaults(project_root, &language.id)?;
    validate_agent_discovery_config(project_root, &language.id)?;

    let enabled_features = load_enabled_features(project_root, &language.id)?;
    let registry_ids = load_registry_feature_ids(project_root)?;
    for feature_id in enabled_features {
        if !registry_ids
            .iter()
            .any(|candidate| candidate == &feature_id)
        {
            return Err(format!(
                "enabled feature not found in registry: {}",
                feature_id
            ));
        }
        validate_feature_assets(project_root, &feature_id)?;
    }

    Ok(format!("doctor ok: {}", project_root.display()))
}

fn validate_provider_selection(project_root: &Path, language_id: &str) -> Result<(), String> {
    match language_id {
        "python" => {
            let config_text = read_agentkit_toml(project_root)?;
            for required in [
                "default_provider = \"",
                "[providers.openai]",
                "[providers.anthropic]",
                "model = \"",
            ] {
                if !config_text.contains(required) {
                    return Err(format!(
                        "missing provider config in {}: {}",
                        project_root.join("agentkit.toml").display(),
                        required
                    ));
                }
            }
            Ok(())
        }
        "typescript" => {
            let path = project_root.join("boilerplate.config.ts");
            let config_text = std::fs::read_to_string(&path)
                .map_err(|err| format!("failed to read {}: {err}", path.display()))?;
            for required in [
                "defaultProvider:",
                "providers:",
                "anthropic:",
                "openai:",
                "model:",
                "apiKeyEnv:",
            ] {
                if !config_text.contains(required) {
                    return Err(format!(
                        "missing provider config in {}: {}",
                        path.display(),
                        required
                    ));
                }
            }
            Ok(())
        }
        "rust" => {
            let path = project_root.join(".claw.json");
            let config_text = std::fs::read_to_string(&path)
                .map_err(|err| format!("failed to read {}: {err}", path.display()))?;
            let value: serde_json::Value = serde_json::from_str(&config_text)
                .map_err(|err| format!("failed to parse {}: {err}", path.display()))?;
            let providers = value
                .get("providers")
                .and_then(|providers| providers.as_object())
                .ok_or_else(|| {
                    format!("missing provider config in {}: providers", path.display())
                })?;
            let default_provider = value
                .get("defaultProvider")
                .and_then(|provider| provider.as_str())
                .ok_or_else(|| {
                    format!(
                        "missing provider config in {}: defaultProvider",
                        path.display()
                    )
                })?;
            let selected = providers
                .get(default_provider)
                .and_then(|provider| provider.as_object())
                .ok_or_else(|| {
                    format!(
                        "missing provider config in {}: provider '{}'",
                        path.display(),
                        default_provider
                    )
                })?;
            if selected
                .get("model")
                .and_then(|model| model.as_str())
                .is_none()
            {
                return Err(format!(
                    "missing provider config in {}: model",
                    path.display()
                ));
            }
            if selected
                .get("apiKeyEnv")
                .and_then(|env| env.as_str())
                .is_none()
            {
                return Err(format!(
                    "missing provider config in {}: apiKeyEnv",
                    path.display()
                ));
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

fn validate_web_fetch_support(project_root: &Path, language_id: &str) -> Result<(), String> {
    match language_id {
        "python" => {
            let config_text = read_agentkit_toml(project_root)?;
            if !config_text.contains("\"web_fetch\"") {
                return Err(format!(
                    "missing web fetch config in {}: web_fetch",
                    project_root.join("agentkit.toml").display()
                ));
            }
            Ok(())
        }
        "typescript" => {
            let path = project_root.join("boilerplate.config.ts");
            let config_text = std::fs::read_to_string(&path)
                .map_err(|err| format!("failed to read {}: {err}", path.display()))?;
            if !config_text.contains("'web_fetch'") {
                return Err(format!(
                    "missing web fetch config in {}: web_fetch",
                    path.display()
                ));
            }
            Ok(())
        }
        "rust" => {
            let path = project_root.join(".claw.json");
            let config_text = std::fs::read_to_string(&path)
                .map_err(|err| format!("failed to read {}: {err}", path.display()))?;
            let value: serde_json::Value = serde_json::from_str(&config_text)
                .map_err(|err| format!("failed to parse {}: {err}", path.display()))?;
            let tools = value
                .get("tools")
                .and_then(|tools| tools.get("enabled"))
                .and_then(|enabled| enabled.as_array())
                .ok_or_else(|| {
                    format!(
                        "missing web fetch config in {}: tools.enabled",
                        path.display()
                    )
                })?;
            if !tools.iter().any(|item| item.as_str() == Some("web_fetch")) {
                return Err(format!(
                    "missing web fetch config in {}: web_fetch",
                    path.display()
                ));
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

fn validate_permission_controls(project_root: &Path, language_id: &str) -> Result<(), String> {
    match language_id {
        "python" => {
            let config_text = read_agentkit_toml(project_root)?;
            for required in ["approval_mode = \"", "deny = []"] {
                if !config_text.contains(required) {
                    return Err(format!(
                        "missing permission config in {}: {}",
                        project_root.join("agentkit.toml").display(),
                        required
                    ));
                }
            }
            Ok(())
        }
        "typescript" => {
            let path = project_root.join("boilerplate.config.ts");
            let config_text = std::fs::read_to_string(&path)
                .map_err(|err| format!("failed to read {}: {err}", path.display()))?;
            for required in ["approvalMode:", "deny:"] {
                if !config_text.contains(required) {
                    return Err(format!(
                        "missing permission config in {}: {}",
                        path.display(),
                        required
                    ));
                }
            }
            Ok(())
        }
        "rust" => {
            let path = project_root.join(".claw.json");
            let config_text = std::fs::read_to_string(&path)
                .map_err(|err| format!("failed to read {}: {err}", path.display()))?;
            let value: serde_json::Value = serde_json::from_str(&config_text)
                .map_err(|err| format!("failed to parse {}: {err}", path.display()))?;
            let permissions = value
                .get("permissions")
                .and_then(|permissions| permissions.as_object())
                .ok_or_else(|| {
                    format!(
                        "missing permission config in {}: permissions",
                        path.display()
                    )
                })?;
            if permissions
                .get("defaultMode")
                .and_then(|mode| mode.as_str())
                .is_none()
            {
                return Err(format!(
                    "missing permission config in {}: defaultMode",
                    path.display()
                ));
            }
            if permissions
                .get("deny")
                .and_then(|deny| deny.as_array())
                .is_none()
            {
                return Err(format!(
                    "missing permission config in {}: deny",
                    path.display()
                ));
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

fn validate_tool_runtime_defaults(project_root: &Path, language_id: &str) -> Result<(), String> {
    match language_id {
        "python" => {
            let config_text = read_agentkit_toml(project_root)?;
            for required in [
                "bash_timeout_ms = ",
                "web_fetch_timeout_ms = ",
                "max_tool_calls_per_turn = ",
            ] {
                if !config_text.contains(required) {
                    return Err(format!(
                        "missing tool default config in {}: {}",
                        project_root.join("agentkit.toml").display(),
                        required
                    ));
                }
            }
            Ok(())
        }
        "typescript" => {
            let path = project_root.join("boilerplate.config.ts");
            let config_text = std::fs::read_to_string(&path)
                .map_err(|err| format!("failed to read {}: {err}", path.display()))?;
            for required in [
                "bashTimeoutMs:",
                "webFetchTimeoutMs:",
                "maxToolCallsPerTurn:",
            ] {
                if !config_text.contains(required) {
                    return Err(format!(
                        "missing tool default config in {}: {}",
                        path.display(),
                        required
                    ));
                }
            }
            Ok(())
        }
        "rust" => {
            let path = project_root.join(".claw.json");
            let config_text = std::fs::read_to_string(&path)
                .map_err(|err| format!("failed to read {}: {err}", path.display()))?;
            let value: serde_json::Value = serde_json::from_str(&config_text)
                .map_err(|err| format!("failed to parse {}: {err}", path.display()))?;
            let tools = value
                .get("tools")
                .and_then(|tools| tools.as_object())
                .ok_or_else(|| {
                    format!("missing tool default config in {}: tools", path.display())
                })?;
            for key in ["bashTimeoutMs", "webFetchTimeoutMs", "maxToolCallsPerTurn"] {
                if tools.get(key).and_then(|value| value.as_i64()).is_none() {
                    return Err(format!(
                        "missing tool default config in {}: {}",
                        path.display(),
                        key
                    ));
                }
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

fn validate_agent_discovery_config(project_root: &Path, language_id: &str) -> Result<(), String> {
    match language_id {
        "python" => {
            let config_text = read_agentkit_toml(project_root)?;
            for required in [
                "[agents]",
                "directories = [\".agent/agents\"]",
                "default = \"executor\"",
            ] {
                if !config_text.contains(required) {
                    return Err(format!(
                        "missing agent config in {}: {}",
                        project_root.join("agentkit.toml").display(),
                        required
                    ));
                }
            }
            for required in ["planner", "executor", "reviewer"] {
                if !config_text.contains(&format!("\"{required}\"")) {
                    return Err(format!(
                        "missing agent config in {}: {}",
                        project_root.join("agentkit.toml").display(),
                        required
                    ));
                }
            }
            Ok(())
        }
        "typescript" => {
            let path = project_root.join("boilerplate.config.ts");
            let config_text = std::fs::read_to_string(&path)
                .map_err(|err| format!("failed to read {}: {err}", path.display()))?;
            for required in ["agents:", "directories:", "enabled:", "defaultAgent:"] {
                if !config_text.contains(required) {
                    return Err(format!(
                        "missing agent config in {}: {}",
                        path.display(),
                        required
                    ));
                }
            }
            Ok(())
        }
        "rust" => {
            let path = project_root.join(".claw.json");
            let config_text = std::fs::read_to_string(&path)
                .map_err(|err| format!("failed to read {}: {err}", path.display()))?;
            let value: serde_json::Value = serde_json::from_str(&config_text)
                .map_err(|err| format!("failed to parse {}: {err}", path.display()))?;
            let agents = value
                .get("agents")
                .and_then(|agents| agents.as_object())
                .ok_or_else(|| format!("missing agent config in {}: agents", path.display()))?;
            let directories = agents
                .get("directories")
                .and_then(|value| value.as_array())
                .ok_or_else(|| {
                    format!("missing agent config in {}: directories", path.display())
                })?;
            if !directories
                .iter()
                .any(|item| item.as_str() == Some(".agent/agents"))
            {
                return Err(format!(
                    "missing agent config in {}: .agent/agents",
                    path.display()
                ));
            }
            let enabled = agents
                .get("enabled")
                .and_then(|value| value.as_array())
                .ok_or_else(|| format!("missing agent config in {}: enabled", path.display()))?;
            for required in ["planner", "executor", "reviewer"] {
                if !enabled.iter().any(|item| item.as_str() == Some(required)) {
                    return Err(format!(
                        "missing agent config in {}: {}",
                        path.display(),
                        required
                    ));
                }
            }
            if agents
                .get("defaultAgent")
                .and_then(|value| value.as_str())
                .is_none()
            {
                return Err(format!(
                    "missing agent config in {}: defaultAgent",
                    path.display()
                ));
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

fn validate_feature_assets(project_root: &Path, feature_id: &str) -> Result<(), String> {
    let manifest_path = project_root
        .join(".agent/features")
        .join(feature_id)
        .join("feature.json");
    let raw = std::fs::read_to_string(&manifest_path)
        .map_err(|err| format!("failed to read {}: {err}", manifest_path.display()))?;
    let manifest: FeatureManifest = serde_json::from_str(&raw)
        .map_err(|err| format!("failed to parse {}: {err}", manifest_path.display()))?;

    for agent in manifest.adds.agents {
        let path = project_root.join(".agent/agents").join(agent);
        if !path.exists() {
            return Err(format!("missing feature asset: {}", path.display()));
        }
    }
    for skill in manifest.adds.skills {
        let path = project_root
            .join(".agent/skills")
            .join(skill)
            .join("SKILL.md");
        if !path.exists() {
            return Err(format!("missing feature asset: {}", path.display()));
        }
    }
    for prompt in manifest.adds.prompts {
        let path = project_root.join(".agent/prompts/sections").join(prompt);
        if !path.exists() {
            return Err(format!("missing feature asset: {}", path.display()));
        }
    }
    for tool in manifest.adds.tools {
        let tool = tool.trim();
        if tool.is_empty() {
            continue;
        }
    }

    if feature_id == "local-plugins" {
        let path = project_root.join(".claw/plugins/README.md");
        if !path.exists() {
            return Err(format!("missing feature asset: {}", path.display()));
        }
    }

    Ok(())
}

fn required_paths(language_id: &str) -> &'static [&'static str] {
    match language_id {
        "python" => &[
            "README.md",
            "agentkit.toml",
            "pyproject.toml",
            ".agent/prompts/system.md",
            ".agent/prompts/sections/coding-style.md",
            ".agent/prompts/sections/verification.md",
            ".agent/context/README.md",
            ".agent/agents/planner.agent.json",
            ".agent/agents/executor.agent.json",
            ".agent/agents/reviewer.agent.json",
            ".agent/sessions/README.md",
            ".agent/usage/README.md",
            ".agent/skills/plan/SKILL.md",
            ".agent/skills/add-feature/SKILL.md",
            ".agent/skills/verify/SKILL.md",
            ".agent/features/registry.json",
            ".agent/features/github-pr-review/feature.json",
        ],
        "typescript" => &[
            "README.md",
            "boilerplate.config.ts",
            "package.json",
            "tsconfig.json",
            "src/index.ts",
            ".agent/prompts/system.md",
            ".agent/prompts/sections/style.md",
            ".agent/prompts/sections/verification.md",
            ".agent/prompts/sections/security.md",
            ".agent/context/README.md",
            ".agent/agents/planner.agent.json",
            ".agent/agents/executor.agent.json",
            ".agent/agents/reviewer.agent.json",
            ".agent/sessions/README.md",
            ".agent/usage/README.md",
            ".agent/skills/plan/SKILL.md",
            ".agent/skills/add-feature/SKILL.md",
            ".agent/skills/verify/SKILL.md",
            ".agent/features/registry.json",
        ],
        "rust" => &[
            "README.md",
            "CLAW.md",
            ".claw.json",
            ".claw/settings.local.json",
            "Cargo.toml",
            "src/main.rs",
            ".agent/prompts/system.md",
            ".agent/prompts/sections/style.md",
            ".agent/prompts/sections/verification.md",
            ".agent/context/README.md",
            ".agent/agents/planner.agent.json",
            ".agent/agents/executor.agent.json",
            ".agent/agents/reviewer.agent.json",
            ".agent/sessions/README.md",
            ".agent/usage/README.md",
            ".agent/skills/plan/SKILL.md",
            ".agent/skills/add-feature/SKILL.md",
            ".agent/skills/verify/SKILL.md",
            ".agent/features/registry.json",
        ],
        _ => &[],
    }
}

fn parse_enabled_features(config_text: &str) -> Vec<String> {
    let mut in_features = false;
    for line in config_text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            in_features = trimmed == "[features]";
            continue;
        }
        if in_features && trimmed.starts_with("enabled = [") {
            let start = trimmed.find('[').unwrap_or(trimmed.len());
            let end = trimmed.rfind(']').unwrap_or(start);
            return trimmed[start + 1..end]
                .split(',')
                .map(|part| part.trim().trim_matches('"'))
                .filter(|part| !part.is_empty())
                .map(|part| part.to_string())
                .collect();
        }
    }
    Vec::new()
}

fn parse_typescript_enabled_features(config_text: &str) -> Vec<String> {
    let mut in_features = false;
    for line in config_text.lines() {
        let trimmed = line.trim();
        if !in_features {
            if trimmed == "features: {" {
                in_features = true;
            }
            continue;
        }
        if trimmed.starts_with("enabled: [") {
            let start = trimmed.find('[').unwrap_or(trimmed.len());
            let end = trimmed.rfind(']').unwrap_or(start);
            return trimmed[start + 1..end]
                .split(',')
                .map(|part| part.trim().trim_matches('\'').trim_matches('"'))
                .filter(|part| !part.is_empty())
                .map(|part| part.to_string())
                .collect();
        }
        if trimmed == "}," || trimmed == "}" {
            break;
        }
    }
    Vec::new()
}

fn load_enabled_features(project_root: &Path, language_id: &str) -> Result<Vec<String>, String> {
    match language_id {
        "python" => Ok(parse_enabled_features(&read_agentkit_toml(project_root)?)),
        "typescript" => {
            let path = project_root.join("boilerplate.config.ts");
            let text = std::fs::read_to_string(&path)
                .map_err(|err| format!("failed to read {}: {err}", path.display()))?;
            Ok(parse_typescript_enabled_features(&text))
        }
        "rust" => {
            let path = project_root.join(".claw.json");
            let text = std::fs::read_to_string(&path)
                .map_err(|err| format!("failed to read {}: {err}", path.display()))?;
            let value: serde_json::Value = serde_json::from_str(&text)
                .map_err(|err| format!("failed to parse {}: {err}", path.display()))?;
            Ok(value
                .get("features")
                .and_then(|features| features.get("enabled"))
                .and_then(|enabled| enabled.as_array())
                .map(|entries| {
                    entries
                        .iter()
                        .filter_map(|entry| entry.as_str().map(ToString::to_string))
                        .collect()
                })
                .unwrap_or_default())
        }
        _ => Ok(Vec::new()),
    }
}

fn load_registry_feature_ids(project_root: &Path) -> Result<Vec<String>, String> {
    let registry_path = project_root.join(".agent/features/registry.json");
    let raw = std::fs::read_to_string(&registry_path)
        .map_err(|err| format!("failed to read {}: {err}", registry_path.display()))?;
    let value: serde_json::Value = serde_json::from_str(&raw)
        .map_err(|err| format!("failed to parse {}: {err}", registry_path.display()))?;
    let entries = value
        .get("features")
        .and_then(|features| features.as_array())
        .ok_or_else(|| format!("invalid feature registry: {}", registry_path.display()))?;

    Ok(entries
        .iter()
        .filter_map(|entry| {
            entry
                .get("id")
                .and_then(|id| id.as_str())
                .map(ToString::to_string)
        })
        .collect())
}
