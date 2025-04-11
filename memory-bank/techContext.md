# Technical Context: Bevy Playground

## Technologies Used

### Core Technologies
- **Rust**: Systems programming language (Edition 2024)
- **Bevy**: Game engine and ECS framework (v0.15.3)
- **Cargo**: Rust package manager and build system
- **WebAssembly (WASM)**: Binary instruction format for a stack-based virtual machine
- **wasm-bindgen**: Facilitating high-level interactions between Wasm modules and JavaScript
- **HTML/CSS/JavaScript**: Web technologies for the browser interface

### Development Environment
- **VS Code**: Primary IDE with DevContainer support
- **DevContainer**: Docker-based development environment
  - Base image: `mcr.microsoft.com/devcontainers/rust:latest`
  - No additional dependencies for native UI support in the container
  - Browser-only UI support when running in the container
  - Includes necessary dependencies for headless Chromium/Puppeteer
  - Configured to install browser dependencies automatically on container creation

### Dependencies
The project has dependencies focused on the Bevy ecosystem and WASM support:

```toml
[dependencies]
bevy = "0.15.3"
bevy_input = "0.15.3"

# Dependencies for WASM support
[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
web-sys = { version = "0.3", features = [
    "Document",
    "Window",
    "Element",
    "HtmlCanvasElement",
] }
console_error_panic_hook = "0.1.7"
```

The `console_error_panic_hook` dependency is particularly important for WASM builds as it ensures that Rust panics are properly displayed in the browser console, making debugging easier.

### Build Configuration
The project uses custom optimization settings and is configured for both native and WASM builds:

```toml
# Enable optimization in debug mode
[profile.dev]
opt-level = 1

# Enable high optimizations for dependencies (incl. Bevy), but not for our code:
[profile.dev.package."*"]
opt-level = 3

# Configure the WASM build
[lib]
crate-type = ["cdylib", "rlib"]
```

This configuration:
- Applies level 1 optimization to the project code in development mode
- Applies level 3 optimization to dependencies in development mode
- Balances build speed with runtime performance for a better development experience
- Configures the library to be built as both a dynamic library (for WASM) and a Rust library (for native)

The `cdylib` crate type is essential for WASM builds as it produces a dynamic library without Rust-specific metadata, suitable for use in non-Rust environments like browsers.

## Technical Constraints

### Performance Considerations
- Bevy is designed for performance, but complex scenes can be resource-intensive
- Development container may have limited GPU capabilities compared to native
- Optimization settings help balance development experience with performance
- WASM performance may differ from native performance, especially for complex scenes
- Browser-specific limitations may affect WASM performance and capabilities
- JavaScript garbage collection can cause occasional stutters in WASM applications

### Platform Support
- The development environment is configured for Linux with X11/Wayland support
- Additional configuration would be needed for Windows or macOS development
- Cross-platform considerations should be kept in mind for any platform-specific code
- Web version supports any modern browser with WebAssembly support
- Mobile browsers may have different performance characteristics and input methods
- Touch input would require additional handling for mobile web support

### Bevy Version Constraints
- Using Bevy 0.15.3, which has specific API patterns and features
- Future Bevy versions may introduce breaking changes
- Code should follow current Bevy best practices while being adaptable to future changes
- WASM support in Bevy may evolve over time, requiring updates to the web configuration
- Bevy's WASM support uses exceptions for control flow, which can appear as errors in the browser console

## Development Setup

### Required System Dependencies
The development container no longer installs additional dependencies for native UI support. 
Native UI dependencies are only needed when running outside the container.


For WASM development, additional tools are required:
```
wasm-bindgen-cli
rustup target add wasm32-unknown-unknown
```

The `wasm-bindgen-cli` tool is essential for generating the JavaScript bindings that allow the browser to interact with the compiled WASM module.

### Running the Project

#### Native Version
To run the native version:
```bash
cd bevy_playground
cargo run
```

For release builds with better performance:
```bash
cargo run --release
```

#### Web Version
To build and run the web version:
```bash
cd bevy_playground
# Build the WASM version
./web/build.sh

# Serve the web files
./web/serve.sh
```

Then open a browser and navigate to `http://localhost:8000`.

The build script performs these key steps:
1. Compiles the Rust code to WASM targeting `wasm32-unknown-unknown`
2. Processes the WASM file with `wasm-bindgen` to generate JavaScript bindings
3. Outputs the processed files to the web directory

The serve script starts a Python HTTP server that binds to all interfaces (0.0.0.0), making it accessible from both inside the container and from the host machine. This allows for testing the web version of the game from any browser, whether it's running inside the container or on the host system.

### Development Workflow

#### Native Development
1. Make code changes in the `src` directory
2. Run the application with `cargo run`
3. Observe output in the window
4. Iterate on changes

#### Web Development
1. Make code changes in the `src` directory
2. Build the WASM version with `./web/build.sh`
3. Serve the files with `./web/serve.sh`
4. Test in a browser at `http://localhost:8000`
5. Use browser developer tools to debug any issues
6. Make changes and repeat the build and test process
   (Note: The automatic watch/rebuild mode has been removed)

#### Debugging WASM
1. Open browser developer tools (F12 in most browsers)
2. Check the Console tab for any error messages
3. Use the Network tab to verify WASM file loading
4. Use the Performance tab to identify any performance bottlenecks

## Tool Usage Patterns

### Cargo Commands
- `cargo build`: Compile the project
- `cargo run`: Build and run the project
- `cargo check`: Check for errors without building
- `cargo test`: Run tests (when added)
- `cargo clippy`: Run the Rust linter for code quality checks
- `cargo build --target wasm32-unknown-unknown`: Build for WASM target
- Note: cargo-watch is no longer used for development workflow

### WASM Tools
- `wasm-bindgen`: Generate JavaScript bindings for Rust WASM modules
- `wasm-bindgen-cli`: Command-line interface for wasm-bindgen
- Python's HTTP server: For serving the web files locally

### Web Development Tools
- Browser developer tools (F12): For debugging WASM applications
- Network inspector: For monitoring WASM file loading
- Console: For viewing Rust panic messages and other logs
- Performance profiler: For identifying performance bottlenecks

### VS Code Integration
- The project is configured for VS Code with DevContainer support
- Recommended extensions for Rust development:
  - rust-analyzer: For code intelligence
  - CodeLLDB: For debugging
  - crates: For dependency management
- Browser developer tools for debugging the web version

### Git Workflow
Standard Git workflow with `.gitignore` configured for Rust projects:
- Ignores `target/` directory with build artifacts
- Ignores backup files from rustfmt
- Ignores debugging information files
