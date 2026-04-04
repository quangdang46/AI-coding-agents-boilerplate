mod args;
mod bash;
mod bash_validation;
mod bootstrap;
mod commands;
mod config;
mod conversation;
mod file_ops;
mod init;
mod input;
mod permission_enforcer;
mod permissions;
mod prompt;
mod providers;
mod recovery_recipes;
mod render;
mod runtime_summary;
mod sandbox;
mod session;
mod stale_branch;
mod tools;
mod trust_resolver;
mod usage;
mod worker_boot;

use std::env;
use std::io::{self, Write};

use args::{Cli, Command, OutputFormat};
use bootstrap::run_session_loop;
use init::initialize_repo;
use input::{LineEditor, ReadOutcome};
use render::{Spinner, TerminalRenderer};

fn runtime_shell_summary() -> String {
    format!(
        "__PROJECT_NAME__ session loop completed {}",
        run_session_loop()
    )
}

fn prompt_report(prompt: &str, cli: &Cli) -> String {
    match cli.output_format {
        OutputFormat::Text => format!(
            "Prompt\n  Model            {}\n  Permission mode  {}\n  Prompt           {}\n  Runtime          {}",
            cli.model,
            cli.permission_mode.as_label(),
            prompt,
            runtime_shell_summary()
        ),
        OutputFormat::Json => format!(
            "{{\"model\":\"{}\",\"permission_mode\":\"{}\",\"prompt\":\"{}\",\"runtime\":\"{}\"}}",
            cli.model,
            cli.permission_mode.as_label(),
            escape_json(prompt),
            escape_json(&runtime_shell_summary())
        ),
        OutputFormat::Ndjson => format!(
            "{{\"type\":\"prompt\",\"model\":\"{}\",\"permission_mode\":\"{}\",\"prompt\":\"{}\",\"runtime\":\"{}\"}}",
            cli.model,
            cli.permission_mode.as_label(),
            escape_json(prompt),
            escape_json(&runtime_shell_summary())
        ),
    }
}

fn status_report(cli: &Cli) -> String {
    format!(
        "Status\n  Model            {}\n  Permission mode  {}\n  Output format    {:?}\n  Config           {}\n  Runtime          {}",
        cli.model,
        cli.permission_mode.as_label(),
        cli.output_format,
        cli.config
            .as_ref()
            .map_or_else(|| String::from("<none>"), |path| path.display().to_string()),
        runtime_shell_summary()
    )
}

fn repl_help() -> String {
    format!(
        "Interactive shell\n  /help            Show CLI shell help\n  /status          Show current CLI shell status\n  /init            Create starter local guidance files\n  /clear           Clear local shell transcript\n\n{}",
        commands::render_slash_command_help()
    )
}

fn run_prompt(cli: &Cli, prompt: &str, out: &mut impl Write) -> io::Result<()> {
    let renderer = TerminalRenderer::new();
    let mut spinner = Spinner::new();
    spinner.tick("Running prompt", out)?;
    spinner.finish("Prompt complete", out)?;
    renderer.stream_markdown(&prompt_report(prompt, cli), out)
}

fn run_init(out: &mut impl Write) -> Result<(), Box<dyn std::error::Error>> {
    let report = initialize_repo(&env::current_dir()?)?;
    writeln!(out, "{}", report.render())?;
    Ok(())
}

fn run_repl(cli: &Cli) -> io::Result<()> {
    let mut editor = LineEditor::new(
        "› ",
        vec![
            "/help".to_string(),
            "/status".to_string(),
            "/init".to_string(),
            "/clear".to_string(),
        ],
    );
    let renderer = TerminalRenderer::new();
    let mut transcript = Vec::<String>::new();
    let mut stdout = io::stdout();
    writeln!(stdout, "__PROJECT_NAME__ interactive shell")?;
    writeln!(stdout, "Type /help for commands. Ctrl-D exits.")?;

    loop {
        match editor.read_line()? {
            ReadOutcome::Submit(input) => {
                let trimmed = input.trim();
                if trimmed.is_empty() {
                    continue;
                }
                match trimmed {
                    "/help" => renderer.stream_markdown(&repl_help(), &mut stdout)?,
                    "/status" => renderer.stream_markdown(&status_report(cli), &mut stdout)?,
                    "/init" => {
                        let cwd = env::current_dir()
                            .map_err(|error| io::Error::other(error.to_string()))?;
                        let report = initialize_repo(&cwd)
                            .map_err(|error| io::Error::other(error.to_string()))?;
                        renderer.stream_markdown(&report.render(), &mut stdout)?;
                    }
                    "/clear" => {
                        transcript.clear();
                        renderer.stream_markdown("Transcript cleared.", &mut stdout)?;
                    }
                    command if command.starts_with('/') => {
                        renderer.stream_markdown(
                            &format!("Unknown shell command: {command}"),
                            &mut stdout,
                        )?;
                    }
                    prompt => {
                        transcript.push(prompt.to_string());
                        editor.push_history(prompt);
                        renderer.stream_markdown(&prompt_report(prompt, cli), &mut stdout)?;
                    }
                }
            }
            ReadOutcome::Cancel => continue,
            ReadOutcome::Exit => break,
        }
    }

    Ok(())
}

fn main_output() -> Result<String, String> {
    let cli = Cli::parse_from(env::args())?;
    match &cli.command {
        Some(Command::Login) => Ok(String::from(
            "Login\n  Result           not implemented in the generated shell\n  Guidance         use the host runtime for OAuth flows",
        )),
        Some(Command::Logout) => Ok(String::from(
            "Logout\n  Result           not implemented in the generated shell\n  Guidance         remove local credentials from the host runtime",
        )),
        Some(Command::Init) => {
            run_init(&mut io::stdout()).map_err(|error| error.to_string())?;
            Ok(String::new())
        }
        Some(Command::Prompt { prompt }) => Ok(prompt_report(prompt, &cli)),
        None => {
            run_repl(&cli).map_err(|error| error.to_string())?;
            Ok(String::new())
        }
    }
}

fn escape_json(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

fn main() {
    match main_output() {
        Ok(output) if !output.is_empty() => println!("{output}"),
        Ok(_) => {}
        Err(error) => {
            eprintln!("error: {error}");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{escape_json, prompt_report, runtime_shell_summary, status_report};
    use crate::args::{Cli, OutputFormat, PermissionMode};

    fn sample_cli() -> Cli {
        Cli {
            model: "claude-opus-4-6".into(),
            permission_mode: PermissionMode::DangerFullAccess,
            config: None,
            output_format: OutputFormat::Text,
            command: None,
        }
    }

    #[test]
    fn runtime_summary_includes_existing_bootstrap_output() {
        let output = runtime_shell_summary();
        assert!(output.contains("session loop completed"));
        assert!(output.contains("command_registry=owned"));
        assert!(output.contains("tool_registry=owned"));
    }

    #[test]
    fn prompt_report_renders_text_output() {
        let cli = sample_cli();
        let report = prompt_report("ship it", &cli);
        assert!(report.contains("Prompt"));
        assert!(report.contains("ship it"));
        assert!(report.contains("claude-opus-4-6"));
    }

    #[test]
    fn status_report_reflects_cli_flags() {
        let cli = Cli {
            model: "claude-sonnet-4-6".into(),
            permission_mode: PermissionMode::ReadOnly,
            config: None,
            output_format: OutputFormat::Json,
            command: None,
        };
        let report = status_report(&cli);
        assert!(report.contains("Status"));
        assert!(report.contains("claude-sonnet-4-6"));
        assert!(report.contains("read-only"));
    }

    #[test]
    fn escape_json_escapes_quotes_and_newlines() {
        assert_eq!(escape_json("a\"b\nc"), "a\\\"b\\nc");
    }
}
