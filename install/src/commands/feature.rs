use std::path::{Path, PathBuf};

use crate::brand::{infer_brand_paths, resolve_brand_placeholders, BrandPaths};
use crate::fs_ops::{copy_tree_contents, copy_tree_contents_with_brand_placeholders};
use crate::manifest::{detect_project_language, repo_root, LanguageManifest};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct FeatureManifest {
    #[serde(rename = "schemaVersion")]
    schema_version: String,
    kind: String,
    id: String,
    #[serde(rename = "displayName")]
    display_name: String,
    version: String,
    description: String,
    #[serde(rename = "appliesTo")]
    applies_to: Vec<String>,
    adds: FeatureAdds,
    #[serde(default)]
    patches: Vec<FeaturePatch>,
}

#[derive(Debug, Deserialize, Clone)]
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

#[derive(Debug, Deserialize)]
struct FeaturePatch {
    target: String,
    strategy: String,
    path: String,
    value: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct FeatureArgs {
    pub feature_id: String,
    pub project: PathBuf,
}

pub fn add(args: &FeatureArgs) -> Result<String, String> {
    let language = detect_project_language(&args.project)?;
    let brand = infer_brand_paths(&args.project)?;
    let feature_id = normalize_feature_id(&language.id, &args.feature_id)?;
    if !language.supports.feature_add {
        return Err(format!(
            "unsupported capability feature add for language: {}",
            language.id
        ));
    }
    let manifest_path = authoring_feature_manifest_path(&language, &feature_id);
    if !manifest_path.exists() {
        return Err(format!(
            "unknown feature '{}': {}",
            args.feature_id,
            manifest_path.display()
        ));
    }
    let manifest = load_feature_manifest(&language, &feature_id)?;
    let feature_files_root = authoring_feature_files_root(&language, &feature_id);
    copy_tree_contents_with_brand_placeholders(&feature_files_root, &args.project, &brand)
        .map_err(|err| format!("feature add failed: {err}"))?;
    materialize_feature_agents(&args.project, &brand, &language, &feature_id, &manifest)?;
    materialize_feature_skill(&args.project, &brand, &language, &feature_id, &manifest)?;
    apply_feature_patches(&args.project, &manifest, true)?;
    Ok(format!(
        "added feature {} to {}",
        feature_id,
        args.project.display()
    ))
}

fn normalize_feature_id(language_id: &str, feature_id: &str) -> Result<String, String> {
    match feature_id.split_once(':') {
        Some((prefix, name)) if prefix == language_id => Ok(name.to_string()),
        Some((_prefix, _)) => Err(format!(
            "feature '{}' does not belong to language: {}",
            feature_id, language_id
        )),
        None => Ok(feature_id.to_string()),
    }
}

fn load_feature_manifest(
    language: &LanguageManifest,
    feature_id: &str,
) -> Result<FeatureManifest, String> {
    let manifest_path = authoring_feature_manifest_path(language, feature_id);
    let raw = std::fs::read_to_string(&manifest_path)
        .map_err(|err| format!("failed to read {}: {err}", manifest_path.display()))?;
    let manifest: FeatureManifest = serde_json::from_str(&raw)
        .map_err(|err| format!("failed to parse {}: {err}", manifest_path.display()))?;
    if manifest.schema_version != "1" {
        return Err(format!(
            "invalid feature schemaVersion in {}",
            manifest_path.display()
        ));
    }
    if manifest.kind != "feature-pack" {
        return Err(format!(
            "invalid feature kind in {}",
            manifest_path.display()
        ));
    }
    if manifest.id != feature_id {
        return Err(format!(
            "feature id mismatch in {}",
            manifest_path.display()
        ));
    }
    if manifest.display_name.is_empty()
        || manifest.version.is_empty()
        || manifest.description.is_empty()
    {
        return Err(format!(
            "missing feature metadata in {}",
            manifest_path.display()
        ));
    }
    if manifest.applies_to.is_empty() {
        return Err(format!("missing appliesTo in {}", manifest_path.display()));
    }
    if !manifest.applies_to.iter().any(|item| item == &language.id) {
        return Err(format!(
            "feature {} does not apply to language: {}",
            feature_id, language.id
        ));
    }
    Ok(manifest)
}

fn materialize_feature_skill(
    project_root: &Path,
    brand: &BrandPaths,
    language: &LanguageManifest,
    feature_id: &str,
    manifest: &FeatureManifest,
) -> Result<(), String> {
    let Some(skill_name) = manifest.skill_name() else {
        return Ok(());
    };
    let source_path = authoring_feature_skill_path(language, feature_id);
    if !source_path.exists() {
        return Err(format!("missing feature skill: {}", source_path.display()));
    }
    for target_dir in [
        project_root.join(brand.hidden_root()).join("skills").join(skill_name),
        project_root.join(".agents/skills").join(skill_name),
    ] {
        std::fs::create_dir_all(&target_dir)
            .map_err(|err| format!("failed to create {}: {err}", target_dir.display()))?;
        std::fs::copy(&source_path, target_dir.join("SKILL.md"))
            .map_err(|err| format!("failed to copy {}: {err}", source_path.display()))?;
    }
    Ok(())
}

fn materialize_feature_agents(
    project_root: &Path,
    brand: &BrandPaths,
    language: &LanguageManifest,
    feature_id: &str,
    manifest: &FeatureManifest,
) -> Result<(), String> {
    if manifest.adds.agents.is_empty() {
        return Ok(());
    }
    let source_root = authoring_feature_agents_root(language, feature_id);
    if !source_root.exists() {
        return Err(format!("missing feature agents root: {}", source_root.display()));
    }
    let target_root = project_root.join(brand.hidden_root()).join("agents");
    std::fs::create_dir_all(&target_root)
        .map_err(|err| format!("failed to create {}: {err}", target_root.display()))?;
    copy_tree_contents(&source_root, &target_root)
        .map_err(|err| format!("feature add failed: {err}"))?;
    Ok(())
}

fn apply_feature_patches(
    project_root: &Path,
    manifest: &FeatureManifest,
    enabled: bool,
) -> Result<(), String> {
    let brand = infer_brand_paths(project_root)?;
    for patch in &manifest.patches {
        if patch.strategy != "merge" {
            return Err(format!(
                "unsupported feature patch strategy: {}",
                patch.strategy
            ));
        }
        match patch.target.as_str() {
            "agentkit.toml" => apply_toml_merge_patch(project_root, patch, enabled)?,
            "boilerplate.config.ts" => apply_typescript_merge_patch(project_root, patch, enabled)?,
            ".claw.json" | ".<brand>.json" => apply_json_merge_patch(project_root, patch, enabled, &brand)?,
            other => return Err(format!("unsupported feature patch target: {other}")),
        }
    }
    Ok(())
}

fn apply_toml_merge_patch(
    project_root: &Path,
    patch: &FeaturePatch,
    enabled: bool,
) -> Result<(), String> {
    let path = project_root.join(&patch.target);
    let text = std::fs::read_to_string(&path)
        .map_err(|err| format!("failed to read {}: {err}", path.display()))?;
    let (section, key) = patch
        .path
        .split_once('.')
        .ok_or_else(|| format!("invalid feature patch path: {}", patch.path))?;
    let updated = update_toml_array(&text, section, key, &patch.value, enabled)?;
    std::fs::write(&path, updated)
        .map_err(|err| format!("failed to write {}: {err}", path.display()))
}

fn update_toml_array(
    text: &str,
    section: &str,
    key: &str,
    values: &[String],
    enabled: bool,
) -> Result<String, String> {
    let mut lines: Vec<String> = text.lines().map(ToString::to_string).collect();
    let mut in_section = false;
    let mut index = 0;
    while index < lines.len() {
        let trimmed = lines[index].trim();
        if trimmed.starts_with('[') {
            in_section = trimmed == format!("[{section}]");
            index += 1;
            continue;
        }
        if in_section && trimmed.starts_with(&format!("{key} = [")) {
            let end = find_array_end(&lines, index)?;
            let mut items = parse_array_items(&lines[index..=end].join(" "));
            merge_items(&mut items, values, enabled);
            lines.splice(
                index..=end,
                [format!("{key} = [{}]", format_toml_items(&items))],
            );
            return Ok(lines.join("\n") + "\n");
        }
        index += 1;
    }
    Err(format!("missing {section}.{key} in config"))
}

fn apply_typescript_merge_patch(
    project_root: &Path,
    patch: &FeaturePatch,
    enabled: bool,
) -> Result<(), String> {
    let path = project_root.join(&patch.target);
    let text = std::fs::read_to_string(&path)
        .map_err(|err| format!("failed to read {}: {err}", path.display()))?;
    let (section, key) = patch
        .path
        .split_once('.')
        .ok_or_else(|| format!("invalid feature patch path: {}", patch.path))?;
    let updated = update_typescript_array(&text, section, key, &patch.value, enabled)?;
    std::fs::write(&path, updated)
        .map_err(|err| format!("failed to write {}: {err}", path.display()))
}

fn apply_json_merge_patch(
    project_root: &Path,
    patch: &FeaturePatch,
    enabled: bool,
    brand: &BrandPaths,
) -> Result<(), String> {
    let path = project_root.join(resolve_brand_placeholders(
        &patch.target.replace("<brand>", "__BRAND__"),
        brand,
    ));
    let text = std::fs::read_to_string(&path)
        .map_err(|err| format!("failed to read {}: {err}", path.display()))?;
    let mut root: serde_json::Value = serde_json::from_str(&text)
        .map_err(|err| format!("failed to parse {}: {err}", path.display()))?;
    let (section, key) = patch
        .path
        .split_once('.')
        .ok_or_else(|| format!("invalid feature patch path: {}", patch.path))?;

    let object = root.as_object_mut().ok_or_else(|| {
        format!(
            "feature patch target is not a JSON object: {}",
            path.display()
        )
    })?;
    let section_value = object
        .entry(section.to_string())
        .or_insert_with(|| serde_json::json!({}));
    let section_object = section_value
        .as_object_mut()
        .ok_or_else(|| format!("feature patch section is not an object: {section}"))?;
    let array_value = section_object
        .entry(key.to_string())
        .or_insert_with(|| serde_json::json!([]));
    let items = array_value
        .as_array_mut()
        .ok_or_else(|| format!("feature patch key is not an array: {section}.{key}"))?;

    if enabled {
        for value in &patch.value {
            if !items.iter().any(|item| item.as_str() == Some(value)) {
                items.push(serde_json::Value::String(value.clone()));
            }
        }
    } else {
        items.retain(|item| !patch.value.iter().any(|value| item.as_str() == Some(value)));
    }

    let updated = serde_json::to_string_pretty(&root)
        .map_err(|err| format!("failed to serialize {}: {err}", path.display()))?;
    std::fs::write(&path, updated + "\n")
        .map_err(|err| format!("failed to write {}: {err}", path.display()))
}

fn update_typescript_array(
    text: &str,
    section: &str,
    key: &str,
    values: &[String],
    enabled: bool,
) -> Result<String, String> {
    let mut lines: Vec<String> = text.lines().map(ToString::to_string).collect();
    let mut in_section = false;
    let mut depth = 0_i32;
    let mut index = 0;
    while index < lines.len() {
        let trimmed = lines[index].trim();
        if !in_section {
            if trimmed == format!("{section}: {{") {
                in_section = true;
                depth = 1;
            }
            index += 1;
            continue;
        }

        if trimmed.starts_with(&format!("{key}: [")) {
            let end = find_array_end(&lines, index)?;
            let indent = lines[index]
                .chars()
                .take_while(|ch| ch.is_whitespace())
                .collect::<String>();
            let mut items = parse_array_items(&lines[index..=end].join(" "));
            merge_items(&mut items, values, enabled);
            lines.splice(
                index..=end,
                [format!(
                    "{indent}{key}: [{}],",
                    format_typescript_items(&items)
                )],
            );
            return Ok(lines.join("\n") + "\n");
        }

        depth += trimmed.matches('{').count() as i32;
        depth -= trimmed.matches('}').count() as i32;
        if depth <= 0 {
            break;
        }
        index += 1;
    }
    Err(format!("missing {section}.{key} in config"))
}

fn find_array_end(lines: &[String], start: usize) -> Result<usize, String> {
    for (offset, line) in lines[start..].iter().enumerate() {
        if line.contains(']') {
            return Ok(start + offset);
        }
    }
    Err("unterminated array in config".to_string())
}

fn parse_array_items(line: &str) -> Vec<String> {
    let start = line.find('[').unwrap_or(line.len());
    let end = line.rfind(']').unwrap_or(start);
    line[start + 1..end]
        .split(',')
        .map(|part| part.trim().trim_matches('"').trim_matches('\''))
        .filter(|part| !part.is_empty())
        .map(ToString::to_string)
        .collect()
}

fn merge_items(items: &mut Vec<String>, values: &[String], enabled: bool) {
    if !enabled {
        return;
    }
    for value in values {
        if !items.iter().any(|item| item == value) {
            items.push(value.clone());
        }
    }
}

fn format_toml_items(items: &[String]) -> String {
    items
        .iter()
        .map(|item| format!("\"{item}\""))
        .collect::<Vec<_>>()
        .join(", ")
}

fn format_typescript_items(items: &[String]) -> String {
    items
        .iter()
        .map(|item| format!("'{item}'"))
        .collect::<Vec<_>>()
        .join(", ")
}

impl FeatureManifest {
    fn skill_name(&self) -> Option<&str> {
        self.adds
            .skill
            .as_deref()
            .or_else(|| self.adds.skills.first().map(|item| item.as_str()))
    }
}

fn authoring_features_root(language: &LanguageManifest) -> PathBuf {
    let registry_path = repo_root()
        .join("languages")
        .join(&language.id)
        .join(&language.feature_registry);
    registry_path
        .parent()
        .expect("feature registry should have a parent")
        .to_path_buf()
}

fn authoring_feature_manifest_path(language: &LanguageManifest, feature_id: &str) -> PathBuf {
    authoring_features_root(language)
        .join(feature_id)
        .join("feature.json")
}

fn authoring_feature_files_root(language: &LanguageManifest, feature_id: &str) -> PathBuf {
    authoring_features_root(language).join(feature_id).join("files")
}

fn authoring_feature_skill_path(language: &LanguageManifest, feature_id: &str) -> PathBuf {
    authoring_features_root(language)
        .join(feature_id)
        .join("skill")
        .join("SKILL.md")
}

fn authoring_feature_agents_root(language: &LanguageManifest, feature_id: &str) -> PathBuf {
    authoring_features_root(language)
        .join(feature_id)
        .join("agents")
}
