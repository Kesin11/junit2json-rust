#!/usr/bin/env bash

# Setup development tools for devcontainer

# nextest
curl -LsSf https://get.nexte.st/latest/linux | tar zxf - -C ${CARGO_HOME:-~/.cargo}/bin

# cross
cargo install cross

# wasm, wasi
npm install -g wasm-pack
cargo install cargo-wasi
