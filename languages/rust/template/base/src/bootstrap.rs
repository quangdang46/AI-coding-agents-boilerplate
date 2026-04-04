use crate::commands::{command_registry_summary, render_slash_command_help};
use crate::config::{load_runtime_config, project_root};
use crate::conversation::render_runtime_summary;
use crate::prompt::load_prompt_state;
use crate::runtime_summary::{policy_for_operation, run_core_tools};
use crate::session::persist_session_and_usage;
use crate::tools::tool_registry_summary;

pub fn load_runtime_summary() -> String {
    let root = project_root();
    let config = load_runtime_config(&root);
    let prompt_state = load_prompt_state(&root, &config);
    let tool_results = run_core_tools(&root, &config);
    let command_registry = command_registry_summary();
    let tool_registry = tool_registry_summary(&config);
    let session_summary = persist_session_and_usage(
        &root,
        &config,
        &prompt_state.prompt_digest,
        &prompt_state.context_digest,
        &prompt_state.prompt_texts,
        &prompt_state.context_texts,
        &tool_results,
    );

    render_runtime_summary(
        &config,
        &prompt_state.prompt_digest,
        &prompt_state.context_digest,
        &policy_for_operation(&config.approval_mode, &config.deny, "bash", "bash"),
        &policy_for_operation(
            &config.approval_mode,
            &config.deny,
            "file_write",
            "file_write",
        ),
        &format!(
            "{} slash_help_digest={}",
            command_registry,
            render_slash_command_help().len()
        ),
        &tool_registry,
        &tool_results,
        &session_summary,
    )
}

pub fn run_session_loop() -> String {
    load_runtime_summary()
}
