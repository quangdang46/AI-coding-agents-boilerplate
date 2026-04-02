use crate::catalog::{LANGUAGES, PROMPT_PACKS, PYTHON_FEATURES, PYTHON_TEMPLATES};

pub fn run(topic: &str) -> Result<String, String> {
    let lines = match topic {
        "languages" => LANGUAGES,
        "templates" => PYTHON_TEMPLATES,
        "features" => PYTHON_FEATURES,
        "prompt-packs" => PROMPT_PACKS,
        other => return Err(format!("unknown list topic: {other}")),
    };
    Ok(lines.join(
        "
",
    ))
}
