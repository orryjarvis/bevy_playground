use bevy::prelude::*;
use bevy::window::WindowResolution;

// Component to mark our shape
#[derive(Component)]
struct Player;

// System to spawn a shape
fn setup(mut commands: Commands) {
    // Add a 2D camera
    commands.spawn(Camera2d);
    
    // Spawn a blue square
    commands.spawn((
        Player,
        Sprite {
            color: Color::srgb(0.5, 0.5, 1.0),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

// System to handle keyboard input and move the shape
fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut player_transform = query.single_mut();
    let mut direction = Vec3::ZERO;
    
    // Get input direction
    if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    
    // Normalize direction to prevent faster diagonal movement
    if direction != Vec3::ZERO {
        direction = direction.normalize();
    }
    
    // Move the player
    let speed = 300.0;
    player_transform.translation += direction * speed * time.delta_secs();
}

// This function is used for both native and web builds
pub fn run() {
    let mut app = App::new();
    
    // Configure window for better web experience
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(800.0, 600.0),
            title: "Bevy Playground".to_string(),
            ..default()
        }),
        ..default()
    }));
    
    app.add_systems(Startup, setup)
        .add_systems(Update, player_movement);
    
    app.run();
}

// This section enables wasm bindgen for the web build
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start() {
    // When building for WASM, print panics to the browser console
    console_error_panic_hook::set_once();
    run();
}
