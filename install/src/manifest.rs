use std::fs;
use std::path::Path;

pub fn read_agentkit_toml(project_root: &Path) -> Result<String, String> {
    let path = project_root.join("agentkit.toml");
    fs::read_to_string(&path).map_err(|err| format!("failed to read {}: {err}", path.display()))
}

pub fn validate_agentkit_toml(project_root: &Path) -> Result<(), String> {
    let path = project_root.join("agentkit.toml");
    let text = read_agentkit_toml(project_root)?;
    for section in [
        "[app]",
        "[prompts]",
        "[tools]",
        "[agents]",
        "[skills]",
        "[features]",
    ] {
        if !text.contains(section) {
            return Err(format!("missing section {section} in {}", path.display()));
        }
    }
    Ok(())
}

pub fn set_feature_enabled(
    project_root: &Path,
    feature_id: &str,
    enabled: bool,
) -> Result<(), String> {
    let path = project_root.join("agentkit.toml");
    let text = read_agentkit_toml(project_root)?;
    let mut lines: Vec<String> = text.lines().map(|line| line.to_string()).collect();

    let mut in_features = false;
    for line in &mut lines {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            in_features = trimmed == "[features]";
            continue;
        }
        if in_features && trimmed.starts_with("enabled = [") {
            let mut features = parse_enabled_features(trimmed);
            if enabled {
                if !features.iter().any(|item| item == feature_id) {
                    features.push(feature_id.to_string());
                }
            } else {
                features.retain(|item| item != feature_id);
            }
            *line = format!("enabled = [{}]", format_feature_list(&features));
            fs::write(&path, lines.join("\n") + "\n")
                .map_err(|err| format!("failed to write {}: {err}", path.display()))?;
            return Ok(());
        }
    }

    Err(format!("missing features.enabled in {}", path.display()))
}

fn parse_enabled_features(line: &str) -> Vec<String> {
    let start = line.find('[').unwrap_or(line.len());
    let end = line.rfind(']').unwrap_or(start);
    line[start + 1..end]
        .split(',')
        .map(|part| part.trim().trim_matches('"'))
        .filter(|part| !part.is_empty())
        .map(|part| part.to_string())
        .collect()
}

fn format_feature_list(features: &[String]) -> String {
    features
        .iter()
        .map(|feature| format!("\"{feature}\""))
        .collect::<Vec<_>>()
        .join(", ")
}
