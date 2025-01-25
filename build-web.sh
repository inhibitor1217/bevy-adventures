#!/bin/bash

# Build binaries
cargo build --release --target wasm32-unknown-unknown

# Build wasm bind for apps/bevy-marching-cubes
wasm-bindgen --no-typescript --target web \
    --out-dir ./bevy-marching-cubes/ \
    --out-name "bevy-marching-cubes" \
    ./target/wasm32-unknown-unknown/release/bevy-marching-cubes.wasm
