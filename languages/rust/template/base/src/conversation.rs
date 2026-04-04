use crate::config::RuntimeConfig;
use crate::providers::provider_summary;
use crate::session::SessionSummary;

pub fn render_runtime_summary(
    config: &RuntimeConfig,
    prompt_digest: &str,
    context_digest: &str,
    bash_policy: &str,
    file_write_policy: &str,
    command_registry: &str,
    tool_registry: &str,
    tool_results: &str,
    session_summary: &SessionSummary,
) -> String {
    format!(
        "{} prompt_digest={} context_digest={} approval_mode={} bash_policy={} file_write_policy={} {} {} {} {}",
        provider_summary(config),
        prompt_digest,
        context_digest,
        config.approval_mode,
        bash_policy,
        file_write_policy,
        command_registry,
        tool_registry,
        tool_results,
        session_summary.as_text()
    )
}
