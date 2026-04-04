use crate::config::RuntimeConfig;

pub fn provider_summary(config: &RuntimeConfig) -> String {
    format!(
        "provider={} model={}",
        config.default_provider, config.provider_model
    )
}
