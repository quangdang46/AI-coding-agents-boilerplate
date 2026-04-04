use std::fs;
use std::path::Path;

use crate::config::{checksum, read_text};

pub fn run_file_read(root: &Path) -> String {
    checksum(&[read_text(&root.join(".agent/context/README.md"))])
}

pub fn run_file_write(root: &Path) -> String {
    let usage_path = root.join(".agent/usage/runtime-tool-smoke.txt");
    fs::write(&usage_path, "tool-write-ok")
        .unwrap_or_else(|err| panic!("failed to write tool file: {err}"));
    String::from("tool-write-ok")
}

pub fn run_file_edit(root: &Path) -> String {
    let usage_path = root.join(".agent/usage/runtime-tool-smoke.txt");
    if !usage_path.exists() {
        fs::write(&usage_path, "tool-write-ok")
            .unwrap_or_else(|err| panic!("failed to seed tool file: {err}"));
    }
    let edited = format!("{} edited", read_text(&usage_path));
    fs::write(&usage_path, &edited).unwrap_or_else(|err| panic!("failed to edit tool file: {err}"));
    edited
}
