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

fn brand_root(project_root: &PathBuf) -> PathBuf {
    let mut candidates = fs::read_dir(project_root)
        .expect("read generated project root")
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| {
            path.is_dir()
                && path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .map(|name| {
                        (name.starts_with('.') && !name.ends_with("-plugin"))
                            || name == "__BRAND_ROOT__"
                    })
                    .unwrap_or(false)
                && path.join("settings.json").exists()
        })
        .collect::<Vec<_>>();
    candidates.sort();
    candidates
        .into_iter()
        .next()
        .expect("generated project should include branded root")
}

fn brand_root_name(project_root: &PathBuf) -> String {
    brand_root(project_root)
        .file_name()
        .and_then(|name| name.to_str())
        .expect("brand root name")
        .to_string()
}

fn brand_doc_path(project_root: &PathBuf) -> PathBuf {
    let mut docs = fs::read_dir(project_root)
        .expect("read generated project root")
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| {
            path.is_file()
                && path.extension().and_then(|ext| ext.to_str()) == Some("md")
                && path.file_name().and_then(|name| name.to_str()) != Some("README.md")
        })
        .collect::<Vec<_>>();
    docs.sort();
    docs.into_iter()
        .next()
        .expect("generated project should include branded top-level doc")
}

fn brand_config_path(project_root: &PathBuf) -> PathBuf {
    let mut configs = fs::read_dir(project_root)
        .expect("read generated project root")
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| {
            path.is_file()
                && path.extension().and_then(|ext| ext.to_str()) == Some("json")
                && path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .map(|name| name.starts_with('.'))
                    .unwrap_or(false)
        })
        .collect::<Vec<_>>();
    configs.sort();
    configs
        .into_iter()
        .next()
        .expect("generated project should include branded compat config")
}

fn session_path(project_root: &PathBuf, file: &str) -> PathBuf {
    brand_root(project_root).join("sessions").join(file)
}

fn runtime_tool_file(project_root: &PathBuf) -> PathBuf {
    session_path(project_root, "runtime-tool-smoke.txt")
}

fn expected_export_path(project_root: &PathBuf) -> String {
    format!(
        "export_path={}/sessions/local-main-session.export.md",
        brand_root_name(project_root)
    )
}

fn assert_runtime_state_artifacts(project_root: &PathBuf) {
    assert!(session_path(project_root, "local-main-session.state").exists());
    assert!(session_path(project_root, "latest.state").exists());
    assert!(session_path(project_root, "local-main-session.export.md").exists());
    assert!(session_path(project_root, "summary.state").exists());
}

fn assert_contains_all(haystack: &str, needles: &[&str]) {
    for needle in needles {
        assert!(
            haystack.contains(needle),
            "expected output to contain '{needle}', got: {haystack}"
        );
    }
}

fn clean_runtime_artifacts(project_root: &PathBuf) {
    for relative in [
        "latest.state",
        "local-main-session.state",
        "local-main-session.export.md",
        "summary.state",
        "runtime-tool-smoke.txt",
    ] {
        fs::remove_file(session_path(project_root, relative)).ok();
    }
}

fn python_runtime_command_output(repo: &PathBuf, out: &PathBuf, runtime_output: &str) -> String {
    run_command(
        Command::new("python3")
            .arg("-c")
            .arg(
                "import sys, pathlib; sys.path.insert(0, '.'); from languages.python.runtime.registry.execution_registry import execution_registry; root = pathlib.Path(sys.argv[1]); runtime_output = sys.argv[2]; registry = execution_registry(); run = registry['run_command']; print(run('doctor', root, runtime_output))",
            )
            .arg(out.display().to_string())
            .arg(runtime_output.to_string())
            .current_dir(repo),
    )
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
            "typescript:advanced-planning",
            "typescript:github-pr-review",
            "typescript:interactive-clarification-tools",
            "typescript:lsp-tooling",
            "typescript:mcp-integration",
            "typescript:oauth-onboarding",
            "typescript:prompt-suggestion-services",
            "typescript:session-memory",
            "typescript:team-memory"
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
    assert!(brand_doc_path(&out).exists());
    assert!(brand_root(&out).join("instructions.md").exists());
    assert!(brand_root(&out).join("settings.json").exists());
    assert!(brand_root(&out).join("agents/executor.md").exists());
    assert!(out.join(".agents/skills/plan/SKILL.md").exists());
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
    let runtime_output = run_command(
        Command::new("python3")
            .arg("-c")
            .arg("import sys; sys.path.insert(0, '.'); from src.demo_agent.app import main; print(main())")
            .current_dir(&out),
    );
    assert!(runtime_output.contains("session_id=local-main-session"));
    assert_runtime_state_artifacts(&out);
    let message = doctor::run(&out).expect("doctor works");
    assert!(message.contains("doctor ok"));
    fs::remove_dir_all(out).ok();
}

#[test]
fn feature_add_updates_project() {
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
    let runtime_after_add = run_command(
        Command::new("python3")
            .arg("-c")
            .arg("import sys; sys.path.insert(0, '.'); from src.demo_agent.app import main; print(main())")
            .current_dir(&out),
    );
    assert!(runtime_after_add.contains("session_id=local-main-session"));
    assert_runtime_state_artifacts(&out);
    assert!(brand_root(&out).join("agents/review-agent.md").exists());
    assert!(brand_root(&out).join("skills/review-pr/SKILL.md").exists());
    assert!(out.join(".agents/skills/review-pr/SKILL.md").exists());
    assert!(read_agentkit_toml(&out)
        .unwrap()
        .contains("enabled = [\"github-pr-review\"]"));

    fs::remove_dir_all(out).ok();
}

#[test]
fn namespaced_feature_add_updates_project() {
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
    assert!(brand_root(&out).join("agents/review-agent.md").exists());
    assert!(read_agentkit_toml(&out)
        .unwrap()
        .contains("enabled = [\"github-pr-review\"]"));

    fs::remove_dir_all(out).ok();
}

#[test]
fn feature_add_writes_project_feature_receipt() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("feature-receipt");
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

    let doctor_message = doctor::run(&out).expect("doctor after feature add");
    assert!(doctor_message.contains("doctor ok"));

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
    assert_eq!(python.feature_registry, "features/registry.json");
    assert!(python.supports.init);
    assert!(python.supports.feature_add);
    assert!(python.supports.doctor);
    assert_eq!(python.runtime.project_markers.len(), 2);
    assert_eq!(python.runtime.generic_workspace_root, ".agents");

    let rust = read_language_manifest(&fixture_path("rust.language.manifest.json"))
        .expect("rust manifest should parse");
    assert_eq!(rust.id, "rust");
    assert_eq!(rust.display_name, "Rust");
    assert_eq!(rust.runtime.config_file, ".<brand>.json");
    assert_eq!(rust.feature_registry, "features/registry.json");
    assert!(rust.supports.init);
    assert!(rust.supports.feature_add);
    assert!(rust.supports.doctor);
    assert_eq!(rust.runtime.generic_workspace_root, ".agents");
    assert_eq!(
        rust.runtime.native_workspace_root.as_deref(),
        Some(".<brand>")
    );

    let typescript = read_language_manifest(&fixture_path("typescript.language.manifest.json"))
        .expect("typescript manifest should parse");
    assert_eq!(typescript.id, "typescript");
    assert_eq!(typescript.display_name, "TypeScript");
    assert_eq!(typescript.runtime.config_file, "boilerplate.config.ts");
    assert_eq!(typescript.feature_registry, "features/registry.json");
    assert!(typescript.supports.init);
    assert!(typescript.supports.feature_add);
    assert!(typescript.supports.doctor);
    assert_eq!(typescript.runtime.generic_workspace_root, ".agents");
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
    assert!(brand_doc_path(&out).exists());
    assert!(brand_root(&out).join("instructions.md").exists());
    assert!(brand_root(&out).join("settings.json").exists());
    assert!(brand_root(&out).join("sessions/README.md").exists());
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

    let runtime_output = run_command(Command::new("node").arg("src/index.ts").current_dir(&out));
    assert!(runtime_output.contains("session_id=local-main-session"));
    assert_runtime_state_artifacts(&out);

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
fn typescript_feature_add_updates_project() {
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

    assert!(brand_root(&out).join("agents/review-agent.md").exists());
    assert!(brand_root(&out).join("skills/review-pr/SKILL.md").exists());
    assert!(out.join(".agents/skills/review-pr/SKILL.md").exists());

    let config =
        std::fs::read_to_string(out.join("boilerplate.config.ts")).expect("read typescript config");
    assert!(config.contains("features: {\n    enabled: ['github-pr-review']"));
    assert!(config.contains("'review-agent'"));
    assert!(config.contains("'review-pr'"));

    let doctor_message = doctor::run(&out).expect("typescript doctor after feature add works");
    assert!(doctor_message.contains("doctor ok"));

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

    fs::remove_file(brand_root(&out).join("skills/review-pr/SKILL.md"))
        .expect("remove typescript feature asset");

    let error = doctor::run(&out).expect_err("doctor should detect missing typescript asset");
    assert!(error.contains("missing feature asset:"));
    fs::remove_dir_all(out).ok();
}

#[test]
fn typescript_optional_feature_packs_add_tools_and_skills() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("typescript-optional-feature-packs");
    init::run(&init::InitArgs {
        project_name: "demo-ts".to_string(),
        language: "typescript".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("typescript init works");

    for feature_id in [
        "typescript:advanced-planning",
        "typescript:interactive-clarification-tools",
        "typescript:lsp-tooling",
        "typescript:mcp-integration",
    ] {
        feature::add(&feature::FeatureArgs {
            feature_id: feature_id.to_string(),
            project: out.clone(),
        })
        .expect("typescript optional feature add works");
    }

    assert!(brand_root(&out)
        .join("skills/advanced-plan/SKILL.md")
        .exists());
    assert!(brand_root(&out)
        .join("skills/clarify-requirements/SKILL.md")
        .exists());
    assert!(brand_root(&out)
        .join("skills/use-lsp-tooling/SKILL.md")
        .exists());
    assert!(brand_root(&out)
        .join("skills/use-mcp-integration/SKILL.md")
        .exists());
    assert!(out.join(".agents/skills/advanced-plan/SKILL.md").exists());
    assert!(out
        .join(".agents/skills/clarify-requirements/SKILL.md")
        .exists());
    assert!(out.join(".agents/skills/use-lsp-tooling/SKILL.md").exists());
    assert!(out
        .join(".agents/skills/use-mcp-integration/SKILL.md")
        .exists());

    let config =
        std::fs::read_to_string(out.join("boilerplate.config.ts")).expect("read typescript config");
    for expected in [
        "interactive-clarification-tools",
        "lsp-tooling",
        "mcp-integration",
        "advanced-plan",
        "clarify-requirements",
        "use-lsp-tooling",
        "use-mcp-integration",
        "enter_plan_mode",
        "exit_plan_mode",
        "todo_write",
        "tool_search",
        "skill",
        "ask_user",
        "brief",
        "send_message",
        "lsp",
        "mcp",
        "list_mcp_resources",
        "read_mcp_resource",
        "mcp_auth",
    ] {
        assert!(config.contains(expected), "missing {expected} in config");
    }

    let doctor_message = doctor::run(&out).expect("doctor after optional feature adds works");
    assert!(doctor_message.contains("doctor ok"));

    fs::remove_dir_all(out).ok();
}

#[test]
fn typescript_service_feature_packs_add_optional_skills() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("typescript-service-feature-packs");
    init::run(&init::InitArgs {
        project_name: "demo-ts".to_string(),
        language: "typescript".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("typescript init works");

    for feature_id in [
        "typescript:oauth-onboarding",
        "typescript:prompt-suggestion-services",
        "typescript:session-memory",
        "typescript:team-memory",
    ] {
        feature::add(&feature::FeatureArgs {
            feature_id: feature_id.to_string(),
            project: out.clone(),
        })
        .expect("typescript service feature add works");
    }

    for skill in [
        "use-oauth-onboarding",
        "use-prompt-suggestion-services",
        "use-session-memory",
        "use-team-memory",
    ] {
        assert!(brand_root(&out)
            .join(format!("skills/{skill}/SKILL.md"))
            .exists());
        assert!(out
            .join(format!(".agents/skills/{skill}/SKILL.md"))
            .exists());
    }

    let config =
        std::fs::read_to_string(out.join("boilerplate.config.ts")).expect("read typescript config");
    for expected in [
        "oauth-onboarding",
        "prompt-suggestion-services",
        "session-memory",
        "team-memory",
        "use-oauth-onboarding",
        "use-prompt-suggestion-services",
        "use-session-memory",
        "use-team-memory",
    ] {
        assert!(config.contains(expected), "missing {expected} in config");
    }

    let doctor_message = doctor::run(&out).expect("doctor after service feature adds works");
    assert!(doctor_message.contains("doctor ok"));

    fs::remove_dir_all(out).ok();
}

#[test]
fn typescript_plugin_feature_packs_add_optional_skills() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("typescript-plugin-feature-packs");
    init::run(&init::InitArgs {
        project_name: "demo-ts".to_string(),
        language: "typescript".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("typescript init works");

    for feature_id in [
        "typescript:plugin-runtime",
        "typescript:plugin-marketplace-ui",
    ] {
        feature::add(&feature::FeatureArgs {
            feature_id: feature_id.to_string(),
            project: out.clone(),
        })
        .expect("typescript plugin feature add works");
    }

    for skill in ["use-plugin-runtime", "use-plugin-marketplace-ui"] {
        assert!(brand_root(&out)
            .join(format!("skills/{skill}/SKILL.md"))
            .exists());
        assert!(out
            .join(format!(".agents/skills/{skill}/SKILL.md"))
            .exists());
    }

    let config =
        std::fs::read_to_string(out.join("boilerplate.config.ts")).expect("read typescript config");
    for expected in [
        "plugin-runtime",
        "plugin-marketplace-ui",
        "use-plugin-runtime",
        "use-plugin-marketplace-ui",
    ] {
        assert!(config.contains(expected), "missing {expected} in config");
    }

    let doctor_message = doctor::run(&out).expect("doctor after plugin feature adds works");
    assert!(doctor_message.contains("doctor ok"));

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

    fs::remove_file(brand_root(&out).join("sessions/README.md"))
        .expect("remove typescript session root");

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

    fs::remove_file(brand_root(&out).join("instructions.md"))
        .expect("remove typescript context root");

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

    fs::remove_file(brand_root(&out).join("settings.json"))
        .expect("remove typescript settings root");

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
    assert!(brand_doc_path(&out).exists());
    assert!(brand_config_path(&out).exists());
    assert!(out.join("src/main.rs").exists());
    let entry_text = fs::read_to_string(out.join("src/main.rs")).expect("read rust entry");
    assert!(entry_text.contains("session loop completed"));
    assert!(brand_root(&out).join("instructions.md").exists());
    assert!(brand_root(&out).join("agents/planner.md").exists());
    assert!(brand_root(&out).join("agents/executor.md").exists());
    assert!(brand_root(&out).join("agents/reviewer.md").exists());
    assert!(brand_root(&out).join("sessions/README.md").exists());
    assert!(out.join(".agents/skills/plan/SKILL.md").exists());
    assert!(out.join(".agents/skills/add-feature/SKILL.md").exists());
    assert!(out.join(".agents/skills/verify/SKILL.md").exists());
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

    let runtime_output = run_command(
        Command::new("cargo")
            .arg("run")
            .arg("--quiet")
            .current_dir(&out),
    );
    assert!(runtime_output.contains("session_id=local-main-session"));
    assert_runtime_state_artifacts(&out);

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

    let path = brand_config_path(&out);
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

    let path = brand_config_path(&out);
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

    let path = brand_config_path(&out);
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

    let config_path = brand_config_path(&out);
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
fn generated_python_runtime_executes_core_tools() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("python-runtime-tools");
    init::run(&init::InitArgs {
        project_name: "demo-agent".to_string(),
        language: "python".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("python init works");

    let config_path = out.join("agentkit.toml");
    let config = fs::read_to_string(&config_path).expect("read python config");
    fs::write(
        &config_path,
        config.replace(
            "approval_mode = \"default\"",
            "approval_mode = \"acceptEdits\"",
        ),
    )
    .expect("write python config");

    let output = run_command(Command::new("python3").arg("-c").arg("import sys; sys.path.insert(0, '.'); from src.demo_agent.app import main; print(main())").current_dir(&out));
    assert!(output.contains("bash_result=tool-bash-ok"));
    assert!(output.contains("file_read_result="));
    assert!(output.contains("file_write_result=tool-write-ok"));
    assert!(output.contains("file_edit_result=tool-write-ok edited"));
    assert!(output.contains("web_fetch_result=tool-web-fetch"));
    assert_eq!(
        fs::read_to_string(runtime_tool_file(&out)).expect("read python tool file"),
        "tool-write-ok edited"
    );
    fs::remove_dir_all(out).ok();
}

#[test]
fn generated_typescript_runtime_executes_core_tools() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("typescript-runtime-tools");
    init::run(&init::InitArgs {
        project_name: "demo-ts".to_string(),
        language: "typescript".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("typescript init works");

    let config_path = out.join("boilerplate.config.ts");
    let config = fs::read_to_string(&config_path).expect("read typescript config");
    fs::write(
        &config_path,
        config.replace("approvalMode: 'default'", "approvalMode: 'acceptEdits'"),
    )
    .expect("write typescript config");

    let output = run_command(Command::new("node").arg("src/index.ts").current_dir(&out));
    assert!(output.contains("bash_result=tool-bash-ok"));
    assert!(output.contains("file_read_result="));
    assert!(output.contains("file_write_result=tool-write-ok"));
    assert!(output.contains("file_edit_result=tool-write-ok edited"));
    assert!(output.contains("web_fetch_result=tool-web-fetch"));
    assert_eq!(
        fs::read_to_string(runtime_tool_file(&out)).expect("read typescript tool file"),
        "tool-write-ok edited"
    );
    fs::remove_dir_all(out).ok();
}

#[test]
fn generated_rust_runtime_executes_core_tools() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("rust-runtime-tools");
    init::run(&init::InitArgs {
        project_name: "demo-rust".to_string(),
        language: "rust".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("rust init works");

    let output = run_command(
        Command::new("cargo")
            .arg("run")
            .arg("--quiet")
            .current_dir(&out),
    );
    assert!(output.contains("bash_result=tool-bash-ok"));
    assert!(output.contains("file_read_result="));
    assert!(output.contains("file_write_result=tool-write-ok"));
    assert!(output.contains("file_edit_result=tool-write-ok edited"));
    assert!(output.contains("web_fetch_result=tool-web-fetch"));
    assert_eq!(
        fs::read_to_string(runtime_tool_file(&out)).expect("read rust tool file"),
        "tool-write-ok edited"
    );
    fs::remove_dir_all(out).ok();
}

#[test]
fn generated_python_runtime_persists_session_and_usage() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("python-runtime-session");
    init::run(&init::InitArgs {
        project_name: "demo-agent".to_string(),
        language: "python".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("python init works");

    let first = run_command(Command::new("python3").arg("-c").arg("import sys; sys.path.insert(0, '.'); from src.demo_agent.app import main; print(main())").current_dir(&out));
    let second = run_command(Command::new("python3").arg("-c").arg("import sys; sys.path.insert(0, '.'); from src.demo_agent.app import main; print(main())").current_dir(&out));

    assert!(first.contains("session_id=local-main-session"));
    assert!(first.contains("turn_count=1"));
    assert!(second.contains("turn_count=2"));
    assert!(second.contains("usage_entries=2"));
    assert!(second.contains(&expected_export_path(&out)));
    assert!(session_path(&out, "local-main-session.state").exists());
    assert!(session_path(&out, "latest.state").exists());
    assert!(session_path(&out, "local-main-session.export.md").exists());
    assert!(session_path(&out, "summary.state").exists());
    assert!(
        fs::read_to_string(session_path(&out, "local-main-session.state"))
            .expect("read python session state")
            .contains("turn_count=2")
    );
    fs::remove_dir_all(out).ok();
}

#[test]
fn generated_typescript_runtime_persists_session_and_usage() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("typescript-runtime-session");
    init::run(&init::InitArgs {
        project_name: "demo-ts".to_string(),
        language: "typescript".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("typescript init works");

    let first = run_command(Command::new("node").arg("src/index.ts").current_dir(&out));
    let second = run_command(Command::new("node").arg("src/index.ts").current_dir(&out));

    assert!(first.contains("session_id=local-main-session"));
    assert!(first.contains("turn_count=1"));
    assert!(second.contains("turn_count=2"));
    assert!(second.contains("usage_entries=2"));
    assert!(second.contains(&expected_export_path(&out)));
    assert!(session_path(&out, "local-main-session.state").exists());
    assert!(session_path(&out, "latest.state").exists());
    assert!(session_path(&out, "local-main-session.export.md").exists());
    assert!(session_path(&out, "summary.state").exists());
    assert!(
        fs::read_to_string(session_path(&out, "local-main-session.state"))
            .expect("read typescript session state")
            .contains("turn_count=2")
    );
    fs::remove_dir_all(out).ok();
}

#[test]
fn generated_rust_runtime_persists_session_and_usage() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("rust-runtime-session");
    init::run(&init::InitArgs {
        project_name: "demo-rust".to_string(),
        language: "rust".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("rust init works");

    let first = run_command(
        Command::new("cargo")
            .arg("run")
            .arg("--quiet")
            .current_dir(&out),
    );
    let second = run_command(
        Command::new("cargo")
            .arg("run")
            .arg("--quiet")
            .current_dir(&out),
    );

    assert!(first.contains("session_id=local-main-session"));
    assert!(first.contains("turn_count=1"));
    assert!(second.contains("turn_count=2"));
    assert!(second.contains("usage_entries=2"));
    assert!(second.contains(&expected_export_path(&out)));
    assert!(session_path(&out, "local-main-session.state").exists());
    assert!(session_path(&out, "latest.state").exists());
    assert!(session_path(&out, "local-main-session.export.md").exists());
    assert!(session_path(&out, "summary.state").exists());
    assert!(
        fs::read_to_string(session_path(&out, "local-main-session.state"))
            .expect("read rust session state")
            .contains("turn_count=2")
    );
    fs::remove_dir_all(out).ok();
}

#[test]
fn core_slice_fixture_projects_match_cross_language_parity_profile() {
    let _guard = acquire_cli_test_guard();

    let python_out = temp_dir("core-parity-python");
    init::run(&init::InitArgs {
        project_name: "demo-agent".to_string(),
        language: "python".to_string(),
        output: python_out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("python init works");
    let python_config_path = python_out.join("agentkit.toml");
    let python_config = fs::read_to_string(&python_config_path).expect("read python config");
    fs::write(
        &python_config_path,
        python_config
            .replace(
                "default_provider = \"openai\"",
                "default_provider = \"anthropic\"",
            )
            .replace(
                "approval_mode = \"default\"",
                "approval_mode = \"acceptEdits\"",
            )
            .replace("deny = []", "deny = [\"bash\"]"),
    )
    .expect("write python parity config");

    let typescript_out = temp_dir("core-parity-typescript");
    init::run(&init::InitArgs {
        project_name: "demo-ts".to_string(),
        language: "typescript".to_string(),
        output: typescript_out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("typescript init works");
    let ts_config_path = typescript_out.join("boilerplate.config.ts");
    let ts_config = fs::read_to_string(&ts_config_path).expect("read typescript config");
    fs::write(
        &ts_config_path,
        ts_config
            .replace("approvalMode: 'default'", "approvalMode: 'acceptEdits'")
            .replace("deny: []", "deny: ['bash']"),
    )
    .expect("write typescript parity config");

    let rust_out = temp_dir("core-parity-rust");
    init::run(&init::InitArgs {
        project_name: "demo-rust".to_string(),
        language: "rust".to_string(),
        output: rust_out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("rust init works");
    let rust_config_path = brand_config_path(&rust_out);
    let rust_config = fs::read_to_string(&rust_config_path).expect("read rust config");
    fs::write(
        &rust_config_path,
        rust_config
            .replace(
                "\"defaultMode\": \"dontAsk\"",
                "\"defaultMode\": \"acceptEdits\"",
            )
            .replace("\"deny\": []", "\"deny\": [\"bash\"]"),
    )
    .expect("write rust parity config");

    let python_first = run_command(
        Command::new("python3")
            .arg("-c")
            .arg("import sys; sys.path.insert(0, '.'); from src.demo_agent.app import main; print(main())")
            .current_dir(&python_out),
    );
    let python_second = run_command(
        Command::new("python3")
            .arg("-c")
            .arg("import sys; sys.path.insert(0, '.'); from src.demo_agent.app import main; print(main())")
            .current_dir(&python_out),
    );

    let ts_first = run_command(
        Command::new("node")
            .arg("src/index.ts")
            .current_dir(&typescript_out),
    );
    let ts_second = run_command(
        Command::new("node")
            .arg("src/index.ts")
            .current_dir(&typescript_out),
    );

    let rust_first = run_command(
        Command::new("cargo")
            .arg("run")
            .arg("--quiet")
            .current_dir(&rust_out),
    );
    let rust_second = run_command(
        Command::new("cargo")
            .arg("run")
            .arg("--quiet")
            .current_dir(&rust_out),
    );

    let shared_first = [
        "provider=anthropic",
        "model=claude-sonnet-4-6",
        "approval_mode=acceptEdits",
        "bash_policy=bash=denied",
        "file_write_policy=file_write=allowed",
        "bash_result=bash=denied",
        "file_write_result=tool-write-ok",
        "file_edit_result=tool-write-ok edited",
        "web_fetch_result=tool-web-fetch",
        "session_id=local-main-session",
        "turn_count=1",
        "usage_entries=1",
    ];
    let shared_second = [
        "provider=anthropic",
        "model=claude-sonnet-4-6",
        "approval_mode=acceptEdits",
        "bash_policy=bash=denied",
        "file_write_policy=file_write=allowed",
        "bash_result=bash=denied",
        "file_write_result=tool-write-ok",
        "file_edit_result=tool-write-ok edited",
        "web_fetch_result=tool-web-fetch",
        "session_id=local-main-session",
        "turn_count=2",
        "usage_entries=2",
    ];

    assert_contains_all(&python_first, &shared_first);
    assert_contains_all(&ts_first, &shared_first);
    assert_contains_all(&rust_first, &shared_first);
    assert_contains_all(&python_second, &shared_second);
    assert_contains_all(&ts_second, &shared_second);
    assert_contains_all(&rust_second, &shared_second);
    assert!(python_first.contains(&expected_export_path(&python_out)));
    assert!(ts_first.contains(&expected_export_path(&typescript_out)));
    assert!(rust_first.contains(&expected_export_path(&rust_out)));
    assert!(python_second.contains(&expected_export_path(&python_out)));
    assert!(ts_second.contains(&expected_export_path(&typescript_out)));
    assert!(rust_second.contains(&expected_export_path(&rust_out)));

    assert_runtime_state_artifacts(&python_out);
    assert_runtime_state_artifacts(&typescript_out);
    assert_runtime_state_artifacts(&rust_out);

    fs::remove_dir_all(python_out).ok();
    fs::remove_dir_all(typescript_out).ok();
    fs::remove_dir_all(rust_out).ok();
}

#[test]
fn generated_typescript_core_command_registry_covers_runtime_slice() {
    let _guard = acquire_cli_test_guard();
    let repo = repo_root();
    let out = temp_dir("typescript-command-registry");
    init::run(&init::InitArgs {
        project_name: "demo-ts".to_string(),
        language: "typescript".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("typescript init works");

    let warm = run_command(Command::new("node").arg("src/index.ts").current_dir(&out));
    assert!(warm.contains("session_id=local-main-session"));

    let registry_names = run_command(
        Command::new("node")
            .arg("--input-type=module")
            .arg("-e")
            .arg(
                "import { coreCommands } from './languages/typescript/runtime/registry/coreCommands.ts'; console.log(coreCommands.map((command) => command.name).join(','));",
            )
            .current_dir(&repo),
    );
    assert_eq!(
        registry_names,
        "help,status,session,export,agents,config,doctor,context,theme,output_style,memory,plan,review,model,effort,fast,passes,usage,sandbox,permissions,files,resume,compact,diff,cost,clear,stats,tasks,tag,vim,color,keybindings,copy,terminal,exit,hooks,branch,skills,commit,release_notes,commit_push_pr,statusline,thinkback,thinkback_play,assistant,rename,pr_comments,insights,ide,rate_limit_options,upgrade,init_verifiers,create_moved_to_plugin_command,add_dir,issue"
    );

    let command_outputs = run_command(
        Command::new("node")
            .arg("--input-type=module")
            .arg("-e")
            .arg(
                "import { getCoreCommandRegistry } from './languages/typescript/runtime/registry/coreCommands.ts'; const registry = getCoreCommandRegistry(); const root = process.argv[1]; for (const name of ['help','status','session','export','agents','config','doctor','context','theme','output_style','memory','plan','review','model','effort','fast','passes','usage','sandbox','permissions','files','resume','compact','diff','cost','clear','stats','tasks','tag','vim','color','keybindings','copy','terminal','exit','hooks','branch','skills','commit','release_notes','commit_push_pr','statusline','thinkback','thinkback_play','assistant','rename','pr_comments','insights','ide','rate_limit_options','upgrade','init_verifiers','create_moved_to_plugin_command','add_dir','issue']) { console.log(`${name}:${registry[name](root)}`); }",
            )
            .arg(out.display().to_string())
            .current_dir(&repo),
    );

    assert_contains_all(
        &command_outputs,
        &[
            "help:help_ready=true",
            "status:status=ready session_id=local-main-session turn_count=1 usage_entries=1",
            "session:session_id=local-main-session",
            "agents:agents_ready=true",
            "config:provider=anthropic model=claude-sonnet-4-6 approval_mode=default",
            "doctor:doctor=ok",
            "context:prompt_digest=",
            "theme:theme_ready=true session_id=local-main-session turn_count=1",
            "output_style:output_style=default usage_entries=1 context_digest=",
            "memory:memory_entries=1 context_digest=",
            "plan:plan_ready=true session_id=local-main-session turn_count=1",
            "review:review_ready=true session_id=local-main-session usage_entries=1",
            "model:model_provider=anthropic model_name=claude-sonnet-4-6",
            "effort:effort_level=normal turn_count=1 usage_entries=1",
            "fast:fast_mode=available session_id=local-main-session context_digest=",
            "passes:passes_ready=true usage_entries=1 total_cost_micros=",
            "usage:usage_entries=1 total_cost_micros=",
            "sandbox:sandbox_ready=true approval_mode=default bash_policy=bash=approval-required",
            "permissions:approval_mode=default bash_policy=bash=approval-required file_write_policy=file_write=approval-required",
            "files:session_state=true export_state=true usage_state=true",
            "resume:resume_session_id=local-main-session resume_turn_count=3",
            "compact:compact_ready=true usage_entries=3",
            "diff:diff_ready=true context_digest=",
            "cost:cost_entries=3 total_cost_micros=",
            "clear:clearable=true retained_session_id=local-main-session",
            "stats:stats_ready=true usage_entries=3 total_cost_micros=",
            "tasks:task_count=1 active_task=session-loop turn_count=3",
            "tag:tag_ready=true session_id=local-main-session context_digest=",
            "vim:vim_mode=available",
            "color:color_ready=true session_id=local-main-session usage_entries=3",
            "keybindings:keybindings_ready=true profile=default",
            "copy:copy_ready=true session_id=local-main-session turn_count=3",
            "terminal:terminal_ready=true brand_root=.demo-ts",
            "exit:exit_ready=true session_id=local-main-session usage_entries=3",
            "hooks:hooks_ready=true instructions_present=true",
            "branch:branch_ready=true session_id=local-main-session turn_count=3",
            "skills:skills_ready=true",
            "commit:commit_ready=true session_id=local-main-session usage_entries=3",
            "release_notes:release_notes_ready=true session_id=local-main-session turn_count=3",
            "commit_push_pr:commit_push_pr_ready=true session_id=local-main-session usage_entries=3",
            "statusline:statusline_ready=true session_id=local-main-session turn_count=3",
            "thinkback:thinkback_ready=true session_id=local-main-session context_digest=",
            "thinkback_play:thinkback_play_ready=true session_id=local-main-session turn_count=3",
            "assistant:assistant_ready=true session_id=local-main-session usage_entries=3",
            "rename:rename_ready=true session_id=local-main-session turn_count=3",
            "pr_comments:pr_comments_ready=true session_id=local-main-session usage_entries=3",
            "insights:insights_ready=true context_digest=",
            "ide:ide_ready=true provider=anthropic model=claude-sonnet-4-6 session_id=local-main-session",
            "rate_limit_options:rate_limit_options_ready=true approval_mode=default bash_policy=bash=approval-required file_write_policy=file_write=approval-required",
            "upgrade:upgrade_ready=true provider=anthropic model=claude-sonnet-4-6 approval_mode=default session_id=local-main-session",
            "init_verifiers:init_verifiers_ready=true provider=anthropic model=claude-sonnet-4-6 approval_mode=default session_id=local-main-session turn_count=4",
            "create_moved_to_plugin_command:create_moved_to_plugin_command_ready=true provider=anthropic model=claude-sonnet-4-6 approval_mode=default session_id=local-main-session context_digest=",
            "add_dir:add_dir_ready=true provider=anthropic model=claude-sonnet-4-6 approval_mode=default session_id=local-main-session turn_count=4",
            "issue:issue_ready=true provider=anthropic model=claude-sonnet-4-6 approval_mode=default session_id=local-main-session context_digest=",
        ],
    );
    assert!(command_outputs.contains(&format!(
        "export:{} export_exists=true",
        expected_export_path(&out)
    )));

    fs::remove_dir_all(out).ok();
}

#[test]
fn generated_python_registry_modules_cover_runtime_slice() {
    let _guard = acquire_cli_test_guard();
    let repo = repo_root();
    let out = temp_dir("python-registry-runtime");
    init::run(&init::InitArgs {
        project_name: "demo-agent".to_string(),
        language: "python".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("python init works");

    let warm = run_command(
        Command::new("python3")
            .arg("-c")
            .arg("import sys; sys.path.insert(0, '.'); from src.demo_agent.app import main; print(main())")
            .current_dir(&out),
    );
    assert!(warm.contains("session_id=local-main-session"));

    let registry_names = run_command(
        Command::new("python3")
            .arg("-c")
            .arg(
                "import sys; sys.path.insert(0, '.'); from languages.python.runtime.registry.commands import command_registry; print(','.join(command_registry().keys()))",
            )
            .current_dir(&repo),
    );
    assert_eq!(
        registry_names,
        "status,session,export,config,doctor,context,usage,permissions,files,tasks"
    );

    let tool_names = run_command(
        Command::new("python3")
            .arg("-c")
            .arg(
                "import sys; sys.path.insert(0, '.'); from languages.python.runtime.registry.tools import tool_registry; print(','.join(tool_registry().keys()))",
            )
            .current_dir(&repo),
    );
    assert_eq!(tool_names, "bash,file_read,file_write,file_edit,web_fetch");

    let command_outputs = run_command(
        Command::new("python3")
            .arg("-c")
            .arg(
                "import sys, pathlib; sys.path.insert(0, '.'); from languages.python.runtime.registry.execution_registry import execution_registry; root = pathlib.Path(sys.argv[1]); runtime_output = sys.argv[2]; registry = execution_registry(); run = registry['run_command']; names = ['session','export','config','doctor','context','usage','permissions','files','tasks']; print('\\n'.join(f'{name}:{run(name, root, runtime_output)}' for name in names))",
            )
            .arg(out.display().to_string())
            .arg(warm.clone())
            .current_dir(&repo),
    );

    assert_contains_all(
        &command_outputs,
        &[
            "session:session_id=local-main-session",
            "config:provider=openai model=gpt-5.4 approval_mode=default",
            "doctor:doctor=ok",
            "context:context_digest=",
            "usage:usage_entries=1 total_cost_micros=",
            "permissions:approval_mode=default bash_policy=bash=approval-required file_write_policy=file_write=approval-required",
            "files:session_state=True export_state=True usage_state=True",
            "tasks:task_count=1 active_task=session-loop turn_count=1",
        ],
    );
    assert!(command_outputs.contains(&format!(
        "export:{} export_exists=True",
        expected_export_path(&out)
    )));

    fs::remove_dir_all(out).ok();
}

#[test]
fn owned_python_runtime_entrypoint_matches_generated_shell() {
    let _guard = acquire_cli_test_guard();
    let repo = repo_root();
    let out = temp_dir("python-entrypoint-runtime");
    init::run(&init::InitArgs {
        project_name: "demo-agent".to_string(),
        language: "python".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("python init works");

    let generated_output = run_command(
        Command::new("python3")
            .arg("-c")
            .arg("import sys; sys.path.insert(0, '.'); from src.demo_agent.app import main; print(main())")
            .current_dir(&out),
    );

    let owned_output = run_command(
        Command::new("python3")
            .arg("-c")
            .arg(
                "import pathlib, sys; sys.path.insert(0, '.'); from languages.python.runtime.entrypoints.main import main; project_root = pathlib.Path(sys.argv[1]); runtime_output = sys.argv[2]; print(main(project_root, runtime_output))",
            )
            .arg(out.display().to_string())
            .arg(generated_output.clone())
            .current_dir(&repo),
    );

    assert_contains_all(
        &owned_output,
        &[
            generated_output.as_str(),
            "entry_session_id=local-main-session",
            "entry_turn_count=1",
            "entry_usage_entries=1",
            "entry_prompt_present=True",
            "entry_context_present=True",
        ],
    );

    fs::remove_dir_all(out).ok();
}

#[test]
fn owned_python_query_runtime_modules_cover_session_slice() {
    let _guard = acquire_cli_test_guard();
    let repo = repo_root();
    let out = temp_dir("python-query-runtime");
    init::run(&init::InitArgs {
        project_name: "demo-agent".to_string(),
        language: "python".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("python init works");

    let first = run_command(
        Command::new("python3")
            .arg("-c")
            .arg("import sys; sys.path.insert(0, '.'); from src.demo_agent.app import main; print(main())")
            .current_dir(&out),
    );
    let second = run_command(
        Command::new("python3")
            .arg("-c")
            .arg("import sys; sys.path.insert(0, '.'); from src.demo_agent.app import main; print(main())")
            .current_dir(&out),
    );

    let owned_script = r#"
import pathlib
import sys

sys.path.insert(0, '.')

from languages.python.runtime.session_store import load_latest_session, load_named_session
from languages.python.runtime.history import load_usage_summary, load_usage_ledger
from languages.python.runtime.transcript import load_session_export
from languages.python.runtime.query import summarize_query_state
from languages.python.runtime.query_engine import QueryEngine
from languages.python.runtime.runtime import run_runtime

project_root = pathlib.Path(sys.argv[1])
runtime_output = sys.argv[2]
latest = load_latest_session(project_root)
named = load_named_session(project_root, "local-main-session")
usage = load_usage_summary(project_root)
ledger = load_usage_ledger(project_root)
export_text = load_session_export(project_root)
engine = QueryEngine(project_root)

print(f"latest:{latest['session_id']}:{latest['turn_count']}")
print(f"named:{named['session_id']}:{named['turn_count']}")
print(f"usage:{usage['usage_entries']}:{usage['total_cost_micros']}")
print(f"ledger_count:{len(ledger)}")
print(f"export_has_turn:{'- turn_count: 2' in export_text}")
print(f"query:{summarize_query_state(project_root, runtime_output)}")
print(f"engine:{engine.run(runtime_output)}")
print(f"runtime:{run_runtime(project_root, runtime_output)}")
"#;

    let owned_output = run_command(
        Command::new("python3")
            .arg("-c")
            .arg(owned_script)
            .arg(out.display().to_string())
            .arg(second.clone())
            .current_dir(&repo),
    );

    assert!(first.contains("turn_count=1"));
    assert!(second.contains("turn_count=2"));
    assert_contains_all(
        &owned_output,
        &[
            "latest:local-main-session:2",
            "named:local-main-session:2",
            "usage:2:",
            "ledger_count:2",
            "export_has_turn:True",
            "query:",
            "query_session_id=local-main-session",
            "query_turn_count=2",
            "query_usage_entries=2",
            "engine:",
            "runtime:",
        ],
    );

    fs::remove_dir_all(out).ok();
}

#[test]
fn generated_typescript_core_tool_registry_covers_runtime_slice() {
    let _guard = acquire_cli_test_guard();
    let repo = repo_root();
    let out = temp_dir("typescript-tool-registry");
    init::run(&init::InitArgs {
        project_name: "demo-ts".to_string(),
        language: "typescript".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("typescript init works");

    let config_path = out.join("boilerplate.config.ts");
    let config = fs::read_to_string(&config_path).expect("read typescript config");
    fs::write(
        &config_path,
        config.replace("approvalMode: 'default'", "approvalMode: 'acceptEdits'"),
    )
    .expect("write typescript config");

    let tool_names = run_command(
        Command::new("node")
            .arg("--input-type=module")
            .arg("-e")
            .arg(
                "import { coreTools } from './languages/typescript/runtime/registry/coreTools.ts'; console.log(coreTools.map((tool) => tool.name).join(','));",
            )
            .current_dir(&repo),
    );
    assert_eq!(tool_names, "bash,file_read,file_write,file_edit,web_fetch");

    let tool_outputs = run_command(
        Command::new("node")
            .arg("--input-type=module")
            .arg("-e")
            .arg(
                "import { getCoreToolRegistry } from './languages/typescript/runtime/registry/coreTools.ts'; const registry = getCoreToolRegistry(); const root = process.argv[1]; for (const name of ['bash','file_read','file_write','file_edit','web_fetch']) { console.log(`${name}:${registry[name](root)}`); }",
            )
            .arg(out.display().to_string())
            .current_dir(&repo),
    );

    assert_contains_all(
        &tool_outputs,
        &[
            "bash:tool-bash-ok",
            "file_read:",
            "file_write:tool-write-ok",
            "file_edit:tool-write-ok edited",
            "web_fetch:tool-web-fetch",
        ],
    );
    assert_eq!(
        fs::read_to_string(runtime_tool_file(&out)).expect("read typescript tool file"),
        "tool-write-ok edited"
    );

    fs::remove_dir_all(out).ok();
}

#[test]
fn owned_typescript_entrypoint_state_modules_match_generated_shell() {
    let _guard = acquire_cli_test_guard();
    let repo = repo_root();
    let out = temp_dir("typescript-entrypoint-runtime");
    init::run(&init::InitArgs {
        project_name: "demo-ts".to_string(),
        language: "typescript".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("typescript init works");

    let generated_output = run_command(Command::new("node").arg("src/index.ts").current_dir(&out));

    let owned_output = run_command(
        Command::new("node")
            .arg("--input-type=module")
            .arg("-e")
            .arg(
                "import { buildEntrypointSummary } from './languages/typescript/runtime/entrypoints/main.ts'; console.log(buildEntrypointSummary(process.argv[1]));",
            )
            .arg(out.display().to_string())
            .current_dir(&repo),
    );

    assert_eq!(owned_output, generated_output);
    fs::remove_dir_all(out).ok();
}

#[test]
fn rust_workspace_ownership_map_covers_minimum_archived_crates() {
    let repo = repo_root();
    let ownership_map =
        fs::read_to_string(repo.join("languages/rust/docs/workspace-ownership-map.md"))
            .expect("read rust workspace ownership map");
    let rust_pack_readme =
        fs::read_to_string(repo.join("languages/rust/README.md")).expect("read rust pack readme");

    assert!(ownership_map.contains("references/rust/Cargo.toml"));
    assert!(ownership_map.contains("references/rust/crates/runtime/Cargo.toml"));
    assert!(ownership_map.contains("references/rust/crates/commands/Cargo.toml"));
    assert!(ownership_map.contains("references/rust/crates/tools/Cargo.toml"));
    assert!(ownership_map.contains("references/rust/crates/rusty-claude-cli/Cargo.toml"));
    assert!(ownership_map.contains("owned by `languages/rust/runtime/`"));
    assert!(ownership_map.contains("deferred crate extraction"));
    assert!(rust_pack_readme.contains("docs/workspace-ownership-map.md"));
    assert!(rust_pack_readme
        .contains("archived Rust workspace root and the minimum crate ownership mapping"));
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

    let path = brand_config_path(&out);
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
fn rust_feature_add_updates_project() {
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

    assert!(out
        .join(format!("{}-plugin", brand_root_name(&out)))
        .join("plugin.json")
        .exists());
    let config = std::fs::read_to_string(brand_config_path(&out)).expect("read rust config");
    assert!(config.contains("\"enabled\": [\n      \"local-plugins\"\n    ]"));

    let doctor_message = doctor::run(&out).expect("rust doctor after feature add works");
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
        brand_doc_path(&out),
        "You are the project-local coding agent for __PROJECT_NAME__. New prompt layer.\n",
    )
    .expect("write python prompt");
    fs::write(
        brand_root(&out).join("instructions.md"),
        "# Local Instructions\n\nUpdated runtime context fragment.\n",
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
        brand_doc_path(&out),
        "You are the default coding agent for this generated project.\nPrompt override applied.\n",
    )
    .expect("write typescript prompt");
    fs::write(
        brand_root(&out).join("instructions.md"),
        "# Local Instructions\n\nUpdated TypeScript runtime context fragment.\n",
    )
    .expect("write typescript context");

    let updated = run_command(Command::new("node").arg("src/index.ts").current_dir(&out));
    assert!(updated.contains("provider=openai"));
    assert!(updated.contains("model=gpt-5.2-codex"));
    assert_ne!(initial, updated);
    fs::remove_dir_all(out).ok();
}

#[test]
fn owned_typescript_runtime_engine_matches_template_loop_output() {
    let _guard = acquire_cli_test_guard();
    let repo = repo_root();
    let template_root = repo.join("languages/typescript/template/base");

    clean_runtime_artifacts(&template_root);

    let template_output = run_command(
        Command::new("node")
            .arg("src/index.ts")
            .current_dir(&template_root),
    );

    clean_runtime_artifacts(&template_root);

    let owned_output = run_command(
        Command::new("node")
            .arg("--input-type=module")
            .arg("-e")
            .arg(
                "import { runSessionLoop, runtimeProjectRoot } from './languages/typescript/runtime/engine/sessionLoop.ts'; console.log(runSessionLoop(runtimeProjectRoot(new URL('./languages/typescript/runtime/engine/sessionLoop.ts', import.meta.url).href)));",
            )
            .current_dir(&repo),
    );

    assert_eq!(owned_output, template_output);
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

    let config_path = brand_config_path(&out);
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
        brand_doc_path(&out),
        "# Rust project instructions\n\nUpdated runtime prompt surface.\n",
    )
    .expect("write rust prompt");
    fs::write(
        brand_root(&out).join("instructions.md"),
        "# Local Instructions\n\nUpdated Rust runtime context fragment.\n",
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

    fs::remove_file(
        out.join(format!("{}-plugin", brand_root_name(&out)))
            .join("plugin.json"),
    )
    .expect("remove rust feature asset");

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

    fs::remove_file(out.join(".agents/skills/plan/SKILL.md")).expect("remove rust skill asset");

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

    fs::remove_file(brand_doc_path(&out)).expect("remove rust instruction asset");

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

    fs::remove_file(brand_root(&out).join("instructions.md"))
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

    fs::remove_file(brand_root(&out).join("agents/executor.md")).expect("remove rust agent asset");

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

    fs::remove_file(brand_root(&out).join("sessions/README.md")).expect("remove rust session root");

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

    fs::remove_file(brand_root(&out).join("instructions.md")).expect("remove rust context root");

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

    fs::remove_file(brand_root(&out).join("settings.json")).expect("remove rust usage root");

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

    let path = brand_config_path(&out);
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

    fs::remove_file(brand_root(&out).join("agents/review-agent.md"))
        .expect("remove feature-provided agent");

    let doctor_status = cli::run(vec![
        "aicd".to_string(),
        "doctor".to_string(),
        "--project".to_string(),
        out.display().to_string(),
    ]);
    assert_eq!(doctor_status, 1);

    fs::remove_dir_all(out).ok();
}

#[test]
fn doctor_detects_broken_python_runtime_artifacts() {
    let _guard = acquire_cli_test_guard();
    let out = temp_dir("python-runtime-doctor-regression");
    init::run(&init::InitArgs {
        project_name: "demo-agent".to_string(),
        language: "python".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("python init works");

    let runtime_output = run_command(
        Command::new("python3")
            .arg("-c")
            .arg("import sys; sys.path.insert(0, '.'); from src.demo_agent.app import main; print(main())")
            .current_dir(&out),
    );
    assert!(runtime_output.contains("session_id=local-main-session"));

    fs::write(
        session_path(&out, "summary.state"),
        "usage_entries=not-a-number\ntotal_cost_micros=22\n",
    )
    .expect("corrupt python summary state");

    let error =
        doctor::run(&out).expect_err("doctor should detect broken python runtime artifacts");
    assert!(error.contains("invalid runtime state") || error.contains("python runtime regression"));
    fs::remove_dir_all(out).ok();
}

#[test]
fn owned_python_doctor_detects_runtime_state_mismatch() {
    let _guard = acquire_cli_test_guard();
    let repo = repo_root();
    let out = temp_dir("python-owned-doctor-regression");
    init::run(&init::InitArgs {
        project_name: "demo-agent".to_string(),
        language: "python".to_string(),
        output: out.clone(),
        package_name: None,
        binary_name: None,
    })
    .expect("python init works");

    let runtime_output = run_command(
        Command::new("python3")
            .arg("-c")
            .arg("import sys; sys.path.insert(0, '.'); from src.demo_agent.app import main; print(main())")
            .current_dir(&out),
    );
    assert_eq!(
        python_runtime_command_output(&repo, &out, &runtime_output),
        "doctor=ok"
    );

    fs::write(
        session_path(&out, "latest.state"),
        "session_id=local-main-session\nturn_count=99\nprovider=openai\nmodel=gpt-5.4\nprompt_digest=broken\ncontext_digest=broken\nusage_entries=1\ntotal_cost_micros=1\n",
    )
    .expect("corrupt python latest state");

    let doctor_output = python_runtime_command_output(&repo, &out, &runtime_output);
    assert!(
        doctor_output.contains("invalid:session-state:turn_count")
            || doctor_output.contains("invalid:runtime-mismatch:turn_count")
            || doctor_output.contains("invalid:export:turn_count")
    );
    fs::remove_dir_all(out).ok();
}
