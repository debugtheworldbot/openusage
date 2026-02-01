pub mod host_api;
pub mod manifest;
pub mod runtime;

use manifest::LoadedPlugin;
use std::path::{Path, PathBuf};

pub fn initialize_plugins(
    app_data_dir: &Path,
    resource_dir: &Path,
) -> (PathBuf, Vec<LoadedPlugin>) {
    if let Some(dev_dir) = find_dev_plugins_dir() {
        if !is_dir_empty(&dev_dir) {
            let plugins = manifest::load_plugins_from_dir(&dev_dir);
            return (dev_dir, plugins);
        }
    }

    let install_dir = app_data_dir.join("plugins");
    std::fs::create_dir_all(&install_dir).ok();

    if is_dir_empty(&install_dir) {
        let bundled_dir = resolve_bundled_dir(resource_dir);
        if bundled_dir.exists() {
            copy_dir_recursive(&bundled_dir, &install_dir);
        }
    }

    let plugins = manifest::load_plugins_from_dir(&install_dir);
    (install_dir, plugins)
}

fn find_dev_plugins_dir() -> Option<PathBuf> {
    let cwd = std::env::current_dir().ok()?;
    let direct = cwd.join("plugins");
    if direct.exists() {
        return Some(direct);
    }
    let parent = cwd.join("..").join("plugins");
    if parent.exists() {
        return Some(parent);
    }
    None
}

fn resolve_bundled_dir(resource_dir: &Path) -> PathBuf {
    let nested = resource_dir.join("resources/bundled_plugins");
    if nested.exists() {
        nested
    } else {
        resource_dir.join("bundled_plugins")
    }
}

fn is_dir_empty(path: &Path) -> bool {
    std::fs::read_dir(path)
        .map(|mut d| d.next().is_none())
        .unwrap_or(true)
}

fn copy_dir_recursive(src: &Path, dst: &Path) {
    match std::fs::read_dir(src) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let src_path = entry.path();
                let dst_path = dst.join(entry.file_name());
                if src_path.is_dir() {
                    std::fs::create_dir_all(&dst_path).ok();
                    copy_dir_recursive(&src_path, &dst_path);
                } else {
                    std::fs::copy(&src_path, &dst_path).ok();
                }
            }
        }
        Err(_) => {}
    }
}
