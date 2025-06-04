use bevy::prelude::*;

/// Resource for tracking player input
#[derive(Resource, Default)]
pub struct PlayerInput {
    pub movement: Vec2,
    pub action_pressed: bool,
}

/// System to handle keyboard input
pub fn handle_keyboard_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_input: ResMut<PlayerInput>,
) {
    // Reset movement
    player_input.movement = Vec2::ZERO;
    
    // Handle movement input (WASD or arrow keys)
    if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
        player_input.movement.y = 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
        player_input.movement.y = -1.0;
    }
    if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
        player_input.movement.x = -1.0;
    }
    if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
        player_input.movement.x = 1.0;
    }
    
    // Normalize diagonal movement
    if player_input.movement.length_squared() > 1.0 {
        player_input.movement = player_input.movement.normalize();
    }
    
    // Handle action input (Space or E)
    player_input.action_pressed = keyboard_input.just_pressed(KeyCode::Space) || 
                                 keyboard_input.just_pressed(KeyCode::E);
}