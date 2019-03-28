# rust-mips

[![Build status](https://travis-ci.org/Harry-Chen/rust-mips.svg?branch=master)](https://travis-ci.org/Harry-Chen/rust-mips)

Rust library for low-level abstraction of MIPS processors

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

TBD
