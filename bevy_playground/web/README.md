# Bevy Playground - Web (WASM) Version

This directory contains the files needed to build and run the Bevy Playground in a web browser using WebAssembly (WASM).

## Prerequisites

- Rust and Cargo (latest stable version)
- wasm-bindgen-cli (`cargo install wasm-bindgen-cli`)
- wasm32-unknown-unknown target (`rustup target add wasm32-unknown-unknown`)
- cargo-watch (`cargo install cargo-watch`) - for development workflow
- Python (for the local development server)

## Development Workflow

For the best development experience with automatic rebuilding:

```bash
./dev.sh
```

This script will:
1. Install cargo-watch if not already installed
2. Build the WASM version initially
3. Start a background HTTP server
4. Watch for file changes and automatically rebuild

You can then open your browser to http://localhost:8000 and see your changes reflected when you modify the code.

Alternatively, you can use the Cargo aliases defined in `.cargo/config.toml`:

```bash
# From the project root
cargo wasm-dev  # Watches for changes, builds WASM, and serves
cargo dev       # Standard development for native builds
```

## Building the WASM Version Manually

To build the WASM version without the development workflow:

```bash
./build.sh
```

This script will:
1. Check if the necessary tools are installed
2. Build the project for the wasm32-unknown-unknown target
3. Process the WASM file with wasm-bindgen
4. Output the processed files to the `dist` directory

## Running the Server Manually

To start the server without the development workflow:

```bash
./serve.sh
```

This will start a Python HTTP server on port 8000 in the background. Open your browser and navigate to:

```
http://localhost:8000
```

To stop the server:

```bash
pkill -f "python3 -m http.server"
```

## Directory Structure

- `index.html`: The HTML file that loads the WASM module
- `style.css`: CSS styles for the web page
- `build.sh`: Script to build the WASM version
- `serve.sh`: Script to run a local development server in the background
- `dev.sh`: Script to run the development workflow with auto-rebuilding
- `dist/`: Directory containing generated WASM files (gitignored)
  - `bevy_playground.js`: Generated JavaScript glue code
  - `bevy_playground_bg.wasm`: The compiled WASM module
  - Other generated files

## Controls

- Use arrow keys or WASD to move the blue square

## Troubleshooting

If you encounter any issues:

1. Make sure you have the latest version of wasm-bindgen-cli installed
2. Check that the wasm32-unknown-unknown target is installed
3. Ensure that the Rust code compiles successfully for the native target
4. Check the browser console for any JavaScript errors
5. If you see an error about "Using exceptions for control flow", this is normal behavior and not an actual error
6. If the server is already running, you may need to kill it with `pkill -f "python3 -m http.server"`
