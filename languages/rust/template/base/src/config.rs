use std::fs;
use std::path::{Path, PathBuf};

use crate::permissions::{permission_mode_from_config, PermissionMode};
use crate::sandbox::{FilesystemIsolationMode, SandboxConfig};

#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    pub default_provider: String,
    pub provider_model: String,
    pub approval_mode: String,
    pub permission_mode: PermissionMode,
    pub deny: Vec<String>,
    pub context_paths: Vec<String>,
    pub enabled_tools: Vec<String>,
    pub sandbox: SandboxConfig,
}

pub fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub fn read_text(path: &Path) -> String {
    fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()))
        .trim()
        .to_string()
}

pub fn checksum(parts: &[String]) -> String {
    let mut total: u64 = 0;
    for part in parts {
        for byte in part.as_bytes() {
            total = (total.wrapping_mul(31) + u64::from(*byte)) % 0x7fff_ffff;
        }
        total = (total.wrapping_mul(31) + 1) % 0x7fff_ffff;
    }
    format!("{total:08x}")
}

pub fn read_state(path: &Path) -> Vec<(String, String)> {
    if !path.exists() {
        return Vec::new();
    }
    read_text(path)
        .lines()
        .filter_map(|line| line.split_once('='))
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect()
}

pub fn get_state_value(state: &[(String, String)], key: &str) -> Option<String> {
    state
        .iter()
        .find(|(entry_key, _)| entry_key == key)
        .map(|(_, value)| value.clone())
}

pub fn write_state(path: &Path, entries: &[(String, String)]) {
    let content = entries
        .iter()
        .map(|(key, value)| format!("{key}={value}"))
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(path, format!("{content}\n"))
        .unwrap_or_else(|err| panic!("failed to write {}: {err}", path.display()));
}

pub fn extract_json_string(source: &str, key: &str) -> String {
    let needle = format!("\"{key}\": \"");
    let start = source
        .find(&needle)
        .unwrap_or_else(|| panic!("missing key {key}"))
        + needle.len();
    let end = source[start..]
        .find('"')
        .unwrap_or_else(|| panic!("unterminated value for key {key}"));
    source[start..start + end].to_string()
}

pub fn extract_json_list(source: &str, key: &str) -> Vec<String> {
    let marker = format!("\"{key}\": [");
    let start = source
        .find(&marker)
        .unwrap_or_else(|| panic!("missing list {key}"))
        + marker.len();
    let end = source[start..]
        .find(']')
        .unwrap_or_else(|| panic!("unterminated list {key}"));
    source[start..start + end]
        .split(',')
        .map(|item| item.trim().trim_matches('"'))
        .filter(|item| !item.is_empty())
        .map(|item| item.to_string())
        .collect()
}

pub fn extract_context_paths(source: &str) -> Vec<String> {
    let marker = "\"paths\": [";
    let start = source
        .find(marker)
        .unwrap_or_else(|| panic!("missing context.paths"))
        + marker.len();
    let end = source[start..]
        .find(']')
        .unwrap_or_else(|| panic!("unterminated context.paths"));
    source[start..start + end]
        .split(',')
        .map(|item| item.trim().trim_matches('"'))
        .filter(|item| !item.is_empty())
        .map(|item| item.to_string())
        .collect()
}

pub fn extract_provider_model(source: &str, provider: &str) -> String {
    let marker = format!("\"{provider}\": {{");
    let start = source
        .find(&marker)
        .unwrap_or_else(|| panic!("missing provider {provider}"));
    let provider_slice = &source[start..];
    extract_json_string(provider_slice, "model")
}

pub fn extract_tools_enabled(source: &str) -> Vec<String> {
    let marker = "\"enabled\": [";
    let start = source
        .find(marker)
        .unwrap_or_else(|| panic!("missing tools.enabled"))
        + marker.len();
    let end = source[start..]
        .find(']')
        .unwrap_or_else(|| panic!("unterminated tools.enabled"));
    source[start..start + end]
        .split(',')
        .map(|item| item.trim().trim_matches('"'))
        .filter(|item| !item.is_empty())
        .map(|item| item.to_string())
        .collect()
}

pub fn load_runtime_config(root: &Path) -> RuntimeConfig {
    let config_text = read_text(&root.join(".claw.json"));
    let default_provider = extract_json_string(&config_text, "defaultProvider");
    let approval_mode = extract_json_string(&config_text, "defaultMode");

    RuntimeConfig {
        provider_model: extract_provider_model(&config_text, &default_provider),
        approval_mode: approval_mode.clone(),
        permission_mode: permission_mode_from_config(&approval_mode),
        deny: extract_json_list(&config_text, "deny"),
        context_paths: extract_context_paths(&config_text),
        enabled_tools: extract_tools_enabled(&config_text),
        sandbox: SandboxConfig {
            enabled: None,
            namespace_restrictions: None,
            network_isolation: None,
            filesystem_mode: Some(FilesystemIsolationMode::WorkspaceOnly),
            allowed_mounts: Vec::new(),
        },
        default_provider,
    }
}
