# cargo-tomlfmt

Formatting Cargo.toml.

[![Build Status](https://travis-ci.com/tbrand/cargo-tomlfmt.svg?branch=master)](https://travis-ci.com/tbrand/cargo-tomlfmt)
[![Build status](https://ci.appveyor.com/api/projects/status/yy9gk79t7jl0j8e0?svg=true)](https://ci.appveyor.com/project/tbrand/cargo-tomlfmt)
[![Crates.io](https://img.shields.io/crates/v/cargo-tomlfmt.svg)](https://crates.io/crates/cargo-tomlfmt)

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
> cargo tomlfmt --help

FLAGS:
-p, --path <PATH>  Path to the manifest [default: Cargo.toml]
-d, --dryrun       Do NOT overwrite the file
-k, --keep         Keep the original manifest as Cargo.toml.bak
-c, --create       Create a formatted manifest as Cargo.toml.new when dryrun
-h, --help         Print help
-V, --version      Print version
```
