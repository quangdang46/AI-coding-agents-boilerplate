use std::path::{Path, PathBuf};

const TRUST_PROMPT_CUES: &[&str] = &[
    "do you trust the files in this folder",
    "trust the files in this folder",
    "trust this folder",
    "allow and continue",
    "yes, proceed",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrustPolicy {
    AutoTrust,
    RequireApproval,
    Deny,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrustEvent {
    TrustRequired { cwd: String },
    TrustResolved { cwd: String, policy: TrustPolicy },
    TrustDenied { cwd: String, reason: String },
}

#[derive(Debug, Clone, Default)]
pub struct TrustConfig {
    allowlisted: Vec<PathBuf>,
    denied: Vec<PathBuf>,
}

impl TrustConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_allowlisted(mut self, path: impl Into<PathBuf>) -> Self {
        self.allowlisted.push(path.into());
        self
    }

    pub fn with_denied(mut self, path: impl Into<PathBuf>) -> Self {
        self.denied.push(path.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrustDecision {
    NotRequired,
    Required {
        policy: TrustPolicy,
        events: Vec<TrustEvent>,
    },
}

#[derive(Debug, Clone)]
pub struct TrustResolver {
    config: TrustConfig,
}

impl TrustResolver {
    pub fn new(config: TrustConfig) -> Self {
        Self { config }
    }

    pub fn resolve(&self, cwd: &str, screen_text: &str) -> TrustDecision {
        if !detect_trust_prompt(screen_text) {
            return TrustDecision::NotRequired;
        }

        let mut events = vec![TrustEvent::TrustRequired {
            cwd: cwd.to_owned(),
        }];

        if let Some(matched_root) = self
            .config
            .denied
            .iter()
            .find(|root| path_matches(cwd, root))
        {
            let reason = format!("cwd matches denied trust root: {}", matched_root.display());
            events.push(TrustEvent::TrustDenied {
                cwd: cwd.to_owned(),
                reason,
            });
            return TrustDecision::Required {
                policy: TrustPolicy::Deny,
                events,
            };
        }

        if self
            .config
            .allowlisted
            .iter()
            .any(|root| path_matches(cwd, root))
        {
            events.push(TrustEvent::TrustResolved {
                cwd: cwd.to_owned(),
                policy: TrustPolicy::AutoTrust,
            });
            return TrustDecision::Required {
                policy: TrustPolicy::AutoTrust,
                events,
            };
        }

        TrustDecision::Required {
            policy: TrustPolicy::RequireApproval,
            events,
        }
    }
}

pub fn detect_trust_prompt(screen_text: &str) -> bool {
    let lowered = screen_text.to_ascii_lowercase();
    TRUST_PROMPT_CUES
        .iter()
        .any(|needle| lowered.contains(needle))
}

fn path_matches(candidate: &str, root: &Path) -> bool {
    let candidate = normalize_path(Path::new(candidate));
    let root = normalize_path(root);
    candidate == root || candidate.starts_with(&root)
}

fn normalize_path(path: &Path) -> PathBuf {
    std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}
#![allow(dead_code)]
