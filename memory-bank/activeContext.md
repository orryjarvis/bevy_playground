# Active Context: Bevy Playground

## Current Focus
The Bevy Playground now has a simple 2D game example with a movable shape that can run both natively and in a web browser via WebAssembly (WASM):

- A blue square that can be moved around the screen using arrow keys or WASD
- Proper keyboard input handling using Bevy's input system
- Visual representation using Bevy's sprite rendering
- Movement system with normalized direction and speed
- WebAssembly (WASM) support for browser-based gameplay
- Improved error handling for WASM-specific exceptions

This implementation demonstrates several key Bevy concepts:
- Entity Component System (ECS) architecture
- Input handling with `ButtonInput<KeyCode>`
- 2D rendering with `Sprite` and `Transform` components
- System organization and scheduling
- Cross-platform deployment (native and web)
- WASM-specific error handling and browser integration

## Recent Changes
- Initial project setup with Bevy 0.15.3
- Implemented a simple 2D game with a movable shape
- Updated code to use modern Bevy 0.15.3 APIs:
  - Using `Camera2d` instead of deprecated `Camera2dBundle`
  - Using `Sprite` component directly instead of deprecated `SpriteBundle`
  - Using `Color::srgb` instead of deprecated `Color::rgb`
  - Using `time.delta_secs()` for time-based movement
- Memory bank initialization for project documentation
- Added WebAssembly (WASM) support:
  - Restructured project to support both native and web builds
  - Created library crate with shared game logic
  - Added web-specific dependencies and configurations
  - Created web directory with HTML, CSS, and build scripts
  - Implemented improved error handling for WASM-specific exceptions
  - Successfully tested the web version of the game
- Simplified development environment:
  - Removed "dev / watch" mode for automatic rebuilding
  - Removed native UI dependencies from the devcontainer
  - Streamlined the development workflow to focus on browser-based testing
  - Updated documentation to reflect the simplified approach
- Enhanced browser support in the container:
  - Identified and installed all necessary dependencies for headless Chromium/Puppeteer
  - Updated devcontainer.json to automatically install browser dependencies
  - Successfully tested the game in the browser within the container
  - Documented the required dependencies for future reference
  - Modified the serve.sh script to bind to all interfaces (0.0.0.0) for external access
  - Ensured the web server is accessible from both inside the container and from the host

## Next Steps
Potential next steps for the project include:

1. **Enhance Visual Elements**:
   - Add more shapes or sprites with different appearances
   - Implement animations for the player character
   - Add a background and visual effects

2. **Expand Movement Mechanics**:
   - Add acceleration and deceleration
   - Implement collision detection with boundaries
   - Add obstacles or collectible items

3. **Organize Experiments**:
   - Create a structure for multiple demos/experiments
   - Implement a menu or selection system
   - Allow switching between different examples

4. **Add Game Mechanics**:
   - Implement a scoring system
   - Add enemies or obstacles
   - Create win/lose conditions

5. **Explore Bevy Plugins**:
   - Integrate additional Bevy plugins (e.g., bevy_ui)
   - Create custom plugins for specific functionality
   - Show plugin composition patterns

6. **Further Enhance Web Experience**:
   - Optimize WASM bundle size
   - Add mobile touch controls for web version
   - Implement progressive web app features
   - Add loading screen for WASM initialization
   - Add favicon and other web assets

## Active Decisions

### Code Organization
- Restructured from a single file to a library + binary structure
- Library (`lib.rs`) contains shared game logic for both native and web
- Binary (`main.rs`) provides the native entry point
- Web-specific entry point via wasm-bindgen
- Web directory with HTML, CSS, and build/serve scripts
- Will need to decide on further module structure as the project grows

### Development Approach
- Starting with simple examples and gradually adding complexity
- Focus on learning and demonstrating Bevy concepts
- Prioritize clear, well-documented code over advanced features
- Supporting both native and web platforms with emphasis on web-based testing
- Simplified manual build and test workflow without watch mode
- Browser-based testing as the primary development method in the container

### Technical Choices
- Using Bevy's default plugins for standard functionality
- Using modern Bevy 0.15.3 APIs and avoiding deprecated features
- Component-based design for extensibility
- ECS architecture for all game objects and logic
- WASM build process using wasm-bindgen
- Simple HTTP server for testing web version
- Improved error handling for WASM-specific exceptions
- Minimal devcontainer configuration without native UI dependencies
- Manual build and serve process for web development

## Project Insights

### Learning Focus
- Understanding Bevy's ECS implementation
- Learning input handling and movement systems
- Exploring sprite rendering and 2D graphics
- Gaining familiarity with Bevy's component system
- Learning WebAssembly deployment for Bevy games
- Handling browser-specific behaviors and exceptions

### Challenges
- Understanding Bevy's API changes and deprecations in version 0.15.3
- Implementing smooth movement with normalized vectors
- Configuring the project for both native and web targets
- Managing WASM-specific dependencies and build process
- Handling WASM-specific exceptions that are part of normal operation
- Adapting to browser-only testing in the container environment
- Managing the manual build and test workflow without watch mode
- Balancing the needs of both native and web development

### Patterns and Preferences
- Prefer explicit, well-documented code over clever shortcuts
- Use Bevy's component system for clean separation of concerns
- Follow Rust idioms and best practices
- Use normalized vectors for consistent movement speed in all directions
- Maintain cross-platform compatibility between native and web
- Handle platform-specific behaviors appropriately
- Provide clear user feedback and error handling
- Prefer browser-based testing in the container environment
- Use manual build and test process for web development
- Maintain minimal devcontainer configuration
