#!/usr/bin/env bash
set -x

# Setup development tools for devcontainer

# nextest
curl -LsSf https://get.nexte.st/latest/linux | tar zxf - -C ${CARGO_HOME:-~/.cargo}/bin

# wasm, wasi build tools.
npm install -g wasm-pack
cargo install cargo-wasi
# wasm runtime (wasmtime)
curl https://wasmtime.dev/install.sh -sSf | bash
