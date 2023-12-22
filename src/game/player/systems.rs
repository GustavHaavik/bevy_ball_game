use bevy::{prelude::*, window::PrimaryWindow};

use crate::sprites::{get_index, AnimationIndices, AnimationTimer, SpritesSheet, TileSprite};

use super::{components::Player, PLAYER_SIZE, PLAYER_SPEED};

struct PlayeRunningSprites {
    first: TileSprite,
    last: TileSprite,
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    sprite_sheet: Res<SpritesSheet>,
) {
    let window = window_query.get_single().unwrap();

    let player_sprite = PlayeRunningSprites {
        first: TileSprite { column: 0, row: 5 },
        last: TileSprite { column: 5, row: 5 },
    };

    let first_index = get_index(player_sprite.first, 28);
    let last_index = get_index(player_sprite.last, 28);

    let animation_indices = AnimationIndices {
        first: first_index as usize,
        last: last_index as usize,
    };

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: sprite_sheet.0.clone(),
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform::from_translation(Vec3::new(
                window.width() / 2.0,
                window.height() / 2.0,
                0.0,
            )).with_scale(Vec3::splat(5.0)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Player {},
    ));
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::W) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::A) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction.x += 1.0;
        }

        if direction != Vec3::ZERO {
            transform.translation += direction.normalize() * PLAYER_SPEED * time.delta_seconds();
        }
    }
}

pub fn confine_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        if transform.translation.x < PLAYER_SIZE / 2.0 {
            transform.translation.x = PLAYER_SIZE / 2.0;
        }
        if transform.translation.x > window.width() - PLAYER_SIZE / 2.0 {
            transform.translation.x = window.width() - PLAYER_SIZE / 2.0;
        }

        if transform.translation.y < PLAYER_SIZE / 2.0 {
            transform.translation.y = PLAYER_SIZE / 2.0;
        }

        if transform.translation.y > window.height() - PLAYER_SIZE / 2.0 {
            transform.translation.y = window.height() - PLAYER_SIZE / 2.0;
        }
    }
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}