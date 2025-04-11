# Technical Context: Bevy Playground

## Technologies Used

### Core Technologies
- **Rust**: Systems programming language (Edition 2024)
- **Bevy**: Game engine and ECS framework (v0.15.3)
- **Cargo**: Rust package manager and build system

### Development Environment
- **VS Code**: Primary IDE with DevContainer support
- **DevContainer**: Docker-based development environment
  - Base image: `mcr.microsoft.com/devcontainers/rust:latest`
  - Graphics support: X11/Wayland configuration
  - Additional dependencies installed for Bevy development

### Dependencies
The project has minimal dependencies, focusing on the Bevy ecosystem:

```toml
[dependencies]
bevy = "0.15.3"
```

### Build Configuration
The project uses custom optimization settings for development:

```toml
# Enable a small amount of optimization in the dev profile.
[profile.dev]
opt-level = 1

# Enable a large amount of optimization in the dev profile for dependencies.
[profile.dev.package."*"]
opt-level = 3
```

This configuration:
- Applies level 1 optimization to the project code in development mode
- Applies level 3 optimization to dependencies in development mode
- Balances build speed with runtime performance for a better development experience

## Technical Constraints

### Performance Considerations
- Bevy is designed for performance, but complex scenes can be resource-intensive
- Development container may have limited GPU capabilities compared to native
- Optimization settings help balance development experience with performance

### Platform Support
- The development environment is configured for Linux with X11/Wayland support
- Additional configuration would be needed for Windows or macOS development
- Cross-platform considerations should be kept in mind for any platform-specific code

### Bevy Version Constraints
- Using Bevy 0.15.3, which has specific API patterns and features
- Future Bevy versions may introduce breaking changes
- Code should follow current Bevy best practices while being adaptable to future changes

## Development Setup

### Required System Dependencies
The development container installs these dependencies:
```
g++ pkg-config libx11-dev libx11-xcb-dev libasound2-dev libudev-dev 
libxkbcommon-x11-0 libxcursor-dev libxrandr-dev libxi-dev libxinerama-dev 
mesa-vulkan-drivers lld clang
```

These provide:
- Graphics support (X11, Vulkan)
- Audio support (ALSA)
- Input handling
- Additional compilation tools (LLVM, Clang)

### Running the Project
To run the project:
```bash
cd bevy_playground
cargo run
```

For release builds with better performance:
```bash
cargo run --release
```

### Development Workflow
1. Make code changes in the `src` directory
2. Run the application with `cargo run`
3. Observe output in the terminal (currently text-based)
4. Iterate on changes

## Tool Usage Patterns

### Cargo Commands
- `cargo build`: Compile the project
- `cargo run`: Build and run the project
- `cargo check`: Check for errors without building
- `cargo test`: Run tests (when added)
- `cargo clippy`: Run the Rust linter for code quality checks

### VS Code Integration
- The project is configured for VS Code with DevContainer support
- Recommended extensions for Rust development:
  - rust-analyzer: For code intelligence
  - CodeLLDB: For debugging
  - crates: For dependency management

### Git Workflow
Standard Git workflow with `.gitignore` configured for Rust projects:
- Ignores `target/` directory with build artifacts
- Ignores backup files from rustfmt
- Ignores debugging information files
