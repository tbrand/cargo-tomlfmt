use env_logger::Env;

mod cli;
mod fmt;

type Result<T> = anyhow::Result<T>;

#[derive(Clone, Copy)]
struct Flags {
    dryrun: bool,
    keep: bool,
    create: bool,
}

fn fmt_toml(orig: &str) -> Result<String> {
    let mut doc = orig.parse::<toml_edit::DocumentMut>()?;
    let keys = doc
        .iter()
        .map(|(key, _)| key.to_owned())
        .collect::<Vec<String>>();

    for key in &keys {
        if key == "package" {
            // we don't format the 'package' table.
            continue;
        }

        if doc[key].is_table() {
            let table = doc[key.as_str()].as_table_mut().unwrap();
            fmt::fmt_table(table)?;
            if key == "features" {
                fmt::reorder_features_table(table);
            }
        } else if doc[key].is_array_of_tables() {
            fmt::fmt_array_of_tables(doc[key.as_str()].as_array_of_tables_mut().unwrap())?;
        } else if doc[key].is_value() {
            fmt::fmt_value(doc[key.as_str()].as_value_mut().unwrap())?;
        }
    }

    Ok(doc.to_string())
}

fn backup_path(path: &std::path::Path) -> std::path::PathBuf {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy())
        .unwrap_or_else(|| "Cargo.toml".into());
    path.with_file_name(format!("{file_name}.bak"))
}

fn new_path(path: &std::path::Path) -> std::path::PathBuf {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy())
        .unwrap_or_else(|| "Cargo.toml".into());
    path.with_file_name(format!("{file_name}.new"))
}

fn workspace_member_manifests(
    manifest_path: &std::path::Path,
    doc: &toml_edit::DocumentMut,
) -> Vec<std::path::PathBuf> {
    let Some(workspace) = doc.get("workspace").and_then(|item| item.as_table()) else {
        return Vec::new();
    };
    let Some(members) = workspace
        .get("members")
        .and_then(|item| item.as_value())
        .and_then(|value| value.as_array())
    else {
        return Vec::new();
    };

    let manifest_dir = manifest_path
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."));

    members
        .iter()
        .filter_map(|value| value.as_str())
        .map(|member| manifest_dir.join(member).join("Cargo.toml"))
        .filter(|path| path.exists())
        .collect()
}

fn format_manifest(path: &std::path::Path, flags: Flags) -> Result<bool> {
    let file = std::fs::read(path)?;
    let orig = std::str::from_utf8(file.as_slice())?;
    let formatted = fmt_toml(orig)?;

    if orig != formatted {
        if flags.dryrun {
            log::warn!("dryrun found problems in {}", path.display());

            if flags.create {
                let new_path = new_path(path);
                log::info!("create {}", new_path.display());
                std::fs::write(new_path, formatted)?;
            }

            return Ok(true);
        } else {
            log::info!("overwrite the manifest {}", path.display());
            let backup = backup_path(path);
            std::fs::rename(path, &backup)?;
            std::fs::write(path, formatted)?;

            if !flags.keep {
                std::fs::remove_file(backup)?;
            }
        }
    } else {
        log::debug!("no problem found for {}", path.display());
    }

    Ok(false)
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let app = cli::app();
    let matches = app.get_matches();

    let mut path = matches.get_one::<String>("path").map(|s| s.as_str());

    if path.is_none() {
        path = matches
            .subcommand_matches("tomlfmt")
            .and_then(|m| m.get_one::<String>("path").map(|s| s.as_str()));
    }

    let path = path.unwrap_or("Cargo.toml");

    let flag_dryrun = matches.contains_id("dryrun")
        || matches
            .subcommand_matches("tomlfmt")
            .map(|m| m.contains_id("dryrun"))
            .unwrap_or(false);
    let flag_keep = matches.contains_id("keep")
        || matches
            .subcommand_matches("tomlfmt")
            .map(|m| m.contains_id("keep"))
            .unwrap_or(false);
    let flag_create = matches.contains_id("create")
        || matches
            .subcommand_matches("tomlfmt")
            .map(|m| m.contains_id("create"))
            .unwrap_or(false);

    let flags = Flags {
        dryrun: flag_dryrun,
        keep: flag_keep,
        create: flag_create,
    };

    if flags.dryrun {
        log::debug!("flag: dryrun");
    }

    if flags.keep {
        log::debug!("flag: keep");
    }

    if flags.create {
        log::debug!("flag: create");

        if !flags.dryrun {
            log::warn!("flag: create can be used with dryrun");
        }
    }

    let mut had_changes = format_manifest(std::path::Path::new(path), flags)?;

    let file = std::fs::read(path)?;
    let doc = std::str::from_utf8(file.as_slice())?.parse::<toml_edit::DocumentMut>()?;
    for member_manifest in workspace_member_manifests(std::path::Path::new(path), &doc) {
        if format_manifest(&member_manifest, flags)? {
            had_changes = true;
        }
    }

    if flags.dryrun && had_changes {
        log::warn!("exit with -1");
        std::process::exit(-1);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_inline_table() {
        let file = std::fs::read("test/fixtures/value_after_table.toml").unwrap();
        let file_str = std::str::from_utf8(file.as_slice()).unwrap();

        assert!(fmt_toml(&file_str).is_ok());
    }

    #[test]
    fn preserve_comments() {
        let file = std::fs::read("test/fixtures/keep_comment.toml").unwrap();
        let file_str = std::str::from_utf8(file.as_slice()).unwrap();
        let formatted = fmt_toml(&file_str);

        assert!(formatted.is_ok());

        let formatted = formatted.unwrap();

        assert!(formatted.contains("# this is an inline comment"));
        assert!(formatted.contains("# this is a suffix comment"));
    }

    #[test]
    fn default_feature_first() {
        let file = std::fs::read("test/fixtures/features_unsorted.toml").unwrap();
        let file_str = std::str::from_utf8(file.as_slice()).unwrap();
        let formatted = fmt_toml(&file_str).unwrap();

        let default_pos = formatted.find("default = [\"serde\"]").unwrap();
        let serde_pos = formatted.find("serde = []").unwrap();
        let unstable_pos = formatted.find("unstable = []").unwrap();

        assert!(default_pos < serde_pos);
        assert!(default_pos < unstable_pos);
    }
}
