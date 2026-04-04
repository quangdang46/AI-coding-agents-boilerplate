use std::path::Path;
use std::process::Command;

use crate::bash_validation::validate_bash_command;

pub fn run_bash(root: &Path, command: &str) -> String {
    validate_bash_command(command)
        .unwrap_or_else(|err| panic!("failed to validate bash tool: {err}"));
    let output = Command::new("bash")
        .args(["-lc", command])
        .current_dir(root)
        .output()
        .unwrap_or_else(|err| panic!("failed to run bash tool: {err}"));
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::run_bash;

    #[test]
    fn bash_runs_simple_command() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        assert_eq!(run_bash(root, "printf rust-bash-ok"), "rust-bash-ok");
    }
}
