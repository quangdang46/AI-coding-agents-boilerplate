mod config;
mod providers;
mod runtime_summary;

use runtime_summary::load_runtime_summary;

fn run_session_loop() -> String {
    format!(
        "__PROJECT_NAME__ session loop completed {}",
        load_runtime_summary()
    )
}

fn main() {
    println!("{}", run_session_loop());
}
