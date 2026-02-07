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
    let doc = raw.parse::<toml_edit::DocumentMut>()?;
    let mut config = FormatConfig::default();

    let table = doc
        .get("tomlfmt")
        .and_then(|item| item.as_table())
        .unwrap_or_else(|| doc.as_table());

    if let Some(value) = table
        .get("sort_keys")
        .and_then(|item| item.as_value())
        .and_then(|value| value.as_bool())
    {
        config.sort_keys = value;
    }

    Ok(config)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    fn temp_dir(prefix: &str) -> PathBuf {
        let mut dir = std::env::temp_dir();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        dir.push(format!("cargo-tomlfmt-{prefix}-{nanos}-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn load_config_defaults_when_missing() {
        let dir = temp_dir("missing");
        let manifest = dir.join("Cargo.toml");
        fs::write(&manifest, "[package]\nname = \"demo\"\n").unwrap();

        let config = load_config(&manifest).unwrap();
        assert!(config.sort_keys);

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn load_config_from_root_table() {
        let dir = temp_dir("root");
        let manifest = dir.join("Cargo.toml");
        fs::write(&manifest, "[package]\nname = \"demo\"\n").unwrap();
        fs::write(dir.join("tomlfmt.toml"), "sort_keys = false\n").unwrap();

        let config = load_config(&manifest).unwrap();
        assert!(!config.sort_keys);

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn load_config_from_tomlfmt_table() {
        let dir = temp_dir("table");
        let manifest = dir.join("Cargo.toml");
        fs::write(&manifest, "[package]\nname = \"demo\"\n").unwrap();
        fs::write(
            dir.join("tomlfmt.toml"),
            "[tomlfmt]\nsort_keys = false\n",
        )
        .unwrap();

        let config = load_config(&manifest).unwrap();
        assert!(!config.sort_keys);

        fs::remove_dir_all(&dir).unwrap();
    }
}
