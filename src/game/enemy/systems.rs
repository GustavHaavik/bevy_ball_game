use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

use crate::constants::SAND;
use crate::game::enemy::components::*;
use crate::game::enemy::ENEMIES;
use crate::sprites::TileMap;
use crate::sprites::get_index;

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    sprite_sheet: Res<TileMap>,
) {
    let window = window_query.get_single().unwrap();

    let enemy_sprite = get_index(SAND).unwrap() as usize;

    for _ in 0..ENEMIES {
        let rand_x: f32 = random::<f32>() * window.width();
        let rand_y: f32 = random::<f32>() * window.height();

        commands.spawn((
            SpriteSheetBundle {
                transform: Transform::from_translation(Vec3::new(rand_x, rand_y, 0.0)),
                sprite: TextureAtlasSprite::new(enemy_sprite),
                texture_atlas: sprite_sheet.0.clone(),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &mut Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * 75.0 * time.delta_seconds();
    }
}
