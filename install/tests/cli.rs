use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use aicd_install::commands::{doctor, feature, init, list};
use aicd_install::manifest::read_agentkit_toml;

fn temp_dir(name: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("aicd-{name}-{unique}"))
}

#[test]
fn list_languages_contains_python() {
    let output = list::run("languages").expect("list languages");
    assert!(output.contains("python"));
}

#[test]
fn init_renders_python_template() {
    let out = temp_dir("init");
    let message = init::run(&init::InitArgs {
        project_name: "demo-agent".to_string(),
        language: "python".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("init works");
    assert!(message.contains("initialized python project"));
    assert!(out.join("agentkit.toml").exists());
    assert!(out.join(".agent/prompts/system.md").exists());
    assert!(out.join("src/demo_agent/app.py").exists());
    assert!(read_agentkit_toml(&out).unwrap().contains("enabled = []"));
    fs::remove_dir_all(out).ok();
}

#[test]
fn doctor_validates_generated_project() {
    let out = temp_dir("doctor");
    init::run(&init::InitArgs {
        project_name: "demo-agent".to_string(),
        language: "python".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("init works");
    let message = doctor::run(&out).expect("doctor works");
    assert!(message.contains("doctor ok"));
    fs::remove_dir_all(out).ok();
}

#[test]
fn feature_add_and_remove_updates_project() {
    let out = temp_dir("feature");
    init::run(&init::InitArgs {
        project_name: "demo-agent".to_string(),
        language: "python".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("init works");

    feature::add(&feature::FeatureArgs {
        feature_id: "github-pr-review".to_string(),
        project: out.clone(),
    })
    .expect("feature add works");
    assert!(out
        .join(".agent/prompts/sections/github-review.md")
        .exists());
    assert!(out.join(".agent/skills/review-pr/SKILL.md").exists());
    assert!(read_agentkit_toml(&out)
        .unwrap()
        .contains("enabled = [\"github-pr-review\"]"));

    feature::remove(&feature::FeatureArgs {
        feature_id: "github-pr-review".to_string(),
        project: out.clone(),
    })
    .expect("feature remove works");
    assert!(!out
        .join(".agent/prompts/sections/github-review.md")
        .exists());
    assert!(!out.join(".agent/skills/review-pr/SKILL.md").exists());
    assert!(read_agentkit_toml(&out).unwrap().contains("enabled = []"));

    fs::remove_dir_all(out).ok();
}

#[test]
fn doctor_fails_for_missing_files() {
    let out = temp_dir("broken");
    fs::create_dir_all(&out).unwrap();
    let error = doctor::run(&out).expect_err("doctor should fail");
    assert!(error.contains("missing required file"));
    fs::remove_dir_all(out).ok();
}
