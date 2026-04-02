use std::io;
use std::path::{Path, PathBuf};

use crate::fs_ops::{copy_dir_with_placeholders, normalize_package_name};

pub fn python_template_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../python/templates/base")
}

pub fn render_python_template(
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
        &python_template_root(),
        output_root,
        project_name,
        &package_name,
        binary_name,
    )
}
