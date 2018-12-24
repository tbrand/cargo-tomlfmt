use clap::{crate_authors, crate_description, crate_name, crate_version};

type Result<T> = std::result::Result<T, failure::Error>;

mod cli;
mod fmt;

fn fmt_toml(orig: &str) -> Result<String> {
    let mut doc = orig.parse::<toml_edit::Document>()?;
    let doc_keys = doc.clone();
    let keys = doc_keys
        .as_table()
        .iter()
        .map(|(key, _)| key.to_owned())
        .collect::<Vec<String>>();

    for key in keys.iter() {
        if key == "package" {
            // we don't format the 'package' table.
            continue;
        }

        if doc[key].is_table() {
            fmt::fmt_table(&mut doc[key.as_str()].as_table_mut().unwrap())?;
        } else if doc[key].is_array_of_tables() {
            fmt::fmt_array_of_tables(&mut doc[key.as_str()].as_array_of_tables_mut().unwrap())?;
        } else if doc[key].is_value() {
            fmt::fmt_value(&mut doc[key.as_str()].as_value_mut().unwrap())?;
        }
    }

    Ok(doc.to_string())
}

fn main() -> Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    env_logger::init();

    let matches = clap::App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .subcommand(
            clap::SubCommand::with_name("tomlfmt")
                .about("It's just an alias as a subcommand of cargo.")
                .arg(cli::arg_path())
                .arg(cli::arg_dry_run())
                .arg(cli::arg_keep())
                .arg(cli::arg_create()),
        )
        .arg(cli::arg_path())
        .arg(cli::arg_dry_run())
        .arg(cli::arg_keep())
        .arg(cli::arg_create())
        .get_matches();

    let mut path = matches.value_of("path");

    if path.is_none() {
        path = matches
            .subcommand_matches("tomlfmt")
            .and_then(|m| m.value_of("path"));
    }

    let path = path.unwrap_or("Cargo.toml");

    let flag_dryrun = matches.is_present("dryrun")
        || matches
            .subcommand_matches("tomlfmt")
            .map(|m| m.is_present("dryrun"))
            .unwrap_or(false);
    let flag_keep = matches.is_present("keep")
        || matches
            .subcommand_matches("tomlfmt")
            .map(|m| m.is_present("keep"))
            .unwrap_or(false);
    let flag_create = matches.is_present("create")
        || matches
            .subcommand_matches("tomlfmt")
            .map(|m| m.is_present("create"))
            .unwrap_or(false);

    if flag_dryrun {
        log::debug!("flag: dryrun");
    }

    if flag_keep {
        log::debug!("flag: keep");
    }

    if flag_create {
        log::debug!("flag: create");

        if !flag_dryrun {
            log::warn!("flag: create can be used with dryrun");
        }
    }

    let file = std::fs::read(path)?;
    let orig = std::str::from_utf8(file.as_slice())?;
    let formatted = fmt_toml(&orig)?;

    if orig != formatted {
        if flag_dryrun {
            log::warn!("dryrun founds problems in Cargo.toml");

            if flag_create {
                log::info!("create Cargo.toml.new");
                std::fs::write("Cargo.toml.new", formatted)?;
            }

            log::warn!("exit with -1");
            std::process::exit(-1);
        } else {
            log::info!("overwrite the manifest");
            std::fs::rename(path, "Cargo.toml.bak")?;
            std::fs::write(path, formatted)?;

            if !flag_keep {
                std::fs::remove_file("Cargo.toml.bak")?;
            }
        }
    } else {
        log::info!("no problem found. good job! :)");
        std::process::exit(0);
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
}
