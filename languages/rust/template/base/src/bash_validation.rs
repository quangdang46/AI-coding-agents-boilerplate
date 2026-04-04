#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandIntent {
    ReadOnly,
    Write,
    Unknown,
}

pub fn classify_command(command: &str) -> CommandIntent {
    let first = command.split_whitespace().next().unwrap_or_default();
    match first {
        "ls" | "cat" | "grep" | "find" | "pwd" | "echo" | "printf" => CommandIntent::ReadOnly,
        "cp" | "mv" | "rm" | "mkdir" | "touch" | "tee" => CommandIntent::Write,
        _ => CommandIntent::Unknown,
    }
}

pub fn validate_bash_command(command: &str) -> Result<(), String> {
    if command.trim().is_empty() {
        return Err(String::from("bash command must not be empty"));
    }

    if command.contains("rm -rf /") {
        return Err(String::from("destructive root deletion is not allowed"));
    }

    let _ = classify_command(command);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{classify_command, validate_bash_command, CommandIntent};

    #[test]
    fn classifies_read_only_and_write_commands() {
        assert_eq!(
            classify_command("grep needle file.txt"),
            CommandIntent::ReadOnly
        );
        assert_eq!(classify_command("mkdir tmp-dir"), CommandIntent::Write);
    }

    #[test]
    fn rejects_empty_and_destructive_root_delete_commands() {
        assert!(validate_bash_command("   ").is_err());
        assert!(validate_bash_command("rm -rf /").is_err());
    }
}
