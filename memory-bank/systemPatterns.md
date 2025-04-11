# System Patterns: Bevy Playground

## Architecture Overview
The Bevy Playground is built on the Bevy game engine, which uses an Entity Component System (ECS) architecture. This document outlines the key architectural patterns and design decisions used in the project.

## ECS Architecture
The Entity Component System (ECS) is a software architectural pattern that follows composition over inheritance and is used extensively in game development:

1. **Entities**: Unique identifiers that represent game objects
2. **Components**: Data attached to entities (e.g., position, velocity, health)
3. **Systems**: Logic that operates on entities with specific components
4. **Resources**: Global data accessible by systems

### Current Implementation
The current implementation demonstrates basic ECS concepts:
- **Entities**: Created in the `add_people` system
- **Components**: `Person` (marker) and `Name` (data)
- **Systems**: `add_people`, `greet_people`, and `update_people`
- **Resources**: `GreetTimer` for periodic actions
- **Plugin**: `HelloPlugin` for organizing systems and resources

## Design Patterns

### Plugin Pattern
The project uses Bevy's plugin system to organize functionality:
```rust
pub struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (update_people, greet_people).chain());
    }
}
```

This pattern allows for:
- Modular organization of game features
- Clean separation of concerns
- Easy enabling/disabling of functionality
- Reusable components across projects

### System Scheduling
Systems are scheduled to run at specific times:
- **Startup**: Systems that run once when the app starts (e.g., `add_people`)
- **Update**: Systems that run every frame (e.g., `greet_people`, `update_people`)

Systems can also be chained to ensure execution order:
```rust
app.add_systems(Update, (update_people, greet_people).chain());
```

### Component Design
Components are designed to be:
- **Focused**: Each component has a single responsibility
- **Composable**: Entities can have multiple components
- **Data-oriented**: Components store data, not behavior

## Critical Implementation Paths

### Entity Creation
Entities are created using the `Commands` API:
```rust
fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}
```

### Query System
Data is accessed using Bevy's query system:
```rust
fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // ...
    for name in &query {
        println!("hello {}!", name.0);
    }
}
```

### Resource Management
Resources are used for global state:
```rust
#[derive(Resource)]
struct GreetTimer(Timer);

// Inserting a resource
app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));

// Accessing a resource
fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, ...) {
    if timer.0.tick(time.delta()).just_finished() {
        // ...
    }
}
```

## Future Architectural Considerations

As the playground evolves, consider:
1. **Scene organization**: How to structure multiple experiments/demos
2. **State management**: Using Bevy's state system for different application states
3. **Asset management**: Handling textures, models, sounds, etc.
4. **UI integration**: Adding debug UI for experiment parameters
5. **Event system**: Using Bevy's event system for decoupled communication
