use std::fs;
use std::path::{Path, PathBuf};

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read_text(path: &Path) -> String {
    fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()))
        .trim()
        .to_string()
}

fn checksum(parts: &[String]) -> String {
    let mut total: u64 = 0;
    for part in parts {
        for byte in part.as_bytes() {
            total = (total.wrapping_mul(31) + u64::from(*byte)) % 0x7fff_ffff;
        }
        total = (total.wrapping_mul(31) + 1) % 0x7fff_ffff;
    }
    format!("{total:08x}")
}

fn extract_json_string(source: &str, key: &str) -> String {
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

fn extract_context_paths(source: &str) -> Vec<String> {
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

fn extract_provider_model(source: &str, provider: &str) -> String {
    let marker = format!("\"{provider}\": {{");
    let start = source
        .find(&marker)
        .unwrap_or_else(|| panic!("missing provider {provider}"));
    let provider_slice = &source[start..];
    extract_json_string(provider_slice, "model")
}

fn extract_json_list(source: &str, key: &str) -> Vec<String> {
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

fn policy_for_operation(
    approval_mode: &str,
    deny: &[String],
    operation: &str,
    tool_name: &str,
) -> String {
    if deny.iter().any(|value| value == tool_name) {
        return format!("{operation}=denied");
    }
    if approval_mode == "never" {
        return format!("{operation}=blocked");
    }
    if approval_mode == "default" {
        return format!("{operation}=approval-required");
    }
    format!("{operation}=allowed")
}

fn load_runtime_summary() -> String {
    let root = project_root();
    let config_text = read_text(&root.join(".claw.json"));
    let default_provider = extract_json_string(&config_text, "defaultProvider");
    let provider_model = extract_provider_model(&config_text, &default_provider);
    let approval_mode = extract_json_string(&config_text, "defaultMode");
    let deny = extract_json_list(&config_text, "deny");

    let prompt_texts = vec![
        read_text(&root.join("CLAW.md")),
        read_text(&root.join(".agent/prompts/system.md")),
        read_text(&root.join(".agent/prompts/sections/style.md")),
        read_text(&root.join(".agent/prompts/sections/verification.md")),
    ];

    let context_texts = extract_context_paths(&config_text)
        .into_iter()
        .map(|path| read_text(&root.join(path)))
        .collect::<Vec<_>>();

    format!(
        "provider={default_provider} model={provider_model} prompt_digest={} context_digest={} approval_mode={} bash_policy={} file_write_policy={}",
        checksum(&prompt_texts),
        checksum(&context_texts),
        approval_mode,
        policy_for_operation(&approval_mode, &deny, "bash", "bash"),
        policy_for_operation(&approval_mode, &deny, "file_write", "file_write")
    )
}

fn run_session_loop() -> String {
    format!(
        "__PROJECT_NAME__ session loop completed {}",
        load_runtime_summary()
    )
}

fn main() {
    println!("{}", run_session_loop());
}
