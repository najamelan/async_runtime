#!/usr/bin/bash

# fail fast
#
set -e

# print each command before it's executed
#
set -x

export RUSTFLAGS="-D warnings"

cargo test
cargo test --features localpool
cargo test --features juliex
cargo test --features threadpool
cargo test --features async_std
cargo test --features macros
cargo test --features "macros async_std juliex"
cargo test --features "macros async_std localpool"
cargo test --features "macros async_std juliex localpool"
cargo test --features "macros juliex localpool"
cargo test --features "macros threadpool localpool"

cargo run --example localpool  --features "macros localpool"
cargo run --example juliex     --features "macros juliex"
cargo run --example threadpool --features "macros threadpool"
cargo run --example async-file --features "macros localpool"
cargo run --example attribute  --features "macros localpool"

cargo doc --no-deps --all-features


# we would like to get doc tests for the examples in the readme, but rustdoc does not
# seem to enable the features, so they don't work
#
# cargo test --features external_doc async_std juliex localpool

# --no-default-features is needed to turn of notwasm so this won't try to compile examples
# features don't work in wasm-pack, so using cargo test directly here
#
cargo test --target wasm32-unknown-unknown --no-default-features
cargo test --target wasm32-unknown-unknown --no-default-features --features macros
cargo test --target wasm32-unknown-unknown --no-default-features --features bindgen
cargo test --target wasm32-unknown-unknown --no-default-features --features "bindgen macros"

# this doesn't run it, but at least compiles it
#
cd examples/wasm/
wasm-pack build --dev --target web

