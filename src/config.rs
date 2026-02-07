use crate::Result;
use std::path::{Path, PathBuf};

#[derive(Clone, Copy, Debug)]
pub struct FormatConfig {
    pub sort_keys: bool,
}

impl Default for FormatConfig {
    fn default() -> Self {
        Self { sort_keys: true }
    }
}

pub fn config_path(manifest_path: &Path) -> PathBuf {
    manifest_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("tomlfmt.toml")
}

pub fn load_config(manifest_path: &Path) -> Result<FormatConfig> {
    let path = config_path(manifest_path);
    if !path.exists() {
        return Ok(FormatConfig::default());
    }

    let raw = std::fs::read_to_string(&path)?;
    let doc = raw.parse::<toml_edit::Document>()?;
    let mut config = FormatConfig::default();

    let table = if doc["tomlfmt"].is_table() {
        doc["tomlfmt"].as_table().unwrap()
    } else {
        doc.as_table()
    };

    if let Some(value) = table
        .get("sort_keys")
        .and_then(|item| item.as_value())
        .and_then(|value| value.as_bool())
    {
        config.sort_keys = value;
    }

    Ok(config)
}
