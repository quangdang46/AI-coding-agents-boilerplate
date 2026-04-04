#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandManifestEntry {
    pub name: &'static str,
    pub source: CommandSource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandSource {
    Builtin,
    InternalOnly,
    FeatureGated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SlashCommandSpec {
    pub name: &'static str,
    pub summary: &'static str,
    pub resume_supported: bool,
}

const COMMAND_MANIFEST: &[CommandManifestEntry] = &[
    CommandManifestEntry {
        name: "help",
        source: CommandSource::Builtin,
    },
    CommandManifestEntry {
        name: "status",
        source: CommandSource::Builtin,
    },
    CommandManifestEntry {
        name: "config",
        source: CommandSource::Builtin,
    },
    CommandManifestEntry {
        name: "session",
        source: CommandSource::Builtin,
    },
    CommandManifestEntry {
        name: "doctor",
        source: CommandSource::Builtin,
    },
    CommandManifestEntry {
        name: "agents",
        source: CommandSource::InternalOnly,
    },
    CommandManifestEntry {
        name: "skills",
        source: CommandSource::FeatureGated,
    },
];

const SLASH_COMMANDS: &[SlashCommandSpec] = &[
    SlashCommandSpec {
        name: "help",
        summary: "Show available slash commands",
        resume_supported: true,
    },
    SlashCommandSpec {
        name: "status",
        summary: "Show current session status",
        resume_supported: true,
    },
    SlashCommandSpec {
        name: "config",
        summary: "Inspect runtime config surfaces",
        resume_supported: true,
    },
    SlashCommandSpec {
        name: "session",
        summary: "Inspect or switch persisted sessions",
        resume_supported: false,
    },
    SlashCommandSpec {
        name: "doctor",
        summary: "Diagnose runtime setup and generated assets",
        resume_supported: true,
    },
];

pub fn command_registry_summary() -> String {
    let command_count = COMMAND_MANIFEST.len();
    let slash_count = SLASH_COMMANDS.len();
    let resume_count = SLASH_COMMANDS
        .iter()
        .filter(|spec| spec.resume_supported)
        .count();
    let names = COMMAND_MANIFEST
        .iter()
        .map(|entry| entry.name)
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "command_registry=owned command_count={command_count} slash_count={slash_count} resume_supported={resume_count} commands={names}"
    )
}

pub fn render_slash_command_help() -> String {
    SLASH_COMMANDS
        .iter()
        .map(|spec| {
            format!(
                "/{} [{}] - {}",
                spec.name,
                if spec.resume_supported {
                    "resume"
                } else {
                    "repl"
                },
                spec.summary
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::{command_registry_summary, render_slash_command_help};

    #[test]
    fn registry_summary_reports_owned_command_surface() {
        let summary = command_registry_summary();
        assert!(summary.contains("command_registry=owned"));
        assert!(summary.contains("command_count=7"));
        assert!(summary.contains("slash_count=5"));
        assert!(summary.contains("commands=help,status,config,session,doctor,agents,skills"));
    }

    #[test]
    fn slash_help_lists_resume_and_repl_commands() {
        let help = render_slash_command_help();
        assert!(help.contains("/help [resume] - Show available slash commands"));
        assert!(help.contains("/session [repl] - Inspect or switch persisted sessions"));
    }
}
