use std::collections::HashMap;
use std::fs;
use std::path::Path;

const BRAND_CONFIG_SUFFIX: &str = ".json";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrandPaths {
    brand: String,
}

impl BrandPaths {
    pub fn new(raw_brand: &str) -> Result<Self, String> {
        let brand = normalize_brand(raw_brand);
        if brand.is_empty() {
            return Err(format!("invalid brand: {raw_brand}"));
        }
        Ok(Self { brand })
    }

    pub fn brand(&self) -> &str {
        &self.brand
    }

    pub fn hidden_root(&self) -> String {
        format!(".{}", self.brand)
    }

    pub fn config_file(&self) -> String {
        format!("{}.json", self.hidden_root())
    }

    pub fn doc_file(&self) -> String {
        format!("{}.md", self.brand.to_ascii_uppercase())
    }

    pub fn plugin_root(&self) -> String {
        format!(".{}-plugin", self.brand)
    }
}

pub fn normalize_brand(raw: &str) -> String {
    let mut normalized = String::new();
    let mut previous_dash = false;
    for ch in raw.chars() {
        let ch = ch.to_ascii_lowercase();
        if ch.is_ascii_alphanumeric() {
            normalized.push(ch);
            previous_dash = false;
        } else if !previous_dash && !normalized.is_empty() {
            normalized.push('-');
            previous_dash = true;
        }
    }
    normalized.trim_matches('-').to_string()
}

pub fn resolve_brand_placeholders(source: &str, brand: &BrandPaths) -> String {
    source
        .replace("__BRAND__", brand.brand())
        .replace("__BRAND_ROOT__", &brand.hidden_root())
        .replace("__BRAND_CONFIG__", &brand.config_file())
        .replace("__BRAND_DOC__", &brand.doc_file())
        .replace("__BRAND_PLUGIN_ROOT__", &brand.plugin_root())
}

pub fn infer_brand_paths(project_root: &Path) -> Result<BrandPaths, String> {
    let mut scores: HashMap<String, usize> = HashMap::new();
    let entries = fs::read_dir(project_root)
        .map_err(|err| format!("failed to read {}: {err}", project_root.display()))?;

    for entry in entries {
        let entry = entry.map_err(|err| format!("failed to read project entry: {err}"))?;
        let path = entry.path();
        let Some(name) = path.file_name().and_then(|value| value.to_str()) else {
            continue;
        };

        if path.is_dir() {
            if let Some(brand) = hidden_root_candidate(name) {
                let mut score = 1;
                for child in [
                    "settings.json",
                    "settings.local.json",
                    "instructions.md",
                    "agents",
                    "skills",
                    "commands",
                    "sessions",
                ] {
                    if path.join(child).exists() {
                        score += 3;
                    }
                }
                if score > 1 {
                    *scores.entry(brand).or_default() += score;
                }
            }
            continue;
        }

        if let Some(brand) = config_file_candidate(name) {
            *scores.entry(brand).or_default() += 4;
            continue;
        }

        if let Some(brand) = doc_file_candidate(name) {
            *scores.entry(brand).or_default() += 2;
        }
    }

    scores
        .into_iter()
        .max_by(|(left_brand, left_score), (right_brand, right_score)| {
            left_score
                .cmp(right_score)
                .then_with(|| right_brand.len().cmp(&left_brand.len()))
        })
        .map(|(brand, _)| BrandPaths::new(&brand))
        .transpose()?
        .ok_or_else(|| format!("unable to infer project brand for {}", project_root.display()))
}

fn hidden_root_candidate(name: &str) -> Option<String> {
    if !name.starts_with('.') || name.contains("-plugin") {
        return None;
    }
    let brand = normalize_brand(name.trim_start_matches('.'));
    if brand.is_empty() {
        None
    } else {
        Some(brand)
    }
}

fn config_file_candidate(name: &str) -> Option<String> {
    if !name.starts_with('.') || !name.ends_with(BRAND_CONFIG_SUFFIX) {
        return None;
    }
    let brand = normalize_brand(
        name.trim_start_matches('.')
            .trim_end_matches(BRAND_CONFIG_SUFFIX),
    );
    if brand.is_empty() {
        None
    } else {
        Some(brand)
    }
}

fn doc_file_candidate(name: &str) -> Option<String> {
    if !name.ends_with(".md") {
        return None;
    }
    let stem = name.trim_end_matches(".md");
    if matches!(stem, "AGENTS" | "CLAUDE" | "README") {
        return None;
    }
    let brand = normalize_brand(stem);
    if brand.is_empty() {
        None
    } else {
        Some(brand)
    }
}
