# cargo-tomlfmt

Formatting Cargo.toml.

[![Build Status](https://travis-ci.com/tbrand/cargo-tomlfmt.svg?branch=master)](https://travis-ci.com/tbrand/cargo-tomlfmt)
[![Build status](https://ci.appveyor.com/api/projects/status/yy9gk79t7jl0j8e0?svg=true)](https://ci.appveyor.com/project/tbrand/cargo-tomlfmt)

## Installation
```bash
cargo install cargo-tomlfmt
```

## Usage
```bash
cargo tomlfmt
```

## Options
```bash
> cargo tomlfmt

FLAGS:
-c, --create     Create a formatted manifest as Cargo.toml.new when dryrun.
-d, --dryrun     Do NOT overwrite the file.
-h, --help       Prints help information
-k, --keep       Keep the original manifest as Cargo.toml.bak.
-p, --path       Path of the manifest. (default is Cargo.toml)
-V, --version    Prints version information
```

## Configuration
If a `tomlfmt.toml` file exists next to the manifest, it will be loaded to
control formatting behavior.

Example:
```toml
sort_keys = false
```

You can also nest options under a `[tomlfmt]` table:
```toml
[tomlfmt]
sort_keys = false
```
