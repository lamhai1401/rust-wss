#!bin/bash

cargo install cargo-edit
cargo install cargo-update
cargo clean
cargo update
cargo build --release
cargo build --target # x86_64-unknown-linux-musl x86_64-unknown-linux-gnu stable-x86_64-apple-darwin

rustup show # We can use to show us the installed toolchains
rustup update # and to keep them up to date with Rust's releases.
rustup target add  x86_64-unknown-linux-musl x86_64-unknown-linux-gnu stable-x86_64-apple-darwin
