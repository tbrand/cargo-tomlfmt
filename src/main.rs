mod cli;
mod fmt;

use clap::Parser;
use env_logger::Env;
use std::process::exit;

use cli::{Client, Command};

type Result<T> = anyhow::Result<T>;

fn fmt_toml<T: AsRef<str>>(orig: T) -> Result<String> {
    let mut doc = orig.as_ref().parse::<toml_edit::Document>()?;
    let doc_keys = doc.clone();
    let keys = doc_keys
        .as_table()
        .iter()
        .map(|(key, _)| key.to_owned())
        .collect::<Vec<String>>();

    for key in &keys {
        if key == "package" {
            // we don't format the 'package' table.
            continue;
        }

        if doc[key].is_table() {
            fmt::fmt_table(doc[key.as_str()].as_table_mut().unwrap())?;
        } else if doc[key].is_array_of_tables() {
            fmt::fmt_array_of_tables(doc[key.as_str()].as_array_of_tables_mut().unwrap())?;
        } else if doc[key].is_value() {
            fmt::fmt_value(doc[key.as_str()].as_value_mut().unwrap())?;
        }
    }

    Ok(doc.to_string())
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let client = Client::parse();
    let options = client.command.map_or(client.options, |c| match c {
        Command::Tomlfmt(options) => options,
    });

    if options.dryrun {
        log::debug!("flag: dryrun");
    }

    if options.keep {
        log::debug!("flag: keep");
    }

    if options.create {
        log::debug!("flag: create");

        if !options.dryrun {
            log::warn!("flag: create can only be used with dryrun");
        }
    }

    if !options.path.exists() {
        log::error!("{} does not exist", options.path.display());
        exit(-1);
    }
    if !options.path.is_file() {
        log::error!("{} is not a file", options.path.display());
        exit(-1);
    }
    let file_path = &options.path;
    let file_name = String::from(file_path.file_name().unwrap().to_str().unwrap());

    let orig = std::fs::read_to_string(file_path)?;
    let formatted = fmt_toml(&orig)?;

    if orig != formatted {
        if options.dryrun {
            log::warn!("dryrun found problems in Cargo.toml");

            if options.create {
                let mut new_file = file_path.clone();
                new_file.set_file_name(file_name + ".new");
                log::info!("create {}", new_file.display());
                std::fs::write(new_file, formatted)?;
            }

            log::warn!("exit with -1");
            exit(-1);
        } else {
            log::info!("overwrite the manifest");
            if options.keep {
                let mut bak_file = file_path.clone();
                bak_file.set_file_name(file_name + ".bak");
                std::fs::rename(file_path, bak_file)?;
            }
            std::fs::write(file_path, formatted)?;
        }
    } else {
        log::debug!("no problem found. good job! :)");
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
