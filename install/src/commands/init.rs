use std::path::PathBuf;

use crate::renderer::render_python_template;

#[derive(Clone, Debug)]
pub struct InitArgs {
    pub project_name: String,
    pub language: String,
    pub output: PathBuf,
    pub package_name: Option<String>,
    pub binary_name: Option<String>,
}

pub fn run(args: &InitArgs) -> Result<String, String> {
    if args.language != "python" {
        return Err(format!(
            "unsupported language for current slice: {}",
            args.language
        ));
    }
    render_python_template(
        &args.project_name,
        &args.output,
        args.package_name.as_deref(),
        args.binary_name.as_deref(),
    )
    .map_err(|err| format!("init failed: {err}"))?;
    Ok(format!(
        "initialized python project at {}",
        args.output.display()
    ))
}
