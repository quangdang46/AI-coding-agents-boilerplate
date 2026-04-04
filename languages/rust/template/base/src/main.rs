mod bash;
mod bash_validation;
mod bootstrap;
mod config;
mod conversation;
mod file_ops;
mod prompt;
mod providers;
mod runtime_summary;
mod session;
mod usage;

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
