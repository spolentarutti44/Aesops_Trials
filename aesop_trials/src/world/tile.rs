use bevy::prelude::*;

/// Enum representing different tile types
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TileType {
    Floor,
    Wall,
    Door,
    Empty,
}

/// Component for a tile in the grid
#[derive(Component, Clone)]
pub struct Tile {
    pub tile_type: TileType,
    pub grid_position: IVec2,
}

impl Tile {
    pub fn new(tile_type: TileType, grid_position: IVec2) -> Self {
        Self {
            tile_type,
            grid_position,
        }
    }
    
    /// Get the color for rendering based on tile type
    pub fn get_color(&self) -> Color {
        match self.tile_type {
            TileType::Floor => Color::rgb(0.5, 0.5, 0.5),  // Gray
            TileType::Wall => Color::rgb(0.3, 0.3, 0.3),   // Dark gray
            TileType::Door => Color::rgb(0.6, 0.4, 0.2),   // Brown
            TileType::Empty => Color::rgba(0.0, 0.0, 0.0, 0.0), // Transparent
        }
    }
    
    /// Check if the tile is walkable
    pub fn is_walkable(&self) -> bool {
        matches!(self.tile_type, TileType::Floor | TileType::Door)
    }
}