use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, Command};

pub fn app() -> Command {
    clap::Command::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .subcommand(
            clap::Command::new("tomlfmt")
                .about("It's just an alias as a subcommand of cargo.")
                .arg(arg_path())
                .arg(arg_dry_run())
                .arg(arg_keep())
                .arg(arg_create()),
        )
        .arg(arg_path())
        .arg(arg_dry_run())
        .arg(arg_keep())
        .arg(arg_create())
}

pub fn arg_path() -> Arg {
    Arg::new("path")
        .long("path")
        .short('p')
        .help("Path to the manifest. (default is Cargo.toml)")
        .action(clap::ArgAction::Set)
}

pub fn arg_dry_run() -> Arg {
    Arg::new("dryrun")
        .long("dryrun")
        .short('d')
        .help("Do NOT overwrite the file.")
        .action(clap::ArgAction::SetTrue)
}

pub fn arg_keep() -> Arg {
    Arg::new("keep")
        .long("keep")
        .short('k')
        .help("Keep the original manifest as Cargo.toml.bak.")
        .action(clap::ArgAction::SetTrue)
}

pub fn arg_create() -> Arg {
    Arg::new("create")
        .long("create")
        .short('c')
        .help("Create a formatted manifest as Cargo.toml.new when dryrun.")
        .action(clap::ArgAction::SetTrue)
}

#[cfg(test)]
mod test {
    #[test]
    fn verify_app() {
        super::app().debug_assert();
    }
}
