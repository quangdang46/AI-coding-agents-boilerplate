mod bash;
mod bash_validation;
mod bootstrap;
mod config;
mod conversation;
mod file_ops;
mod permission_enforcer;
mod permissions;
mod prompt;
mod providers;
mod recovery_recipes;
mod runtime_summary;
mod sandbox;
mod session;
mod stale_branch;
mod trust_resolver;
mod usage;
mod worker_boot;

use bootstrap::run_session_loop;

fn main_output() -> String {
    format!(
        "__PROJECT_NAME__ session loop completed {}",
        run_session_loop()
    )
}

fn main() {
    println!("{}", main_output());
}
