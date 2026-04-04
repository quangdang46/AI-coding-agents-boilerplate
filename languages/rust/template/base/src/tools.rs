use crate::config::RuntimeConfig;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolManifestEntry {
    pub name: &'static str,
    pub source: ToolSource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolSource {
    Base,
    Conditional,
}

const BASE_TOOLS: &[ToolManifestEntry] = &[
    ToolManifestEntry {
        name: "bash",
        source: ToolSource::Base,
    },
    ToolManifestEntry {
        name: "file_read",
        source: ToolSource::Base,
    },
    ToolManifestEntry {
        name: "file_edit",
        source: ToolSource::Base,
    },
    ToolManifestEntry {
        name: "file_write",
        source: ToolSource::Base,
    },
    ToolManifestEntry {
        name: "web_fetch",
        source: ToolSource::Conditional,
    },
];

pub fn tool_registry_summary(config: &RuntimeConfig) -> String {
    let enabled = BASE_TOOLS
        .iter()
        .filter(|entry| config.enabled_tools.iter().any(|tool| tool == entry.name))
        .map(|entry| entry.name)
        .collect::<Vec<_>>();
    let aliases = ["read=file_read", "write=file_write", "edit=file_edit"];
    format!(
        "tool_registry=owned available_count={} enabled_count={} enabled={} aliases={}",
        BASE_TOOLS.len(),
        enabled.len(),
        enabled.join(","),
        aliases.join(",")
    )
}

#[cfg(test)]
mod tests {
    use crate::config::load_runtime_config;

    use super::tool_registry_summary;

    #[test]
    fn registry_summary_uses_runtime_enabled_tools() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let config = load_runtime_config(root);
        let summary = tool_registry_summary(&config);
        assert!(summary.contains("tool_registry=owned"));
        assert!(summary.contains("available_count=5"));
        assert!(summary.contains("enabled_count=5"));
        assert!(summary.contains("enabled=bash,file_read,file_edit,file_write,web_fetch"));
        assert!(summary.contains("aliases=read=file_read,write=file_write,edit=file_edit"));
    }
}
