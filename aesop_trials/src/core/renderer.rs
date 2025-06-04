use bevy::prelude::*;

/// Component for rendering a tile with a specific color
#[derive(Component)]
pub struct TileRenderer {
    pub color: Color,
}

/// System to render tiles with different colors
pub fn render_tiles(
    mut query: Query<(&TileRenderer, &mut Sprite)>,
) {
    for (renderer, mut sprite) in query.iter_mut() {
        sprite.color = renderer.color;
    }
}