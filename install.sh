#!bin/bash

cargo install cargo-edit
cargo install cargo-update
cargo clean
cargo update
cargo build --release