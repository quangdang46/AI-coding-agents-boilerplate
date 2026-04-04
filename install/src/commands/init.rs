use std::path::PathBuf;

use crate::brand::{normalize_brand, BrandPaths};
use crate::manifest::discover_language;
use crate::renderer::render_template_from_manifest;

#[derive(Clone, Debug)]
pub struct InitArgs {
    pub project_name: String,
    pub language: String,
    pub output: PathBuf,
    pub package_name: Option<String>,
    pub binary_name: Option<String>,
}

pub fn run(args: &InitArgs) -> Result<String, String> {
    let manifest = discover_language(&args.language)?;
    if !manifest.supports.init {
        return Err(format!(
            "unsupported capability init for language: {}",
            args.language
        ));
    }
    let brand_name = normalize_brand(args.binary_name.as_deref().unwrap_or(&args.project_name));
    let brand = BrandPaths::new(&brand_name)?;
    render_template_from_manifest(
        &manifest,
        &args.project_name,
        &args.output,
        args.package_name.as_deref(),
        args.binary_name.as_deref(),
        &brand,
    )
    .map_err(|err| format!("init failed: {err}"))?;
    Ok(format!(
        "initialized {} project at {}",
        manifest.id,
        args.output.display()
    ))
}
