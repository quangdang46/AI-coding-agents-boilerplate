use std::fs;
use std::fs::OpenOptions;
use std::path::Path;
use std::process::Command;

use crate::config::{
    checksum, get_state_value, load_runtime_config, read_state, read_text, write_state,
    RuntimeConfig,
};
use crate::providers::provider_summary;

pub fn policy_for_operation(
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

pub fn run_core_tools(root: &Path, config: &RuntimeConfig) -> String {
    let usage_path = root.join(".agent/usage/runtime-tool-smoke.txt");
    let mut results = Vec::new();

    let bash_status = if config.enabled_tools.iter().any(|tool| tool == "bash") {
        policy_for_operation(&config.approval_mode, &config.deny, "bash", "bash")
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

    let file_read_status = if config.enabled_tools.iter().any(|tool| tool == "file_read") {
        policy_for_operation(
            &config.approval_mode,
            &config.deny,
            "file_read",
            "file_read",
        )
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

    let file_write_status = if config.enabled_tools.iter().any(|tool| tool == "file_write") {
        policy_for_operation(
            &config.approval_mode,
            &config.deny,
            "file_write",
            "file_write",
        )
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

    let file_edit_status = if config.enabled_tools.iter().any(|tool| tool == "file_edit") {
        policy_for_operation(
            &config.approval_mode,
            &config.deny,
            "file_edit",
            "file_edit",
        )
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

    let web_fetch_status = if config.enabled_tools.iter().any(|tool| tool == "web_fetch") {
        policy_for_operation(
            &config.approval_mode,
            &config.deny,
            "web_fetch",
            "web_fetch",
        )
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

pub fn persist_session_and_usage(
    root: &Path,
    config: &RuntimeConfig,
    prompt_digest: &str,
    context_digest: &str,
    prompt_texts: &[String],
    context_texts: &[String],
    tool_results: &str,
) -> String {
    let session_id = String::from("local-main-session");
    let session_path = root.join(".agent/sessions/local-main-session.state");
    let latest_path = root.join(".agent/sessions/latest.state");
    let export_relative = ".agent/sessions/local-main-session.export.md";
    let export_path = root.join(export_relative);
    let usage_log_path = root.join(".agent/usage/ledger.log");
    let usage_summary_path = root.join(".agent/usage/summary.state");

    let previous_session = read_state(&session_path);
    let turn_count = get_state_value(&previous_session, "turn_count")
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(0)
        + 1;
    let previous_summary = read_state(&usage_summary_path);
    let usage_entries = get_state_value(&previous_summary, "usage_entries")
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(0)
        + 1;
    let cost_micros = ((prompt_texts
        .iter()
        .chain(context_texts.iter())
        .map(|text| text.len() as u64)
        .sum::<u64>())
        * 2)
        + ((tool_results.len() as u64) * 3);
    let total_cost_micros = get_state_value(&previous_summary, "total_cost_micros")
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(0)
        + cost_micros;

    let state_entries = vec![
        (String::from("session_id"), session_id.clone()),
        (String::from("turn_count"), turn_count.to_string()),
        (String::from("provider"), config.default_provider.clone()),
        (String::from("model"), config.provider_model.clone()),
        (String::from("prompt_digest"), prompt_digest.to_string()),
        (String::from("context_digest"), context_digest.to_string()),
    ];
    write_state(&session_path, &state_entries);
    write_state(&latest_path, &state_entries);
    fs::write(
        &export_path,
        format!(
            "# Session Export\n\n- session_id: {session_id}\n- turn_count: {turn_count}\n- provider: {}\n- model: {}\n- prompt_digest: {prompt_digest}\n- context_digest: {context_digest}\n",
            config.default_provider, config.provider_model
        ),
    )
    .unwrap_or_else(|err| panic!("failed to write {}: {err}", export_path.display()));

    let mut ledger = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&usage_log_path)
        .unwrap_or_else(|err| panic!("failed to open {}: {err}", usage_log_path.display()));
    use std::io::Write;
    writeln!(
        ledger,
        "session_id={session_id} turn_count={turn_count} cost_micros={cost_micros}"
    )
    .unwrap_or_else(|err| panic!("failed to append {}: {err}", usage_log_path.display()));
    write_state(
        &usage_summary_path,
        &[
            (String::from("usage_entries"), usage_entries.to_string()),
            (
                String::from("total_cost_micros"),
                total_cost_micros.to_string(),
            ),
        ],
    );

    format!(
        "session_id={session_id} turn_count={turn_count} export_path={export_relative} usage_entries={usage_entries} total_cost_micros={total_cost_micros}"
    )
}

pub fn load_runtime_summary() -> String {
    let root = crate::config::project_root();
    let config = load_runtime_config(&root);

    let prompt_texts = vec![
        read_text(&root.join("CLAW.md")),
        read_text(&root.join(".agent/prompts/system.md")),
        read_text(&root.join(".agent/prompts/sections/style.md")),
        read_text(&root.join(".agent/prompts/sections/verification.md")),
    ];

    let context_texts = config
        .context_paths
        .iter()
        .map(|path| read_text(&root.join(path)))
        .collect::<Vec<_>>();
    let prompt_digest = checksum(&prompt_texts);
    let context_digest = checksum(&context_texts);
    let tool_results = run_core_tools(&root, &config);
    let session_summary = persist_session_and_usage(
        &root,
        &config,
        &prompt_digest,
        &context_digest,
        &prompt_texts,
        &context_texts,
        &tool_results,
    );

    format!(
        "{} prompt_digest={} context_digest={} approval_mode={} bash_policy={} file_write_policy={} {} {}",
        provider_summary(&config),
        prompt_digest,
        context_digest,
        config.approval_mode,
        policy_for_operation(&config.approval_mode, &config.deny, "bash", "bash"),
        policy_for_operation(&config.approval_mode, &config.deny, "file_write", "file_write"),
        tool_results,
        session_summary
    )
}
