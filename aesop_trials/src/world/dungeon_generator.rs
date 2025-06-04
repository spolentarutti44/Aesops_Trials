use bevy::prelude::*;
use rand::Rng;
use std::collections::HashMap;

use crate::core::game::GameState;
use crate::world::room::Room;
use crate::world::tile::{Tile, TileType};

/// Constants for dungeon generation
pub const GRID_SIZE: f32 = 32.0;
const DUNGEON_WIDTH: i32 = 30;
const DUNGEON_HEIGHT: i32 = 30;
const MIN_ROOM_SIZE: IVec2 = IVec2::new(5, 5);
const MAX_ROOM_SIZE: IVec2 = IVec2::new(10, 10);
const MAX_ROOMS: usize = 10;

/// Resource to store the dungeon grid
#[derive(Resource)]
pub struct DungeonGrid {
    pub tiles: HashMap<IVec2, Entity>,
    pub rooms: Vec<Room>,
}

impl Default for DungeonGrid {
    fn default() -> Self {
        Self {
            tiles: HashMap::new(),
            rooms: Vec::new(),
        }
    }
}

/// Plugin for dungeon generation
pub struct DungeonGeneratorPlugin;

impl Plugin for DungeonGeneratorPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<DungeonGrid>()
            .add_systems(OnEnter(GameState::Playing), generate_dungeon);
    }
}

/// System to generate a simple dungeon with rooms
fn generate_dungeon(
    mut commands: Commands,
    mut dungeon_grid: ResMut<DungeonGrid>,
) {
    // Clear any existing dungeon
    dungeon_grid.tiles.clear();
    dungeon_grid.rooms.clear();
    
    // Generate rooms
    let mut rng = rand::thread_rng();
    let mut attempts = 0;
    
    while dungeon_grid.rooms.len() < MAX_ROOMS && attempts < 100 {
        // Create a random room
        let room = Room::random(
            IVec2::new(1, 1),
            IVec2::new(DUNGEON_WIDTH - 1, DUNGEON_HEIGHT - 1),
            MIN_ROOM_SIZE,
            MAX_ROOM_SIZE,
        );
        
        // Check if it overlaps with existing rooms
        let mut overlaps = false;
        for existing_room in &dungeon_grid.rooms {
            if room.intersects(existing_room) {
                overlaps = true;
                break;
            }
        }
        
        if !overlaps {
            // Add room to the list
            dungeon_grid.rooms.push(room);
        }
        
        attempts += 1;
    }
    
    // Generate tiles for each room
    let rooms = dungeon_grid.rooms.clone();
    for room in &rooms {
        for (grid_pos, tile) in room.generate_tiles() {
            // Convert grid position to world position
            let world_pos = Vec3::new(
                grid_pos.x as f32 * GRID_SIZE,
                grid_pos.y as f32 * GRID_SIZE,
                0.0,
            );
            
            // Spawn tile entity
            let entity = commands.spawn((
                tile.clone(),
                SpriteBundle {
                    sprite: Sprite {
                        color: tile.get_color(),
                        custom_size: Some(Vec2::new(GRID_SIZE, GRID_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_translation(world_pos),
                    ..default()
                },
            )).id();
            
            // Store tile in the grid
            dungeon_grid.tiles.insert(grid_pos, entity);
        }
    }
    
    // Connect rooms with corridors (simple implementation)
    if rooms.len() > 1 {
        for i in 0..rooms.len() - 1 {
            let room1_center = rooms[i].center();
            let room2_center = rooms[i + 1].center();
            
            create_corridor_between_rooms(&mut commands, &mut dungeon_grid, room1_center, room2_center);
        }
    }
    
    // Debug info
    info!("Generated {} rooms", rooms.len());
    info!("Generated {} tiles", dungeon_grid.tiles.len());
}

/// Create a simple L-shaped corridor between two points
fn create_corridor_between_rooms(
    commands: &mut Commands,
    dungeon_grid: &mut DungeonGrid,
    start: IVec2,
    end: IVec2,
) {
    // Determine the corner point for the L-shape
    let corner = IVec2::new(start.x, end.y);
    
    // Create horizontal corridor
    let x_start = start.x.min(corner.x);
    let x_end = start.x.max(corner.x);
    
    for x in x_start..=x_end {
        create_corridor_tile(commands, dungeon_grid, IVec2::new(x, start.y));
    }
    
    // Create vertical corridor
    let y_start = corner.y.min(end.y);
    let y_end = corner.y.max(end.y);
    
    for y in y_start..=y_end {
        create_corridor_tile(commands, dungeon_grid, IVec2::new(corner.x, y));
    }
}

/// Create a single corridor tile
fn create_corridor_tile(
    commands: &mut Commands,
    dungeon_grid: &mut DungeonGrid,
    grid_pos: IVec2,
) {
    // Skip if there's already a floor tile here
    if dungeon_grid.tiles.contains_key(&grid_pos) {
        return;
    }
    
    // Create a floor tile
    let tile = Tile::new(TileType::Floor, grid_pos);
    
    // Convert grid position to world position
    let world_pos = Vec3::new(
        grid_pos.x as f32 * GRID_SIZE,
        grid_pos.y as f32 * GRID_SIZE,
        0.0,
    );
    
    // Spawn tile entity
    let entity = commands.spawn((
        tile.clone(),
        SpriteBundle {
            sprite: Sprite {
                color: tile.get_color(),
                custom_size: Some(Vec2::new(GRID_SIZE, GRID_SIZE)),
                ..default()
            },
            transform: Transform::from_translation(world_pos),
            ..default()
        },
    )).id();
    
    // Store tile in the grid
    dungeon_grid.tiles.insert(grid_pos, entity);
}