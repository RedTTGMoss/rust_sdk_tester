#!/bin/bash
rustup target add wasm32-wasip1
cargo build --target wasm32-wasip1
cp target/wasm32-wasip1/debug/*.wasm .