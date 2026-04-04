use std::path::PathBuf;

use crate::commands::{doctor, feature, init, list};

pub fn run(args: Vec<String>) -> i32 {
    match dispatch(args) {
        Ok(message) => {
            println!("{message}");
            0
        }
        Err(message) => {
            eprintln!("{message}");
            1
        }
    }
}

fn dispatch(args: Vec<String>) -> Result<String, String> {
    let mut iter = args.into_iter();
    let _bin = iter.next();
    let command = iter.next().ok_or_else(|| usage("missing command"))?;
    match command.as_str() {
        "list" => {
            let topic = iter.next().ok_or_else(|| usage("missing list topic"))?;
            list::run(&topic)
        }
        "init" => {
            let project_name = iter.next().ok_or_else(|| usage("missing project name"))?;
            let mut language = None;
            let mut output = None;
            let mut package_name = None;
            let mut binary_name = None;
            let rest: Vec<String> = iter.collect();
            let mut index = 0usize;
            while index < rest.len() {
                match rest[index].as_str() {
                    "--language" => {
                        index += 1;
                        language = rest.get(index).cloned();
                    }
                    "--output" => {
                        index += 1;
                        output = rest.get(index).map(PathBuf::from);
                    }
                    "--package-name" => {
                        index += 1;
                        package_name = rest.get(index).cloned();
                    }
                    "--binary-name" => {
                        index += 1;
                        binary_name = rest.get(index).cloned();
                    }
                    "--yes" | "--dry-run" => {}
                    other => return Err(usage(&format!("unknown init flag: {other}"))),
                }
                index += 1;
            }
            let language = language.ok_or_else(|| usage("--language is required"))?;
            let output = output.unwrap_or_else(|| PathBuf::from(&project_name));
            init::run(&init::InitArgs {
                project_name,
                language,
                output,
                package_name,
                binary_name,
            })
        }
        "feature" => {
            let action = iter.next().ok_or_else(|| usage("missing feature action"))?;
            let feature_id = iter.next().ok_or_else(|| usage("missing feature id"))?;
            let rest: Vec<String> = iter.collect();
            let mut project = None;
            let mut index = 0usize;
            while index < rest.len() {
                match rest[index].as_str() {
                    "--project" => {
                        index += 1;
                        project = rest.get(index).map(PathBuf::from);
                    }
                    "--language" => {
                        index += 1;
                    }
                    "--dry-run" => {}
                    other => return Err(usage(&format!("unknown feature flag: {other}"))),
                }
                index += 1;
            }
            let args = feature::FeatureArgs {
                feature_id,
                project: project.unwrap_or_else(|| PathBuf::from(".")),
            };
            match action.as_str() {
                "add" => feature::add(&args),
                other => Err(usage(&format!("unknown feature action: {other}"))),
            }
        }
        "doctor" => {
            let rest: Vec<String> = iter.collect();
            let mut project = None;
            let mut index = 0usize;
            while index < rest.len() {
                match rest[index].as_str() {
                    "--project" => {
                        index += 1;
                        project = rest.get(index).map(PathBuf::from);
                    }
                    "--language" => {
                        index += 1;
                    }
                    other => return Err(usage(&format!("unknown doctor flag: {other}"))),
                }
                index += 1;
            }
            let project = project.unwrap_or_else(|| PathBuf::from("."));
            doctor::run(&project)
        }
        other => Err(usage(&format!("unknown command: {other}"))),
    }
}

fn usage(message: &str) -> String {
    format!(
        "{message}\nusage:\n  aicd list <languages|templates|features|prompt-packs>\n  aicd init <project-name> --language python [--output <dir>] [--package-name <name>] [--binary-name <name>] [--yes]\n  aicd feature add <feature-id> --project <dir>\n  aicd doctor --project <dir>"
    )
}
