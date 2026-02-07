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

## Behavior
- Formats all top-level tables, including `[package]`.
- In `[features]`, keeps `default` first and sorts the rest alphabetically.
- In `[workspace]`, formats `members` as a multi-line array for readability.
- When run on a workspace root manifest, also formats each member's `Cargo.toml`.

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

When formatting a workspace root, the same `.bak` / `.new` behavior applies
to each member manifest.

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
