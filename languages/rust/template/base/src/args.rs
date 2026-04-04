use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cli {
    pub model: String,
    pub permission_mode: PermissionMode,
    pub config: Option<PathBuf>,
    pub output_format: OutputFormat,
    pub command: Option<Command>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Login,
    Logout,
    Init,
    Prompt { prompt: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionMode {
    ReadOnly,
    WorkspaceWrite,
    DangerFullAccess,
}

impl PermissionMode {
    fn parse(value: &str) -> Result<Self, String> {
        match value {
            "read-only" => Ok(Self::ReadOnly),
            "workspace-write" => Ok(Self::WorkspaceWrite),
            "danger-full-access" => Ok(Self::DangerFullAccess),
            other => Err(format!(
                "unsupported permission mode '{other}'. Use read-only, workspace-write, or danger-full-access."
            )),
        }
    }

    pub fn as_label(self) -> &'static str {
        match self {
            Self::ReadOnly => "read-only",
            Self::WorkspaceWrite => "workspace-write",
            Self::DangerFullAccess => "danger-full-access",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Text,
    Json,
    Ndjson,
}

impl OutputFormat {
    fn parse(value: &str) -> Result<Self, String> {
        match value {
            "text" => Ok(Self::Text),
            "json" => Ok(Self::Json),
            "ndjson" => Ok(Self::Ndjson),
            other => Err(format!(
                "unsupported output format '{other}'. Use text, json, or ndjson."
            )),
        }
    }
}

impl Cli {
    pub fn parse_from<I>(args: I) -> Result<Self, String>
    where
        I: IntoIterator,
        I::Item: Into<String>,
    {
        let mut args = args.into_iter().map(Into::into).collect::<Vec<_>>();
        if !args.is_empty() {
            args.remove(0);
        }

        let mut model = String::from("claude-opus-4-6");
        let mut permission_mode = PermissionMode::DangerFullAccess;
        let mut config = None;
        let mut output_format = OutputFormat::Text;
        let mut index = 0;

        while index < args.len() {
            match args[index].as_str() {
                "--model" => {
                    let value = args
                        .get(index + 1)
                        .ok_or_else(|| "missing value for --model".to_string())?;
                    model = value.clone();
                    index += 2;
                }
                "--permission-mode" => {
                    let value = args
                        .get(index + 1)
                        .ok_or_else(|| "missing value for --permission-mode".to_string())?;
                    permission_mode = PermissionMode::parse(value)?;
                    index += 2;
                }
                "--config" => {
                    let value = args
                        .get(index + 1)
                        .ok_or_else(|| "missing value for --config".to_string())?;
                    config = Some(PathBuf::from(value));
                    index += 2;
                }
                "--output-format" => {
                    let value = args
                        .get(index + 1)
                        .ok_or_else(|| "missing value for --output-format".to_string())?;
                    output_format = OutputFormat::parse(value)?;
                    index += 2;
                }
                value if value.starts_with('-') => {
                    return Err(format!("unknown option: {value}"));
                }
                _ => break,
            }
        }

        let tail = &args[index..];
        let command = match tail {
            [] => None,
            [command] if command == "login" => Some(Command::Login),
            [command] if command == "logout" => Some(Command::Logout),
            [command] if command == "init" => Some(Command::Init),
            [command, prompt @ ..] if command == "prompt" => {
                let prompt = prompt.join(" ");
                if prompt.trim().is_empty() {
                    return Err("prompt subcommand requires a prompt string".to_string());
                }
                Some(Command::Prompt { prompt })
            }
            prompt_tokens => Some(Command::Prompt {
                prompt: prompt_tokens.join(" "),
            }),
        };

        Ok(Self {
            model,
            permission_mode,
            config,
            output_format,
            command,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{Cli, Command, OutputFormat, PermissionMode};

    #[test]
    fn parses_requested_flags() {
        let cli = Cli::parse_from([
            "tool",
            "--model",
            "claude-3-5-haiku",
            "--permission-mode",
            "read-only",
            "--config",
            "/tmp/config.toml",
            "--output-format",
            "ndjson",
            "prompt",
            "hello",
            "world",
        ])
        .expect("cli parses");

        assert_eq!(cli.model, "claude-3-5-haiku");
        assert_eq!(cli.permission_mode, PermissionMode::ReadOnly);
        assert_eq!(cli.config.as_deref(), Some(Path::new("/tmp/config.toml")));
        assert_eq!(cli.output_format, OutputFormat::Ndjson);
        assert_eq!(
            cli.command,
            Some(Command::Prompt {
                prompt: "hello world".into(),
            })
        );
    }

    #[test]
    fn parses_login_and_logout_commands() {
        let login = Cli::parse_from(["tool", "login"]).expect("login parses");
        assert_eq!(login.command, Some(Command::Login));

        let logout = Cli::parse_from(["tool", "logout"]).expect("logout parses");
        assert_eq!(logout.command, Some(Command::Logout));
    }

    #[test]
    fn defaults_to_danger_full_access_permission_mode() {
        let cli = Cli::parse_from(["tool"]).expect("defaults parse");
        assert_eq!(cli.permission_mode, PermissionMode::DangerFullAccess);
        assert_eq!(cli.output_format, OutputFormat::Text);
    }
}
