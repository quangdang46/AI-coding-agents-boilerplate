use std::fs;
use std::path::Path;
use std::process::Command;

use crate::config::{checksum, read_text, RuntimeConfig};

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
