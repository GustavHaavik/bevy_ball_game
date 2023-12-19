use bevy::prelude::*;

use super::{TILE_H, TILE_W, components::TileSprite};

#[derive(Resource)]
pub struct KingdomSpriteSheet(pub Handle<TextureAtlas>);

impl FromWorld for KingdomSpriteSheet {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        let texture_handle = asset_server.load("sprites/kingdom_spritsheet.png");

        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(TILE_W as f32, TILE_H as f32),
            12,
            11,
            Some(Vec2::new(1.0, 1.0)),
            None,
        );

        let mut texture_atlases = world.get_resource_mut::<Assets<TextureAtlas>>().unwrap();
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        Self(texture_atlas_handle)
    }
}

pub fn get_index(sprite: TileSprite) -> Result<i32, String> {
    let (col, row) = (sprite.column, sprite.row);

    if col > 11 || row > 10 {
        return Err(String::from("Invalid sprite index"));
    }

    Ok(row * 12 + col)
}
