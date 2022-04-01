#!/bin/bash
set -e

RUSTFLAGS='-A dead_code -A unused_variables -C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/*.wasm ./res/

