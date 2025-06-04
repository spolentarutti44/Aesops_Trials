use bevy::prelude::*;
use crate::core::game::GameState;
use crate::entities::player::Player;
use crate::world::dungeon_generator::{DungeonGrid, GRID_SIZE};
use crate::world::tile::Tile;

/// Plugin for collision detection
pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            check_collision.run_if(in_state(GameState::Playing)),
        );
    }
}

/// System to check for collisions between the player and walls
fn check_collision(
    dungeon_grid: Res<DungeonGrid>,
    mut player_query: Query<&mut Transform, With<Player>>,
    tile_query: Query<&Tile>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        // Convert player position to grid coordinates
        let player_grid_pos = IVec2::new(
            (player_transform.translation.x / GRID_SIZE).round() as i32,
            (player_transform.translation.y / GRID_SIZE).round() as i32,
        );
        
        // Check if the player is on a non-walkable tile
        if let Some(tile_entity) = dungeon_grid.tiles.get(&player_grid_pos) {
            if let Ok(tile) = tile_query.get(*tile_entity) {
                if !tile.is_walkable() {
                    // Find the nearest walkable tile
                    let mut nearest_walkable = None;
                    let mut min_distance = f32::MAX;
                    
                    // Check adjacent tiles
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            if dx == 0 && dy == 0 {
                                continue;
                            }
                            
                            let check_pos = player_grid_pos + IVec2::new(dx, dy);
                            
                            if let Some(check_entity) = dungeon_grid.tiles.get(&check_pos) {
                                if let Ok(check_tile) = tile_query.get(*check_entity) {
                                    if check_tile.is_walkable() {
                                        let world_pos = Vec3::new(
                                            check_pos.x as f32 * GRID_SIZE,
                                            check_pos.y as f32 * GRID_SIZE,
                                            player_transform.translation.z,
                                        );
                                        
                                        let distance = player_transform.translation.distance_squared(world_pos);
                                        
                                        if distance < min_distance {
                                            min_distance = distance;
                                            nearest_walkable = Some(world_pos);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    
                    // Move player to nearest walkable tile
                    if let Some(pos) = nearest_walkable {
                        player_transform.translation = pos;
                    }
                }
            }
        }
    }
}