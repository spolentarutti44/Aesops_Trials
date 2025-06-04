use bevy::prelude::*;
use crate::core::game::GameState;
use crate::core::input::{PlayerInput, handle_keyboard_input};

/// Component to mark an entity as the player
#[derive(Component)]
pub struct Player;

/// Component for player stats
#[derive(Component)]
pub struct PlayerStats {
    pub health: f32,
    pub max_health: f32,
    pub speed: f32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            health: 100.0,
            max_health: 100.0,
            speed: 100.0,
        }
    }
}

/// Plugin for player-related systems
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PlayerInput>()
            .add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(
                Update,
                (
                    handle_keyboard_input,
                    player_movement,
                )
                .chain()
                .run_if(in_state(GameState::Playing)),
            );
    }
}

/// Spawn the player entity
fn spawn_player(mut commands: Commands) {
    // Spawn player with a simple square sprite
    commands.spawn((
        Player,
        PlayerStats::default(),
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.5, 0.9),
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
            ..default()
        },
    ));
    
    info!("Player spawned");
}

/// System to handle player movement based on input
fn player_movement(
    time: Res<Time>,
    player_input: Res<PlayerInput>,
    mut query: Query<(&PlayerStats, &mut Transform), With<Player>>,
) {
    if let Ok((stats, mut transform)) = query.get_single_mut() {
        let movement = player_input.movement * stats.speed * time.delta_seconds();
        transform.translation.x += movement.x;
        transform.translation.y += movement.y;
        
        if movement != Vec2::ZERO {
            // Only log when actually moving
            debug!("Player moved: {:?}", movement);
        }
    }
}