use bevy::prelude::*;

pub mod components;
pub mod systems;

use systems::*;

pub const ENEMIES: u32 = 10;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemies)
            .add_systems(Update, enemy_movement);
    }
}
