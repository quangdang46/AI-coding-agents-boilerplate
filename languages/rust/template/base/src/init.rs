use std::fs;
use std::path::{Path, PathBuf};

const STARTER_CONFIG: &str = concat!(
    "{\n",
    "  \"permissions\": {\n",
    "    \"defaultMode\": \"dontAsk\"\n",
    "  }\n",
    "}\n",
);
const GITIGNORE_COMMENT: &str = "# Generated coding-agent local artifacts";
const GITIGNORE_ENTRIES: [&str; 2] = [
    "__BRAND_ROOT__/settings.local.json",
    "__BRAND_ROOT__/sessions/",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InitStatus {
    Created,
    Updated,
    Skipped,
}

impl InitStatus {
    fn label(self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::Updated => "updated",
            Self::Skipped => "skipped (already exists)",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InitArtifact {
    pub name: &'static str,
    pub status: InitStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InitReport {
    pub project_root: PathBuf,
    pub artifacts: Vec<InitArtifact>,
}

impl InitReport {
    pub fn render(&self) -> String {
        let mut lines = vec![
            "Init".to_string(),
            format!("  Project          {}", self.project_root.display()),
        ];
        for artifact in &self.artifacts {
            lines.push(format!(
                "  {:<16} {}",
                artifact.name,
                artifact.status.label()
            ));
        }
        lines.push("  Next step        Review and tailor the generated guidance".to_string());
        lines.join("\n")
    }
}

pub fn initialize_repo(cwd: &Path) -> Result<InitReport, Box<dyn std::error::Error>> {
    let mut artifacts = Vec::new();
    let brand_dir = cwd.join("__BRAND_ROOT__");
    artifacts.push(InitArtifact {
        name: "__BRAND_ROOT__/",
        status: ensure_dir(&brand_dir)?,
    });
    artifacts.push(InitArtifact {
        name: "__BRAND_ROOT__/sessions/",
        status: ensure_dir(&brand_dir.join("sessions"))?,
    });
    artifacts.push(InitArtifact {
        name: ".gitignore",
        status: ensure_gitignore_entries(&cwd.join(".gitignore"))?,
    });
    artifacts.push(InitArtifact {
        name: ".claude.json",
        status: write_file_if_missing(&cwd.join(".claude.json"), STARTER_CONFIG)?,
    });
    artifacts.push(InitArtifact {
        name: "CLAUDE.md",
        status: write_file_if_missing(&cwd.join("CLAUDE.md"), &render_init_claude_md(cwd))?,
    });

    Ok(InitReport {
        project_root: cwd.to_path_buf(),
        artifacts,
    })
}

fn ensure_dir(path: &Path) -> Result<InitStatus, std::io::Error> {
    if path.is_dir() {
        return Ok(InitStatus::Skipped);
    }
    fs::create_dir_all(path)?;
    Ok(InitStatus::Created)
}

fn write_file_if_missing(path: &Path, content: &str) -> Result<InitStatus, std::io::Error> {
    if path.exists() {
        return Ok(InitStatus::Skipped);
    }
    fs::write(path, content)?;
    Ok(InitStatus::Created)
}

fn ensure_gitignore_entries(path: &Path) -> Result<InitStatus, std::io::Error> {
    if !path.exists() {
        let mut lines = vec![GITIGNORE_COMMENT.to_string()];
        lines.extend(GITIGNORE_ENTRIES.iter().map(|entry| entry.to_string()));
        fs::write(path, format!("{}\n", lines.join("\n")))?;
        return Ok(InitStatus::Created);
    }

    let existing = fs::read_to_string(path)?;
    let mut lines = existing.lines().map(ToOwned::to_owned).collect::<Vec<_>>();
    let mut changed = false;

    if !lines.iter().any(|line| line == GITIGNORE_COMMENT) {
        lines.push(GITIGNORE_COMMENT.to_string());
        changed = true;
    }
    for entry in GITIGNORE_ENTRIES {
        if !lines.iter().any(|line| line == entry) {
            lines.push(entry.to_string());
            changed = true;
        }
    }

    if !changed {
        return Ok(InitStatus::Skipped);
    }

    fs::write(path, format!("{}\n", lines.join("\n")))?;
    Ok(InitStatus::Updated)
}

pub fn render_init_claude_md(_cwd: &Path) -> String {
    [
        "# CLAUDE.md",
        "",
        "This file provides guidance to the generated coding-agent runtime.",
        "",
        "## Detected stack",
        "- Languages: Rust.",
        "- Frameworks: none detected from the starter template.",
        "",
        "## Verification",
        "- Run `cargo fmt` and `cargo test` before shipping changes.",
        "",
        "## Working agreement",
        "- Prefer small, reviewable changes.",
        "- Keep shared defaults in `__BRAND_CONFIG__` and `__BRAND_ROOT__/settings.json`.",
        "- Reserve `__BRAND_ROOT__/settings.local.json` for machine-local overrides.",
    ]
    .join("\n")
}

#[cfg(test)]
mod tests {
    use super::initialize_repo;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir() -> std::path::PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time after epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("rust-shell-init-{nanos}"))
    }

    #[test]
    fn initialize_repo_creates_expected_files() {
        let root = temp_dir();
        fs::create_dir_all(&root).expect("root created");

        let report = initialize_repo(&root).expect("init succeeds");
        let rendered = report.render();
        assert!(rendered.contains("__BRAND_ROOT__/"));
        assert!(rendered.contains("created"));
        assert!(rendered.contains("CLAUDE.md        created"));
        assert!(root.join("__BRAND_ROOT__").is_dir());
        assert!(root.join(".claude.json").is_file());
        assert!(root.join("CLAUDE.md").is_file());

        fs::remove_dir_all(root).expect("cleanup temp dir");
    }
}
