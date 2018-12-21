use clap::Arg;

pub fn arg_path<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("path")
        .long("path")
        .short("p")
        .help("Path to the manifest. (default is Cargo.toml)")
        .takes_value(true)
        .multiple(false)
}

pub fn arg_dry_run<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("dryrun")
        .long("dryrun")
        .short("d")
        .help("Do NOT overwrite the file.")
        .takes_value(false)
        .multiple(false)
}

pub fn arg_keep<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("keep")
        .long("keep")
        .short("k")
        .help("Keep the original manifest as Cargo.toml.bak.")
        .takes_value(false)
        .multiple(false)
}

pub fn arg_create<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("create")
        .long("create")
        .short("c")
        .help("Create a formatted manifest as Cargo.toml.new when dryrun.")
        .takes_value(false)
        .multiple(false)
}
