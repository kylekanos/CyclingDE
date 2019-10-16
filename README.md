# CyclingDE
Test code for numerically integrating a ODE using [Rust](https://www.rust-lang.org/)

## Building & Running  
This is built by running `cargo build` in the main folder (where `cargo.toml` is located). It is then run using `cargo run`. Note that output is sent to console by default, so you would need to pipe it to the terminal afterwards (e.g., `cargo run | tee data.txt`).

By default, `cargo build` builds the code in debug mode (cf. [this rust-lang post](https://users.rust-lang.org/t/why-does-cargo-build-not-optimise-by-default/4150)). To use release (which would probably be barely noticably faster execution time but noticably slower build time), you use `cargo build --release`.
