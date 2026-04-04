use std::path::Path;

use crate::bash::run_bash;
use crate::config::RuntimeConfig;
use crate::file_ops::{run_file_edit, run_file_read, run_file_write};
use crate::permission_enforcer::{EnforcementResult, PermissionEnforcer};
use crate::permissions::{permission_mode_from_config, PermissionPolicy};
use crate::sandbox::resolve_sandbox_status_for_request;

pub fn policy_for_operation(
    approval_mode: &str,
    deny: &[String],
    operation: &str,
    tool_name: &str,
) -> String {
    let policy = PermissionPolicy::new(permission_mode_from_config(approval_mode))
        .with_permission_rules(deny);
    let enforcer = PermissionEnforcer::new(policy);

    match enforcer.check(tool_name, "{}") {
        EnforcementResult::Allowed => {
            if approval_mode == "default"
                && ["bash", "file_edit", "file_write"].contains(&tool_name)
            {
                format!("{operation}=approval-required")
            } else {
                format!("{operation}=allowed")
            }
        }
        EnforcementResult::Denied { reason, .. } => {
            if reason.contains("denied by rule") {
                format!("{operation}=denied")
            } else if approval_mode == "never" {
                format!("{operation}=blocked")
            } else {
                format!("{operation}=approval-required")
            }
        }
    }
}

pub fn run_core_tools(root: &Path, config: &RuntimeConfig) -> String {
    let mut results = Vec::new();
    let policy = PermissionPolicy::new(config.permission_mode).with_permission_rules(&config.deny);
    let enforcer = PermissionEnforcer::new(policy);
    let _sandbox_status = resolve_sandbox_status_for_request(&config.sandbox.resolve_request());

    let bash_status = if config.enabled_tools.iter().any(|tool| tool == "bash") {
        policy_for_operation(&config.approval_mode, &config.deny, "bash", "bash")
    } else {
        String::from("bash=disabled")
    };
    if bash_status == "bash=allowed" {
        results.push(format!(
            "bash_result={}",
            run_bash(root, "printf tool-bash-ok")
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
        results.push(format!("file_read_result={}", run_file_read(root)));
    } else {
        results.push(format!("file_read_result={file_read_status}"));
    }

    let workspace_root = root.display().to_string();

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
    if matches!(
        enforcer.check_file_write(
            "__BRAND_ROOT__/sessions/runtime-tool-smoke.txt",
            &workspace_root
        ),
        EnforcementResult::Allowed
    ) && file_write_status == "file_write=allowed"
    {
        results.push(format!("file_write_result={}", run_file_write(root)));
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
    if matches!(
        enforcer.check_file_write(
            "__BRAND_ROOT__/sessions/runtime-tool-smoke.txt",
            &workspace_root
        ),
        EnforcementResult::Allowed
    ) && file_edit_status == "file_edit=allowed"
    {
        results.push(format!("file_edit_result={}", run_file_edit(root)));
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
