use bevy::{prelude::*, window::PrimaryWindow};

use crate::game::sprites::{
    constants::ARCHER,
    resources::{get_index, KingdomSpriteSheet},
};

use super::{components::Player, PLAYER_SIZE, PLAYER_SPEED};

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    sprite_sheet: Res<KingdomSpriteSheet>,
) {
    let window = window_query.get_single().unwrap();

    let player_sprite = get_index(ARCHER).unwrap() as usize;

    commands.spawn((
        SpriteSheetBundle {
            transform: Transform::from_translation(Vec3::new(
                window.width() / 2.0,
                window.height() / 2.0,
                0.0,
            )),
            sprite: TextureAtlasSprite::new(player_sprite),
            texture_atlas: sprite_sheet.0.clone(),
            ..default()
        },
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
