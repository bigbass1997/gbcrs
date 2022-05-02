[![License: BSD 2-Clause](https://img.shields.io/badge/License-BSD%202--Clause-blue)](LICENSE)
### Description
Currently intended only for personal research, this is a WIP cycle-accurate GB/C emulator written in Rust.

### Building
If you wish to build from source, for your own system, Rust is integrated with the `cargo` build system. To install Rust and `cargo`, just follow [these instructions](https://doc.rust-lang.org/cargo/getting-started/installation.html). Once installed, while in the project directory, run `cargo build --release` to build, or use `cargo run --release` to run directly. The built binary will be available at `./target/release/gbcrs`

To cross-compile builds for other operating systems, you can use [rust-embedded/cross](https://github.com/rust-embedded/cross).