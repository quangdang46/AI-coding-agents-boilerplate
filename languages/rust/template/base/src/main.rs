use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

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
    if approval_mode == "default" && ["bash", "file_edit", "file_write"].contains(&tool_name) {
        return format!("{operation}=approval-required");
    }
    format!("{operation}=allowed")
}

fn extract_tools_enabled(source: &str) -> Vec<String> {
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

fn run_core_tools(root: &Path, config_text: &str, approval_mode: &str, deny: &[String]) -> String {
    let enabled_tools = extract_tools_enabled(config_text);
    let usage_path = root.join(".agent/usage/runtime-tool-smoke.txt");
    let mut results = Vec::new();

    let bash_status = if enabled_tools.iter().any(|tool| tool == "bash") {
        policy_for_operation(approval_mode, deny, "bash", "bash")
    } else {
        String::from("bash=disabled")
    };
    if bash_status == "bash=allowed" {
        let output = Command::new("bash")
            .args(["-lc", "printf tool-bash-ok"])
            .output()
            .unwrap_or_else(|err| panic!("failed to run bash tool: {err}"));
        results.push(format!(
            "bash_result={}",
            String::from_utf8_lossy(&output.stdout).trim()
        ));
    } else {
        results.push(format!("bash_result={bash_status}"));
    }

    let file_read_status = if enabled_tools.iter().any(|tool| tool == "file_read") {
        policy_for_operation(approval_mode, deny, "file_read", "file_read")
    } else {
        String::from("file_read=disabled")
    };
    if file_read_status == "file_read=allowed" {
        results.push(format!(
            "file_read_result={}",
            checksum(&[read_text(&root.join(".agent/context/README.md"))])
        ));
    } else {
        results.push(format!("file_read_result={file_read_status}"));
    }

    let file_write_status = if enabled_tools.iter().any(|tool| tool == "file_write") {
        policy_for_operation(approval_mode, deny, "file_write", "file_write")
    } else {
        String::from("file_write=disabled")
    };
    if file_write_status == "file_write=allowed" {
        fs::write(&usage_path, "tool-write-ok")
            .unwrap_or_else(|err| panic!("failed to write tool file: {err}"));
        results.push(String::from("file_write_result=tool-write-ok"));
    } else {
        results.push(format!("file_write_result={file_write_status}"));
    }

    let file_edit_status = if enabled_tools.iter().any(|tool| tool == "file_edit") {
        policy_for_operation(approval_mode, deny, "file_edit", "file_edit")
    } else {
        String::from("file_edit=disabled")
    };
    if file_edit_status == "file_edit=allowed" {
        if !usage_path.exists() {
            fs::write(&usage_path, "tool-write-ok")
                .unwrap_or_else(|err| panic!("failed to seed tool file: {err}"));
        }
        let edited = format!("{} edited", read_text(&usage_path));
        fs::write(&usage_path, &edited)
            .unwrap_or_else(|err| panic!("failed to edit tool file: {err}"));
        results.push(format!("file_edit_result={edited}"));
    } else {
        results.push(format!("file_edit_result={file_edit_status}"));
    }

    let web_fetch_status = if enabled_tools.iter().any(|tool| tool == "web_fetch") {
        policy_for_operation(approval_mode, deny, "web_fetch", "web_fetch")
    } else {
        String::from("web_fetch=disabled")
    };
    if web_fetch_status == "web_fetch=allowed" {
        results.push(String::from("web_fetch_result=tool-web-fetch"));
    } else {
        results.push(format!("web_fetch_result={web_fetch_status}"));
    }

    results.join(" ")
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
        "provider={default_provider} model={provider_model} prompt_digest={} context_digest={} approval_mode={} bash_policy={} file_write_policy={} {}",
        checksum(&prompt_texts),
        checksum(&context_texts),
        approval_mode,
        policy_for_operation(&approval_mode, &deny, "bash", "bash"),
        policy_for_operation(&approval_mode, &deny, "file_write", "file_write"),
        run_core_tools(&root, &config_text, &approval_mode, &deny)
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
