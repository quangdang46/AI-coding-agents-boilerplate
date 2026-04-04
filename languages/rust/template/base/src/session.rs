use std::fs;
use std::path::Path;

use crate::config::{get_state_value, read_state, write_state, RuntimeConfig};
use crate::usage::calculate_cost_micros;

pub struct SessionSummary {
    pub session_id: String,
    pub turn_count: u64,
    pub export_path: String,
    pub usage_entries: u64,
    pub total_cost_micros: u64,
}

impl SessionSummary {
    pub fn as_text(&self) -> String {
        format!(
            "session_id={} turn_count={} export_path={} usage_entries={} total_cost_micros={}",
            self.session_id,
            self.turn_count,
            self.export_path,
            self.usage_entries,
            self.total_cost_micros
        )
    }
}

pub fn persist_session_and_usage(
    root: &Path,
    config: &RuntimeConfig,
    prompt_digest: &str,
    context_digest: &str,
    prompt_texts: &[String],
    context_texts: &[String],
    tool_results: &str,
) -> SessionSummary {
    let session_id = String::from("local-main-session");
    let session_path = root.join("__BRAND_ROOT__/sessions/local-main-session.state");
    let latest_path = root.join("__BRAND_ROOT__/sessions/latest.state");
    let export_relative = "__BRAND_ROOT__/sessions/local-main-session.export.md";
    let export_path = root.join(export_relative);
    let usage_summary_path = root.join("__BRAND_ROOT__/sessions/summary.state");

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
    let cost_micros = calculate_cost_micros(prompt_texts, context_texts, tool_results);
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
        (String::from("usage_entries"), usage_entries.to_string()),
        (
            String::from("total_cost_micros"),
            total_cost_micros.to_string(),
        ),
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

    SessionSummary {
        session_id,
        turn_count,
        export_path: export_relative.to_string(),
        usage_entries,
        total_cost_micros,
    }
}
