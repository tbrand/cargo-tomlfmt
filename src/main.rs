use clap::Parser;
use cli::Arguments;

mod cli;
mod fmt;

type Result<T> = anyhow::Result<T>;

fn fmt_toml(orig: &str) -> Result<String> {
    let mut doc = orig.parse::<toml_edit::Document>()?;
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
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    env_logger::init();

    let args = Arguments::parse();
    dbg!(&args);

    let file = std::fs::read(&args.path)?;
    let orig = std::str::from_utf8(file.as_slice())?;
    let formatted = fmt_toml(orig)?;

    if orig != formatted {
        if args.dryrun {
            log::warn!("dryrun founds problems in Cargo.toml");

            if args.create {
                log::info!("create Cargo.toml.new");
                std::fs::write("Cargo.toml.new", formatted)?;
            }

            log::warn!("exit with -1");
            std::process::exit(-1);
        } else {
            log::info!("overwrite the manifest");
            std::fs::rename(&args.path, "Cargo.toml.bak")?;
            std::fs::write(args.path, formatted)?;

            if !args.keep {
                std::fs::remove_file("Cargo.toml.bak")?;
            }
        }
    } else {
        log::debug!("no problem found. good job! :)");
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
