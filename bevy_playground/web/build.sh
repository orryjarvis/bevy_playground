#!/bin/bash
set -e

# Navigate to the project root
cd "$(dirname "$0")/.."

# Check if wasm-bindgen-cli is installed
if ! command -v wasm-bindgen &> /dev/null; then
    echo "Installing wasm-bindgen-cli..."
    cargo install wasm-bindgen-cli
fi

# Check if wasm32-unknown-unknown target is installed
if ! rustup target list | grep -q "wasm32-unknown-unknown (installed)"; then
    echo "Adding wasm32-unknown-unknown target..."
    rustup target add wasm32-unknown-unknown
fi

# Create dist directory if it doesn't exist
mkdir -p web/dist

# Build the project for wasm
echo "Building project for wasm32-unknown-unknown target..."
cargo build --target wasm32-unknown-unknown --release

# Process the wasm file with wasm-bindgen
echo "Processing wasm with wasm-bindgen..."
wasm-bindgen --out-dir web/dist --target web target/wasm32-unknown-unknown/release/bevy_playground.wasm

echo "Build complete! Files are in the web/dist directory."
echo "To serve the files, run: cd web && python3 -m http.server"
