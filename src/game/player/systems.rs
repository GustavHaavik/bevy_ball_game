use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    constants::SAND,
    sprites::{get_index, get_texture_atlas, TileMap},
};

use super::{components::Player, resources::PlayerMovementSheet, PLAYER_SIZE, PLAYER_SPEED};

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    tilemap: Res<PlayerMovementSheet>,
) {
    let window = window_query.get_single().unwrap();

    let player_sprite = get_index(SAND).unwrap() as usize;

    commands.spawn((
        SpriteSheetBundle {
            transform: Transform::from_translation(Vec3::new(
                window.width() / 2.0,
                window.height() / 2.0,
                0.0,
            )),
            sprite: TextureAtlasSprite::new(player_sprite),
            texture_atlas: tilemap.0.clone(),
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

pub fn load_player_sprites(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let player_movement_handle = get_texture_atlas(
        "sprites/player_movement.png".to_string(),
        10,
        1,
        None,
        PLAYER_SIZE,
        PLAYER_SIZE,
        &mut texture_atlases,
        &assets_server,
    );

    commands.insert_resource(PlayerMovementSheet(player_movement_handle));
}
