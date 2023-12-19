use bevy::{prelude::*, utils::hashbrown::HashSet};
use bracket_noise::prelude::{FastNoise, NoiseType};
use rand::Rng;

use crate::game::{
    sprites::{
        constants::{DIRT, SAND},
        resources::{get_index, KingdomSpriteSheet},
        SPRITE_SCALE_FACTOR, TILE_H, TILE_W,
    },
    world::components::Tile,
    NOISE_SCALE,
};

pub fn init_world(mut commands: Commands, sprite_sheet: Res<KingdomSpriteSheet>) {
    let mut noise: FastNoise = FastNoise::new();
    noise.set_seed(rand::thread_rng().gen());
    noise.set_noise_type(NoiseType::SimplexFractal);

    let (dirt_sprite, sand_sprite) = (get_index(DIRT).unwrap(), get_index(SAND).unwrap());

    let mut noise_map = HashSet::new();
    for x in 0..200 {
        for y in 0..100 {
            let noise_value = noise.get_noise(x as f32 / NOISE_SCALE, y as f32 / NOISE_SCALE);

            let sprite: i32 = if noise_value > 0.15 {
                dirt_sprite
            } else {
                sand_sprite
            };

            noise_map.insert(Tile {
                x: x as i32,
                y: y as i32,
                sprite,
            });
        }
    }

    for tile in noise_map.iter() {
        let (x, y) = grid_to_world(tile.x as f32, tile.y as f32);

        commands.spawn(SpriteSheetBundle {
            transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
            sprite: TextureAtlasSprite::new(tile.sprite as usize),
            texture_atlas: sprite_sheet.0.clone(),
            ..default()
        });
    }

    println!("World initialized!");
}

fn grid_to_world(x: f32, y: f32) -> (f32, f32) {
    (
        x * TILE_W as f32 * SPRITE_SCALE_FACTOR as f32,
        y * TILE_H as f32 * SPRITE_SCALE_FACTOR as f32,
    )
}
