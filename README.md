# rust-mips

[![Build status](https://travis-ci.org/Harry-Chen/rust-mips.svg?branch=master)](https://travis-ci.org/Harry-Chen/rust-mips)
[![Crate version](https://img.shields.io/crates/v/mips.svg)](https://crates.io/crates/mips)

Rust library for low-level abstraction of MIPS32 processors

## License

This project is licensed under the terms of the MIT license.

## Build

First install `rustup`, then execute:

```bash
rustup component add rust-src
cargo install cargo-xbuild
make # for debug version
make mode=release # for release version
```

Use `make clean` to remove all build outputs, `make dist-clean` to also remove cached cargos.

## Documentation

See [doc.rs/rust-mips](<https://docs.rs/mips>) for documentation, or run `cargo doc` to generate it locally.
