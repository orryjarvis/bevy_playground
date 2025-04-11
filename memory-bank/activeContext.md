# Active Context: Bevy Playground

## Current Focus
The Bevy Playground now has a simple 2D game example with a movable shape:

- A blue square that can be moved around the screen using arrow keys or WASD
- Proper keyboard input handling using Bevy's input system
- Visual representation using Bevy's sprite rendering
- Movement system with normalized direction and speed

This implementation demonstrates several key Bevy concepts:
- Entity Component System (ECS) architecture
- Input handling with `ButtonInput<KeyCode>`
- 2D rendering with `Sprite` and `Transform` components
- System organization and scheduling

## Recent Changes
- Initial project setup with Bevy 0.15.3
- Implemented a simple 2D game with a movable shape
- Updated code to use modern Bevy 0.15.3 APIs:
  - Using `Camera2d` instead of deprecated `Camera2dBundle`
  - Using `Sprite` component directly instead of deprecated `SpriteBundle`
  - Using `Color::srgb` instead of deprecated `Color::rgb`
  - Using `time.delta_secs()` for time-based movement
- Memory bank initialization for project documentation

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

## Active Decisions

### Code Organization
- Currently using a single file (`main.rs`) for simplicity
- Will need to decide on module structure as the project grows
- Consider organizing by feature or by experiment

### Development Approach
- Starting with simple examples and gradually adding complexity
- Focus on learning and demonstrating Bevy concepts
- Prioritize clear, well-documented code over advanced features

### Technical Choices
- Using Bevy's default plugins for standard functionality
- Using modern Bevy 0.15.3 APIs and avoiding deprecated features
- Component-based design for extensibility
- ECS architecture for all game objects and logic

## Project Insights

### Learning Focus
- Understanding Bevy's ECS implementation
- Learning input handling and movement systems
- Exploring sprite rendering and 2D graphics
- Gaining familiarity with Bevy's component system

### Challenges
- Setting up the development environment with proper graphics support
- Understanding Bevy's API changes and deprecations in version 0.15.3
- Implementing smooth movement with normalized vectors
- Running graphical applications in a container environment

### Patterns and Preferences
- Prefer explicit, well-documented code over clever shortcuts
- Use Bevy's component system for clean separation of concerns
- Follow Rust idioms and best practices
- Use normalized vectors for consistent movement speed in all directions
