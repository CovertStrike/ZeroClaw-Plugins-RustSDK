#!/bin/bash
set -e

# Build the WASM plugin
cargo build --release --target wasm32-unknown-unknown

# Create release folder
mkdir -p release

# Copy artifacts
cp target/wasm32-unknown-unknown/release/zeroclaw_plugin_sdk.wasm release/
cp plugin.toml release/

echo "Build complete! Output in release/"
ls -lh release/
