use std::fs;
use std::path::Path;

use crate::config::{checksum, read_optional_text, read_text};

pub fn run_file_read(root: &Path) -> String {
    checksum(&[
        read_optional_text(&root.join("__BRAND_ROOT__/instructions.md")).unwrap_or_default(),
    ])
}

pub fn run_file_write(root: &Path) -> String {
    let usage_path = root.join("__BRAND_ROOT__/sessions/runtime-tool-smoke.txt");
    fs::write(&usage_path, "tool-write-ok")
        .unwrap_or_else(|err| panic!("failed to write tool file: {err}"));
    String::from("tool-write-ok")
}

pub fn run_file_edit(root: &Path) -> String {
    let usage_path = root.join("__BRAND_ROOT__/sessions/runtime-tool-smoke.txt");
    if !usage_path.exists() {
        fs::write(&usage_path, "tool-write-ok")
            .unwrap_or_else(|err| panic!("failed to seed tool file: {err}"));
    }
    let edited = format!("{} edited", read_text(&usage_path));
    fs::write(&usage_path, &edited).unwrap_or_else(|err| panic!("failed to edit tool file: {err}"));
    edited
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::{run_file_edit, run_file_read, run_file_write};

    fn temp_root(name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should move forward")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("aicd-rust-{name}-{unique}"));
        fs::create_dir_all(root.join("__BRAND_ROOT__/sessions")).expect("session dir should exist");
        fs::create_dir_all(root.join("__BRAND_ROOT__")).expect("brand root should exist");
        root
    }

    #[test]
    fn file_read_hashes_instruction_surface() {
        let root = temp_root("file-read");
        fs::write(
            root.join("__BRAND_ROOT__/instructions.md"),
            "owned-runtime-surface",
        )
        .expect("instructions should write");
        assert!(!run_file_read(&root).is_empty());
    }

    #[test]
    fn file_write_and_edit_update_runtime_smoke_file() {
        let root = temp_root("file-write-edit");
        assert_eq!(run_file_write(&root), "tool-write-ok");
        let edited = run_file_edit(&root);
        assert_eq!(edited, "tool-write-ok edited");
    }
}
