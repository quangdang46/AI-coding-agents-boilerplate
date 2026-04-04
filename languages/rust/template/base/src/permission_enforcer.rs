use crate::permissions::{PermissionMode, PermissionOutcome, PermissionPolicy};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnforcementResult {
    Allowed,
    Denied {
        tool: String,
        active_mode: String,
        required_mode: String,
        reason: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PermissionEnforcer {
    policy: PermissionPolicy,
}

impl PermissionEnforcer {
    pub fn new(policy: PermissionPolicy) -> Self {
        Self { policy }
    }

    pub fn check(&self, tool_name: &str, input: &str) -> EnforcementResult {
        let outcome = self.policy.authorize(tool_name, input, None);

        match outcome {
            PermissionOutcome::Allow => EnforcementResult::Allowed,
            PermissionOutcome::Deny { reason } => {
                let active_mode = self.policy.active_mode();
                let required_mode = self.policy.required_mode_for(tool_name);
                EnforcementResult::Denied {
                    tool: tool_name.to_owned(),
                    active_mode: active_mode.as_config_str().to_owned(),
                    required_mode: required_mode.as_config_str().to_owned(),
                    reason,
                }
            }
        }
    }

    pub fn check_file_write(&self, path: &str, workspace_root: &str) -> EnforcementResult {
        let mode = self.policy.active_mode();

        match mode {
            PermissionMode::Never => EnforcementResult::Denied {
                tool: "write_file".to_owned(),
                active_mode: mode.as_config_str().to_owned(),
                required_mode: PermissionMode::Default.as_config_str().to_owned(),
                reason: format!(
                    "file writes are not allowed in '{}' mode",
                    mode.as_config_str()
                ),
            },
            PermissionMode::Default | PermissionMode::DontAsk => {
                if is_within_workspace(path, workspace_root) {
                    EnforcementResult::Allowed
                } else {
                    EnforcementResult::Denied {
                        tool: "write_file".to_owned(),
                        active_mode: mode.as_config_str().to_owned(),
                        required_mode: PermissionMode::DontAsk.as_config_str().to_owned(),
                        reason: format!(
                            "path '{}' is outside workspace root '{}'",
                            path, workspace_root
                        ),
                    }
                }
            }
        }
    }

    pub fn check_bash(&self, command: &str) -> EnforcementResult {
        let mode = self.policy.active_mode();

        match mode {
            PermissionMode::Never => EnforcementResult::Denied {
                tool: "bash".to_owned(),
                active_mode: mode.as_config_str().to_owned(),
                required_mode: PermissionMode::DontAsk.as_config_str().to_owned(),
                reason: format!(
                    "command may modify state; not allowed in '{}' mode",
                    mode.as_config_str()
                ),
            },
            PermissionMode::Default if !is_read_only_command(command) => {
                EnforcementResult::Denied {
                    tool: "bash".to_owned(),
                    active_mode: mode.as_config_str().to_owned(),
                    required_mode: PermissionMode::DontAsk.as_config_str().to_owned(),
                    reason: "bash requires approval in default mode".to_owned(),
                }
            }
            _ => EnforcementResult::Allowed,
        }
    }
}

fn is_within_workspace(path: &str, workspace_root: &str) -> bool {
    let normalized = if path.starts_with('/') {
        path.to_owned()
    } else {
        format!("{workspace_root}/{path}")
    };

    let root = if workspace_root.ends_with('/') {
        workspace_root.to_owned()
    } else {
        format!("{workspace_root}/")
    };

    normalized.starts_with(&root) || normalized == workspace_root.trim_end_matches('/')
}

fn is_read_only_command(command: &str) -> bool {
    let first_token = command
        .split_whitespace()
        .next()
        .unwrap_or("")
        .rsplit('/')
        .next()
        .unwrap_or("");

    matches!(
        first_token,
        "cat"
            | "head"
            | "tail"
            | "less"
            | "more"
            | "wc"
            | "ls"
            | "find"
            | "grep"
            | "rg"
            | "awk"
            | "sed"
            | "echo"
            | "printf"
            | "which"
            | "where"
            | "whoami"
            | "pwd"
            | "env"
            | "printenv"
            | "date"
            | "cal"
            | "df"
            | "du"
            | "free"
            | "uptime"
            | "uname"
            | "file"
            | "stat"
            | "diff"
            | "sort"
            | "uniq"
            | "tr"
            | "cut"
            | "paste"
            | "tee"
            | "xargs"
            | "test"
            | "true"
            | "false"
            | "type"
            | "readlink"
            | "realpath"
            | "basename"
            | "dirname"
            | "sha256sum"
            | "md5sum"
            | "b3sum"
            | "xxd"
            | "hexdump"
            | "od"
            | "strings"
            | "tree"
            | "jq"
            | "yq"
            | "python3"
            | "python"
            | "node"
            | "ruby"
            | "cargo"
            | "rustc"
            | "git"
            | "gh"
    ) && !command.contains("-i ")
        && !command.contains("--in-place")
        && !command.contains(" > ")
        && !command.contains(" >> ")
}
#![allow(dead_code)]
