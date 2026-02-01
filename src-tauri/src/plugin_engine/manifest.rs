use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginManifest {
    pub schema_version: u32,
    pub id: String,
    pub name: String,
    pub version: String,
    pub entry: String,
}

#[derive(Debug, Clone)]
pub struct LoadedPlugin {
    pub manifest: PluginManifest,
    pub plugin_dir: PathBuf,
    pub entry_script: String,
}

pub fn load_plugins_from_dir(plugins_dir: &std::path::Path) -> Vec<LoadedPlugin> {
    let mut plugins = Vec::new();
    let entries = match std::fs::read_dir(plugins_dir) {
        Ok(e) => e,
        Err(_) => return plugins,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let manifest_path = path.join("plugin.json");
        if !manifest_path.exists() {
            continue;
        }
        if let Ok(p) = load_single_plugin(&path) {
            plugins.push(p);
        }
    }

    plugins.sort_by(|a, b| a.manifest.id.cmp(&b.manifest.id));
    plugins
}

fn load_single_plugin(
    plugin_dir: &std::path::Path,
) -> Result<LoadedPlugin, Box<dyn std::error::Error>> {
    let manifest_path = plugin_dir.join("plugin.json");
    let manifest_text = std::fs::read_to_string(&manifest_path)?;
    let manifest: PluginManifest = serde_json::from_str(&manifest_text)?;

    let entry_path = plugin_dir.join(&manifest.entry);
    let entry_script = std::fs::read_to_string(&entry_path)?;

    Ok(LoadedPlugin {
        manifest,
        plugin_dir: plugin_dir.to_path_buf(),
        entry_script,
    })
}
