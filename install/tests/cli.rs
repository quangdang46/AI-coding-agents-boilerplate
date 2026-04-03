use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

use aicd_install::cli;
use aicd_install::commands::{doctor, feature, init, list};
use aicd_install::manifest::{read_agentkit_toml, read_language_manifest};

fn temp_dir(name: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("aicd-{name}-{unique}"))
}

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("manifests")
        .join(name)
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

fn cli_test_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

fn cleanup_broken_language_dirs() {
    let root = repo_root().join("languages");
    let Ok(entries) = fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with("broken-language-") {
            fs::remove_dir_all(entry.path()).ok();
        }
    }
}

fn run_command(command: &mut Command) -> String {
    let output = command.output().expect("command should run");
    assert!(
        output.status.success(),
        "command failed: {:?}\nstdout:{}\nstderr:{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn acquire_cli_test_guard() -> MutexGuard<'static, ()> {
    let guard = match cli_test_lock().lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    cleanup_broken_language_dirs();
    guard
}

struct TempPathCleanup(PathBuf);

impl Drop for TempPathCleanup {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).ok();
    }
}

#[test]
fn list_languages_contains_python() {
    let _guard = acquire_cli_test_guard();
    let output = list::run("languages").expect("list languages");
    assert!(output.contains("python"));
    assert!(output.contains("rust"));
    assert!(output.contains("typescript"));
}

#[test]
fn list_templates_reports_manifest_backed_templates() {
    let _guard = acquire_cli_test_guard();
    let output = list::run("templates").expect("list templates");
    let lines: Vec<&str> = output.lines().collect();
    assert_eq!(lines, vec!["python:base", "rust:base", "typescript:base"]);
}

#[test]
fn list_features_reports_shipped_feature_packs() {
    let _guard = acquire_cli_test_guard();
    let output = list::run("features").expect("list features");
    let lines: Vec<&str> = output.lines().collect();
    assert_eq!(
        lines,
        vec![
            "python:github-pr-review",
            "rust:local-plugins",
            "typescript:github-pr-review"
        ]
    );
}

#[test]
fn init_renders_python_template() {
    let _guard = acquire_cli_test_guard();
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
    assert!(out.join(".agent/context/README.md").exists());
    assert!(out.join(".agent/usage/README.md").exists());
    assert!(out.join("src/demo_agent/app.py").exists());
    let app_text = fs::read_to_string(out.join("src/demo_agent/app.py")).expect("read python app");
    assert!(app_text.contains("session loop completed"));
    assert!(read_agentkit_toml(&out).unwrap().contains("enabled = []"));
    fs::remove_dir_all(out).ok();
}

#[test]
fn doctor_validates_generated_project() {
    let _guard = acquire_cli_test_guard();
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
    let _guard = acquire_cli_test_guard();
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
    assert!(out.join(".agent/agents/review-agent.agent.json").exists());
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
    assert!(!out.join(".agent/agents/review-agent.agent.json").exists());
    assert!(!out
        .join(".agent/prompts/sections/github-review.md")
        .exists());
    assert!(!out.join(".agent/skills/review-pr/SKILL.md").exists());
    assert!(read_agentkit_toml(&out).unwrap().contains("enabled = []"));

    fs::remove_dir_all(out).ok();
}

#[test]
fn namespaced_feature_add_and_remove_updates_project() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("feature-namespaced");
    init::run(&init::InitArgs {
        project_name: "demo-agent".to_string(),
        language: "python".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("init works");

    feature::add(&feature::FeatureArgs {
        feature_id: "python:github-pr-review".to_string(),
        project: out.clone(),
    })
    .expect("namespaced feature add works");
    assert!(out.join(".agent/agents/review-agent.agent.json").exists());
    assert!(read_agentkit_toml(&out)
        .unwrap()
        .contains("enabled = [\"github-pr-review\"]"));

    feature::remove(&feature::FeatureArgs {
        feature_id: "python:github-pr-review".to_string(),
        project: out.clone(),
    })
    .expect("namespaced feature remove works");
    assert!(!out.join(".agent/agents/review-agent.agent.json").exists());
    assert!(read_agentkit_toml(&out).unwrap().contains("enabled = []"));

    fs::remove_dir_all(out).ok();
}

#[test]
fn feature_add_rejects_invalid_feature_manifest_shape() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("feature-invalid-manifest");
    init::run(&init::InitArgs {
        project_name: "demo-agent".to_string(),
        language: "python".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("init works");

    let manifest_path = out.join(".agent/features/github-pr-review/feature.json");
    let raw = fs::read_to_string(&manifest_path).expect("read feature manifest");
    let broken = raw.replace("\"appliesTo\": [\"python\"],\n", "");
    fs::write(&manifest_path, broken).expect("write broken feature manifest");

    let error = feature::add(&feature::FeatureArgs {
        feature_id: "github-pr-review".to_string(),
        project: out.clone(),
    })
    .expect_err("invalid feature manifest should fail");
    assert!(error.contains("missing appliesTo") || error.contains("failed to parse"));

    fs::remove_dir_all(out).ok();
}

#[test]
fn doctor_fails_for_missing_files() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("broken");
    fs::create_dir_all(&out).unwrap();
    let error = doctor::run(&out).expect_err("doctor should fail");
    assert!(error.contains("missing required file"));
    fs::remove_dir_all(out).ok();
}

#[test]
fn manifest_schema_accepts_valid_examples() {
    let _guard = acquire_cli_test_guard();
    let python = read_language_manifest(&fixture_path("python.language.manifest.json"))
        .expect("python manifest should parse");
    assert_eq!(python.id, "python");
    assert_eq!(python.display_name, "Python");
    assert_eq!(
        python.feature_registry.as_deref(),
        Some(".agent/features/registry.json")
    );
    assert!(python.supports.init);
    assert!(python.supports.feature_add);
    assert!(python.supports.feature_remove);
    assert!(python.supports.doctor);
    assert_eq!(python.runtime.project_markers.len(), 2);

    let rust = read_language_manifest(&fixture_path("rust.language.manifest.json"))
        .expect("rust manifest should parse");
    assert_eq!(rust.id, "rust");
    assert_eq!(rust.display_name, "Rust");
    assert_eq!(rust.runtime.config_file, ".claw.json");
    assert_eq!(
        rust.feature_registry.as_deref(),
        Some(".agent/features/registry.json")
    );
    assert!(rust.supports.init);
    assert!(rust.supports.feature_add);
    assert!(rust.supports.feature_remove);
    assert!(rust.supports.doctor);

    let typescript = read_language_manifest(&fixture_path("typescript.language.manifest.json"))
        .expect("typescript manifest should parse");
    assert_eq!(typescript.id, "typescript");
    assert_eq!(typescript.display_name, "TypeScript");
    assert_eq!(typescript.runtime.config_file, "boilerplate.config.ts");
    assert_eq!(
        typescript.feature_registry.as_deref(),
        Some(".agent/features/registry.json")
    );
    assert!(typescript.supports.init);
    assert!(typescript.supports.feature_add);
    assert!(typescript.supports.feature_remove);
    assert!(typescript.supports.doctor);
}

#[test]
fn manifest_schema_rejects_incomplete_examples() {
    let _guard = acquire_cli_test_guard();
    let err = read_language_manifest(&fixture_path("invalid.language.manifest.json"))
        .expect_err("invalid manifest should fail");
    assert!(err.contains("missing field `runtime`") || err.contains("missing featureRegistry"));
}

#[test]
fn languages_list_reads_manifests() {
    let _guard = acquire_cli_test_guard();
    let output = list::run("languages").expect("list languages should read manifests");
    let lines: Vec<&str> = output.lines().collect();
    assert_eq!(lines, vec!["python", "rust", "typescript"]);
}

#[test]
fn languages_list_rejects_invalid_manifest() {
    let _guard = acquire_cli_test_guard();
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let invalid_root = repo_root()
        .join("languages")
        .join(format!("broken-language-{unique}"));
    let _cleanup = TempPathCleanup(invalid_root.clone());
    let invalid_path = invalid_root.join("language.manifest.json");

    fs::create_dir_all(invalid_path.parent().unwrap()).expect("create invalid language root");
    fs::write(&invalid_path, "{\n").expect("write invalid manifest");

    let err = list::run("languages").expect_err("invalid manifest should fail listing");
    assert!(
        err.contains("missing field `runtime`")
            || err.contains("missing featureRegistry")
            || err.contains("failed to parse")
    );
}

#[test]
fn python_cli_lifecycle_end_to_end() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("python-lifecycle");

    let init_status = cli::run(vec![
        "aicd".to_string(),
        "init".to_string(),
        "demo-agent".to_string(),
        "--language".to_string(),
        "python".to_string(),
        "--output".to_string(),
        out.display().to_string(),
    ]);
    assert_eq!(init_status, 0);

    let feature_add_status = cli::run(vec![
        "aicd".to_string(),
        "feature".to_string(),
        "add".to_string(),
        "github-pr-review".to_string(),
        "--project".to_string(),
        out.display().to_string(),
    ]);
    assert_eq!(feature_add_status, 0);

    let doctor_status = cli::run(vec![
        "aicd".to_string(),
        "doctor".to_string(),
        "--project".to_string(),
        out.display().to_string(),
    ]);
    assert_eq!(doctor_status, 0);

    let feature_remove_status = cli::run(vec![
        "aicd".to_string(),
        "feature".to_string(),
        "remove".to_string(),
        "github-pr-review".to_string(),
        "--project".to_string(),
        out.display().to_string(),
    ]);
    assert_eq!(feature_remove_status, 0);

    fs::remove_dir_all(out).ok();
}

#[test]
fn init_renders_typescript_template() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("typescript-init");
    let message = init::run(&init::InitArgs {
        project_name: "demo-ts".to_string(),
        language: "typescript".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("typescript init works");

    assert!(message.contains("initialized typescript project"));
    assert!(out.join("boilerplate.config.ts").exists());
    assert!(out.join("package.json").exists());
    assert!(out.join("tsconfig.json").exists());
    assert!(out.join("src/index.ts").exists());
    let entry_text = fs::read_to_string(out.join("src/index.ts")).expect("read typescript entry");
    assert!(entry_text.contains("session loop completed"));
    assert!(out.join(".agent/usage/README.md").exists());
    assert!(out.join(".agent/context/README.md").exists());
    assert!(out.join(".agent/sessions/README.md").exists());
    fs::remove_dir_all(out).ok();
}

#[test]
fn doctor_validates_typescript_project() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("typescript-doctor");
    init::run(&init::InitArgs {
        project_name: "demo-ts".to_string(),
        language: "typescript".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("typescript init works");

    let message = doctor::run(&out).expect("typescript doctor works");
    assert!(message.contains("doctor ok"));
    fs::remove_dir_all(out).ok();
}

#[test]
fn doctor_detects_missing_typescript_provider_selection() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("typescript-provider-missing");
    init::run(&init::InitArgs {
        project_name: "demo-ts".to_string(),
        language: "typescript".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("typescript init works");

    let path = out.join("boilerplate.config.ts");
    let config = std::fs::read_to_string(&path).expect("read typescript config");
    std::fs::write(
        &path,
        config.replace("defaultProvider:", "defaultProviderMissing:"),
    )
    .expect("corrupt typescript provider config");

    let error =
        doctor::run(&out).expect_err("doctor should detect missing typescript provider selection");
    assert!(error.contains("missing provider config"));
    fs::remove_dir_all(out).ok();
}

#[test]
fn doctor_detects_missing_typescript_web_fetch_config() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("typescript-web-fetch-missing");
    init::run(&init::InitArgs {
        project_name: "demo-ts".to_string(),
        language: "typescript".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("typescript init works");

    let path = out.join("boilerplate.config.ts");
    let config = std::fs::read_to_string(&path).expect("read typescript config");
    std::fs::write(&path, config.replace("'web_fetch'", "'web_fetch_missing'"))
        .expect("corrupt typescript web fetch config");

    let error =
        doctor::run(&out).expect_err("doctor should detect missing typescript web fetch config");
    assert!(error.contains("missing web fetch config"));
    fs::remove_dir_all(out).ok();
}

#[test]
fn doctor_detects_missing_typescript_permission_config() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("typescript-permissions-missing");
    init::run(&init::InitArgs {
        project_name: "demo-ts".to_string(),
        language: "typescript".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("typescript init works");

    let path = out.join("boilerplate.config.ts");
    let config = std::fs::read_to_string(&path).expect("read typescript config");
    std::fs::write(
        &path,
        config.replace("approvalMode:", "approvalModeMissing:"),
    )
    .expect("corrupt typescript permission config");

    let error =
        doctor::run(&out).expect_err("doctor should detect missing typescript permission config");
    assert!(error.contains("missing permission config"));
    fs::remove_dir_all(out).ok();
}

#[test]
fn doctor_detects_missing_typescript_tool_defaults() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("typescript-tool-defaults-missing");
    init::run(&init::InitArgs {
        project_name: "demo-ts".to_string(),
        language: "typescript".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("typescript init works");

    let path = out.join("boilerplate.config.ts");
    let config = std::fs::read_to_string(&path).expect("read typescript config");
    std::fs::write(
        &path,
        config.replace("bashTimeoutMs:", "bashTimeoutMissing:"),
    )
    .expect("corrupt typescript tool defaults");

    let error =
        doctor::run(&out).expect_err("doctor should detect missing typescript tool defaults");
    assert!(error.contains("missing tool default config"));
    fs::remove_dir_all(out).ok();
}

#[test]
fn typescript_feature_add_and_remove_updates_project() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("typescript-feature");
    init::run(&init::InitArgs {
        project_name: "demo-ts".to_string(),
        language: "typescript".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("typescript init works");

    feature::add(&feature::FeatureArgs {
        feature_id: "typescript:github-pr-review".to_string(),
        project: out.clone(),
    })
    .expect("typescript feature add works");

    assert!(out.join(".agent/agents/review-agent.agent.json").exists());
    assert!(out
        .join(".agent/prompts/sections/github-review.md")
        .exists());
    assert!(out.join(".agent/skills/review-pr/SKILL.md").exists());

    let config =
        std::fs::read_to_string(out.join("boilerplate.config.ts")).expect("read typescript config");
    assert!(config.contains("features: {\n    enabled: ['github-pr-review']"));
    assert!(config.contains("'.agent/prompts/sections/github-review.md'"));
    assert!(config.contains("'review-agent'"));
    assert!(config.contains("'review-pr'"));

    let doctor_message = doctor::run(&out).expect("typescript doctor after feature add works");
    assert!(doctor_message.contains("doctor ok"));

    feature::remove(&feature::FeatureArgs {
        feature_id: "typescript:github-pr-review".to_string(),
        project: out.clone(),
    })
    .expect("typescript feature remove works");

    assert!(!out.join(".agent/agents/review-agent.agent.json").exists());
    assert!(!out
        .join(".agent/prompts/sections/github-review.md")
        .exists());
    assert!(!out.join(".agent/skills/review-pr/SKILL.md").exists());

    let config = std::fs::read_to_string(out.join("boilerplate.config.ts"))
        .expect("read typescript config after remove");
    assert!(config.contains("features: {\n    enabled: []"));
    assert!(!config.contains("'.agent/prompts/sections/github-review.md'"));
    assert!(!config.contains("'review-agent'"));
    assert!(!config.contains("'review-pr'"));
    fs::remove_dir_all(out).ok();
}

#[test]
fn doctor_detects_missing_typescript_feature_asset() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("typescript-feature-missing-asset");
    init::run(&init::InitArgs {
        project_name: "demo-ts".to_string(),
        language: "typescript".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("typescript init works");

    feature::add(&feature::FeatureArgs {
        feature_id: "typescript:github-pr-review".to_string(),
        project: out.clone(),
    })
    .expect("typescript feature add works");

    fs::remove_file(out.join(".agent/skills/review-pr/SKILL.md"))
        .expect("remove typescript feature asset");

    let error = doctor::run(&out).expect_err("doctor should detect missing typescript asset");
    assert!(error.contains("missing feature asset:"));
    fs::remove_dir_all(out).ok();
}

#[test]
fn doctor_detects_missing_typescript_session_root() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("typescript-session-root-missing");
    init::run(&init::InitArgs {
        project_name: "demo-ts".to_string(),
        language: "typescript".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("typescript init works");

    fs::remove_file(out.join(".agent/sessions/README.md")).expect("remove typescript session root");

    let error =
        doctor::run(&out).expect_err("doctor should detect missing typescript session root");
    assert!(error.contains("missing required file:"));
    fs::remove_dir_all(out).ok();
}

#[test]
fn doctor_detects_missing_typescript_context_root() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("typescript-context-root-missing");
    init::run(&init::InitArgs {
        project_name: "demo-ts".to_string(),
        language: "typescript".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("typescript init works");

    fs::remove_file(out.join(".agent/context/README.md")).expect("remove typescript context root");

    let error =
        doctor::run(&out).expect_err("doctor should detect missing typescript context root");
    assert!(error.contains("missing required file:"));
    fs::remove_dir_all(out).ok();
}

#[test]
fn doctor_detects_missing_typescript_usage_root() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("typescript-usage-root-missing");
    init::run(&init::InitArgs {
        project_name: "demo-ts".to_string(),
        language: "typescript".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("typescript init works");

    fs::remove_file(out.join(".agent/usage/README.md")).expect("remove typescript usage root");

    let error = doctor::run(&out).expect_err("doctor should detect missing typescript usage root");
    assert!(error.contains("missing required file:"));
    fs::remove_dir_all(out).ok();
}

#[test]
fn doctor_detects_missing_typescript_agent_config() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("typescript-agent-config-missing");
    init::run(&init::InitArgs {
        project_name: "demo-ts".to_string(),
        language: "typescript".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("typescript init works");

    let path = out.join("boilerplate.config.ts");
    let config = std::fs::read_to_string(&path).expect("read typescript config");
    std::fs::write(
        &path,
        config.replace("defaultAgent:", "defaultAgentMissing:"),
    )
    .expect("corrupt typescript agent config");

    let error =
        doctor::run(&out).expect_err("doctor should detect missing typescript agent config");
    assert!(error.contains("missing agent config"));
    fs::remove_dir_all(out).ok();
}

#[test]
fn init_renders_rust_template() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("rust-init");
    let message = init::run(&init::InitArgs {
        project_name: "demo-rust".to_string(),
        language: "rust".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("rust init works");

    assert!(message.contains("initialized rust project"));
    assert!(out.join("Cargo.toml").exists());
    assert!(out.join("CLAW.md").exists());
    assert!(out.join(".claw.json").exists());
    assert!(out.join("src/main.rs").exists());
    let entry_text = fs::read_to_string(out.join("src/main.rs")).expect("read rust entry");
    assert!(entry_text.contains("session loop completed"));
    assert!(out.join(".agent/prompts/system.md").exists());
    assert!(out.join(".agent/context/README.md").exists());
    assert!(out.join(".agent/prompts/sections/style.md").exists());
    assert!(out.join(".agent/prompts/sections/verification.md").exists());
    assert!(out.join(".agent/agents/planner.agent.json").exists());
    assert!(out.join(".agent/agents/executor.agent.json").exists());
    assert!(out.join(".agent/agents/reviewer.agent.json").exists());
    assert!(out.join(".agent/sessions/README.md").exists());
    assert!(out.join(".agent/usage/README.md").exists());
    assert!(out.join(".agent/skills/plan/SKILL.md").exists());
    assert!(out.join(".agent/skills/add-feature/SKILL.md").exists());
    assert!(out.join(".agent/skills/verify/SKILL.md").exists());
    assert!(out.join(".agent/features/registry.json").exists());
    fs::remove_dir_all(out).ok();
}

#[test]
fn doctor_validates_rust_project() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("rust-doctor");
    init::run(&init::InitArgs {
        project_name: "demo-rust".to_string(),
        language: "rust".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("rust init works");

    let message = doctor::run(&out).expect("rust doctor works");
    assert!(message.contains("doctor ok"));
    fs::remove_dir_all(out).ok();
}

#[test]
fn doctor_detects_missing_rust_provider_selection() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("rust-provider-missing");
    init::run(&init::InitArgs {
        project_name: "demo-rust".to_string(),
        language: "rust".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("rust init works");

    let path = out.join(".claw.json");
    let config = std::fs::read_to_string(&path).expect("read rust config");
    std::fs::write(
        &path,
        config.replace("\"defaultProvider\": \"anthropic\",\n", ""),
    )
    .expect("corrupt rust provider config");

    let error =
        doctor::run(&out).expect_err("doctor should detect missing rust provider selection");
    assert!(error.contains("missing provider config"));
    fs::remove_dir_all(out).ok();
}

#[test]
fn doctor_detects_missing_rust_web_fetch_config() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("rust-web-fetch-missing");
    init::run(&init::InitArgs {
        project_name: "demo-rust".to_string(),
        language: "rust".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("rust init works");

    let path = out.join(".claw.json");
    let config = std::fs::read_to_string(&path).expect("read rust config");
    std::fs::write(
        &path,
        config.replace("\"web_fetch\"", "\"web_fetch_missing\""),
    )
    .expect("corrupt rust web fetch config");

    let error = doctor::run(&out).expect_err("doctor should detect missing rust web fetch config");
    assert!(error.contains("missing web fetch config"));
    fs::remove_dir_all(out).ok();
}

#[test]
fn doctor_detects_missing_rust_permission_config() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("rust-permissions-missing");
    init::run(&init::InitArgs {
        project_name: "demo-rust".to_string(),
        language: "rust".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("rust init works");

    let path = out.join(".claw.json");
    let config = std::fs::read_to_string(&path).expect("read rust config");
    std::fs::write(
        &path,
        config.replace(
            "  \"permissions\": {\n    \"defaultMode\": \"dontAsk\",\n    \"deny\": []\n  },\n",
            "",
        ),
    )
    .expect("corrupt rust permission config");

    let error = doctor::run(&out).expect_err("doctor should detect missing rust permission config");
    assert!(error.contains("missing permission config"));
    fs::remove_dir_all(out).ok();
}

#[test]
fn generated_python_runtime_enforces_permission_policy() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("python-runtime-permissions");
    init::run(&init::InitArgs {
        project_name: "demo-agent".to_string(),
        language: "python".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("python init works");

    let initial = run_command(Command::new("python3").arg("-c").arg("import sys; sys.path.insert(0, '.'); from src.demo_agent.app import main; print(main())").current_dir(&out));
    assert!(initial.contains("approval_mode=default"));
    assert!(initial.contains("bash_policy=bash=approval-required"));

    let config_path = out.join("agentkit.toml");
    let config = fs::read_to_string(&config_path).expect("read python config");
    fs::write(
        &config_path,
        config
            .replace(
                "approval_mode = \"default\"",
                "approval_mode = \"acceptEdits\"",
            )
            .replace("deny = []", "deny = [\"bash\"]"),
    )
    .expect("write python permission config");

    let updated = run_command(Command::new("python3").arg("-c").arg("import sys; sys.path.insert(0, '.'); from src.demo_agent.app import main; print(main())").current_dir(&out));
    assert!(updated.contains("approval_mode=acceptEdits"));
    assert!(updated.contains("bash_policy=bash=denied"));
    assert!(updated.contains("file_write_policy=file_write=allowed"));
    assert_ne!(initial, updated);
    fs::remove_dir_all(out).ok();
}

#[test]
fn generated_typescript_runtime_enforces_permission_policy() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("typescript-runtime-permissions");
    init::run(&init::InitArgs {
        project_name: "demo-ts".to_string(),
        language: "typescript".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("typescript init works");

    let initial = run_command(Command::new("node").arg("src/index.ts").current_dir(&out));
    assert!(initial.contains("approval_mode=default"));
    assert!(initial.contains("bash_policy=bash=approval-required"));

    let config_path = out.join("boilerplate.config.ts");
    let config = fs::read_to_string(&config_path).expect("read typescript config");
    fs::write(
        &config_path,
        config
            .replace("approvalMode: 'default'", "approvalMode: 'acceptEdits'")
            .replace("deny: []", "deny: ['bash']"),
    )
    .expect("write typescript permission config");

    let updated = run_command(Command::new("node").arg("src/index.ts").current_dir(&out));
    assert!(updated.contains("approval_mode=acceptEdits"));
    assert!(updated.contains("bash_policy=bash=denied"));
    assert!(updated.contains("file_write_policy=file_write=allowed"));
    assert_ne!(initial, updated);
    fs::remove_dir_all(out).ok();
}

#[test]
fn generated_rust_runtime_enforces_permission_policy() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("rust-runtime-permissions");
    init::run(&init::InitArgs {
        project_name: "demo-rust".to_string(),
        language: "rust".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("rust init works");

    let initial = run_command(
        Command::new("cargo")
            .arg("run")
            .arg("--quiet")
            .current_dir(&out),
    );
    assert!(initial.contains("approval_mode=dontAsk"));
    assert!(initial.contains("bash_policy=bash=allowed"));

    let config_path = out.join(".claw.json");
    let config = fs::read_to_string(&config_path).expect("read rust config");
    fs::write(
        &config_path,
        config
            .replace("\"defaultMode\": \"dontAsk\"", "\"defaultMode\": \"never\"")
            .replace("\"deny\": []", "\"deny\": [\"file_write\"]"),
    )
    .expect("write rust permission config");

    let updated = run_command(
        Command::new("cargo")
            .arg("run")
            .arg("--quiet")
            .current_dir(&out),
    );
    assert!(updated.contains("approval_mode=never"));
    assert!(updated.contains("bash_policy=bash=blocked"));
    assert!(updated.contains("file_write_policy=file_write=denied"));
    assert_ne!(initial, updated);
    fs::remove_dir_all(out).ok();
}

#[test]
fn doctor_detects_missing_rust_tool_defaults() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("rust-tool-defaults-missing");
    init::run(&init::InitArgs {
        project_name: "demo-rust".to_string(),
        language: "rust".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("rust init works");

    let path = out.join(".claw.json");
    let config = std::fs::read_to_string(&path).expect("read rust config");
    std::fs::write(
        &path,
        config.replace("    \"bashTimeoutMs\": 120000,\n", ""),
    )
    .expect("corrupt rust tool defaults");

    let error = doctor::run(&out).expect_err("doctor should detect missing rust tool defaults");
    assert!(error.contains("missing tool default config"));
    fs::remove_dir_all(out).ok();
}

#[test]
fn rust_feature_add_and_remove_updates_project() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("rust-feature");
    init::run(&init::InitArgs {
        project_name: "demo-rust".to_string(),
        language: "rust".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("rust init works");

    feature::add(&feature::FeatureArgs {
        feature_id: "rust:local-plugins".to_string(),
        project: out.clone(),
    })
    .expect("rust feature add works");

    assert!(out.join(".claw/plugins/README.md").exists());
    let config = std::fs::read_to_string(out.join(".claw.json")).expect("read rust config");
    assert!(config.contains("\"enabled\": [\n      \"local-plugins\"\n    ]"));

    let doctor_message = doctor::run(&out).expect("rust doctor after feature add works");
    assert!(doctor_message.contains("doctor ok"));

    feature::remove(&feature::FeatureArgs {
        feature_id: "rust:local-plugins".to_string(),
        project: out.clone(),
    })
    .expect("rust feature remove works");

    assert!(!out.join(".claw/plugins/README.md").exists());
    let config =
        std::fs::read_to_string(out.join(".claw.json")).expect("read rust config after remove");
    assert!(config.contains("\"enabled\": []"));

    let doctor_message = doctor::run(&out).expect("rust doctor after feature remove works");
    assert!(doctor_message.contains("doctor ok"));
    fs::remove_dir_all(out).ok();
}

#[test]
fn generated_python_runtime_uses_provider_prompt_and_context() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("python-runtime-summary");
    init::run(&init::InitArgs {
        project_name: "demo-agent".to_string(),
        language: "python".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("python init works");

    let initial = run_command(Command::new("python3").arg("-c").arg("import sys; sys.path.insert(0, '.'); from src.demo_agent.app import main; print(main())").current_dir(&out));
    assert!(initial.contains("provider=openai"));
    assert!(initial.contains("model=gpt-5.4"));

    let config_path = out.join("agentkit.toml");
    let config = fs::read_to_string(&config_path).expect("read python config");
    fs::write(
        &config_path,
        config.replace(
            "default_provider = \"openai\"",
            "default_provider = \"anthropic\"",
        ),
    )
    .expect("write python config");
    fs::write(
        out.join(".agent/prompts/system.md"),
        "You are the project-local coding agent for __PROJECT_NAME__. New prompt layer.\n",
    )
    .expect("write python prompt");
    fs::write(
        out.join(".agent/context/README.md"),
        "# Context\n\nUpdated runtime context fragment.\n",
    )
    .expect("write python context");

    let updated = run_command(Command::new("python3").arg("-c").arg("import sys; sys.path.insert(0, '.'); from src.demo_agent.app import main; print(main())").current_dir(&out));
    assert!(updated.contains("provider=anthropic"));
    assert!(updated.contains("model=claude-sonnet-4-6"));
    assert_ne!(initial, updated);
    fs::remove_dir_all(out).ok();
}

#[test]
fn generated_typescript_runtime_uses_provider_prompt_and_context() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("typescript-runtime-summary");
    init::run(&init::InitArgs {
        project_name: "demo-ts".to_string(),
        language: "typescript".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("typescript init works");

    let initial = run_command(Command::new("node").arg("src/index.ts").current_dir(&out));
    assert!(initial.contains("provider=anthropic"));
    assert!(initial.contains("model=claude-sonnet-4-6"));

    let config_path = out.join("boilerplate.config.ts");
    let config = fs::read_to_string(&config_path).expect("read typescript config");
    fs::write(
        &config_path,
        config.replace("defaultProvider: 'anthropic'", "defaultProvider: 'openai'"),
    )
    .expect("write typescript config");
    fs::write(
        out.join(".agent/prompts/system.md"),
        "You are the default coding agent for this generated project.\nPrompt override applied.\n",
    )
    .expect("write typescript prompt");
    fs::write(
        out.join(".agent/context/README.md"),
        "# Context\n\nUpdated TypeScript runtime context fragment.\n",
    )
    .expect("write typescript context");

    let updated = run_command(Command::new("node").arg("src/index.ts").current_dir(&out));
    assert!(updated.contains("provider=openai"));
    assert!(updated.contains("model=gpt-5.2-codex"));
    assert_ne!(initial, updated);
    fs::remove_dir_all(out).ok();
}

#[test]
fn generated_rust_runtime_uses_provider_prompt_and_context() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("rust-runtime-summary");
    init::run(&init::InitArgs {
        project_name: "demo-rust".to_string(),
        language: "rust".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("rust init works");

    let initial = run_command(
        Command::new("cargo")
            .arg("run")
            .arg("--quiet")
            .current_dir(&out),
    );
    assert!(initial.contains("provider=anthropic"));
    assert!(initial.contains("model=claude-sonnet-4-6"));

    let config_path = out.join(".claw.json");
    let config = fs::read_to_string(&config_path).expect("read rust config");
    fs::write(
        &config_path,
        config.replace(
            "\"defaultProvider\": \"anthropic\"",
            "\"defaultProvider\": \"openai\"",
        ),
    )
    .expect("write rust config");
    fs::write(
        out.join(".agent/prompts/system.md"),
        "# Rust project instructions\n\nUpdated runtime prompt surface.\n",
    )
    .expect("write rust prompt");
    fs::write(
        out.join(".agent/context/README.md"),
        "# Context\n\nUpdated Rust runtime context fragment.\n",
    )
    .expect("write rust context");

    let updated = run_command(
        Command::new("cargo")
            .arg("run")
            .arg("--quiet")
            .current_dir(&out),
    );
    assert!(updated.contains("provider=openai"));
    assert!(updated.contains("model=gpt-5.2-codex"));
    assert_ne!(initial, updated);
    fs::remove_dir_all(out).ok();
}

#[test]
fn doctor_detects_missing_rust_feature_asset() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("rust-feature-missing-asset");
    init::run(&init::InitArgs {
        project_name: "demo-rust".to_string(),
        language: "rust".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("rust init works");

    feature::add(&feature::FeatureArgs {
        feature_id: "rust:local-plugins".to_string(),
        project: out.clone(),
    })
    .expect("rust feature add works");

    fs::remove_file(out.join(".claw/plugins/README.md")).expect("remove rust feature asset");

    let error = doctor::run(&out).expect_err("doctor should detect missing rust asset");
    assert!(error.contains("missing feature asset:"));
    fs::remove_dir_all(out).ok();
}

#[test]
fn doctor_detects_missing_rust_skill_asset() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("rust-skill-missing-asset");
    init::run(&init::InitArgs {
        project_name: "demo-rust".to_string(),
        language: "rust".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("rust init works");

    fs::remove_file(out.join(".agent/skills/plan/SKILL.md")).expect("remove rust skill asset");

    let error = doctor::run(&out).expect_err("doctor should detect missing rust skill asset");
    assert!(error.contains("missing required file:"));
    fs::remove_dir_all(out).ok();
}

#[test]
fn doctor_detects_missing_rust_instruction_asset() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("rust-instruction-missing-asset");
    init::run(&init::InitArgs {
        project_name: "demo-rust".to_string(),
        language: "rust".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("rust init works");

    fs::remove_file(out.join(".agent/prompts/system.md")).expect("remove rust instruction asset");

    let error = doctor::run(&out).expect_err("doctor should detect missing rust instruction asset");
    assert!(error.contains("missing required file:"));
    fs::remove_dir_all(out).ok();
}

#[test]
fn doctor_detects_missing_rust_prompt_layer() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("rust-prompt-layer-missing-asset");
    init::run(&init::InitArgs {
        project_name: "demo-rust".to_string(),
        language: "rust".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("rust init works");

    fs::remove_file(out.join(".agent/prompts/sections/style.md"))
        .expect("remove rust prompt layer asset");

    let error = doctor::run(&out).expect_err("doctor should detect missing rust prompt layer");
    assert!(error.contains("missing required file:"));
    fs::remove_dir_all(out).ok();
}

#[test]
fn doctor_detects_missing_rust_agent_asset() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("rust-agent-missing-asset");
    init::run(&init::InitArgs {
        project_name: "demo-rust".to_string(),
        language: "rust".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("rust init works");

    fs::remove_file(out.join(".agent/agents/executor.agent.json"))
        .expect("remove rust agent asset");

    let error = doctor::run(&out).expect_err("doctor should detect missing rust agent asset");
    assert!(error.contains("missing required file:"));
    fs::remove_dir_all(out).ok();
}

#[test]
fn doctor_detects_missing_rust_session_root() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("rust-session-root-missing");
    init::run(&init::InitArgs {
        project_name: "demo-rust".to_string(),
        language: "rust".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("rust init works");

    fs::remove_file(out.join(".agent/sessions/README.md")).expect("remove rust session root");

    let error = doctor::run(&out).expect_err("doctor should detect missing rust session root");
    assert!(error.contains("missing required file:"));
    fs::remove_dir_all(out).ok();
}

#[test]
fn doctor_detects_missing_rust_context_root() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("rust-context-root-missing");
    init::run(&init::InitArgs {
        project_name: "demo-rust".to_string(),
        language: "rust".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("rust init works");

    fs::remove_file(out.join(".agent/context/README.md")).expect("remove rust context root");

    let error = doctor::run(&out).expect_err("doctor should detect missing rust context root");
    assert!(error.contains("missing required file:"));
    fs::remove_dir_all(out).ok();
}

#[test]
fn doctor_detects_missing_rust_usage_root() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("rust-usage-root-missing");
    init::run(&init::InitArgs {
        project_name: "demo-rust".to_string(),
        language: "rust".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("rust init works");

    fs::remove_file(out.join(".agent/usage/README.md")).expect("remove rust usage root");

    let error = doctor::run(&out).expect_err("doctor should detect missing rust usage root");
    assert!(error.contains("missing required file:"));
    fs::remove_dir_all(out).ok();
}

#[test]
fn doctor_detects_missing_rust_agent_config() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("rust-agent-config-missing");
    init::run(&init::InitArgs {
        project_name: "demo-rust".to_string(),
        language: "rust".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("rust init works");

    let path = out.join(".claw.json");
    let config = std::fs::read_to_string(&path).expect("read rust config");
    let mut value: serde_json::Value = serde_json::from_str(&config).expect("parse rust config");
    value
        .get_mut("agents")
        .and_then(|agents| agents.as_object_mut())
        .expect("agents object")
        .remove("defaultAgent");
    std::fs::write(
        &path,
        serde_json::to_string_pretty(&value).expect("serialize rust config") + "\n",
    )
    .expect("corrupt rust agent config");

    let error = doctor::run(&out).expect_err("doctor should detect missing rust agent config");
    assert!(error.contains("missing agent config"));
    fs::remove_dir_all(out).ok();
}

#[test]
fn python_proving_slice_repo_gate() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("python-proving-slice");

    let init_status = cli::run(vec![
        "aicd".to_string(),
        "init".to_string(),
        "demo-agent".to_string(),
        "--language".to_string(),
        "python".to_string(),
        "--output".to_string(),
        out.display().to_string(),
    ]);
    assert_eq!(init_status, 0);

    let feature_add_status = cli::run(vec![
        "aicd".to_string(),
        "feature".to_string(),
        "add".to_string(),
        "github-pr-review".to_string(),
        "--project".to_string(),
        out.display().to_string(),
    ]);
    assert_eq!(feature_add_status, 0);

    let doctor_status = cli::run(vec![
        "aicd".to_string(),
        "doctor".to_string(),
        "--project".to_string(),
        out.display().to_string(),
    ]);
    assert_eq!(doctor_status, 0);

    let feature_remove_status = cli::run(vec![
        "aicd".to_string(),
        "feature".to_string(),
        "remove".to_string(),
        "github-pr-review".to_string(),
        "--project".to_string(),
        out.display().to_string(),
    ]);
    assert_eq!(feature_remove_status, 0);

    fs::remove_dir_all(out).ok();
}

#[test]
fn python_doctor_detects_broken_feature_registry() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("broken-feature-registry");

    let init_status = cli::run(vec![
        "aicd".to_string(),
        "init".to_string(),
        "demo-agent".to_string(),
        "--language".to_string(),
        "python".to_string(),
        "--output".to_string(),
        out.display().to_string(),
    ]);
    assert_eq!(init_status, 0);

    let feature_add_status = cli::run(vec![
        "aicd".to_string(),
        "feature".to_string(),
        "add".to_string(),
        "github-pr-review".to_string(),
        "--project".to_string(),
        out.display().to_string(),
    ]);
    assert_eq!(feature_add_status, 0);

    let registry_path = out.join(".agent/features/registry.json");
    fs::write(&registry_path, "{\n  \"features\": []\n}\n").expect("corrupt feature registry");

    let doctor_status = cli::run(vec![
        "aicd".to_string(),
        "doctor".to_string(),
        "--project".to_string(),
        out.display().to_string(),
    ]);
    assert_eq!(doctor_status, 1);

    fs::remove_dir_all(out).ok();
}
