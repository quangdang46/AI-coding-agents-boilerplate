use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn normalize_package_name(project_name: &str) -> String {
    project_name.to_lowercase().replace(['-', ' '], "_")
}

pub fn copy_dir_with_placeholders(
    source_root: &Path,
    output_root: &Path,
    project_name: &str,
    package_name: &str,
    binary_name: &str,
) -> io::Result<()> {
    if output_root.exists() {
        fs::remove_dir_all(output_root)?;
    }
    fs::create_dir_all(output_root)?;

    for entry in walk(source_root)? {
        let relative = entry.strip_prefix(source_root).unwrap();
        let replaced = relative
            .components()
            .map(|component| {
                component
                    .as_os_str()
                    .to_string_lossy()
                    .replace("__PACKAGE_NAME__", package_name)
            })
            .collect::<Vec<_>>();
        let target = replaced
            .iter()
            .fold(output_root.to_path_buf(), |mut acc, part| {
                acc.push(part);
                acc
            });
        if entry.is_dir() {
            fs::create_dir_all(&target)?;
            continue;
        }
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut text = fs::read_to_string(&entry)?;
        text = text.replace("__PROJECT_NAME__", project_name);
        text = text.replace("__PACKAGE_NAME__", package_name);
        text = text.replace("__BINARY_NAME__", binary_name);
        fs::write(&target, text)?;
    }
    Ok(())
}

pub fn copy_tree_contents(source_root: &Path, target_root: &Path) -> io::Result<()> {
    if !source_root.exists() {
        return Ok(());
    }
    for entry in walk(source_root)? {
        let relative = entry.strip_prefix(source_root).unwrap();
        let target = target_root.join(relative);
        if entry.is_dir() {
            fs::create_dir_all(&target)?;
            continue;
        }
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(&entry, &target)?;
    }
    Ok(())
}

pub fn remove_tree_contents(source_root: &Path, target_root: &Path) -> io::Result<()> {
    if !source_root.exists() {
        return Ok(());
    }
    let mut entries = walk(source_root)?;
    entries.reverse();
    for entry in entries {
        let relative = entry.strip_prefix(source_root).unwrap();
        let target = target_root.join(relative);
        if entry.is_dir() {
            if target.exists() {
                let is_empty = fs::read_dir(&target)?.next().is_none();
                if is_empty {
                    fs::remove_dir(&target)?;
                }
            }
            continue;
        }
        if target.exists() {
            fs::remove_file(&target)?;
        }
    }
    Ok(())
}

fn walk(root: &Path) -> io::Result<Vec<PathBuf>> {
    let mut items = vec![root.to_path_buf()];
    let mut out = Vec::new();
    while let Some(path) = items.pop() {
        if path != root {
            out.push(path.clone());
        }
        if path.is_dir() {
            let mut entries = fs::read_dir(&path)?
                .map(|entry| entry.map(|e| e.path()))
                .collect::<Result<Vec<_>, _>>()?;
            entries.sort();
            for entry in entries.into_iter().rev() {
                items.push(entry);
            }
        }
    }
    Ok(out)
}
