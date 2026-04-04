use std::io::{self, IsTerminal, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReadOutcome {
    Submit(String),
    Cancel,
    Exit,
}

pub struct LineEditor {
    prompt: String,
    completions: Vec<String>,
    history: Vec<String>,
}

impl LineEditor {
    pub fn new(prompt: impl Into<String>, completions: Vec<String>) -> Self {
        Self {
            prompt: prompt.into(),
            completions: normalize_completions(completions),
            history: Vec::new(),
        }
    }

    pub fn push_history(&mut self, entry: impl Into<String>) {
        let entry = entry.into();
        if entry.trim().is_empty() {
            return;
        }
        self.history.push(entry);
    }

    pub fn set_completions(&mut self, completions: Vec<String>) {
        self.completions = normalize_completions(completions);
    }

    pub fn completions(&self) -> &[String] {
        &self.completions
    }

    pub fn history(&self) -> &[String] {
        &self.history
    }

    pub fn read_line(&mut self) -> io::Result<ReadOutcome> {
        if !io::stdin().is_terminal() || !io::stdout().is_terminal() {
            return self.read_line_fallback();
        }

        self.read_line_fallback()
    }

    fn read_line_fallback(&self) -> io::Result<ReadOutcome> {
        let mut stdout = io::stdout();
        write!(stdout, "{}", self.prompt)?;
        stdout.flush()?;

        let mut buffer = String::new();
        let bytes_read = io::stdin().read_line(&mut buffer)?;
        if bytes_read == 0 {
            return Ok(ReadOutcome::Exit);
        }

        while matches!(buffer.chars().last(), Some('\n' | '\r')) {
            buffer.pop();
        }

        if buffer == "/cancel" {
            return Ok(ReadOutcome::Cancel);
        }

        Ok(ReadOutcome::Submit(buffer))
    }
}

fn normalize_completions(completions: Vec<String>) -> Vec<String> {
    let mut normalized = Vec::new();
    for completion in completions {
        if !completion.starts_with('/') {
            continue;
        }
        if !normalized.contains(&completion) {
            normalized.push(completion);
        }
    }
    normalized
}

#[cfg(test)]
mod tests {
    use super::LineEditor;

    #[test]
    fn push_history_ignores_blank_entries() {
        let mut editor = LineEditor::new("> ", vec!["/help".to_string()]);
        editor.push_history("   ");
        editor.push_history("/help");

        assert_eq!(editor.history(), ["/help"]);
    }

    #[test]
    fn set_completions_replaces_and_normalizes_candidates() {
        let mut editor = LineEditor::new("> ", vec!["/help".to_string()]);
        editor.set_completions(vec![
            "/model opus".to_string(),
            "/model opus".to_string(),
            "status".to_string(),
        ]);

        assert_eq!(editor.completions(), ["/model opus"]);
    }
}
