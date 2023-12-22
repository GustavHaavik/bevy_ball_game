use bevy::prelude::*;

mod enemy;
mod player;
mod world;

pub use enemy::*;
pub use player::*;
pub use world::*;

pub struct GamePlugin;

pub const NOISE_SCALE: f32 = 15.0;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            // WorldPlugin,
            PlayerPlugin,
            // EnemyPlugin,
        ));
    }
}
