use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version)]
pub struct Client {
    #[command(flatten)]
    pub options: Options,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Just an alias as a subcommand of cargo.
    Tomlfmt(Options),
}

#[derive(Args, Debug, Clone)]
pub struct Options {
    #[clap(short, long, default_value = "Cargo.toml")]
    /// Path to the manifest.
    pub path: PathBuf,

    #[clap(short, long)]
    /// Do NOT overwrite the file.
    pub dryrun: bool,

    #[clap(short, long)]
    /// Keep the original manifest as Cargo.toml.bak.
    pub keep: bool,

    #[clap(short, long)]
    /// Create a formatted manifest as Cargo.toml.new when dryrun.
    pub create: bool,
}
