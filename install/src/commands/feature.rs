use std::path::{Path, PathBuf};

use crate::fs_ops::{copy_tree_contents, remove_tree_contents};
use crate::manifest::detect_project_language;
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
    #[serde(default)]
    patches: Vec<FeaturePatch>,
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
    let feature_id = normalize_feature_id(&language.id, &args.feature_id)?;
    if !language.supports.feature_add {
        return Err(format!(
            "unsupported capability feature add for language: {}",
            language.id
        ));
    }
    let feature_files_root = feature_files_root(&args.project, &feature_id);
    if !feature_manifest_path(&args.project, &feature_id).exists() {
        return Err(format!(
            "unknown feature '{}': {}",
            args.feature_id,
            feature_manifest_path(&args.project, &feature_id).display()
        ));
    }
    let manifest = load_feature_manifest(&args.project, &feature_id)?;
    copy_tree_contents(&feature_files_root, &args.project)
        .map_err(|err| format!("feature add failed: {err}"))?;
    apply_feature_patches(&args.project, &manifest, true)?;
    Ok(format!(
        "added feature {} to {}",
        feature_id,
        args.project.display()
    ))
}

pub fn remove(args: &FeatureArgs) -> Result<String, String> {
    let language = detect_project_language(&args.project)?;
    let feature_id = normalize_feature_id(&language.id, &args.feature_id)?;
    if !language.supports.feature_remove {
        return Err(format!(
            "unsupported capability feature remove for language: {}",
            language.id
        ));
    }
    let feature_files_root = feature_files_root(&args.project, &feature_id);
    if !feature_manifest_path(&args.project, &feature_id).exists() {
        return Err(format!(
            "unknown feature '{}': {}",
            args.feature_id,
            feature_manifest_path(&args.project, &feature_id).display()
        ));
    }
    let manifest = load_feature_manifest(&args.project, &feature_id)?;
    remove_tree_contents(&feature_files_root, &args.project)
        .map_err(|err| format!("feature remove failed: {err}"))?;
    apply_feature_patches(&args.project, &manifest, false)?;
    Ok(format!(
        "removed feature {} from {}",
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

fn load_feature_manifest(project_root: &Path, feature_id: &str) -> Result<FeatureManifest, String> {
    let manifest_path = feature_manifest_path(project_root, feature_id);
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
    Ok(manifest)
}

fn apply_feature_patches(
    project_root: &Path,
    manifest: &FeatureManifest,
    enabled: bool,
) -> Result<(), String> {
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
            ".claw.json" => apply_json_merge_patch(project_root, patch, enabled)?,
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
) -> Result<(), String> {
    let path = project_root.join(&patch.target);
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
    if enabled {
        for value in values {
            if !items.iter().any(|item| item == value) {
                items.push(value.clone());
            }
        }
    } else {
        items.retain(|item| !values.iter().any(|value| value == item));
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

fn feature_manifest_path(project_root: &Path, feature_id: &str) -> PathBuf {
    project_root
        .join(".agent/features")
        .join(feature_id)
        .join("feature.json")
}

fn feature_files_root(project_root: &Path, feature_id: &str) -> PathBuf {
    project_root
        .join(".agent/features")
        .join(feature_id)
        .join("files")
}
