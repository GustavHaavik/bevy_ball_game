use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use systems::*;

pub const PLAYER_SPEED: f32 = 150.0;
pub const PLAYER_SIZE: f32 = 32.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_player_sprites)
            .add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement)
            .add_systems(Update, confine_movement);
    }
}
