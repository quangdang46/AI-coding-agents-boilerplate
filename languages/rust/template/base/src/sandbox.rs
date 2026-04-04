#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FilesystemIsolationMode {
    Off,
    #[default]
    WorkspaceOnly,
    AllowList,
}

impl FilesystemIsolationMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::WorkspaceOnly => "workspace-only",
            Self::AllowList => "allow-list",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SandboxConfig {
    pub enabled: Option<bool>,
    pub namespace_restrictions: Option<bool>,
    pub network_isolation: Option<bool>,
    pub filesystem_mode: Option<FilesystemIsolationMode>,
    pub allowed_mounts: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SandboxRequest {
    pub enabled: bool,
    pub namespace_restrictions: bool,
    pub network_isolation: bool,
    pub filesystem_mode: FilesystemIsolationMode,
    pub allowed_mounts: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SandboxStatus {
    pub enabled: bool,
    pub active: bool,
    pub filesystem_mode: FilesystemIsolationMode,
    pub fallback_reason: Option<String>,
}

impl SandboxConfig {
    pub fn resolve_request(&self) -> SandboxRequest {
        SandboxRequest {
            enabled: self.enabled.unwrap_or(true),
            namespace_restrictions: self.namespace_restrictions.unwrap_or(true),
            network_isolation: self.network_isolation.unwrap_or(false),
            filesystem_mode: self.filesystem_mode.unwrap_or_default(),
            allowed_mounts: self.allowed_mounts.clone(),
        }
    }
}

pub fn resolve_sandbox_status_for_request(request: &SandboxRequest) -> SandboxStatus {
    let fallback_reason = (request.filesystem_mode == FilesystemIsolationMode::AllowList
        && request.allowed_mounts.is_empty())
    .then(|| "filesystem allow-list requested without configured mounts".to_string());

    SandboxStatus {
        enabled: request.enabled,
        active: request.enabled,
        filesystem_mode: request.filesystem_mode,
        fallback_reason,
    }
}
