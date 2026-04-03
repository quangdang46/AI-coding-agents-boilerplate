use std::io;
use std::path::{Path, PathBuf};

use crate::fs_ops::{copy_dir_with_placeholders, normalize_package_name};
use crate::manifest::LanguageManifest;

pub fn template_root(manifest: &LanguageManifest) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("languages")
        .join(&manifest.id)
        .join(&manifest.templates.base)
}

pub fn render_template_from_manifest(
    manifest: &LanguageManifest,
    project_name: &str,
    output_root: &Path,
    package_name: Option<&str>,
    binary_name: Option<&str>,
) -> io::Result<()> {
    let package_name = package_name
        .map(|value| value.to_string())
        .unwrap_or_else(|| normalize_package_name(project_name));
    let binary_name = binary_name.unwrap_or(project_name);
    copy_dir_with_placeholders(
        &template_root(manifest),
        output_root,
        project_name,
        &package_name,
        binary_name,
    )
}
