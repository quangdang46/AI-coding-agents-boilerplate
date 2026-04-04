use std::path::Path;

use crate::brand::{infer_brand_paths, BrandPaths};
use crate::manifest::{
    detect_project_language, read_agentkit_toml, repo_root, validate_agentkit_toml, LanguageManifest,
};
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
    skill: Option<String>,
    #[serde(default)]
    skills: Vec<String>,
    #[serde(default)]
    prompts: Vec<String>,
    #[serde(default)]
    tools: Vec<String>,
}

impl FeatureManifest {
    fn skill_name(&self) -> Option<&str> {
        self.adds
            .skill
            .as_deref()
            .or_else(|| self.adds.skills.first().map(|item| item.as_str()))
    }
}

fn rust_config_path(project_root: &Path, brand: &BrandPaths) -> std::path::PathBuf {
    project_root.join(brand.config_file())
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
    let brand = infer_brand_paths(project_root)?;
    if !language.supports.doctor {
        return Err(format!(
            "unsupported capability doctor for language: {}",
            language.id
        ));
    }
    for required in required_paths(&language, &brand) {
        let path = project_root.join(required);
        if !path.exists() {
            return Err(format!("missing required file: {}", path.display()));
        }
    }
    if language.id == "python" {
        validate_agentkit_toml(project_root)?;
    }
    validate_provider_selection(project_root, &language.id, &brand)?;
    validate_web_fetch_support(project_root, &language.id, &brand)?;
    validate_permission_controls(project_root, &language.id, &brand)?;
    validate_tool_runtime_defaults(project_root, &language.id, &brand)?;
    validate_agent_discovery_config(project_root, &language.id, &brand)?;

    let enabled_features = load_enabled_features(project_root, &language.id)?;
    let registry_ids = load_registry_feature_ids(&language)?;
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
        validate_feature_assets(project_root, &language, &brand, &feature_id)?;
    }

    Ok(format!("doctor ok: {}", project_root.display()))
}

fn validate_provider_selection(
    project_root: &Path,
    language_id: &str,
    brand: &BrandPaths,
) -> Result<(), String> {
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
            let path = rust_config_path(project_root, brand);
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

fn validate_web_fetch_support(
    project_root: &Path,
    language_id: &str,
    brand: &BrandPaths,
) -> Result<(), String> {
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
            let path = rust_config_path(project_root, brand);
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

fn validate_permission_controls(
    project_root: &Path,
    language_id: &str,
    brand: &BrandPaths,
) -> Result<(), String> {
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
            let path = rust_config_path(project_root, brand);
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

fn validate_tool_runtime_defaults(
    project_root: &Path,
    language_id: &str,
    brand: &BrandPaths,
) -> Result<(), String> {
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
            let path = rust_config_path(project_root, brand);
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

fn validate_agent_discovery_config(
    project_root: &Path,
    language_id: &str,
    brand: &BrandPaths,
) -> Result<(), String> {
    match language_id {
        "python" => {
            let config_text = read_agentkit_toml(project_root)?;
            for required in [
                "[agents]",
                &format!("directories = [\"{}/agents\"]", brand.hidden_root()),
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
            let path = rust_config_path(project_root, brand);
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
                .any(|item| item.as_str() == Some(&format!("{}/agents", brand.hidden_root())))
            {
                return Err(format!(
                    "missing agent config in {}: {}/agents",
                    path.display()
                    ,
                    brand.hidden_root()
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

fn validate_feature_assets(
    project_root: &Path,
    language: &LanguageManifest,
    brand: &BrandPaths,
    feature_id: &str,
) -> Result<(), String> {
    let manifest_path = repo_root()
        .join("languages")
        .join(&language.id)
        .join(&language.feature_registry)
        .parent()
        .expect("feature registry should have a parent")
        .join(feature_id)
        .join("feature.json");
    let raw = std::fs::read_to_string(&manifest_path)
        .map_err(|err| format!("failed to read {}: {err}", manifest_path.display()))?;
    let manifest: FeatureManifest = serde_json::from_str(&raw)
        .map_err(|err| format!("failed to parse {}: {err}", manifest_path.display()))?;

    for agent in &manifest.adds.agents {
        let path = project_root.join(brand.hidden_root()).join("agents").join(agent);
        if !path.exists() {
            return Err(format!("missing feature asset: {}", path.display()));
        }
    }
    if let Some(skill) = manifest.skill_name() {
        let path = project_root
            .join(brand.hidden_root())
            .join("skills")
            .join(skill)
            .join("SKILL.md");
        if !path.exists() {
            return Err(format!("missing feature asset: {}", path.display()));
        }
    }
    for tool in &manifest.adds.tools {
        let tool = tool.trim();
        if tool.is_empty() {
            continue;
        }
    }

    if feature_id == "local-plugins" {
        let path = project_root.join(format!("{}-plugin", brand.hidden_root())).join("plugin.json");
        if !path.exists() {
            return Err(format!("missing feature asset: {}", path.display()));
        }
    }

    Ok(())
}

fn required_paths(language: &LanguageManifest, brand: &BrandPaths) -> Vec<String> {
    let mut paths = vec!["README.md".to_string(), brand.doc_file(), brand.config_file()];
    paths.push(format!("{}/settings.json", brand.hidden_root()));
    paths.push(format!("{}/settings.local.json", brand.hidden_root()));
    paths.push(format!("{}/instructions.md", brand.hidden_root()));
    paths.push(format!("{}/sessions/README.md", brand.hidden_root()));
    paths.push(format!("{}/agents/planner.md", brand.hidden_root()));
    paths.push(format!("{}/agents/executor.md", brand.hidden_root()));
    paths.push(format!("{}/agents/reviewer.md", brand.hidden_root()));
    paths.push(format!("{}/skills/plan/SKILL.md", brand.hidden_root()));
    paths.push(format!("{}/skills/add-feature/SKILL.md", brand.hidden_root()));
    paths.push(format!("{}/skills/verify/SKILL.md", brand.hidden_root()));
    paths.push(".agents/skills/plan/SKILL.md".to_string());
    paths.push(".agents/skills/add-feature/SKILL.md".to_string());
    paths.push(".agents/skills/verify/SKILL.md".to_string());

    match language.id.as_str() {
        "python" => {
            paths.push("agentkit.toml".to_string());
            paths.push("pyproject.toml".to_string());
        }
        "typescript" => {
            paths.push("boilerplate.config.ts".to_string());
            paths.push("package.json".to_string());
            paths.push("tsconfig.json".to_string());
            paths.push("src/index.ts".to_string());
        }
        "rust" => {
            paths.push("Cargo.toml".to_string());
            paths.push("src/main.rs".to_string());
        }
        _ => {}
    }

    paths
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
            let brand = infer_brand_paths(project_root)?;
            let path = rust_config_path(project_root, &brand);
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

fn load_registry_feature_ids(language: &LanguageManifest) -> Result<Vec<String>, String> {
    let registry_path = repo_root()
        .join("languages")
        .join(&language.id)
        .join(&language.feature_registry);
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
