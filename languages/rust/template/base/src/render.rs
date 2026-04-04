use std::io::{self, Write};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Spinner {
    active: bool,
}

impl Spinner {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&mut self, label: &str, out: &mut impl Write) -> io::Result<()> {
        self.active = true;
        write!(out, "… {label}\r")?;
        out.flush()
    }

    pub fn finish(&mut self, label: &str, out: &mut impl Write) -> io::Result<()> {
        self.active = false;
        writeln!(out, "✔ {label}")
    }

    pub fn fail(&mut self, label: &str, out: &mut impl Write) -> io::Result<()> {
        self.active = false;
        writeln!(out, "✘ {label}")
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TerminalRenderer;

impl TerminalRenderer {
    pub fn new() -> Self {
        Self
    }

    pub fn render_markdown(&self, markdown: &str) -> String {
        markdown.to_string()
    }

    pub fn stream_markdown(&self, markdown: &str, out: &mut impl Write) -> io::Result<()> {
        write!(out, "{}", self.render_markdown(markdown))?;
        if !markdown.ends_with('\n') {
            writeln!(out)?;
        }
        out.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::{Spinner, TerminalRenderer};

    #[test]
    fn renders_markdown_verbatim() {
        let renderer = TerminalRenderer::new();
        let rendered = renderer.render_markdown("# Heading\n\n- item");
        assert_eq!(rendered, "# Heading\n\n- item");
    }

    #[test]
    fn spinner_writes_labels() {
        let mut spinner = Spinner::new();
        let mut out = Vec::new();
        spinner.tick("Working", &mut out).expect("tick succeeds");
        spinner.finish("Done", &mut out).expect("finish succeeds");

        let output = String::from_utf8_lossy(&out);
        assert!(output.contains("Working"));
        assert!(output.contains("Done"));
    }
}
