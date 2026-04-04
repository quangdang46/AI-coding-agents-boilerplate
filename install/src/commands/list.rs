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
        let registry_path = repo_root()
            .join("languages")
            .join(&manifest.id)
            .join(&manifest.feature_registry);
        if !registry_path.exists() {
            continue;
        }
        let raw = std::fs::read_to_string(&registry_path)
            .map_err(|err| format!("failed to read {}: {err}", registry_path.display()))?;
        let value: serde_json::Value = serde_json::from_str(&raw)
            .map_err(|err| format!("failed to parse {}: {err}", registry_path.display()))?;
        let entries = value
            .get("features")
            .and_then(|features| features.as_array())
            .ok_or_else(|| format!("invalid feature registry: {}", registry_path.display()))?;
        for entry in entries {
            if let Some(id) = entry.get("id").and_then(|id| id.as_str()) {
                features.push(format!("{}:{id}", manifest.id));
            }
        }
    }
    features.sort();
    Ok(features.join("\n"))
}
