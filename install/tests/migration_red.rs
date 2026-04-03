use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use aicd_install::cli;

fn temp_dir(name: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("aicd-red-{name}-{unique}"))
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

#[test]
fn migration_red_harness_fails_on_hardcoded_python() {
    let out = temp_dir("typescript-init");
    let status = cli::run(vec![
        "aicd".to_string(),
        "init".to_string(),
        "demo-ts".to_string(),
        "--language".to_string(),
        "typescript".to_string(),
        "--output".to_string(),
        out.display().to_string(),
    ]);
    fs::remove_dir_all(&out).ok();

    assert_eq!(
        status, 0,
        "red harness target: manifest-driven init should support declared languages instead of rejecting non-python"
    );
}

#[test]
fn migration_guard_rejects_reference_runtime_paths() {
    let references_root = repo_root().join("references");
    let source_references_root = repo_root().join("source-references");

    assert!(
        references_root.exists(),
        "red harness target: canonical archived source evidence should live under references/"
    );
    assert!(
        !source_references_root.exists(),
        "red harness target: shipped runtime should eventually retire the legacy source-references/ root once canonical archived evidence lives under references/"
    );
}
