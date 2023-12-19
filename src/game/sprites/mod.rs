use self::resources::KingdomSpriteSheet;
use bevy::prelude::*;

pub const TILE_W: usize = 16;
pub const TILE_H: usize = 16;

pub const SPRITE_SCALE_FACTOR: usize = 2;

pub mod resources;
pub mod components;
pub mod constants;

pub struct SpritesPlugin;

impl Plugin for SpritesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<KingdomSpriteSheet>();
    }
}
