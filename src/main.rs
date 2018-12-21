use clap::{crate_authors, crate_description, crate_name, crate_version};

type Result<T> = std::result::Result<T, failure::Error>;

mod cli;
mod re_define;

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
    let manifest: re_define::TomlManifest = toml::from_slice(file.as_slice())?;
    let pretty = toml::to_string_pretty(&manifest)?;

    if orig != pretty {
        if flag_dryrun {
            log::warn!("dryrun founds problems in Cargo.toml");

            if flag_create {
                log::info!("create Cargo.toml.new");
                std::fs::write("Cargo.toml.new", pretty)?;
            }

            log::warn!("exit with -1");
            std::process::exit(-1);
        } else {
            log::info!("overwrite the manifest");
            std::fs::rename(path, "Cargo.toml.bak")?;
            std::fs::write(path, pretty)?;

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
