use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LanguageRuntimeManifest {
    #[serde(rename = "configFile")]
    pub config_file: String,
    #[serde(rename = "projectMarkers", default)]
    pub project_markers: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct LanguageTemplateManifest {
    pub base: String,
}

#[derive(Debug, Deserialize)]
pub struct LanguageSupportsManifest {
    pub init: bool,
    #[serde(rename = "featureAdd")]
    pub feature_add: bool,
    #[serde(rename = "featureRemove")]
    pub feature_remove: bool,
    pub doctor: bool,
}

#[derive(Debug, Deserialize)]
pub struct LanguageManifest {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    pub kind: String,
    pub id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub version: String,
    pub description: String,
    pub runtime: LanguageRuntimeManifest,
    pub templates: LanguageTemplateManifest,
    pub supports: LanguageSupportsManifest,
    #[serde(rename = "featureRegistry")]
    pub feature_registry: Option<String>,
}

pub fn read_language_manifest(path: &Path) -> Result<LanguageManifest, String> {
    let raw = fs::read_to_string(path)
        .map_err(|err| format!("failed to read {}: {err}", path.display()))?;
    let manifest: LanguageManifest = serde_json::from_str(&raw)
        .map_err(|err| format!("failed to parse {}: {err}", path.display()))?;

    if manifest.schema_version != "1" {
        return Err(format!(
            "invalid schemaVersion in {}: expected 1",
            path.display()
        ));
    }
    if manifest.kind != "language-pack" {
        return Err(format!(
            "invalid kind in {}: expected language-pack",
            path.display()
        ));
    }
    if manifest
        .feature_registry
        .as_deref()
        .unwrap_or_default()
        .is_empty()
    {
        return Err(format!("missing featureRegistry in {}", path.display()));
    }
    if manifest.runtime.config_file.is_empty() {
        return Err(format!("missing runtime.configFile in {}", path.display()));
    }
    if manifest.templates.base.is_empty() {
        return Err(format!("missing templates.base in {}", path.display()));
    }

    Ok(manifest)
}

pub fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

pub fn discover_language_manifest_paths() -> Result<Vec<PathBuf>, String> {
    let languages_root = repo_root().join("languages");
    if !languages_root.exists() {
        return Ok(Vec::new());
    }

    let mut paths = Vec::new();
    let entries = fs::read_dir(&languages_root)
        .map_err(|err| format!("failed to read {}: {err}", languages_root.display()))?;
    for entry in entries {
        let entry = entry.map_err(|err| format!("failed to read language entry: {err}"))?;
        let manifest_path = entry.path().join("language.manifest.json");
        if manifest_path.exists() {
            paths.push(manifest_path);
        }
    }
    paths.sort();
    Ok(paths)
}

pub fn discover_languages() -> Result<Vec<LanguageManifest>, String> {
    discover_language_manifest_paths()?
        .into_iter()
        .map(|path| read_language_manifest(&path))
        .collect()
}

pub fn discover_language(language_id: &str) -> Result<LanguageManifest, String> {
    let manifests = discover_languages()?;
    manifests
        .into_iter()
        .find(|manifest| manifest.id == language_id)
        .ok_or_else(|| format!("unknown language manifest: {language_id}"))
}

pub fn detect_project_language(project_root: &Path) -> Result<LanguageManifest, String> {
    for manifest in discover_languages()? {
        let config_path = project_root.join(&manifest.runtime.config_file);
        let has_config = config_path.exists();
        let has_marker = manifest
            .runtime
            .project_markers
            .iter()
            .any(|marker| project_root.join(marker).exists());
        if has_config && has_marker {
            return Ok(manifest);
        }
    }

    Err(format!(
        "unable to detect project language for {}",
        project_root.display()
    ))
}

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
