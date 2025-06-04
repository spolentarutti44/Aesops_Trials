use bevy::prelude::*;
use rand::Rng;
use crate::world::tile::{Tile, TileType};

/// Struct representing a rectangular room
#[derive(Clone, Debug)]
pub struct Room {
    pub position: IVec2,
    pub size: IVec2,
}

impl Room {
    /// Create a new room with the given position and size
    pub fn new(position: IVec2, size: IVec2) -> Self {
        Self {
            position,
            size,
        }
    }
    
    /// Create a random room within the given constraints
    pub fn random(
        min_pos: IVec2,
        max_pos: IVec2,
        min_size: IVec2,
        max_size: IVec2,
    ) -> Self {
        let mut rng = rand::thread_rng();
        
        // Generate random size
        let width = rng.gen_range(min_size.x..=max_size.x);
        let height = rng.gen_range(min_size.y..=max_size.y);
        let size = IVec2::new(width, height);
        
        // Generate random position
        let x = rng.gen_range(min_pos.x..=(max_pos.x - size.x));
        let y = rng.gen_range(min_pos.y..=(max_pos.y - size.y));
        let position = IVec2::new(x, y);
        
        Self {
            position,
            size,
        }
    }
    
    /// Check if this room intersects with another room
    pub fn intersects(&self, other: &Room) -> bool {
        let self_end = self.position + self.size;
        let other_end = other.position + other.size;
        
        // Add a buffer of 1 tile to avoid rooms being directly adjacent
        !(self_end.x + 1 <= other.position.x || 
          self.position.x >= other_end.x + 1 || 
          self_end.y + 1 <= other.position.y || 
          self.position.y >= other_end.y + 1)
    }
    
    /// Get the center position of the room
    pub fn center(&self) -> IVec2 {
        self.position + self.size / 2
    }
    
    /// Generate tiles for this room
    pub fn generate_tiles(&self) -> Vec<(IVec2, Tile)> {
        let mut tiles = Vec::new();
        
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let grid_pos = self.position + IVec2::new(x, y);
                
                // Determine tile type (walls on the edges, floor inside)
                let tile_type = if x == 0 || y == 0 || x == self.size.x - 1 || y == self.size.y - 1 {
                    TileType::Wall
                } else {
                    TileType::Floor
                };
                
                tiles.push((grid_pos, Tile::new(tile_type, grid_pos)));
            }
        }
        
        tiles
    }
}