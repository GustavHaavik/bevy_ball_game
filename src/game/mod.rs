use bevy::prelude::*;

mod enemy;
mod player;
mod sprites;
mod world;

pub use enemy::*;
pub use player::*;
pub use sprites::*;
pub use world::*;

pub struct GamePlugin;

pub const NOISE_SCALE: f32 = 10.0;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SpritesPlugin, WorldPlugin, PlayerPlugin));
    }
}
