use std::path::{Path, PathBuf};

use crate::fs_ops::{copy_tree_contents, remove_tree_contents};
use crate::manifest::set_feature_enabled;

#[derive(Clone, Debug)]
pub struct FeatureArgs {
    pub feature_id: String,
    pub project: PathBuf,
}

pub fn add(args: &FeatureArgs) -> Result<String, String> {
    let feature_files_root = feature_files_root(&args.project, &args.feature_id);
    if !feature_manifest_path(&args.project, &args.feature_id).exists() {
        return Err(format!(
            "unknown feature '{}': {}",
            args.feature_id,
            feature_manifest_path(&args.project, &args.feature_id).display()
        ));
    }
    copy_tree_contents(&feature_files_root, &args.project)
        .map_err(|err| format!("feature add failed: {err}"))?;
    set_feature_enabled(&args.project, &args.feature_id, true)?;
    Ok(format!(
        "added feature {} to {}",
        args.feature_id,
        args.project.display()
    ))
}

pub fn remove(args: &FeatureArgs) -> Result<String, String> {
    let feature_files_root = feature_files_root(&args.project, &args.feature_id);
    if !feature_manifest_path(&args.project, &args.feature_id).exists() {
        return Err(format!(
            "unknown feature '{}': {}",
            args.feature_id,
            feature_manifest_path(&args.project, &args.feature_id).display()
        ));
    }
    remove_tree_contents(&feature_files_root, &args.project)
        .map_err(|err| format!("feature remove failed: {err}"))?;
    set_feature_enabled(&args.project, &args.feature_id, false)?;
    Ok(format!(
        "removed feature {} from {}",
        args.feature_id,
        args.project.display()
    ))
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
