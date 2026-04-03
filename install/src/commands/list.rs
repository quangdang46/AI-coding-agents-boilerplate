use crate::catalog::PROMPT_PACKS;
use crate::manifest::{discover_languages, repo_root};

pub fn run(topic: &str) -> Result<String, String> {
    let lines = match topic {
        "languages" => {
            let manifests = discover_languages()?;
            return Ok(manifests
                .into_iter()
                .map(|manifest| manifest.id)
                .collect::<Vec<_>>()
                .join("\n"));
        }
        "templates" => return list_templates(),
        "features" => return list_features(),
        "prompt-packs" => PROMPT_PACKS,
        other => return Err(format!("unknown list topic: {other}")),
    };
    Ok(lines.join(
        "
",
    ))
}

fn list_templates() -> Result<String, String> {
    let mut templates = discover_languages()?
        .into_iter()
        .filter(|manifest| {
            repo_root()
                .join("languages")
                .join(&manifest.id)
                .join(&manifest.templates.base)
                .exists()
        })
        .map(|manifest| format!("{}:base", manifest.id))
        .collect::<Vec<_>>();
    templates.sort();
    Ok(templates.join("\n"))
}

fn list_features() -> Result<String, String> {
    let mut features = Vec::new();
    for manifest in discover_languages()? {
        let features_root = repo_root()
            .join("languages")
            .join(&manifest.id)
            .join(&manifest.templates.base)
            .join(".agent/features");
        if !features_root.exists() {
            continue;
        }
        let entries = std::fs::read_dir(&features_root)
            .map_err(|err| format!("failed to read {}: {err}", features_root.display()))?;
        for entry in entries {
            let entry = entry.map_err(|err| format!("failed to read feature entry: {err}"))?;
            let feature_manifest = entry.path().join("feature.json");
            if feature_manifest.exists() {
                let name = entry.file_name().to_string_lossy().to_string();
                features.push(format!("{}:{}", manifest.id, name));
            }
        }
    }
    features.sort();
    Ok(features.join("\n"))
}
