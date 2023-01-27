use std::path::PathBuf;

use clap::{command, Parser};

#[derive(Debug, Parser)]
#[command(version, author)]
pub struct Arguments {
    /// Path to the manifest.
    #[arg(long, short = 'p', default_value = "Cargo.toml")]
    pub path: PathBuf,

    /// Do NOT overwrite the file.
    #[arg(long, short = 'd', default_value_t = false)]
    pub dryrun: bool,

    /// Keep the original manifest as Cargo.toml.bak.
    #[arg(long, short = 'k', default_value_t = false)]
    pub keep: bool,

    /// Create a formatted manifest as Cargo.toml.new when dryrun.
    #[arg(long, short = 'c', default_value_t = false)]
    pub create: bool,
}

#[cfg(test)]
mod test {
    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        super::Arguments::command().debug_assert()
    }
}
