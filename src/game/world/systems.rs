use bevy::{prelude::*, utils::hashbrown::HashSet};
use bracket_noise::prelude::{FastNoise, NoiseType};
use rand::Rng;

use crate::{
    game::{world::components::Tile, NOISE_SCALE},
    sprites::{get_index, TileMap, SPRITE_SCALE_FACTOR, TILE_H, TILE_W}, constants::{DIRT, WATER},
};

pub fn init_world(mut commands: Commands, tilemap: Res<TileMap>) {
    let mut noise: FastNoise = FastNoise::new();
    noise.set_seed(rand::thread_rng().gen());
    noise.set_noise_type(NoiseType::SimplexFractal);

    let mut noise_map = HashSet::new();
    for x in 0..200 {
        for y in 0..100 {
            let noise_value = noise.get_noise(x as f32 / NOISE_SCALE, y as f32 / NOISE_SCALE);

            let sprite: usize = if noise_value > 0.15 {
                get_index(DIRT).unwrap() as usize
            } else {
                get_index(WATER).unwrap() as usize
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
            transform: Transform {
                translation: Vec3::new(x, y, 0.0),
                scale: Vec3::splat(4.0),
                ..default()
            },
            texture_atlas: tilemap.0.clone(),
            sprite: TextureAtlasSprite::new(tile.sprite),
            ..default()
        });
        // commands.spawn(SpriteBundle {
        //     transform: Transform {
        //         translation: Vec3::new(x, y, 0.0),
        //         scale: Vec3::splat(4.0),
        //         ..default()
        //     },
        //     texture: tile.sprite.clone(),
        //     ..default() // sprite: TextureAtlasSprite::new(tile.sprite),
        //                 // texture_atlas: atlas_handle.clone(),
        // });
    }

    println!("World initialized!");
}

fn grid_to_world(x: f32, y: f32) -> (f32, f32) {
    (
        x * TILE_W as f32 * SPRITE_SCALE_FACTOR as f32,
        y * TILE_H as f32 * SPRITE_SCALE_FACTOR as f32,
    )
}
