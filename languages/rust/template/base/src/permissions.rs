#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PermissionMode {
    Default,
    DontAsk,
    Never,
}

impl PermissionMode {
    pub fn as_config_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::DontAsk => "dontAsk",
            Self::Never => "never",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionOverride {
    Allow,
    Deny,
    Ask,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PermissionContext {
    override_decision: Option<PermissionOverride>,
    override_reason: Option<String>,
}

impl PermissionContext {
    pub fn new(
        override_decision: Option<PermissionOverride>,
        override_reason: Option<String>,
    ) -> Self {
        Self {
            override_decision,
            override_reason,
        }
    }

    pub fn override_decision(&self) -> Option<PermissionOverride> {
        self.override_decision
    }

    pub fn override_reason(&self) -> Option<&str> {
        self.override_reason.as_deref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PermissionRequest {
    pub tool_name: String,
    pub input: String,
    pub current_mode: PermissionMode,
    pub required_mode: PermissionMode,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PermissionPromptDecision {
    Allow,
    Deny { reason: String },
}

pub trait PermissionPrompter {
    fn decide(&mut self, request: &PermissionRequest) -> PermissionPromptDecision;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PermissionOutcome {
    Allow,
    Deny { reason: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PermissionPolicy {
    active_mode: PermissionMode,
    deny_rules: Vec<String>,
}

impl PermissionPolicy {
    pub fn new(active_mode: PermissionMode) -> Self {
        Self {
            active_mode,
            deny_rules: Vec::new(),
        }
    }

    pub fn with_permission_rules(mut self, deny: &[String]) -> Self {
        self.deny_rules = deny.to_vec();
        self
    }

    pub fn active_mode(&self) -> PermissionMode {
        self.active_mode
    }

    pub fn required_mode_for(&self, tool_name: &str) -> PermissionMode {
        match tool_name {
            "file_read" | "web_fetch" => PermissionMode::DontAsk,
            "bash" | "file_write" | "file_edit" => PermissionMode::Default,
            _ => PermissionMode::DontAsk,
        }
    }

    pub fn authorize(
        &self,
        tool_name: &str,
        input: &str,
        prompter: Option<&mut dyn PermissionPrompter>,
    ) -> PermissionOutcome {
        self.authorize_with_context(tool_name, input, &PermissionContext::default(), prompter)
    }

    pub fn authorize_with_context(
        &self,
        tool_name: &str,
        input: &str,
        context: &PermissionContext,
        prompter: Option<&mut dyn PermissionPrompter>,
    ) -> PermissionOutcome {
        if self.deny_rules.iter().any(|rule| rule == tool_name) {
            return PermissionOutcome::Deny {
                reason: format!(
                    "Permission to use {tool_name} has been denied by rule '{tool_name}'"
                ),
            };
        }

        let current_mode = self.active_mode();
        let required_mode = self.required_mode_for(tool_name);

        match context.override_decision() {
            Some(PermissionOverride::Deny) => {
                return PermissionOutcome::Deny {
                    reason: context.override_reason().map_or_else(
                        || format!("tool '{tool_name}' denied by hook"),
                        ToOwned::to_owned,
                    ),
                };
            }
            Some(PermissionOverride::Allow) => return PermissionOutcome::Allow,
            Some(PermissionOverride::Ask) => {
                return Self::prompt_or_deny(
                    tool_name,
                    input,
                    current_mode,
                    required_mode,
                    context.override_reason().map(ToOwned::to_owned),
                    prompter,
                )
            }
            None => {}
        }

        match current_mode {
            PermissionMode::DontAsk => PermissionOutcome::Allow,
            PermissionMode::Never => PermissionOutcome::Deny {
                reason: format!(
                    "tool '{tool_name}' requires {} permission; current mode is {}",
                    required_mode.as_config_str(),
                    current_mode.as_config_str()
                ),
            },
            PermissionMode::Default => {
                if matches!(tool_name, "bash" | "file_write" | "file_edit") {
                    Self::prompt_or_deny(
                        tool_name,
                        input,
                        current_mode,
                        required_mode,
                        Some(format!(
                            "tool '{tool_name}' requires approval in default mode"
                        )),
                        prompter,
                    )
                } else {
                    PermissionOutcome::Allow
                }
            }
        }
    }

    fn prompt_or_deny(
        tool_name: &str,
        input: &str,
        current_mode: PermissionMode,
        required_mode: PermissionMode,
        reason: Option<String>,
        mut prompter: Option<&mut dyn PermissionPrompter>,
    ) -> PermissionOutcome {
        let request = PermissionRequest {
            tool_name: tool_name.to_string(),
            input: input.to_string(),
            current_mode,
            required_mode,
            reason: reason.clone(),
        };

        match prompter.as_mut() {
            Some(prompter) => match prompter.decide(&request) {
                PermissionPromptDecision::Allow => PermissionOutcome::Allow,
                PermissionPromptDecision::Deny { reason } => PermissionOutcome::Deny { reason },
            },
            None => PermissionOutcome::Deny {
                reason: reason.unwrap_or_else(|| {
                    format!(
                        "tool '{tool_name}' requires approval to run while mode is {}",
                        current_mode.as_config_str()
                    )
                }),
            },
        }
    }
}

pub fn permission_mode_from_config(value: &str) -> PermissionMode {
    match value {
        "default" => PermissionMode::Default,
        "never" => PermissionMode::Never,
        _ => PermissionMode::DontAsk,
    }
}
#![allow(dead_code)]
