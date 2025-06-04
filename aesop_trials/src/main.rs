use bevy::prelude::*;

mod core;
mod entities;
mod systems;
mod world;
mod ui;
mod plugins;

use crate::core::game::GamePlugin;
use crate::entities::player::PlayerPlugin;
use crate::world::dungeon_generator::DungeonGeneratorPlugin;
use crate::systems::collision::CollisionPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Aesop Trials".into(),
                resolution: (800.0, 600.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(GamePlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(DungeonGeneratorPlugin)
        .add_plugins(CollisionPlugin)
        .add_systems(Startup, || {
            info!("Aesop Trials starting up");
        })
        .run();
}